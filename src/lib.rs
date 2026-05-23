mod wayland;
mod wayland_sock;
mod wayland_sock_msg;
mod wayland_display;
mod wayland_registry;
mod wayland_object;
mod wayland_callback;

pub mod prelude{
    pub use crate::wayland_object::WaylandObject;
    pub use crate::wayland::Wayland;
    pub use crate::wayland_display::WaylandDisplay;
    pub use crate::wayland_registry::WaylandRegistry;
    pub use crate::wayland_sock::WaylandSock;
    pub use crate::wayland_sock_msg::WaylandSockMsg;
    pub use crate::wayland_callback::WaylandCallback;
}