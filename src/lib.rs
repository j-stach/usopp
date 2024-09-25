
/// Mesh component.
pub mod node;

/// Instructions for nodes.
pub mod signal;

/// Communication procedures.
/// (Scheduling, collision, security, etc.)
pub mod comms;
// TODO, these should use functional builder approach with closures.

/// Traits for building radio hardware interfaces.
pub mod hw;

/// Distributed logic for hive activity and coherency.
pub mod swarm;

///// Mesh geometry algorithms. 
///// (Structure, routing, movement & physics, healing, etc.)
///// Math only, no signals sent here.
////pub mod mesh;
//
///// Signal communication for a mesh of radio nodes.
///// (Triangulation, relay, maneuvering, replacement, etc.)
////pub mod hive;
//
//
///// Centrally-coordinated logic for swarm activity.
///// Calculates the tree of drone instructions and maneuvers to execute.
////pub mod task;
//
///// Procedural/functional implementation of multiple individual tasks.
////pub mod mission;
//

