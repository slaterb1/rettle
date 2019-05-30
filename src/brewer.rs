use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub use super::tea::Tea;
pub use super::ingredient::{Ingredient, Steep, Pour};

struct Components<'a> {
    tea: Box<dyn Tea>,
    recipe: &'a Vec<Box<dyn Ingredient<'a>>>,
}

struct Brewery<'a> {
    brewers: Vec<Brewer>,
    sender: mpsc::Sender<Components<'a>>
}

impl<'a> Brewery<'a> {
    pub fn new(size: usize) -> Brewery<'a> {
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
}

/// Worker that runs the recipe and brew tea.
struct Brewer {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    tea: Box<dyn Tea>,
}

impl Brewer {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Components>>>) -> Brewer {
        let thread = thread::spawn(move || {
            loop {
                let components = receiver.lock()
                    .unwrap()
                    .recv()
                    .unwrap();

                self.update_brew(components.tea);
                self.make_tea(components.recipe);
            }
        });

        Brewer { 
            id, 
            thread: Some(thread),
        }
    }
    pub fn get_tea(&self) -> &Box<dyn Tea> {
        &self.tea
    }
    fn update_brew(&mut self, tea: Box<dyn Tea>) {
        self.tea = tea;
    }
    ///
    /// This function iterates over the brewer's steps to produce the final tea.
    pub fn make_tea(&mut self, recipe: &Vec<Box<dyn Ingredient>>) {
        // Save initial state of tea in brewer
        for step in recipe.iter() {
            step.print();
            if let Some(steep) = step.as_any().downcast_ref::<Steep>() {
                println!("Steep operation!");
                let tea = steep.exec(self.get_tea());
                self.update_brew(tea);
            } else if let Some(pour) = step.as_any().downcast_ref::<Pour>() {
                println!("Pour operation!");
                let _tea = pour.exec(self.get_tea());
            }
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


