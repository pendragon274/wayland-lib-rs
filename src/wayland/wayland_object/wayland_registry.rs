pub mod wayland_registry_event;

pub use wayland_registry_event::{
    WaylandRegistryEvent,
    constants::*};

use {
    std::{
        rc::Rc,
        cell::RefCell},
    crate::{
        util::{
            ByteBuilder,
            ByteBuilderCompatible},
        wayland_object::{
            WaylandObject,
            WaylandObjectImpl,
            wayland_shm::WaylandSHM,
            wayland_compositor::WaylandCompositor,
            wayland_xdg_wm_base::WaylandXDGWMBase},
        wayland_sock::{
            WaylandSockMsg}}};

pub trait RegistryCallbackHandle{
    fn global_add(&mut self, wl_registry_object: WaylandRegistryEvent);
    fn global_remove(&mut self, wl_registry_object: WaylandRegistryEvent);
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
    pub fn add_event_handler(&mut self, glob_callback: Rc<RefCell<dyn RegistryCallbackHandle>>){
        self.global_callbacks.push(glob_callback);
    }

    pub fn bind(&mut self, new_id: u32, wl_registry_event: WaylandRegistryEvent){
        match wl_registry_event.interface_str().as_str(){
            "wl_shm" =>{
                let child = WaylandObject::WaylandSHM(WaylandSHM::new(new_id));
                self.children.push(child);
            }, "wl_compositor" =>{
                let child = WaylandObject::WaylandCompositor(WaylandCompositor::new(new_id));
                self.children.push(child);
            }, "xdg_wm_base" =>{
                let child = WaylandObject::WaylandXDGWMBase(WaylandXDGWMBase::new(new_id));
                self.children.push(child);
            }, _=>{
                todo!("Unimplemented and unknown interface for WaylandRegistry::bind!")
            }
        }

        //let bind_request = self.make_bind_request_msg(new_id, wl_registry_event);
        //println!("Bind request: {:?}", bind_request.to_raw_vec());
        //self.upstream_msgs.push(bind_request);
        self.upstream_msgs.push(self.make_bind_request_msg(new_id, wl_registry_event));
        self.upstream_flagged = true;
    }

    // ***** Private Functions *****
    fn respond_to_msg(&mut self, msg: WaylandSockMsg){
        match msg.opcode(){
            WL_REGISTRY_EVENT_GLOBAL => {
                for callback in &self.global_callbacks {
                    callback.borrow_mut().global_add(WaylandRegistryEvent::from(msg.clone()));
                }
            }, WL_REGISTRY_EVENT_REMOVE => {
                for callback in &self.global_callbacks {
                    callback.borrow_mut().global_remove(WaylandRegistryEvent::from(msg.clone()));
                }
            }, _ => println!("Received registry event with unknown opcode: {}", msg)
        }
    }

    fn make_bind_request_msg(&self, new_id: u32, wl_registry_event: WaylandRegistryEvent) -> WaylandSockMsg{
        let msg = ByteBuilder::from(wl_registry_event.name())
            .with((wl_registry_event.interface_str().len() + 1) as u32)
            .with(wl_registry_event.interface_str())
            .with(0x0u8)
            .align(4)
            .with(wl_registry_event.version())
            .with(new_id)
            .to_bytes();

        WaylandSockMsg::new(self.id, 0, msg)
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