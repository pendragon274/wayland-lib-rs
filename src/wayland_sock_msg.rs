use std::fmt::Display;

pub struct WaylandSockMsg{
    objID: u32,
    opcode: u16,
    msgLen: u16,
    msg: Vec<u8>
}

impl Display for WaylandSockMsg{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>{
        match String::from_utf8(self.msg.clone()){
            Ok(s) => {
                write!(f, "ObjID: {}, Len: {}, Opcode: {}, Message: {}", self.objID, self.msgLen, self.opcode, s);
            }, Err(_) => {
                let mut str = String::from("");
                for u in &self.msg{
                    str.push(*u as char);
                }
                write!(f, "ObjID: {}, Len: {}, Opcode: {}, Message: {}", self.objID, self.msgLen, self.opcode, str);
            }
        }

        Ok(())
    }
}

impl WaylandSockMsg{
    pub fn new(objID: u32, opcode: u16, msg: Vec<u8>) -> WaylandSockMsg{
        WaylandSockMsg{
            objID: objID,
            opcode: opcode,
            msgLen: (8 + msg.len()) as u16,
            msg: msg
        }
    }

    pub fn from(rawMsg: Vec<u8>) -> WaylandSockMsg{
        if rawMsg.len() < 8{
            return WaylandSockMsg{
                objID: 0,
                opcode: 0,
                msgLen: 8,
                msg: Vec::new()
            };
        }

        let len = u16::from_ne_bytes(rawMsg[6..8].try_into().unwrap());
        if len > 8 {
            WaylandSockMsg {
                objID: u32::from_ne_bytes(rawMsg[0..4].try_into().unwrap()),
                opcode: u16::from_ne_bytes(rawMsg[4..6].try_into().unwrap()),
                msgLen: len,
                msg: rawMsg[8..(len as usize)].to_vec()
            }
        }else{
            WaylandSockMsg{
                objID: u32::from_ne_bytes(rawMsg[0..4].try_into().unwrap()),
                opcode: u16::from_ne_bytes(rawMsg[4..6].try_into().unwrap()),
                msgLen: len,
                msg: Vec::new()
            }
        }
    }

    pub fn to_raw_vec(&self) -> Vec<u8>{
        let mut raw: Vec<u8> = Vec::with_capacity(self.msgLen as usize);

        let ne_objID = self.objID.to_ne_bytes();
        for i in 0..4{
            raw.push(ne_objID[i]);
        }

        let ne_opcode = self.opcode.to_ne_bytes();
        for i in 0..2{
            raw.push(ne_opcode[i]);
        }

        let ne_msgLen = self.msgLen.to_ne_bytes();
        for i in 0..2{
            raw.push(ne_msgLen[i]);
        }

        if self.msgLen > 8 {
            for i in 0..(self.msgLen - 8) as usize {
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
        self.msgLen
    }
}