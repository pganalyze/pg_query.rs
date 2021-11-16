mod protobuf {
	include!(concat!(env!("OUT_DIR"), "/pg_query.rs"));
}

pub type ParseResult = protobuf::ParseResult;
pub type Node = protobuf::Node;

#[allow(non_snake_case)]
pub mod Nodes {
	pub use super::protobuf::node::Node::*;
}
