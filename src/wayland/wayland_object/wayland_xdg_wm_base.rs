use crate::wayland_object::{WaylandObject, WaylandObjectImpl};
use crate::wayland_sock::WaylandSockMsg;

pub struct WaylandXDGWMBase{
    id: u32,
    children: Vec<WaylandObject>
}

impl WaylandXDGWMBase{
    // ***** Public Functions *****
    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("xdg_wm_base received event: {}", msg);
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandXDGWMBase{
        WaylandXDGWMBase{
            id: new_id,
            children: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandXDGWMBase{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        false
    }

    fn get_child(&mut self, _child_id: u32) -> Option<&mut WaylandObject> {
        todo!()
    }

    fn msg_downstream(&mut self, msg: WaylandSockMsg) {
        if msg.message_id() == self.get_id(){
            self.respond_to_msg(msg);
        }else{
            for child in self.children.iter_mut(){
                child.msg_downstream(msg.clone());
            }
        }
    }

    fn rcv_upstream_msg(&mut self) -> Vec<WaylandSockMsg> {
        todo!()
    }
}