use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::WaylandRegistryObject;
use crate::wayland_callback::WaylandCallbackHandle;
use crate::wayland_registry::RegistryCallbackHandle;

/*static global_add_buf: LazyLock<Arc<RwLock<Vec<WaylandRegistryObject>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
static global_remove_buf: LazyLock<Arc<RwLock<Vec<WaylandRegistryObject>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
static signal_buf: LazyLock<Arc<RwLock<Vec<u32>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));*/

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
    global_add_buf: Vec<WaylandRegistryObject>,
    global_remove_buf: Vec<WaylandRegistryObject>,
    signal_buf: Vec<u32>
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

    // ***** Private Functions *****

    // ***** Static Functions *****
    /*fn borrow_add_buf<'a>() -> &'a mut Vec<WaylandRegistryObject> {
        global_add_buf.write().unwrap().as_mut::<'a>()
    }*/

    // ***** Init Struct *****
    pub fn new() -> WaylandEventBufferInternal {
        WaylandEventBufferInternal {
            global_add_buf: Vec::new(),
            global_remove_buf: Vec::new(),
            signal_buf: Vec::new()
        }
    }
}

impl RegistryCallbackHandle for WaylandEventBufferInternal {
    fn global_add(&mut self, wl_registry_object: WaylandRegistryObject) {
        self.global_add_buf.push(wl_registry_object);
    }

    fn global_remove(&mut self, wl_registry_object: WaylandRegistryObject) {
        self.global_remove_buf.push(wl_registry_object);
    }
}

impl WaylandCallbackHandle for WaylandEventBufferInternal {
    fn signal(&mut self, id: u32) {
        self.signal_buf.push(id);
    }
}