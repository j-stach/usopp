
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


pub struct Package<I: NodeId> {
    sender: I,
    recipient: I,
    signal: Signal,
}

#[derive(Serialize, Deserialize)]
pub enum Signal {
    CORE(CoreInstruction),
    MOVE(MoveInstruction),
    SWARM(SwarmInstruction),
    ASSEMBLE(AssemblyInstruction), 
    ALERT(Alert),
}

#[derive(Serialize, Deserialize)]
pub enum CoreInstruction {
    PING,
    IDENTIFY,
    STATUS,
    EXECUTE,
    ACKNOWLEDGE,
    RELAY,
    ROUTE,
}

#[derive(Serialize, Deserialize)]
pub enum MoveInstruction {
    HOLD,
    ORIENT,
    APPROACH,
    RETREAT,
    KITE,
    ORBIT,
    ACCELERATE,
}

#[derive(Serialize, Deserialize)]
pub enum SwarmInstruction {
    HOWDY,
    SYNCHRONIZE,
    FOLLOW,
    DISENGAGE,
    FORMATION,
    SCATTER,
    REGROUP,
}

#[derive(Serialize, Deserialize)]
pub enum AssemblyInstruction {
    DOCK,
    RELEASE,
}

#[derive(Serialize, Deserialize)]
pub enum Alert {
    INFO,
    HELP,
    WARN,
    ERROR,
    FAILURE,
}


