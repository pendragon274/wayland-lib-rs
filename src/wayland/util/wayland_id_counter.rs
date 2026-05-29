//TODO: Should be a far more sophisticated id manager that makes use of freed id's

use std::{
    rc::Rc,
    cell::RefCell};

#[derive(Clone)]
pub struct WaylandIDCounter{
    id_counter_internal: Rc<RefCell<WaylandIDCounterInternal>>
}

impl WaylandIDCounter {
    pub fn get_new_id(&self) -> u32{
        self.id_counter_internal.borrow_mut().get_new_id()
    }
    
    pub fn new() -> WaylandIDCounter {
        WaylandIDCounter{
            id_counter_internal: Rc::new(RefCell::new(WaylandIDCounterInternal::new()))
        }
    }
}

struct WaylandIDCounterInternal{
    current_id: u32
}

impl WaylandIDCounterInternal{
    pub fn get_new_id(&mut self) -> u32{
        let ret = self.current_id;
        self.current_id += 1;
        ret
    }
    
    pub fn new() -> WaylandIDCounterInternal{
        WaylandIDCounterInternal{
            current_id: 1
        }
    }
}