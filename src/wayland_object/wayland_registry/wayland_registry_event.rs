use std::fmt::{Display, Formatter};
use crate::wayland_sock::wayland_sock_msg::WaylandSockMsg;

pub struct WaylandRegistryEvent{
    name: u32,
    interface_str: String,
    version: u32
}

impl Display for WaylandRegistryEvent{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Global Registry Object Name: {}, Interface: {}, Version: {}", self.name, self.interface_str, self.version)
    }
}

impl WaylandRegistryEvent{
    // ***** Public Functions *****
    pub fn name(&self) -> u32{
        self.name
    }

    pub fn interface_str(&self) -> String{
        self.interface_str.clone()
    }

    pub fn version(&self) -> u32{
        self.version
    }

    // ***** Private Functions *****

    // ***** Init Struct *****
    pub fn from(msg: WaylandSockMsg) -> WaylandRegistryEvent{
        let message = msg.message();
        let str_len = u32::from_ne_bytes(message[4..8].try_into().unwrap());
        let interface_str_end = 7 + (str_len as usize);
        let align_start = interface_str_end + (4 - (interface_str_end % 4));
        let version = u32::from_ne_bytes(message[align_start..align_start+4].try_into().unwrap());

        WaylandRegistryEvent{
            name: u32::from_ne_bytes(message[0..4].try_into().unwrap()),
            interface_str: String::from_utf8(message[8..interface_str_end].to_vec()).unwrap(),
            version
        }
    }
}