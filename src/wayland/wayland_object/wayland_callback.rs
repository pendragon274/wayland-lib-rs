use{
    std::{
        cell::RefCell,
        rc::Rc},
    crate::{
        wayland_sock::WaylandSockMsg,
        wayland_object::{
            WaylandObject,
            WaylandObjectImpl}}};

pub trait WaylandCallbackHandle{
    fn signal(&mut self, id: u32);
}

pub struct WaylandCallback {
    id: u32,
    callbacks: Vec<Rc<RefCell<dyn WaylandCallbackHandle>>>
}

impl WaylandCallback {
    // ***** Public Functions *****
    pub fn add_event_handler(&mut self, callback_fn: Rc<RefCell<dyn WaylandCallbackHandle>>){
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
        println!("Creating WaylandCallback object with id: {}", new_id);
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

    fn get_child(&mut self, _: u32) -> Option<&mut WaylandObject> {
        None
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