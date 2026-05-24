use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::{WaylandObject, WaylandSockMsg};
use crate::wayland_object::WaylandObjectImpl;

pub trait WaylandCallbackHandle{
    fn signal(&mut self, id: u32);
}

pub struct WaylandCallback {
    id: u32,
    callbacks: Vec<Rc<RefCell<dyn WaylandCallbackHandle>>>
}

impl WaylandCallback {
    // ***** Public Functions *****
    pub fn callback(&mut self, callback_fn: Rc<RefCell<dyn WaylandCallbackHandle>>){
        self.callbacks.push(callback_fn);
    }

    // ***** Private Functions *****
    fn received_signal(&mut self){
        for callback in self.callbacks.iter_mut(){
            callback.borrow_mut().signal(self.id);
        }
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandCallback {
        WaylandCallback{
            id: new_id,
            callbacks: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandCallback {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        false
    }

    fn get_children(&mut self) -> Vec<&mut WaylandObject> {
        Vec::new()
    }

    fn msg_downstream(&mut self, msg: WaylandSockMsg) {
        if msg.message_id() == self.get_id(){
            self.received_signal();
        }
    }

    fn rcv_upstream_msg(&mut self) -> Vec<WaylandSockMsg> {
        Vec::new()
    }
}