use wayland_lib::prelude::*;
use wayland_lib::wayland_object::WaylandObject;

pub struct Client{
    wayland: Wayland,
    events: WaylandEventBuffer,
    id_counter: WaylandIDCounter
}

impl RegistryCallbackHandle for Client {
    fn global_add(&mut self, wl_registry_object: WaylandRegistryEvent) {
        match wl_registry_object.interface_str().as_str(){
            "wl_shm" =>{
                self.wayland.get_display().get_registry_no_create().unwrap().bind(self.id_counter.get_new_id(), wl_registry_object);
                //self.wayland.get_display().sync(self.id_counter.get_new_id());
            }, "wl_compositor" =>{
                let compositor_id = self.id_counter.get_new_id();
                self.wayland.get_display().get_registry_no_create().unwrap().bind(compositor_id, wl_registry_object);
                if let Some(WaylandObject::WaylandCompositor(compositor)) = self.wayland.get_child(compositor_id){
                    compositor.create_surface(self.id_counter.get_new_id());
                }
                //self.wayland.get_display().sync(self.id_counter.get_new_id());
            }, "xdg_wm_base" =>{
                //println!("Creating WaylandXDGWMBase.");
                self.wayland.get_display().get_registry_no_create().unwrap().bind(self.id_counter.get_new_id(), wl_registry_object);
                //self.wayland.get_display().sync(self.id_counter.get_new_id());
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
        self.wayland.get_display().get_registry(self.id_counter.get_new_id()).add_event_handler(self.events.get_callback_ref());
        self.wayland.get_display().sync(self.id_counter.get_new_id()).add_event_handler(self.events.get_callback_ref());

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
        let internal_borrow = self.events.borrow_internal();
        let mut events_borrow = internal_borrow.borrow_mut();
        events_borrow.dispatch_registry_callback(self);
        events_borrow.dispatch_callback_handle(self);
        events_borrow.dispatch_display_callback(self);
    }

    pub fn new() -> Client{
        let wl = Wayland::new();
        let counter = wl.get_id_counter();
        Client{
            wayland: wl,
            events: WaylandEventBuffer::new(),
            id_counter: counter
        }
    }
}

fn main() {
    let mut client = Client::new();
    client.start();
}