//TODO: Should be a far more sophisticated id manager that makes use of freed id's

pub struct WaylandIDCounter{
    current_id: u32
}

impl WaylandIDCounter {
    pub fn get_new_id(&mut self) -> u32{
        let ret = self.current_id;
        self.current_id += 1;
        ret
    }
    
    pub fn new() -> WaylandIDCounter {
        WaylandIDCounter{
            current_id: 1
        }
    }
}