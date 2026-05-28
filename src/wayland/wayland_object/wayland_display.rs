pub mod wayland_display_event;
pub use wayland_display_event::{
    WaylandDisplayEvent,
    constants::*};

use std::{
    cell::RefCell,
    rc::Rc};

use crate::{
    wayland_object::{
        WaylandObject,
        WaylandObjectImpl,
        wayland_callback::WaylandCallback,
        wayland_registry::WaylandRegistry},
    wayland_sock::WaylandSockMsg};

pub trait DisplayCallbackHandle{
    fn error(&mut self, event: WaylandDisplayEvent);
    fn delete_id(&mut self, event: WaylandDisplayEvent);
}

pub struct WaylandDisplay{
    id: u32,
    children: Vec<WaylandObject>,
    upstream_flagged: bool,
    upstream_msgs: Vec<WaylandSockMsg>,
    display_callbacks: Vec<Rc<RefCell<dyn DisplayCallbackHandle>>>
}

impl WaylandObjectImpl for WaylandDisplay{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn is_upstream_flagged(&self) -> bool {
        match self.upstream_flagged{
            true => true,
            false => {
                for child in &self.children {
                    if child.is_upstream_flagged() {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn get_child(&mut self, child_id: u32) -> Option<&mut WaylandObject> {
        for child in self.children.iter_mut(){
            if child.get_id() == child_id{
                return Some(child);
            }else if let Some(internal_child) = child.get_child(child_id){
                return Some(internal_child);
            }
        }

        None
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

        let mut vec: Vec<WaylandSockMsg> = self.upstream_msgs.drain(..).collect();
        for child in self.children.iter_mut() {
            if child.is_upstream_flagged() {
                vec.extend(child.rcv_upstream_msg());
            }
        }

        vec
    }
}

impl WaylandDisplay{
    // ***** Public Functions *****
    pub fn get_registry(&mut self, new_id: u32) -> &mut WaylandRegistry{
        match self.has_registry(){
            Some(idx) => {
                match &mut self.children[idx]{
                    WaylandObject::WaylandRegistry(reg) => reg,
                    _ => panic!("WaylandDisplay::get_registry unwrapped an index of its child expecting a WaylandRegistry but found something else.")
                }
            }, None => {
                let child = WaylandObject::WaylandRegistry(WaylandRegistry::new(new_id));
                self.children.push(child);
                self.upstream_msgs.push(WaylandSockMsg::new(self.get_id(), 1, new_id.to_ne_bytes().to_vec()));
                self.upstream_flagged = true;
                let len = self.children.len() - 1;
                match &mut self.children[len]{
                    WaylandObject::WaylandRegistry(reg) => reg,
                    _ => panic!("WaylandDisplay::get_registry unwrapped an index of its child expecting a WaylandRegistry but found something else.")
                }
            }
        }
    }

    pub fn get_registry_no_create(&mut self) -> Option<&mut WaylandRegistry>{
        for child in self.children.iter_mut(){
            match child {
                WaylandObject::WaylandRegistry(reg) => return Some(reg),
                _ => {}
            }
        }

        None
    }

    pub fn sync(&mut self, callback_id: u32) -> &mut WaylandCallback{
        self.upstream_flagged = true;

        self.upstream_msgs.push(WaylandSockMsg::new(self.get_id(), 0, callback_id.to_ne_bytes().to_vec()));
        let WaylandObject::WaylandCallback(callback) = self.children.push_mut(WaylandObject::WaylandCallback(WaylandCallback::new(callback_id))) else {
            panic!("WaylandDisplay::sync expects an item it just pushed to its children to be the same type it pushed.");
        };

        callback
    }

    pub fn add_event_handler(&mut self, callback: Rc<RefCell<dyn DisplayCallbackHandle>>){
        self.display_callbacks.push(callback);
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        match msg.opcode(){
            WL_DISPLAY_EVENT_ERROR =>{
                for callback in &self.display_callbacks{
                    callback.borrow_mut().error(WaylandDisplayEvent::from(msg.clone()));
                }
            }, WL_DISPLAY_EVENT_DELETE_ID => {
                for callback in &self.display_callbacks{
                    callback.borrow_mut().delete_id(WaylandDisplayEvent::from(msg.clone()));
                }
            }, _=>{
                println!("WaylandDisplay received unrecognized opcode in event: {}", msg);
            }
        }
    }

    fn has_registry(&self) -> Option<usize> {
        for index in 0..self.children.len(){
            if matches!(self.children[index], WaylandObject::WaylandRegistry(_)){
                return Some(index);
            }
        }
        None
    }

    // ***** Init Struct *****
    pub fn new(new_id: u32) -> WaylandDisplay{
        println!("Creating WaylandDisplay object with id: {}", new_id);

        WaylandDisplay{
            id: new_id,
            children: Vec::new(),
            upstream_flagged: false,
            upstream_msgs: Vec::new(),
            display_callbacks: Vec::new()
        }
    }
}