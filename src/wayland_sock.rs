use std::os::unix::net::UnixStream;
use std::env::var;
use std::net::Shutdown;
use std::io::{Error, ErrorKind, Read, Result, Write};
use crate::wayland_sock_msg::WaylandSockMsg;

pub struct WaylandSock{
    stream: Option<UnixStream>
}

impl Drop for WaylandSock{
    fn drop(&mut self){
        match self.stream{
            Some(ref mut stream) => {
                stream.shutdown(Shutdown::Both).expect("Shutdown failed on connection to wayland server.");
                println!("Connection to wayland closed.");
            },
            None => {}
        };
    }
}

impl WaylandSock{
    fn gen_err() -> Result<()>{
        Err(Error::new(ErrorKind::NotConnected, "NotConnected"))
    }

    fn read_all(&mut self) -> Result<Vec<u8>>{
        let ref mut my_stream: UnixStream;
        match &mut self.stream{
            Some(stream_ref) => {
                my_stream = stream_ref;
            }, None => {
                return Err(Error::new(ErrorKind::NotConnected, "NotConnected"));
            }
        };

        let mut vec: Vec<u8> = Vec::new();
        match my_stream.read_to_end(&mut vec){
            Ok(len) =>{
                Ok(vec[0..len].to_vec())
            }, Err(e) => {
                match e.kind(){
                    ErrorKind::WouldBlock =>{
                        Ok(vec)
                    }, _ => {
                        println!("Read to end failed: {:?}", e);
                        Err(e)
                    }
                }
            }
        }
    }

    fn read_single_message(&mut self) -> Result<Vec<u8>>{
        let ref mut my_stream: UnixStream;
        match &mut self.stream{
            Some(stream_ref) => {
                my_stream = stream_ref;
            }, None => {
                return Err(Error::new(ErrorKind::NotConnected, "NotConnected"));
            }
        };

        let mut header = [0 as u8; 8];
        match my_stream.read_exact(&mut header){
            Ok(()) => {
                let len: u16 = u16::from_ne_bytes(header[6..8].try_into().unwrap());
                if len <= 8{
                    return Ok(header.to_vec());
                }

                let mut vec: Vec<u8> = Vec::with_capacity((len as usize) - 8);
                for _ in 0..(len - 8){
                    vec.push(0);
                }
                match my_stream.read_exact(&mut vec){
                    Ok(()) =>{
                        let mut ret = Vec::from(header);
                        ret.append(&mut vec);
                        Ok(ret)
                    }, Err(e) => {
                        Err(e)
                    }
                }
            }, Err(e) => {
                match e.kind(){
                    ErrorKind::WouldBlock => Ok(Vec::new()),
                    _ => Err(e)
                }
            }
        }
    }

    fn write_all(&mut self, msg: &[u8]) -> Result<()>{
        match self.stream{
            Some(ref mut stream) => {
                stream.write_all(msg)?;
                stream.flush()
            }, None => {
                Self::gen_err()
            }
        }
    }

    pub fn connected(&self) -> bool{
        self.stream.is_some()
    }

    pub fn write(&mut self, sock_msg: WaylandSockMsg) -> Result<()>{
        self.write_all(&sock_msg.to_raw_vec())
    }

    pub fn read(&mut self) -> Result<WaylandSockMsg>{
        let res = self.read_single_message();
        match res {
            Ok(msg) => {
                Ok(WaylandSockMsg::from(msg))
            }, Err(e) => {
                Err(e)
            }
        }
    }

    pub fn new() -> WaylandSock{
        let wayland_display = var("WAYLAND_DISPLAY").unwrap_or_else(|_| String::from("wayland-0"));

        let runtime_dir;
        match var("XDG_RUNTIME_DIR"){
            Ok(val) => runtime_dir = val,
            Err(_) => return WaylandSock{stream: None}
        }

        match UnixStream::connect(runtime_dir + "/" + &wayland_display) {
            Ok(sock) => {
                println!("Connection to wayland opened.");
                match sock.set_nonblocking(true){
                    Ok(_) => WaylandSock{stream: Some(sock)},
                    Err(_) => WaylandSock{stream: None}
                }
            }, Err(_) => WaylandSock{
                stream: None
            }
        }
    }
}