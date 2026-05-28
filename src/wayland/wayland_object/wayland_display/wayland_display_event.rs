use std::fmt::{
    Display,
    Formatter};
use crate::{
    util::ByteCruncher,
    wayland_sock::WaylandSockMsg};

pub mod constants {
    pub const WL_DISPLAY_EVENT_ERROR: u16 = 0;
    pub const WL_DISPLAY_EVENT_DELETE_ID: u16 = 1;
}

use constants::*;

#[derive(Clone, Copy, Debug)]
pub enum WaylandDisplayEventCode{
    InvalidObject,
    InvalidMethod,
    NoMemory,
    Implementation,
    DeleteID
}

impl WaylandDisplayEventCode {
    pub fn from(code: u32) -> Option<WaylandDisplayEventCode>{
        match code{
            0 => Some(WaylandDisplayEventCode::InvalidObject),
            1 => Some(WaylandDisplayEventCode::InvalidMethod),
            2 => Some(WaylandDisplayEventCode::NoMemory),
            3 => Some(WaylandDisplayEventCode::Implementation),
            _ => None
        }
    }
}

impl Display for WaylandDisplayEventCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let my_str = match self{
            WaylandDisplayEventCode::InvalidObject => "InvalidObject",
            WaylandDisplayEventCode::InvalidMethod => "InvalidMethod",
            WaylandDisplayEventCode::NoMemory => "NoMemory",
            WaylandDisplayEventCode::Implementation => "Implementation",
            WaylandDisplayEventCode::DeleteID => "DeleteID"
        };

        write!(f, "{}", my_str)
    }
}

#[derive(Debug, Clone)]
pub struct WaylandDisplayEvent{
    id: u32,
    code: WaylandDisplayEventCode,
    message: String
}

impl Display for WaylandDisplayEvent{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display Event ID: {}, Code: {}, Message: {}", self.id, self.code, self.message)
    }
}

impl WaylandDisplayEvent{
    // ***** Public Functions *****
    pub fn id(&self) -> u32{
        self.id
    }

    pub fn code(&self) -> WaylandDisplayEventCode{
        self.code.clone()
    }

    pub fn message(&self) -> String{
        self.message.clone()
    }

    // ***** Private Functions *****

    // ***** Init Struct *****
    pub fn from(msg: WaylandSockMsg) -> WaylandDisplayEvent{
        match msg.opcode(){
            WL_DISPLAY_EVENT_ERROR =>{
                let mut cruncher = ByteCruncher::from(msg.message());
                let new_id: u32 = cruncher.crunch_sized();
                let new_code: u32 = cruncher.crunch_sized();
                let msg_len: u32 = cruncher.crunch_sized();
                let error_msg: String = cruncher.crunch_unsized(msg_len as usize);
                WaylandDisplayEvent{
                    id: new_id,
                    code: WaylandDisplayEventCode::from(new_code).unwrap(),
                    message: error_msg
                }
            }, WL_DISPLAY_EVENT_DELETE_ID => {
                let mut cruncher = ByteCruncher::from(msg.message());
                WaylandDisplayEvent{
                    id: cruncher.crunch_sized(),
                    code: WaylandDisplayEventCode::DeleteID,
                    message: "".to_string()
                }
            }, _=>{
                println!("Unrecognized opcode in WaylandDisplayEvent: {}", msg);
                WaylandDisplayEvent{
                    id: 0,
                    code: WaylandDisplayEventCode::Implementation,
                    message: String::new()
                }
            }
        }
    }
}