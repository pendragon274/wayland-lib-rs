use crate::{
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl},
    wayland_sock::{
        WaylandSockMsg,
        WaylandSockWriteBuffer}};

pub struct WaylandSHM{
    id: u32,
    children: Vec<WaylandObject>,
    sock: WaylandSockWriteBuffer
}

impl WaylandSHM{
    // ***** Public Functions *****
    // ***** Private Functions *****
    fn respond_to_msg(&mut self, _msg: WaylandSockMsg){
        //println!("SHM got a message: {:?}", msg.message());
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32, new_sock: WaylandSockWriteBuffer) -> WaylandSHM{
        println!("Creating WaylandSHM object with id: {}", new_id);
        WaylandSHM{
            id: new_id,
            children: Vec::new(),
            sock: new_sock
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

    fn get_child(&mut self, child_id: u32) -> Option<&mut WaylandObject> {
        for child in self.children.iter_mut(){
            if child.get_id() == child_id{
                return Some(child);
            }else if let Some(internal_child) = child.get_child(child_id){
                return Some(internal_child);
            }
        }

        None
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