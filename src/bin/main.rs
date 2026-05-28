use wayland_lib::{
    Wayland,
    wayland_event_buf::WaylandEventBuffer,
    wayland_object::{
        wayland_callback::WaylandCallbackHandle,
        wayland_registry::{
            WaylandRegistryEvent,
            RegistryCallbackHandle},
        wayland_display::{
            WaylandDisplayEvent,
            DisplayCallbackHandle}}};

pub struct Client{
    wayland: Wayland,
    events: WaylandEventBuffer
}

impl RegistryCallbackHandle for Client {
    fn global_add(&mut self, wl_registry_object: WaylandRegistryEvent) {
        match wl_registry_object.interface_str().as_str(){
            "wl_shm" =>{
                let id = self.wayland.get_new_id();
                self.wayland.get_display().get_registry_no_create().unwrap().bind(id, wl_registry_object);
                //let id = self.wayland.get_new_id();
                //self.wayland.get_display().sync(id);
            }, "wl_compositor" =>{
                let id = self.wayland.get_new_id();
                //println!("Binding compositor with id: {}", id);
                self.wayland.get_display().get_registry_no_create().unwrap().bind(id, wl_registry_object);
                //let surface_id = self.wayland.get_new_id();
                //if let Some(WaylandObject::WaylandCompositor(compositor)) = self.wayland.get_child(id){
                //    compositor.create_surface(surface_id);
                //}
            }, "xdg_wm_base" =>{
                let id = self.wayland.get_new_id();
                self.wayland.get_display().get_registry_no_create().unwrap().bind(id, wl_registry_object);
            }, _ =>{}
        }
    }

    fn global_remove(&mut self, wl_registry_object: WaylandRegistryEvent) {
        println!("Removed: {}", wl_registry_object);
    }
}

impl WaylandCallbackHandle for Client{
    fn signal(&mut self, id: u32) {
        println!("Signal Received ID {}", id);
    }
}

impl DisplayCallbackHandle for Client{
    fn error(&mut self, event: WaylandDisplayEvent) {
        println!("Got wayland error: {}", event);
    }

    fn delete_id(&mut self, event: WaylandDisplayEvent) {
        println!("Got delete id notification: {}", event);
    }
}

impl Client{
    pub fn start(&mut self){
        self.wayland.get_display().add_event_handler(self.events.get_callback_ref());
        let mut new_id = self.wayland.get_new_id();
        self.wayland.get_display().get_registry(new_id).add_event_handler(self.events.get_callback_ref());
        new_id = self.wayland.get_new_id();
        self.wayland.get_display().sync(new_id).callback(self.events.get_callback_ref());

        //let mut loop_count: u32 = 0;
        loop{
            //if loop_count < 1000 {
            //    println!("Loop count: {}", loop_count);
            //}
            self.poll();

            //loop_count+=1;
        }
    }

    fn poll(&mut self){
        self.wayland.poll();
        self.events.borrow_internal().borrow_mut().dispatch_registry_callback(self);
        self.events.borrow_internal().borrow_mut().dispatch_callback_handle(self);
        self.events.borrow_internal().borrow_mut().dispatch_display_callback(self);
    }

    pub fn new() -> Client{
        Client{
            wayland: Wayland::new(),
            events: WaylandEventBuffer::new()
        }
    }
}

fn main() {
    let mut client = Client::new();
    client.start();
}