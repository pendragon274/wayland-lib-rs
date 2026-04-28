use crate::wayland_sock::WaylandSock;
use crate::wayland_display::WaylandDisplay;

pub struct Wayland{
    wl_socket: WaylandSock,
    current_id: u32
}

impl Wayland{
    pub fn get_display(&self) -> WaylandDisplay{
        WaylandDisplay::new(1)
    }

    pub fn get_sock(&mut self) -> &mut WaylandSock{
        &mut self.wl_socket
    }

    pub fn use_new_id(&mut self) -> u32{
        let ret = self.current_id;
        self.current_id = self.current_id + 1;
        ret
    }

    pub fn new() -> Wayland{
        Wayland{
            wl_socket: WaylandSock::new(),
            current_id: 2
        }
    }
}