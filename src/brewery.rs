use crate::ingredient::{Ingredient, Steep, Skim, Pour};

use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

/// Types of instructions that can be sent to Brewers.
enum OrderTea {
    NewOrder(Order),
    Terminate
}

/// Wrapper to allow sent function in Box to be invokable.
trait FnBox {
    /// Method to call inner function.
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    /// Calls inner function in box.
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

/// Type representing the brew function to be implemented on Tea batch with Recipe.
type Order = Box<dyn FnBox + Send + 'static>;

/// Struct holding the Array of Brewers and sender to push Tea Orders out to them.
pub struct Brewery {
    brewers: Vec<Brewer>,
    sender: mpsc::Sender<OrderTea>,
    start_time: Instant,
}

impl Brewery {
    ///
    /// Creates new Brewery with Brewers and sender/receiver pair for passing jobs to them.
    ///
    /// # Arguments
    ///
    /// * `size` - number of brewers to instantiate
    /// * `start_time` - program start time to expose runtime metrics
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

    ///
    /// Send function (job) with batch of Tea with Recipe to Brewers.
    ///
    /// # Arguments
    ///
    /// * `f` - function to send off to Brewers
    pub fn take_order<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let order = Box::new(f);

        self.sender
            .send(OrderTea::NewOrder(order))
            .unwrap();
    }

    ///
    /// Get info method to display number of Brewers assigned to Brewery.
    pub fn get_brewer_info(&self) {
        println!("Number of brewers: {}", &self.brewers.len());
    }

}

impl Drop for Brewery {
    fn drop(&mut self) {
        // After all jobs are sent terminate message is sent to close out worker pool.
        println!("Sending terminate message to all brewers.");

        for _ in &mut self.brewers {
            self.sender.send(OrderTea::Terminate).unwrap();
        }

        // Run any jobs that have not yet been completed before killing worker.
        for brewer in &mut self.brewers {
            println!("\tLetting go brewer {}", brewer.id);

            if let Some(thread) = brewer.thread.take() {
                thread.join().unwrap();
            }
        }

        // Print out run time metrics.
        println!("Elapsed time: {} ms", self.start_time.elapsed().as_millis());
    }
}

///
/// Worker that runs the Recipe and brews the batch of Tea.
struct Brewer {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Brewer {
    ///
    /// Create Brewer worker with receiver to fetch and process Order jobs.
    ///
    /// # Arguments
    ///
    /// * `id` - brewer number assigned.
    /// * `reciever` - receiver clone to receive jobs on.
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
/// This function is passed to the brewer via a thread for it to process the batch of Tea.
///
/// # Arguments
///
/// * `tea_batch` - Array of Tea structs to be processed
/// * `recipe` - read only clone of recipe containing all steps
pub fn make_tea<T: Send + 'static>(mut tea_batch: Vec<T>, recipe: Arc<RwLock<Vec<Box<dyn Ingredient<T> + Send + Sync>>>>) {
    let recipe = recipe.read().unwrap();
    // TODO: In the future, Fill will become a valid step in the recipe. For simplicity, this is
    // excluded at this stage in the project.
    // TODO: In the future, Tranfuse will become a valid step in the recipe. The Ingredient does not currently
    // exist, and additional logic may need to be introduced to handle how things are combined.
    for step in recipe.iter() {
        if let Some(steep) = step.as_any().downcast_ref::<Steep<T>>() {
            tea_batch = steep.exec(tea_batch);
        } else if let Some(skim) = step.as_any().downcast_ref::<Skim<T>>() {
            tea_batch = skim.exec(tea_batch);
        } else if let Some(pour) = step.as_any().downcast_ref::<Pour<T>>() {
            tea_batch = pour.exec(tea_batch);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Brewery;
    use std::time::Instant;

    #[derive(Debug, PartialEq, Default)]
    struct TestTea {
        x: i32,
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
