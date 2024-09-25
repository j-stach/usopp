
// Signal types depend on node capabilities

use crate::swarm::Register;

pub mod id;
pub use id::NodeId;

pub struct Node<I: NodeId, D> {
    id: I,
    freq: f32,
    inner: D,
    neighbors: Vec<I>,
    register: Register<I>
    //register: bool

    // tbd
    // Node should track changes to the mesh
    // Track node frequencies for neighbors
    // Track neighbors
    // Cache shortest route to node?
}

// Neighbor node id to comms::frequency

// Energy management as part of node type?

// Configuration for node hw




