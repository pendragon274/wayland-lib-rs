use crate::prelude::WaylandSockMsg;
use crate::wayland_display::WaylandDisplay;
use crate::wayland_registry::WaylandRegistry;
use crate::wayland_callback::WaylandCallback;

pub enum WaylandObject{
    WaylandDisplay(WaylandDisplay),
    WaylandRegistry(WaylandRegistry),
    WaylandCallback(WaylandCallback)
}

#[allow(dead_code)]
pub trait WaylandObjectImpl{
    fn get_id(&self) -> u32;
    fn is_upstream_flagged(&self) -> bool;
    fn get_children(&mut self) -> Vec<&mut WaylandObject>;
    fn msg_downstream(&mut self, msg: WaylandSockMsg);
    fn rcv_upstream_msg(&mut self) -> Vec<WaylandSockMsg>;
}

impl WaylandObjectImpl for WaylandObject{
    fn get_id(&self) -> u32 {
        match self {
            WaylandObject::WaylandDisplay(disp) =>{
                disp.get_id()
            }, WaylandObject::WaylandRegistry(reg) =>{
                reg.get_id()
            }, WaylandObject::WaylandCallback(call) =>{
                call.get_id()
            }
        }
    }

    fn is_upstream_flagged(&self) -> bool {
        match self{
            WaylandObject::WaylandDisplay(disp) => disp.is_upstream_flagged(),
            WaylandObject::WaylandRegistry(reg) => reg.is_upstream_flagged(),
            WaylandObject::WaylandCallback(call) => call.is_upstream_flagged()
        }
    }

    fn get_children(&mut self) -> Vec<&mut WaylandObject> {
        match self{
            WaylandObject::WaylandDisplay(disp)=>{
                disp.get_children()
            }, WaylandObject::WaylandRegistry(reg)=>{
                reg.get_children()
            }, WaylandObject::WaylandCallback(call)=>{
               call.get_children()
            }
        }
    }

    fn msg_downstream(&mut self, msg: WaylandSockMsg) {
        match self{
            WaylandObject::WaylandDisplay(disp)=>{
                disp.msg_downstream(msg)
            }, WaylandObject::WaylandRegistry(reg)=>{
                reg.msg_downstream(msg)
            }, WaylandObject::WaylandCallback(call)=>{
                call.msg_downstream(msg)
            }
        }
    }

    fn rcv_upstream_msg(&mut self) -> Vec<WaylandSockMsg> {
        match self{
            WaylandObject::WaylandDisplay(disp)=>{
                disp.rcv_upstream_msg()
            }, WaylandObject::WaylandRegistry(reg)=>{
                reg.rcv_upstream_msg()
            }, WaylandObject::WaylandCallback(call)=>{
                call.rcv_upstream_msg()
            }
        }
    }
}