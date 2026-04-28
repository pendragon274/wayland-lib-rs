mod wayland;
mod wayland_sock;
mod wayland_sock_msg;
mod wayland_display;
mod wayland_registry;

pub mod prelude{
    pub use crate::wayland::Wayland;
    pub use crate::wayland_sock::WaylandSock;
    pub use crate::wayland_sock_msg::WaylandSockMsg;
}