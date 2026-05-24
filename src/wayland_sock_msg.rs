use std::fmt::Display;

pub struct WaylandSockMsg{
    obj_id: u32,
    opcode: u16,
    msg_len: u16,
    msg: Vec<u8>
}

impl Display for WaylandSockMsg{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
        match String::from_utf8(self.msg.clone()){
            Ok(s) => {
                let _ = write!(f, "ObjID: {}, Len: {}, Opcode: {}, Message: {}", self.obj_id, self.msg_len, self.opcode, s);
            }, Err(_) => {
                let mut str = String::from("");
                for u in &self.msg{
                    str.push(*u as char);
                }
                let _ = write!(f, "ObjID: {}, Len: {}, Opcode: {}, Message: {}", self.obj_id, self.msg_len, self.opcode, str);
            }
        }

        Ok(())
    }
}

impl Clone for WaylandSockMsg{
    fn clone(&self) -> Self {
        WaylandSockMsg{
            obj_id: self.obj_id,
            opcode: self.opcode,
            msg_len: self.msg_len,
            msg: self.msg.clone()
        }
    }
}

impl WaylandSockMsg{
    pub fn new(new_obj_id: u32, opcode: u16, msg: Vec<u8>) -> WaylandSockMsg{
        WaylandSockMsg{
            obj_id: new_obj_id,
            opcode: opcode,
            msg_len: (8 + msg.len()) as u16,
            msg: msg
        }
    }

    pub fn from(raw_msg: Vec<u8>) -> WaylandSockMsg{
        if raw_msg.len() < 8{
            return WaylandSockMsg{
                obj_id: 0,
                opcode: 0,
                msg_len: 8,
                msg: Vec::new()
            };
        }

        let len = u16::from_ne_bytes(raw_msg[6..8].try_into().unwrap());
        if len > 8 {
            WaylandSockMsg {
                obj_id: u32::from_ne_bytes(raw_msg[0..4].try_into().unwrap()),
                opcode: u16::from_ne_bytes(raw_msg[4..6].try_into().unwrap()),
                msg_len: len,
                msg: raw_msg[8..(len as usize)].to_vec()
            }
        }else{
            WaylandSockMsg{
                obj_id: u32::from_ne_bytes(raw_msg[0..4].try_into().unwrap()),
                opcode: u16::from_ne_bytes(raw_msg[4..6].try_into().unwrap()),
                msg_len: len,
                msg: Vec::new()
            }
        }
    }

    pub fn to_raw_vec(&self) -> Vec<u8>{
        let mut raw: Vec<u8> = Vec::with_capacity(self.msg_len as usize);

        let ne_obj_id = self.obj_id.to_ne_bytes();
        for i in 0..4{
            raw.push(ne_obj_id[i]);
        }

        let ne_opcode = self.opcode.to_ne_bytes();
        for i in 0..2{
            raw.push(ne_opcode[i]);
        }

        let ne_msg_len = self.msg_len.to_ne_bytes();
        for i in 0..2{
            raw.push(ne_msg_len[i]);
        }

        if self.msg_len > 8 {
            for i in 0..(self.msg_len - 8) as usize {
                raw.push(self.msg[i]);
            }
        }

        raw
    }

    pub fn byte_str(&self) -> String{
        let vec = self.to_raw_vec();
        let mut prelim: String = String::new();
        let len = vec.len();
        let mut cur = 0;
        for i in vec{
            let chars = i.to_string();
            prelim.push_str(&chars);

            if cur < len - 1{
                prelim.push(',');
            }

            cur += 1;
        }

        prelim
    }

    pub fn message_len(&self) -> u16{
        self.msg_len
    }

    pub fn message_id(&self) -> u32{
        self.obj_id
    }
    
    pub fn opcode(&self) -> u16{
        self.opcode
    }
    
    pub fn message(&self) -> Vec<u8>{
        self.msg.clone()
    }
}