use crate::prelude::{WaylandObject, WaylandSockMsg};
use crate::wayland_object::{WaylandObjectImpl};

pub struct WaylandRegistry{
    id: u32,
    upstream_flagged: bool,
    children: Vec<WaylandRegistry>,
    upstream_msgs: Vec<WaylandSockMsg>
}

impl WaylandRegistry{
    fn respond_to_msg(&mut self, _msg: WaylandSockMsg){
        //println!("Wayland Registry got a message: {}", msg);
    }

    pub fn new(new_id: u32) -> WaylandRegistry{
        println!("Creating WaylandRegistry object with id: {}", new_id);

        WaylandRegistry{
            id: new_id,
            upstream_flagged: false,
            children: Vec::new(),
            upstream_msgs: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandRegistry{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        self.upstream_flagged
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
        self.upstream_flagged = false;

        let mut vec: Vec<WaylandSockMsg> = Vec::new();
        for child in self.children.iter_mut() {
            if child.is_upstream_flagged() {
                vec.extend(child.rcv_upstream_msg());
            }
        }

        vec.extend(self.upstream_msgs.drain(..));
        vec
    }

    fn get_children(&mut self) -> Vec<&mut WaylandObject> {
        todo!()
    }
}