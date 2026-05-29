use crate::{
    util::{
        ByteBuilder,
        ByteBuilderCompatible},
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl},
    wayland_sock::{
        WaylandSockMsg,
        WaylandSockWriteBuffer}};

pub struct WaylandCompositor{
    id: u32,
    children: Vec<WaylandObject>,
    sock: WaylandSockWriteBuffer
}

impl WaylandCompositor{
    // ***** Public Functions *****
    /*pub fn create_surface(&mut self, new_id: u32){
        let msg = ByteBuilder::from(new_id);
        self.upstream_flagged = true;
        self.upstream_msgs.push(WaylandSockMsg::new(self.id, 0, msg.to_bytes()));
    }*/
    
    pub fn create_surface(&mut self, new_id: u32){
        self.sock.write_msg(WaylandSockMsg::new(self.id, 0, ByteBuilder::from(new_id).to_bytes()));
    }

    pub fn create_region(&mut self, _new_id: u32){
        todo!("Not implemented yet!");
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("Compositor received event: {}", msg);
    }

    // ***** Init Struct *****
    /*pub fn new(new_id: u32) -> WaylandCompositor{
        println!("Creating WaylandCompositor object with id: {}", new_id);
        WaylandCompositor{
            id: new_id,
            upstream_flagged: false,
            defer_upstream_flag: true,
            children: Vec::new(),
            upstream_msgs: Vec::new()
        }
    }*/

    pub fn new(new_id: u32, new_sock: WaylandSockWriteBuffer) -> WaylandCompositor{
        println!("Creating WaylandCompositor object with id: {}", new_id);
        WaylandCompositor{
            id: new_id,
            children: Vec::new(),
            sock: new_sock
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
        Vec::new()
    }
}