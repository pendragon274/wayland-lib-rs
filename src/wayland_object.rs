use wayland_display::WaylandDisplay;
use wayland_registry::WaylandRegistry;
use wayland_callback::WaylandCallback;
use wayland_shm::WaylandSHM;
use crate::wayland_object::wayland_compositor::WaylandCompositor;
use crate::wayland_object::wayland_xdg_wm_base::WaylandXDGWMBase;
use crate::wayland_sock::WaylandSockMsg;

pub mod wayland_display;
pub mod wayland_registry;
pub mod wayland_callback;
pub mod wayland_shm;
pub mod wayland_compositor;
pub mod wayland_xdg_wm_base;

pub enum WaylandObject{
    WaylandDisplay(WaylandDisplay),
    WaylandRegistry(WaylandRegistry),
    WaylandCallback(WaylandCallback),
    WaylandSHM(WaylandSHM),
    WaylandCompositor(WaylandCompositor),
    WaylandXDGWMBase(WaylandXDGWMBase)
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
            }, WaylandObject::WaylandSHM(shm) =>{
                shm.get_id()
            }, WaylandObject::WaylandCompositor(compositor) =>{
                compositor.get_id()
            }, WaylandObject::WaylandXDGWMBase(xdg) =>{
                xdg.get_id()
            }
        }
    }

    fn is_upstream_flagged(&self) -> bool {
        match self{
            WaylandObject::WaylandDisplay(disp) => disp.is_upstream_flagged(),
            WaylandObject::WaylandRegistry(reg) => reg.is_upstream_flagged(),
            WaylandObject::WaylandCallback(call) => call.is_upstream_flagged(),
            WaylandObject::WaylandSHM(shm) => shm.is_upstream_flagged(),
            WaylandObject::WaylandCompositor(compositor) => compositor.is_upstream_flagged(),
            WaylandObject::WaylandXDGWMBase(xdg) => xdg.is_upstream_flagged()
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
            }, WaylandObject::WaylandSHM(shm)=>{
                shm.get_children()
            }, WaylandObject::WaylandCompositor(compositor)=>{
                compositor.get_children()
            }, WaylandObject::WaylandXDGWMBase(xdg)=>{
                xdg.get_children()
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
            }, WaylandObject::WaylandSHM(shm)=>{
                shm.msg_downstream(msg)
            }, WaylandObject::WaylandCompositor(compositor)=>{
                compositor.msg_downstream(msg)
            }, WaylandObject::WaylandXDGWMBase(xdg)=>{
                xdg.msg_downstream(msg)
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
            }, WaylandObject::WaylandSHM(shm)=>{
                shm.rcv_upstream_msg()
            }, WaylandObject::WaylandCompositor(compositor)=>{
                compositor.rcv_upstream_msg()
            }, WaylandObject::WaylandXDGWMBase(xdg)=>{
                xdg.rcv_upstream_msg()
            }
        }
    }
}