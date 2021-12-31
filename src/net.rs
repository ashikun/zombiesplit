/*! zombiesplit client/server netcode.

zombiesplit uses a client/server separation whereby the client and server talk to each other using
a length-delimited CBOR extrusion of the action (client to server) and event (server to client)
enumerations in the attempt model.  The netcode is asynchronous and built using tokio. */

pub mod client;
mod io;
pub mod server;

pub use client::Client;
pub use server::Manager;
