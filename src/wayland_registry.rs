pub struct WaylandRegistry{
    obj_id: u32
}

impl WaylandRegistry{
    pub fn new(new_id: u32) -> WaylandRegistry{
        WaylandRegistry{obj_id: new_id}
    }
}