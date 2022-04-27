
// For boxes
// For songular event

pub mod eo {
    
    use std::sync::Mutex;
    use std::sync::mpsc::*;
    use std::thread;
    use std::sync::Arc;
    use std::time::Duration;
    use std::cell::RefCell;
    use std::rc::Rc;

   //pub struct state(Sender<bool>, Receiver<bool>);
   
    pub type comunicator = (Sender<bool>, Arc<Mutex<Receiver<bool>>>);
    
    #[derive(Debug)]
    pub struct event<'a, T: Fn() + Send + Sync> {
        name: &'a str,
        action: Arc<T>,
        comm: comunicator,
        handle: Option<thread::JoinHandle<()>>, 
    }

    pub trait Mevent {
        fn on(&mut self) -> ();

        fn suscribe(&self) -> ();
    }

    impl<'a, T: 'static> event<'a, T>
        where T: Fn() + Send + Sync,
              {
                  pub fn new(name: &'a str, f: T) -> Result<Box<dyn Mevent + 'a>, bool> {
                      let (sender, receiver) = channel();
                      let receiber = Arc::new(Mutex::new(receiver));
                      //println!("Counter receiber: {}", Arc::strong_count(&receiber));
                      Ok(Box::new(Self {
                          name,
                          action: Arc::new(f),
                          comm: (sender, receiber),
                          handle: None,
                      }))
                  }
              }

    impl<'a, T: 'static> Mevent for event<'a, T>
        where
            T: Fn() + Send + Sync,
        {
            fn on(&mut self) -> () {
                
                let res = Arc::clone(&self.comm.1);
                //println!("Counter reciever: {}", Arc::strong_count(&res));
                
                let action = Arc::clone(&self.action); // creamos otro propietario para usarlo en el thread
                //action();
                //println!("Action has {} ownerships", Arc::strong_count(&self.action));
                
                //let op = Rc::clone(&self.handle);
                //println!("Has {} ownerships, {:?}", Rc::strong_count(&op), op);
                self.handle = Some(thread::spawn(move || {
                   let pes = res.lock().unwrap();
                   let response_execute_instr = pes.recv().unwrap(); // obtener resultado
                   match response_execute_instr {
                       true => {
                            action();
                       },
                       _ => println!("Building this..."),
                   }
                }));
                

            }

            fn suscribe(&self) -> () {
                self.comm.0.send(true).unwrap();
            }

        
        }            
    }
