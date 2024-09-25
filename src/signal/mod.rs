
#![allow(non_snake_case)]

use serde::{ Serialize, Deserialize };
use crate::node::NodeId;


pub use self::{ 
    Signal::*, 
    CoreInstruction::*,
    MoveInstruction::*,
    SwarmInstruction::*,
    AssemblyInstruction::*,
    Alert::*,
};


/// Contains the information to send.
pub struct Packet<I: NodeId> {
    sender: I,
    recipient: I,
    signal: Signal,
}

/// Signal archetypes.
#[derive(Serialize, Deserialize)]
pub enum Signal {
    /// Basic node query and response.
    CORE(CoreInstruction),
    /// Movement of a single node.
    MOVE(MoveInstruction),
    /// Coordinated networking and movement of many nodes.
    SWARM(SwarmInstruction),
    /// Sometimes, nodes can be physically combined.
    ASSEMBLE(AssemblyInstruction),
    /// Informational messages triggered during operation.
    ALERT(Alert),
}

#[derive(Serialize, Deserialize)]
pub enum CoreInstruction {
    PING, // check if target is networked
    IDENTIFY, // return id from the node that received the message
    STATUS, // broad status 
    EXECUTE, // execute custom routines and procedures
    ACKNOWLEDGE(bool), // good receipt
    RELAY, // pass a message along when direct connection is impossible
    ROUTE, // reqpuest permission to use as relay 
}

#[derive(Serialize, Deserialize)]
pub enum MoveInstruction {
    HOLD, // brake and hold at present location
    ORIENT, // delta pitch, roll, yaw
    APPROACH, // relative velovity towards increase
    RETREAT, // opposite, for clarity
    KITE, // Maintain distance
    ORBIT, // maintain distance and velocity
    ACCELERATE, // delta v
}

#[derive(Serialize, Deserialize)]
pub enum SwarmInstruction {
    HOWDY, // update the neighbors list
    SYNCHRONIZE, // match clocks and status cycles
    PAIR, // extended 1-to-1 connection
    FOLLOW, // set movement to match the sender node 
    DISENGAGE, // un-pair movement to node id
    FORMATION, // pair movement as part of flock
    SCATTER, // put the maximum distance without breaking coherency
    REGROUP, // bring all nodes into range of each other
}

#[derive(Serialize, Deserialize)]
pub enum AssemblyInstruction {
    DOCK, // align docking 
    RELEASE, // undock and free
}

#[derive(Serialize, Deserialize)]
pub enum Alert {
    INFO, // Carries generic data
    HELP, // Request assistance from neighbors 
    WARN, // warning
    ERROR, // recoverable error 
    FAILURE, // unrecoverable
}


