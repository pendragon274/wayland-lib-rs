mod wayland;

pub use wayland::Wayland;
pub use wayland::util;
pub use wayland::wayland_sock;
pub use wayland::wayland_event_buf;
pub use wayland::wayland_object;

pub mod prelude{
    pub use crate::{
        Wayland,
        events_prelude::*};
}

pub mod events_prelude{
    pub use crate::{
        util::WaylandIDCounter,
        wayland_event_buf::WaylandEventBuffer,
        wayland_object::{
            wayland_callback::{
                WaylandCallback,
                WaylandCallbackHandle},
            wayland_display::{
                WaylandDisplayEvent,
                DisplayCallbackHandle},
            wayland_registry::{
                WaylandRegistryEvent,
                RegistryCallbackHandle}}};
}