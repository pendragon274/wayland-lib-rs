pub mod wayland_display;
pub mod wayland_registry;
pub mod wayland_callback;
pub mod wayland_compositor;
pub mod wayland_shm;
pub mod xdg_wm_base;
pub mod wayland_surface;
pub mod xdg_surface;
pub mod xdg_top_level;

pub use crate::wayland_object::{
    wayland_display::WaylandDisplay,
    wayland_registry::WaylandRegistry,
    wayland_callback::WaylandCallback,
    wayland_shm::WaylandSHM,
    wayland_compositor::WaylandCompositor,
    wayland_surface::WaylandSurface,
    xdg_wm_base::XDGWMBase,
    xdg_surface::XDGSurface,
    xdg_top_level::XDGTopLevel};

use crate::wayland_sock::WaylandSockMsg;

pub enum WaylandObject{
    WaylandDisplay(WaylandDisplay),
    WaylandRegistry(WaylandRegistry),
    WaylandCallback(WaylandCallback),
    WaylandSHM(WaylandSHM),
    WaylandCompositor(WaylandCompositor),
    WaylandSurface(WaylandSurface),
    XDGWMBase(XDGWMBase),
    XDGSurface(XDGSurface),
    XDGTopLevel(XDGTopLevel)
}

pub trait WaylandObjectImpl{
    fn get_id(&self) -> u32;

    #[deprecated]
    fn is_upstream_flagged(&self) -> bool;
    fn get_child(&mut self, child_id: u32) -> Option<&mut WaylandObject>;
    fn msg_downstream(&mut self, msg: WaylandSockMsg);

    #[deprecated]
    fn rcv_upstream_msg(&mut self) -> Vec<WaylandSockMsg>;
}

#[allow(deprecated)]
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
            }, WaylandObject::WaylandSurface(surface) =>{
                surface.get_id()
            }, WaylandObject::XDGWMBase(xdg) =>{
                xdg.get_id()
            }, WaylandObject::XDGSurface(xdg_surface) =>{
                xdg_surface.get_id()
            }, WaylandObject::XDGTopLevel(xdg_toplevel) =>{
                xdg_toplevel.get_id()
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
            WaylandObject::WaylandSurface(surface) => surface.is_upstream_flagged(),
            WaylandObject::XDGWMBase(xdg) => xdg.is_upstream_flagged(),
            WaylandObject::XDGSurface(xdg_surface) => xdg_surface.is_upstream_flagged(),
            WaylandObject::XDGTopLevel(xdg_toplevel) => xdg_toplevel.is_upstream_flagged()
        }
    }

    fn get_child(&mut self, child_id: u32) -> Option<&mut WaylandObject> {
        match self{
            WaylandObject::WaylandDisplay(disp)=>{
                disp.get_child(child_id)
            }, WaylandObject::WaylandRegistry(reg)=>{
                reg.get_child(child_id)
            }, WaylandObject::WaylandCallback(call)=>{
               call.get_child(child_id)
            }, WaylandObject::WaylandSHM(shm)=>{
                shm.get_child(child_id)
            }, WaylandObject::WaylandCompositor(compositor)=>{
                compositor.get_child(child_id)
            }, WaylandObject::WaylandSurface(surface)=>{
                surface.get_child(child_id)
            }, WaylandObject::XDGWMBase(xdg)=>{
                xdg.get_child(child_id)
            }, WaylandObject::XDGSurface(xdg_surface)=>{
                xdg_surface.get_child(child_id)
            }, WaylandObject::XDGTopLevel(xdg_toplevel)=>{
                xdg_toplevel.get_child(child_id)
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
            }, WaylandObject::WaylandSurface(surface)=>{
                surface.msg_downstream(msg)
            }, WaylandObject::XDGWMBase(xdg)=>{
                xdg.msg_downstream(msg)
            }, WaylandObject::XDGSurface(xdg_surface)=>{
                xdg_surface.msg_downstream(msg)
            }, WaylandObject::XDGTopLevel(xdg_toplevel)=>{
                xdg_toplevel.msg_downstream(msg)
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
            }, WaylandObject::WaylandSurface(surface)=>{
                surface.rcv_upstream_msg()
            }, WaylandObject::XDGWMBase(xdg)=>{
                xdg.rcv_upstream_msg()
            }, WaylandObject::XDGSurface(xdg_surface)=>{
                xdg_surface.rcv_upstream_msg()
            }, WaylandObject::XDGTopLevel(xdg_toplevel)=>{
                xdg_toplevel.rcv_upstream_msg()
            }
        }
    }
}