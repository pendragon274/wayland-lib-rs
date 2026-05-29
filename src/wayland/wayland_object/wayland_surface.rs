use crate::{
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl},
    wayland_sock::{
        WaylandSockMsg,
        WaylandSockWriteBuffer}};

pub mod constants{
    pub const WL_SURFACE_REQUEST_DESTROY: u16 = 0;
    pub const WL_SURFACE_REQUEST_ATTACH: u16 = 1;
    pub const WL_SURFACE_REQUEST_DAMAGE: u16 = 2;
    pub const WL_SURFACE_REQUEST_FRAME: u16 = 3;
    pub const WL_SURFACE_REQUEST_SET_OPAQUE_REGION: u16 = 4;
    pub const WL_SURFACE_REQUEST_SET_INPUT_REGION: u16 = 5;
    pub const WL_SURFACE_REQUEST_COMMIT: u16 = 6;
    pub const WL_SURFACE_REQUEST_SET_BUFFER_TRANSFORM: u16 = 7;
    pub const WL_SURFACE_REQUEST_SET_BUFFER_SCALE: u16 = 8;
    pub const WL_SURFACE_REQUEST_DAMAGE_BUFFER: u16 = 9;
    pub const WL_SURFACE_REQUEST_OFFSET: u16 = 10;

    pub const WL_SURFACE_EVENT_ENTER: u16 = 0;
    pub const WL_SURFACE_EVENT_LEAVE: u16 = 1;
    pub const WL_SURFACE_EVENT_PREFERRED_BUFFER_SCALE: u16 = 2;
    pub const WL_SURFACE_EVENT_PREFERRED_BUFFER_TRANSFORM: u16 = 3;

}

use constants::*;

pub struct WaylandSurface{
    id: u32,
    children: Vec<WaylandObject>,
    sock: WaylandSockWriteBuffer
}

impl WaylandSurface {
    // ***** Public Functions *****
    pub fn commit(&mut self){
        self.sock.write_msg(WaylandSockMsg::new(self.id, WL_SURFACE_REQUEST_COMMIT, Vec::new()));
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("WaylandSurface received msg: {}", msg);
    }

    // ***** Struct Init *****
    pub fn new(new_id: u32, new_sock: WaylandSockWriteBuffer) -> WaylandSurface {
        println!("Creating WaylandSurface object with id: {}", new_id);
        WaylandSurface{
            id: new_id,
            children: Vec::new(),
            sock: new_sock
        }
    }
}

impl WaylandObjectImpl for WaylandSurface {
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
        Vec::new()
    }
}