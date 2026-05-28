use std::cell::RefCell;
use std::rc::Rc;
use crate::{
    wayland_object::{
        wayland_callback::WaylandCallbackHandle,
        wayland_display::{
            DisplayCallbackHandle,
            wayland_display_event::{
                WaylandDisplayEvent}},
        wayland_registry::{
            RegistryCallbackHandle, 
            WaylandRegistryEvent}}};

pub struct WaylandEventBuffer{
    internal_buffer: Rc<RefCell<WaylandEventBufferInternal>>
}

impl WaylandEventBuffer{
    // ***** Public Functions *****
    pub fn get_callback_ref(&self) -> Rc<RefCell<WaylandEventBufferInternal>> {
        self.internal_buffer.clone()
    }

    pub fn borrow_internal(&self) -> Rc<RefCell<WaylandEventBufferInternal>>{
        Rc::clone(&self.internal_buffer)
    }

    // ***** Private Functions *****

    // ***** Init Struct *****
    pub fn new() -> WaylandEventBuffer{
        WaylandEventBuffer{
            internal_buffer: Rc::new(RefCell::new(WaylandEventBufferInternal::new()))
        }
    }
}

pub struct WaylandEventBufferInternal{
    global_add_buf: Vec<WaylandRegistryEvent>,
    global_remove_buf: Vec<WaylandRegistryEvent>,
    signal_buf: Vec<u32>,
    error_buf: Vec<WaylandDisplayEvent>,
    delete_id_buf: Vec<WaylandDisplayEvent>
}

impl WaylandEventBufferInternal {
    // ***** Public Functions *****
    pub fn dispatch_registry_callback<T: RegistryCallbackHandle>(&mut self, target: &mut T){
        for event in self.global_add_buf.drain(0..){
            target.global_add(event);
        }

        for event in self.global_remove_buf.drain(0..){
            target.global_remove(event);
        }
    }

    pub fn dispatch_callback_handle<T: WaylandCallbackHandle>(&mut self, target: &mut T){
        for event in self.signal_buf.drain(0..){
            target.signal(event);
        }
    }

    pub fn dispatch_display_callback<T: DisplayCallbackHandle>(&mut self, target: &mut T){
        for event in self.error_buf.drain(0..){
            target.error(event);
        }

        for event in self.delete_id_buf.drain(0..){
            target.delete_id(event);
        }
    }

    // ***** Private Functions *****


    // ***** Init Struct *****
    pub fn new() -> WaylandEventBufferInternal {
        WaylandEventBufferInternal {
            global_add_buf: Vec::new(),
            global_remove_buf: Vec::new(),
            signal_buf: Vec::new(),
            error_buf: Vec::new(),
            delete_id_buf: Vec::new()
        }
    }
}

impl RegistryCallbackHandle for WaylandEventBufferInternal {
    fn global_add(&mut self, wl_registry_event: WaylandRegistryEvent) {
        self.global_add_buf.push(wl_registry_event);
    }

    fn global_remove(&mut self, wl_registry_event: WaylandRegistryEvent) {
        self.global_remove_buf.push(wl_registry_event);
    }
}

impl WaylandCallbackHandle for WaylandEventBufferInternal {
    fn signal(&mut self, id: u32) {
        self.signal_buf.push(id);
    }
}

impl DisplayCallbackHandle for WaylandEventBufferInternal {
    fn error(&mut self, event: WaylandDisplayEvent) {
        self.error_buf.push(event);
    }

    fn delete_id(&mut self, event: WaylandDisplayEvent) {
        self.delete_id_buf.push(event);
    }
}