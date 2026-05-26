use crate::wayland_sock::{WaylandSock, WaylandSockMsg};
use crate::wayland_id_counter::WaylandIDCounter;
use crate::wayland_object::{WaylandObject, WaylandObjectImpl, wayland_display::WaylandDisplay};

pub struct Wayland{
    wl_socket: WaylandSock,
    children: Vec<WaylandObject>,
    id_counter: WaylandIDCounter
}

impl Wayland{
    // ***** Public Functions *****
    pub fn get_display(&mut self) -> &mut WaylandDisplay{
        match self.has_display(){
            Some(idx) => {
                match &mut self.children[idx]{
                    WaylandObject::WaylandDisplay(disp) => disp,
                    _ => panic!("Wayland::get_display unwrapped an index of its child expecting a WaylandDisplay but found something else.")
                }
            }, None => {
                let child = WaylandObject::WaylandDisplay(WaylandDisplay::new(self.get_new_id()));
                self.children.push(child);
                //self.collect_upstream_send();
                let len = self.children.len() - 1;
                match &mut self.children[len]{
                    WaylandObject::WaylandDisplay(disp) => disp,
                    _ => panic!("Wayland::get_display unwrapped an index of its child expecting a WaylandDisplay but found something else.")
                }
            }
        }
    }

    pub fn collect_upstream_send(&mut self){
        let mut msgs: Vec<WaylandSockMsg> = Vec::new();
        for child in &mut self.children{
            if child.is_upstream_flagged() {
                msgs.append(&mut child.rcv_upstream_msg());
            }
        }

        /*for msg in &msgs{
            println!("Writing message: {}", msg);
        }*/

        self.wl_socket.write_all_msgs(msgs);
    }

    pub fn read_downstream(&mut self){
        let msgs = self.wl_socket.read_all_msgs();
        for msg in msgs {
            //println!("Read message with id: {}", msg.message_id());
            for child in self.children.iter_mut(){
                child.msg_downstream(msg.clone());
            }
        }
    }

    pub fn poll(&mut self){
        self.read_downstream();
        self.collect_upstream_send();
    }

    pub fn get_new_id(&mut self) -> u32{
        self.id_counter.get_new_id()
    }

    // ***** Private Functions *****
    fn has_display(&self) -> Option<usize>{
        for index in 0..self.children.len(){
            if matches!(self.children[index], WaylandObject::WaylandDisplay(_)){
                return Some(index);
            }
        }
        None
    }

    // ***** Struct Init *****
    pub fn new() -> Wayland{
        Wayland{
            wl_socket: WaylandSock::new(),
            children: Vec::new(),
            id_counter: WaylandIDCounter::new()
        }
    }
}