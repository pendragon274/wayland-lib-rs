use crate::{
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl},
    wayland_sock::{
        WaylandSockMsg}};

pub struct WaylandSHM{
    id: u32,
    children: Vec<WaylandObject>
}

impl WaylandSHM{
    // ***** Public Functions *****
    // ***** Private Functions *****
    fn respond_to_msg(&mut self, _msg: WaylandSockMsg){
        //println!("SHM got a message: {:?}", msg.message());
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandSHM{
        println!("Creating WaylandSHM object with id: {}", new_id);
        WaylandSHM{
            id: new_id,
            children: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandSHM{
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