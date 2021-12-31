//! Helpers for building the tokio I/O stacks for the wire protocol.

use tokio_util::codec;

/// Type of the wire protocol stack (combined source and sink).
///
/// This is a length-delimited CBOR protocol expecting `Input` in the source and providing
/// `Output` at the sink.
pub type Stack<Input, Output> = tokio_serde::Framed<
    codec::Framed<tokio::net::TcpStream, codec::LengthDelimitedCodec>,
    Input,
    Output,
    tokio_serde::formats::Cbor<Input, Output>,
>;

/// Creates a wire protocol stack on top of a TCP stream.
#[must_use]
pub fn build<Input, Output>(socket: tokio::net::TcpStream) -> Stack<Input, Output> {
    // This specific setup heavily inspired by:
    // https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
    let transport = codec::Framed::new(socket, codec::LengthDelimitedCodec::new());
    tokio_serde::Framed::new(transport, tokio_serde::formats::Cbor::default())
}
