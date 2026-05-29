use crate::wayland_object::{WaylandObject, WaylandObjectImpl};
use crate::wayland_sock::{WaylandSockMsg, WaylandSockWriteBuffer};

pub struct XDGTopLevel{
    id: u32,
    sock: WaylandSockWriteBuffer,
    children: Vec<WaylandObject>
}

impl XDGTopLevel {
    // ***** Public Functions *****

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("XDGTopLevel received message: {}", msg);
    }

    // ***** Struct Init *****
    pub fn new(new_id: u32, new_sock: WaylandSockWriteBuffer) -> XDGTopLevel {
        XDGTopLevel {
            id: new_id,
            sock: new_sock,
            children: Vec::new()
        }
    }
}

impl WaylandObjectImpl for XDGTopLevel {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        todo!()
    }

    fn get_child(&mut self, child_id: u32) -> Option<&mut WaylandObject> {
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