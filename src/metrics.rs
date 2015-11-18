use std::collections::HashMap;
use config::*;

pub trait IMetrics {

}

pub struct RustMetr {
    counter: i32,
    item: i32,
    //config: Config,
    items: HashMap<String, i32>,
    counters: HashMap<String, i32>,
    vec: Vec<String>,
    limit: usize,

}

impl RustMetr {
    fn new(&self) -> RustMetr {
        if self.limit <= 0 {
             RustMetr{
                 items: HashMap::new(),
                 counters: HashMap::new(),
                 vec: Vec::new(), 
                 item: 0,
                 limit:self.limit,
                 counter: 0
             }

        } else {
             RustMetr{
                items: HashMap::with_capacity(self.limit),
                counters: HashMap::with_capacity(self.limit),
                vec: Vec::new(), 
                item: 0,
                limit:self.limit,
                counter: 0

          }

        }
    }
    fn event(&self, msg: String, count:i32) -> i32 {
       1 
    }

    //events of several items
    //return number of events in transaction
    fn events(&self, items: Vec<String>) -> i32 {
        if self.vec.len() == 0 {
            return 0
        }

        let expected = self.vec.len();

        1
    }

    fn transaction(items: Vec<Vec<i32>>) -> Option<i32> {
        Some(1)
    }

    fn increment(&mut self, msg: String) -> Option<i32> {
        if msg == "" {
            return None
        }

        match self.counters.get(&msg) {
            Some(x) => {
                let mut hmnew = HashMap::new();
                hmnew.insert(msg, 1);
                //hmnew.get_mut(&self.counters);
                //self.counters.insert(msg, 1);
                return Some(1)
            }
            None => {
                return None
            }
        }
        Some(1)
    }
}


