use std::collections::HashMap;

pub trait IMetrics {

}

pub struct Rustmetr {
    counter: i32,
    item: i32,
    config: Config,
    items: HashMap,
    counters: HashMap,
    vec: Vec<string>,
    limit: i32,

}

impl RustMetr {
    fn new(&self) -> Rustmetr {
        if limit <= 0 {
             Rustmetr{
                 items: HashMap::new(),
                 counters: HashMap::new(),
                 vec: Vec::new(), 
             }

        } else {
             Rustmetr{
                items: HashMap::with_capacity(limit),
                counters: HashMap::with_capacity(limit),
                vec: Vec::new(), 
          }

        }
    }
    fn event(&self, msg: string, count:i32) -> i32 {
        
    }

    //events of several items
    //return number of events in transaction
    fn events(&self, items: Vec<String>) -> i32 {
        if self.vec.len() == 0 {
            0
        }

        let expected = vec.len();

        1
    }

    fn transaction(items: Vec<Vec<i32>>) -> Option<i32> {

    }

    fn increment(&self, msg: string) -> Option<i32> {
        if msg == "" {
            None
        }

        match self.counter.get(msg) {
            Some(x) => {
                self.counter.insert(msg, x+1);
                Some(1)
            }
            None => None
        }
        Some(1)
    }
}


