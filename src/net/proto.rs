/*! Protobuf support for the zombiesplit API.

This module pulls in the autogenerated code coming from `prost`'s code generation pass over
`proto/zombiesplit.proto`, and also contains code for `encode`-ing into, and `decode`-ing out of,
the protobuf representation of the model.
*/

// Required because prost does not generate pedantic-clean code.
#![allow(clippy::derive_partial_eq_without_eq, clippy::pedantic)]

pub mod decode;
pub mod encode;

tonic::include_proto!("zombiesplit");
