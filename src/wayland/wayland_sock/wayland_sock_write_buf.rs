use {
    std::{
        rc::Rc,
        cell::RefCell},
    crate::wayland_sock::{
        WaylandSockInternal,
        WaylandSockMsg}};

#[derive(Clone)]
pub struct WaylandSockWriteBuffer{
    sock_ref: Rc<RefCell<WaylandSockInternal>>
}

impl WaylandSockWriteBuffer{
    // ***** Public Functions *****
    pub fn write_all_msgs(&mut self, msgs: Vec<WaylandSockMsg>) {
        self.sock_ref.borrow_mut().write_all_msgs(msgs);
    }

    // ***** Private Functions *****

    // ***** Struct Init
    pub fn new(sock_ref: Rc<RefCell<WaylandSockInternal>>) -> WaylandSockWriteBuffer {
        WaylandSockWriteBuffer{
            sock_ref
        }
    }
}