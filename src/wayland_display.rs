use crate::wayland_registry::WaylandRegistry;
use crate::wayland_sock::WaylandSock;
use crate::wayland_sock_msg::WaylandSockMsg;

pub struct WaylandDisplay{
    obj_id: u32,
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

    pub fn new(new_id: u32) -> WaylandDisplay{
        WaylandDisplay{
            obj_id: 0,
            callbacks: Vec::new()
        }
    }
}