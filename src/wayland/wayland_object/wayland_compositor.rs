use crate::{
    util::{
        ByteBuilder,
        ByteBuilderCompatible},
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl},
    wayland_sock::WaylandSockMsg};

pub struct WaylandCompositor{
    id: u32,
    upstream_flagged: bool,
    defer_upstream_flag: bool,
    children: Vec<WaylandObject>,
    upstream_msgs: Vec<WaylandSockMsg>
}

impl WaylandCompositor{
    // ***** Public Functions *****
    pub fn create_surface(&mut self, new_id: u32){
        let msg = ByteBuilder::from(new_id);
        self.upstream_flagged = true;
        self.upstream_msgs.push(WaylandSockMsg::new(self.id, 0, msg.to_bytes()));
    }

    pub fn create_region(&mut self, _new_id: u32){
        todo!("Not implemented yet!");
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("Compositor received event: {}", msg);
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandCompositor{
        WaylandCompositor{
            id: new_id,
            upstream_flagged: false,
            defer_upstream_flag: true,
            children: Vec::new(),
            upstream_msgs: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandCompositor{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        match self.upstream_flagged {
            true => true,
            false => {
                for child in &self.children {
                    if child.is_upstream_flagged() {
                        return true;
                    }
                }
                false
            }
        }
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
        println!("rcv_upstream_msg called for WaylandCompositor");
        match self.defer_upstream_flag{
            true =>{
                self.defer_upstream_flag = false;
                Vec::new()
            }, false =>{
                self.upstream_flagged = false;

                let mut vec: Vec<WaylandSockMsg> = self.upstream_msgs.drain(..).collect();
                for child in self.children.iter_mut() {
                    if child.is_upstream_flagged() {
                        vec.extend(child.rcv_upstream_msg());
                    }
                }

                vec
            }
        }
    }
}