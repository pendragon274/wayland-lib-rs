use crate::{
    util::{
        ByteBuilder,
        ByteBuilderCompatible},
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl,
        XDGTopLevel},
    wayland_sock::{
        WaylandSockMsg,
        WaylandSockWriteBuffer}};

pub mod constants{
    pub const XDG_SURFACE_REQUEST_DESTROY: u16 = 0;
    pub const XDG_SURFACE_REQUEST_GET_TOPLEVEL: u16 = 1;
    pub const XDG_SURFACE_REQUEST_GET_POPUP: u16 = 2;
    pub const XDG_SURFACE_REQUEST_SET_WINDOW_GEOMETRY: u16 = 3;
    pub const XDG_SURFACE_REQUEST_ACK_CONFIGURE: u16 = 4;

    pub const XDG_SURFACE_EVENT_CONFIGURE: u16 = 0;
}

use constants::*;

pub struct XDGSurface{
    id: u32,
    sock: WaylandSockWriteBuffer,
    children: Vec<WaylandObject>
}

impl XDGSurface {
    // ***** Public Functions *****
    pub fn get_top_level(&mut self, new_id: u32) -> &mut XDGTopLevel{
        match self.has_top_level(){
            Some(idx) =>{
                match &mut self.children[idx]{
                    WaylandObject::XDGTopLevel(xdg_top) => xdg_top,
                    _ => panic!("XDGSurface::get_top_level unwrapped an index of its child expecting an XDGTopLevel but found something else.")
                }
            }, None =>{
                self.sock.write_msg(WaylandSockMsg::new(self.get_id(), XDG_SURFACE_REQUEST_GET_TOPLEVEL, ByteBuilder::from(new_id).to_bytes()));
                let WaylandObject::XDGTopLevel(xdg_top) = self.children.push_mut(WaylandObject::XDGTopLevel(XDGTopLevel::new(new_id, self.sock.clone())))
                else { panic!("XDGSurface::get_top_level unwrapped an item it just created. This should never happen.") };
                xdg_top
            }
        }
    }

    // ***** Private Functions *****
    fn has_top_level(&mut self) -> Option<usize>{
        for index in 0..self.children.len(){
            if matches!(self.children[index], WaylandObject::XDGTopLevel(_)){
                return Some(index);
            }
        }
        None
    }

    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("XDGSurface received a configure event and will respond with an ACK.");
        //TODO: Should implement at least a way to bring this out to main.
        self.sock.write_msg(WaylandSockMsg::new(self.id, XDG_SURFACE_REQUEST_ACK_CONFIGURE, msg.message()));
    }

    // ***** Struct Init *****
    pub fn new(new_id: u32, new_sock: WaylandSockWriteBuffer) -> XDGSurface{
        XDGSurface {
            id: new_id,
            sock: new_sock,
            children: Vec::new()
        }
    }
}

impl WaylandObjectImpl for XDGSurface {
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