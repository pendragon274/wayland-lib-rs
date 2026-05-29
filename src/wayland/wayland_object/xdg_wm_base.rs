use crate::{
    util::{
        ByteBuilder,
        ByteBuilderCompatible},
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl,
        XDGSurface},
    wayland_sock::{
        WaylandSockMsg,
        WaylandSockWriteBuffer}};

pub mod constants{
    pub const XDG_WM_BASE_REQUEST_DESTROY: u16 = 0;
    pub const XDG_WM_BASE_REQUEST_CREATE_POSITIONER: u16 = 1;
    pub const XDG_WM_BASE_REQUEST_GET_XDG_SURFACE: u16 = 2;
    pub const XDG_WM_BASE_REQUEST_PONG: u16 = 3;

    pub const XDG_WM_BASE_EVENT_PING: u16 = 0;
}

use constants::*;

pub struct XDGWMBase{
    id: u32,
    children: Vec<WaylandObject>,
    sock: WaylandSockWriteBuffer
}

impl XDGWMBase{
    // ***** Public Functions *****
    pub fn destroy(&mut self){
        todo!("Not implemented yet!");
    }

    pub fn create_positioner(&mut self, new_id: u32){
        todo!("Not implemented yet!");
    }

    pub fn get_xdg_surface(&mut self, new_id: u32, surface_id: u32) -> &mut XDGSurface{
        match self.has_xdg_surface(){
            Some(idx) =>{
                match &mut self.children[idx]{
                    WaylandObject::XDGSurface(xdg_surf) => xdg_surf,
                    _ => panic!("XDGWMBase::get_xdg_surface unwrapped an index of its child expecting an XDGSurface but found something else.")
                }
            }, None =>{
                self.sock.write_msg(WaylandSockMsg::new(self.get_id(), XDG_WM_BASE_REQUEST_GET_XDG_SURFACE, ByteBuilder::from(new_id).with(surface_id).to_bytes()));
                let WaylandObject::XDGSurface(xdg_surf) = self.children.push_mut(WaylandObject::XDGSurface(XDGSurface::new(new_id, self.sock.clone())))
                else { panic!("XDGWMBase::get_xdg_surface unwrapped an item it just created. This should never happen.") };
                xdg_surf
            }
        }
    }

    pub fn pong(&mut self, serial: u32){
        todo!("Not implemented yet!");
    }

    // ***** Private Functions *****
    fn has_xdg_surface(&mut self) -> Option<usize>{
        for index in 0..self.children.len(){
            if matches!(self.children[index], WaylandObject::XDGSurface(_)){
                return Some(index);
            }
        }
        None
    }

    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        println!("xdg_wm_base received event: {}", msg);
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32, new_sock: WaylandSockWriteBuffer) -> XDGWMBase{
        println!("Creating XDGWMBase object with id: {}", new_id);
        XDGWMBase{
            id: new_id,
            children: Vec::new(),
            sock: new_sock
        }
    }
}

impl WaylandObjectImpl for XDGWMBase{
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