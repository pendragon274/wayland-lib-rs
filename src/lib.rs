pub mod wayland;
pub mod wayland_sock;
pub mod wayland_object;
pub mod wayland_id_counter;
pub mod wayland_event_buf;
pub mod util;

pub mod prelude{
    pub use crate::*;
    /*pub use crate::wayland_object::WaylandObject;
    pub use crate::wayland::Wayland;
    pub use crate::wayland_object::wayland_display::WaylandDisplay;
    pub use crate::wayland_object::wayland_registry::WaylandRegistry;
    pub use crate::wayland_object::wayland_registry::RegistryCallbackHandle;
    pub use crate::wayland_object::wayland_registry::wayland_registry_event::WaylandRegistryEvent;
    pub use crate::wayland_event_buf::WaylandEventBuffer;
    pub use crate::wayland_sock::WaylandSock;
    pub use crate::wayland_sock::wayland_sock_msg::WaylandSockMsg;
    pub use crate::wayland_object::wayland_callback::WaylandCallback;
    pub use crate::wayland_object::wayland_callback::WaylandCallbackHandle;
    pub use crate::wayland_object::wayland_shm::WaylandSHM;*/
}