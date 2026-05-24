use wayland_lib::prelude::*;

pub struct Client{
    wayland: Wayland,
    events: WaylandEventBuffer
}

impl RegistryCallbackHandle for Client {
    fn global_add(&mut self, wl_registry_object: WaylandRegistryObject) {
        match wl_registry_object.interface_str().as_str(){
            "wl_shm" =>{
                println!("wl_shm: {}", wl_registry_object);
            }, "wl_compositor" =>{
                println!("wl_compositor: {}", wl_registry_object);
            }, "xdg_wm_base" =>{
                println!("xdg_wm_base: {}", wl_registry_object);
            }, _ =>{}
        }
    }

    fn global_remove(&mut self, wl_registry_object: WaylandRegistryObject) {
        println!("{}", wl_registry_object);
    }
}

impl Client{
    pub fn start(&mut self){
        self.wayland.get_display();
        let mut new_id = self.wayland.get_new_id();
        self.wayland.get_display().get_registry(new_id).global(self.events.get_callback_ref());
        new_id = self.wayland.get_new_id();
        self.wayland.get_display().sync(new_id).callback(self.events.get_callback_ref());

        loop{
            self.poll();
        }
    }

    fn poll(&mut self){
        self.wayland.poll();
        self.events.borrow_internal().borrow_mut().dispatch_registry_callback(self);
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