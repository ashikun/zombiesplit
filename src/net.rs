/*! zombiesplit client/server netcode.

zombiesplit uses a client/server separation whereby the client and server talk to each other using
a protocol defined in `proto`.  The netcode is asynchronous and built using tokio. */

pub mod client;
pub mod metadata;
pub mod proto;
pub mod server;

pub use client::Client;
pub use server::Manager;
