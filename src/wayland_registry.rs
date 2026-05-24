use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::{WaylandObject, WaylandSockMsg};
use crate::wayland_object::WaylandObjectImpl;
use crate::wayland_registry_object::WaylandRegistryObject;

const WL_REGISTRY_EVENT_GLOBAL: u16 = 0;
const WL_REGISTRY_EVENT_REMOVE: u16 = 1;

pub trait RegistryCallbackHandle{
    fn global_add(&mut self, wl_registry_object: WaylandRegistryObject);
    fn global_remove(&mut self, wl_registry_object: WaylandRegistryObject);
}

pub struct WaylandRegistry{
    id: u32,
    upstream_flagged: bool,
    children: Vec<WaylandObject>,
    upstream_msgs: Vec<WaylandSockMsg>,
    global_callbacks: Vec<Rc<RefCell<dyn RegistryCallbackHandle>>>
}

impl WaylandRegistry{
    // ***** Public Functions *****
    pub fn global(&mut self, glob_callback: Rc<RefCell<dyn RegistryCallbackHandle>>){
        self.global_callbacks.push(glob_callback);
    }

    pub fn bind(&mut self, _wl_registry_object: WaylandRegistryObject){
        todo!("WaylandRegistry::bind not implemented yet!");
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        for callback in &self.global_callbacks {
            match msg.opcode(){
                WL_REGISTRY_EVENT_GLOBAL => {
                    callback.borrow_mut().global_add(WaylandRegistryObject::from(msg.clone()));
                },
                WL_REGISTRY_EVENT_REMOVE => {
                    callback.borrow_mut().global_remove(WaylandRegistryObject::from(msg.clone()));
                },
                _ => println!("Received registry event with unknown opcode: {}", msg)
            }
        }
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandRegistry{
        println!("Creating WaylandRegistry object with id: {}", new_id);

        WaylandRegistry{
            id: new_id,
            upstream_flagged: false,
            children: Vec::new(),
            upstream_msgs: Vec::new(),
            global_callbacks: Vec::new()
        }
    }
}

impl WaylandObjectImpl for WaylandRegistry{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        self.upstream_flagged
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
        self.upstream_flagged = false;

        let mut vec: Vec<WaylandSockMsg> = Vec::new();
        for child in self.children.iter_mut() {
            if child.is_upstream_flagged() {
                vec.extend(child.rcv_upstream_msg());
            }
        }

        vec.extend(self.upstream_msgs.drain(..));
        vec
    }

    fn get_children(&mut self) -> Vec<&mut WaylandObject> {
        todo!()
    }
}