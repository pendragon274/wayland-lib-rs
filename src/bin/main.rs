use wayland_lib::prelude::*;

fn callback(throwaway: u32){
    println!("Received callback: {}", throwaway);
}

fn main() {
    /*let mut wl: WaylandSock = WaylandSock::new();
    println!("Connected: {}", wl.connected());

    let registryID: u32 = 2;
    
    wl.write(WaylandSockMsg::new(1, 1, registryID.to_ne_bytes().to_vec()));
    println!("Wrote data: {}", WaylandSockMsg::new(1, 1, registryID.to_ne_bytes().to_vec()).byte_str());
    wl.write(WaylandSockMsg::new(1, 0, (3 as u32).to_ne_bytes().to_vec()));
    println!("Wrote data: {}", WaylandSockMsg::new(1, 0, (3 as u32).to_ne_bytes().to_vec()).byte_str());
    while true{
        let response = wl.read();
        match response{
            Ok(msg) => {
                if msg.message_len() > 8 {
                    println!("Got a response: {}", msg);
                }
            }, Err(e) => {
                println!("Got an error: {:?}", e);
            }
        }
    }
    drop(wl);
    println!("Closed.");*/

    /*let mut wayland: Wayland = Wayland::new();
    let mut display = wayland.get_display();
    let mut new_id = wayland.use_new_id();
    let registry = display.get_registry(wayland.get_sock(), new_id);
    new_id = wayland.use_new_id();
    display.sync(wayland.get_sock(), callback, new_id);*/
    let mut wayland: Wayland = Wayland::new();
    println!("Wayland initialized!");
}