use crate::tea::Tea;
use crate::ingredient::{Ingredient, Steep, Pour};

use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

enum OrderTea {
    NewOrder(Order),
    Terminate
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

type Order = Box<dyn FnBox + Send + 'static>;

pub struct Brewery {
    brewers: Vec<Brewer>,
    sender: mpsc::Sender<OrderTea>,
    start_time: Instant,
}

impl Brewery {
    pub fn new(size: usize, start_time: Instant) -> Brewery {
        assert!(size > 0);

        let (sender, plain_rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(plain_rx));

        let mut brewers = Vec::with_capacity(size);
        for id in 0 .. size {
            brewers.push(Brewer::new(id, Arc::clone(&rx)));
        }

        Brewery {
            brewers,
            sender,
            start_time,
        }
    }

    pub fn take_order<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let order = Box::new(f);

        self.sender
            .send(OrderTea::NewOrder(order))
            .unwrap();
    }

    pub fn get_brewer_info(&self) {
        println!("Number of brewers: {}", &self.brewers.len());
    }

}

impl Drop for Brewery {
  fn drop(&mut self) {
    println!("Sending terminate message to all brewers.");

    for _ in &mut self.brewers {
      self.sender.send(OrderTea::Terminate).unwrap();
    }

    for brewer in &mut self.brewers {
      println!("\tLetting go brewer {}", brewer.id);

      if let Some(thread) = brewer.thread.take() {
        thread.join().unwrap();
      }
    }
    println!("Elapsed time: {} ms", self.start_time.elapsed().as_millis());
  }
}

/// Worker that runs the recipe and brew tea.
struct Brewer {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Brewer {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<OrderTea>>>) -> Brewer {
        let thread = thread::spawn(move || {
            loop {
                let make_tea = receiver.lock()
                    .unwrap()
                    .recv()
                    .unwrap();

                match make_tea {
                    OrderTea::NewOrder(order) => {
                        // TODO: Change this to DEBUG logs/
                        //println!("Brewer {} received order! Executing...", id);
                        order.call_box();
                    },
                    OrderTea::Terminate => {
                        println!("Brewer {} was let go...", id);
                        break;
                    }
                }
            }
        });

        Brewer { 
            id, 
            thread: Some(thread),
        }
    }
}

///
/// This function is passed to the brewer via a thread for it to process the tea.
pub fn make_tea(mut tea: Vec<Box<dyn Tea + Send>>, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>) {
    let recipe = recipe.read().unwrap();
    for step in recipe.iter() {
        if let Some(steep) = step.as_any().downcast_ref::<Steep>() {
            tea = steep.exec(tea);
        } else if let Some(pour) = step.as_any().downcast_ref::<Pour>() {
            tea = pour.exec(tea);
        }
    }
}

// TODO: implement Debug for Box<dyn Ingredient>
// impl<'a> fmt::Debug for Brewer<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, 
//                "Brewer {{ steps: {:?}, tea: {:?} }}", 
//                self.steps.iter().map(|step| &*step), 
//                self.tea
//                )
//     } 
// }

#[cfg(test)]
mod tests {
    use super::Brewery;
    use super::super::tea::Tea;
    use std::time::Instant;
    use std::any::Any;

    #[derive(Debug, PartialEq, Default)]
    struct TestTea {
        x: i32,
    }

    impl Tea for TestTea {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn create_brewery_with_brewers() {
        let brewery = Brewery::new(4, Instant::now());
        assert_eq!(brewery.brewers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn create_brewery_with_no_brewers() {
        let _brewery = Brewery::new(0, Instant::now());
    }

    //TODO figure out how to properly test threads
    //#[test]
    //fn brewery_sends_job_done_channel() {
    //    let brewery = Brewery::new(4, Instant::now());
    //    let tea = TestTea::new(Box::new(TestTea::default()));
    //    brewery.take_order(|| {
    //        make_tea(tea, recipe);
    //    });
    //}
}
