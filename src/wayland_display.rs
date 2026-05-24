use crate::prelude::{WaylandCallback, WaylandObject, WaylandSockMsg};
use crate::wayland_object::WaylandObjectImpl;
use crate::wayland_registry::WaylandRegistry;

pub struct WaylandDisplay{
    id: u32,
    children: Vec<WaylandObject>,
    upstream_flagged: bool,
    upstream_msgs: Vec<WaylandSockMsg>
}

impl WaylandDisplay{
    // ***** Public Functions *****
    pub fn get_registry(&mut self, new_id: u32) -> &mut WaylandRegistry{
        match self.has_registry(){
            Some(idx) => {
                match &mut self.children[idx]{
                    WaylandObject::WaylandRegistry(reg) => reg,
                    _ => panic!("WaylandDisplay::get_registry unwrapped an index of its child expecting a WaylandRegistry but found something else.")
                }
            }, None => {
                let child = WaylandObject::WaylandRegistry(WaylandRegistry::new(new_id));
                self.children.push(child);
                self.upstream_msgs.push(WaylandSockMsg::new(self.get_id(), 1, new_id.to_ne_bytes().to_vec()));
                self.upstream_flagged = true;
                let len = self.children.len() - 1;
                match &mut self.children[len]{
                    WaylandObject::WaylandRegistry(reg) => reg,
                    _ => panic!("WaylandDisplay::get_registry unwrapped an index of its child expecting a WaylandRegistry but found something else.")
                }
            }
        }
    }

    pub fn sync(&mut self, callback_id: u32) -> &mut WaylandCallback{
        self.upstream_flagged = true;

        self.upstream_msgs.push(WaylandSockMsg::new(self.get_id(), 0, callback_id.to_ne_bytes().to_vec()));
        let WaylandObject::WaylandCallback(callback) = self.children.push_mut(WaylandObject::WaylandCallback(WaylandCallback::new(callback_id))) else {
            panic!("WaylandDisplay::sync expects an item it just pushed to its children to be the same type it pushed.");
        };
        
        callback
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, _msg: WaylandSockMsg){
        //println!("Wayland Display got a message: {}", msg);
    }

    fn has_registry(&self) -> Option<usize> {
        for index in 0..self.children.len(){
            if matches!(self.children[index], WaylandObject::WaylandRegistry(_)){
                return Some(index);
            }
        }
        None
    }

    // ***** Struct Init *****
    pub fn new(new_id: u32) -> WaylandDisplay{
        println!("Creating WaylandDisplay object with id: {}", new_id);

        WaylandDisplay{
            id: new_id,
            children: Vec::new(),
            upstream_flagged: false,
            upstream_msgs: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandDisplay{
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
        let mut vec: Vec<&mut WaylandObject> = Vec::new();
        for child in self.children.iter_mut(){
            vec.push(child);
        }
        vec
    }
}