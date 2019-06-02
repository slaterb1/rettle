use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub use super::tea::Tea;
pub use super::ingredient::{Ingredient, Steep, Pour};

enum MakeTea {
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

type Order = Box<FnBox + Send + 'static>;

pub struct Brewery {
    brewers: Vec<Brewer>,
    sender: mpsc::Sender<MakeTea>
}

impl Brewery {
    pub fn new(size: usize) -> Brewery {
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
        }
    }

    pub fn take_order<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let order = Box::new(f);

        self.sender
            .send(MakeTea::NewOrder(order))
            .unwrap()
    }

}

/// Worker that runs the recipe and brew tea.
struct Brewer {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Brewer {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<MakeTea>>>) -> Brewer {
        let thread = thread::spawn(move || {
            loop {
                let make_tea = receiver.lock()
                    .unwrap()
                    .recv()
                    .unwrap();

                match make_tea {
                    MakeTea::NewOrder(order) => {
                        println!("Brewer {} received order! Executing...", id);
                        order.call_box();
                    },
                    MakeTea::Terminate => {
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
pub fn make_tea(mut tea: Box<dyn Tea + Send>, recipe: &Vec<Box<dyn Ingredient + Send + Sync>>) {
    for step in recipe.iter() {
        step.print();
        if let Some(steep) = step.as_any().downcast_ref::<Steep>() {
            println!("Steep operation!");
            tea = steep.exec(&tea);
        } else if let Some(pour) = step.as_any().downcast_ref::<Pour>() {
            println!("Pour operation!");
            tea = pour.exec(&tea);
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


