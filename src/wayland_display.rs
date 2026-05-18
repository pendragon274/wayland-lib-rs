use crate::wayland_object::{WaylandObjectImpl, WaylandObject, WaylandObjectRef};
use crate::wayland_registry::WaylandRegistry;
use crate::wayland_sock::WaylandSock;
use crate::wayland_sock_msg::WaylandSockMsg;

pub struct WaylandDisplay{
    obj_id: u32,
    parent_callback: fn(),
    callbacks: Vec<(u32, fn(u32))>
}

impl WaylandDisplay{
    pub fn sync(&mut self, sock: &mut WaylandSock, callback: fn(u32), new_id: u32) {
        sock.write(WaylandSockMsg::new(self.obj_id, 0, new_id.to_ne_bytes().to_vec()));
        self.callbacks.push((new_id, callback));
    }

    pub fn get_registry(&self, sock: &mut WaylandSock, new_id: u32) -> WaylandRegistry {
        sock.write(WaylandSockMsg::new(self.obj_id, 1, new_id.to_ne_bytes().to_vec()));
        WaylandRegistry::new(new_id)
    }

    pub fn new(_new_id: u32, _callback: fn()) -> WaylandObject<WaylandDisplay> {
        todo!()
        /*let display = WaylandDisplay{
            obj_id: 0,
            parent_callback: callback,
            callbacks: Vec::new()
        };

        WaylandObject::from::<WaylandDisplay>(display)*/
    }
}

impl WaylandObjectImpl for WaylandDisplay {
    fn get_id(&self) -> u32{
        todo!()
    }
    fn get_type(&self) -> String{
        todo!()
    }

    fn borrow_mut(&mut self) -> WaylandObjectRef<dyn WaylandObjectImpl> {
        todo!()
    }

    fn borrow_children(&mut self) -> Vec<WaylandObjectRef<dyn WaylandObjectImpl>> {
        todo!()
    }

    fn msg_downstream(&self, _msg: WaylandSockMsg) {
        todo!()
    }

    fn msg_upstream(&mut self, _msg: WaylandSockMsg) {
        todo!()
    }
}