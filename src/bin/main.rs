use wayland_lib::prelude::*;

fn sync(id: u32){
    println!("Sync callback: {}", id);
}

fn main() {
    let mut wayland: Wayland = Wayland::new();
    println!("Wayland initialized!");
    wayland.get_display();
    let mut new_id = wayland.get_new_id();
    wayland.get_display().get_registry(new_id);
    new_id = wayland.get_new_id();
    wayland.get_display().sync(new_id).callback(sync);
    wayland.collect_upstream_send();

    loop{
        wayland.poll();
    }
}