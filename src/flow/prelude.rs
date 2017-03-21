use petgraph;
use std::cell;

// core types
use flow::core;
pub use flow::core::{NodeAddress, LocalNodeIndex};
pub use flow::core::{DataType, Record, Records, Datas};
pub use flow::core::Ingredient;

// graph types
pub use flow::node::Node;
pub use flow::Edge;
pub type Graph = petgraph::Graph<Node, Edge>;

// dataflow types
pub use flow::payload::{Link, Packet};
pub use flow::migrate::materialization::Tag;

// domain local state
use flow::domain::{local, single};
pub type DomainNodes = local::Map<cell::RefCell<single::NodeDescriptor>>;
pub type State = local::State<core::DataType>;
pub type StateMap = local::Map<State>;
pub use flow::domain::local::KeyType;
