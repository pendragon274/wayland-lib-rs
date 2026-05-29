use wayland_lib::prelude::*;
use wayland_lib::wayland_object::WaylandObject;

pub struct Client{
    wayland: Wayland,
    events: WaylandEventBuffer,
    id_counter: WaylandIDCounter,
    surface: Option<u32>,
    xdg_wm_base: Option<u32>,
    xdg_surface: Option<u32>
}

impl RegistryCallbackHandle for Client {
    fn global_add(&mut self, wl_registry_object: WaylandRegistryEvent) {
        match wl_registry_object.interface_str().as_str(){
            "wl_shm" =>{
                self.wayland.get_display().get_registry_no_create().unwrap().bind(self.id_counter.get_new_id(), wl_registry_object);
            }, "wl_compositor" =>{
                let compositor_id = self.id_counter.get_new_id();
                self.wayland.get_display().get_registry_no_create().unwrap().bind(compositor_id, wl_registry_object);
                if let Some(WaylandObject::WaylandCompositor(compositor)) = self.wayland.get_child(compositor_id){
                    let surface_id = self.id_counter.get_new_id();
                    compositor.create_surface(surface_id);
                    self.surface = Some(surface_id);
                }
            }, "xdg_wm_base" =>{
                let xdg_id = self.id_counter.get_new_id();
                self.wayland.get_display().get_registry_no_create().unwrap().bind(xdg_id, wl_registry_object);
                self.xdg_wm_base = Some(xdg_id);
            }, _ =>{
                //println!("Unrecognized global: {}", wl_registry_object);
            }
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

        loop{
            self.poll();

            if self.xdg_surface == None && self.xdg_wm_base != None && self.surface != None{
                let WaylandObject::XDGWMBase(xdg) = self.wayland.get_child(self.xdg_wm_base.clone().unwrap()).expect("XDGWMBase not found as child.")
                else { panic!("XDGWMBase id unexpected type."); };

                self.xdg_surface = Some(self.id_counter.get_new_id());
                xdg.get_xdg_surface(self.xdg_surface.clone().unwrap(), self.surface.clone().unwrap()).get_top_level(self.id_counter.get_new_id());

                let WaylandObject::WaylandSurface(surface) = self.wayland.get_child(self.surface.clone().unwrap()).expect("Wayland surface not found as child.")
                else { panic!("WaylandSurface id unexpected type."); };

                surface.commit();
            }
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
            id_counter: counter,
            surface: None,
            xdg_wm_base: None,
            xdg_surface: None
        }
    }
}

fn main() {
    let mut client = Client::new();
    client.start();
}