use crate::wayland_object::{WaylandObject, WaylandObjectImpl};
use crate::wayland_sock::WaylandSockMsg;

pub struct WaylandCompositor{
    _id: u32
}

impl WaylandCompositor{
    // ***** Public Functions *****
    // ***** Private Functions *****
    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandCompositor{
        WaylandCompositor{
            _id: new_id
        }
    }
}

impl WaylandObjectImpl for WaylandCompositor{
    fn get_id(&self) -> u32 {
        todo!()
    }

    fn is_upstream_flagged(&self) -> bool {
        false
    }

    fn get_children(&mut self) -> Vec<&mut WaylandObject> {
        todo!()
    }

    fn msg_downstream(&mut self, _msg: WaylandSockMsg) {
        todo!()
    }

    fn rcv_upstream_msg(&mut self) -> Vec<WaylandSockMsg> {
        todo!()
    }
}