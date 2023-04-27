#[cfg(feature = "http-client")]
pub mod http_client;

// It will be imported from wasi crate eventually
pub(crate) mod preview_2 {
    wit_bindgen::generate!({
        world: "proxy",
    });
}

mod snapshots {
    pub mod preview_2 {
        // Re-export HTTP related bindings
        pub use crate::preview_2::{default_outgoing_http, types};
    }
}
