use crate::prelude::WaylandSockMsg;
use crate::wayland_sock::WaylandSock;
use crate::wayland_display::WaylandDisplay;
use crate::wayland_object::{WaylandObject, WaylandObjectRef, WaylandObjectImpl};

pub struct Wayland {
    wl_socket: WaylandSock,
    children: Vec<WaylandObject<dyn WaylandObjectImpl>>,
    current_open_id: u32
}

fn blah(){
    todo!()
}

impl Wayland {
    //***** Public Functions *****
    pub fn get_display(&mut self) -> WaylandObjectRef<WaylandDisplay>{
        /*let display;
        if !self.has_display(){
            let mut new_display= WaylandDisplay::new(1, blah);
            display = new_display.borrow_mut();
            self.children.push(new_display);
            display.downcast()
        }*/
        todo!()
    }

    pub fn get_sock(&mut self) -> &mut WaylandSock{
        &mut self.wl_socket
    }

    // ***** Private Functions *****
    fn has_display(&self) -> bool {
        todo!()
    }

    fn get_new_id(&mut self) -> u32{
        let ret = self.current_open_id;
        self.current_open_id = self.current_open_id + 1;
        ret
    }

    // ****** Struct Init *****
    pub fn new() -> Wayland {
        Wayland {
            wl_socket: WaylandSock::new(),
            children: Vec::new(),
            current_open_id: 1,

        }
    }
}

impl WaylandObjectImpl for Wayland {
    fn get_id(&self) -> u32 {
        0
    }

    fn get_type(&self) -> String{
        String::from("wayland")
    }

    fn borrow_mut(&mut self) -> WaylandObjectRef<dyn WaylandObjectImpl> {
        todo!()
    }

    fn borrow_children(&mut self) -> Vec<WaylandObjectRef<dyn WaylandObjectImpl>> {
        let mut ret = Vec::new();
        for child in &mut self.children{
            ret.push(child.borrow_mut());
        }

        ret
    }

    fn msg_downstream(&self, _msg: WaylandSockMsg) {
        todo!()
    }

    fn msg_upstream(&mut self, _msg: WaylandSockMsg) {
        todo!()
    }
}