use crate::wayland_object::{WaylandObject, WaylandObjectImpl};
use crate::wayland_sock::WaylandSockMsg;

pub struct WaylandCompositor{
    id: u32,
    children: Vec<WaylandObject>
}

impl WaylandCompositor{
    // ***** Public Functions *****
    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("Compositor received event: {}", msg);
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandCompositor{
        WaylandCompositor{
            id: new_id,
            children: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandCompositor{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        false
    }

    fn get_children(&mut self) -> Vec<&mut WaylandObject> {
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