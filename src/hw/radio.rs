
use crate::node::NodeId;
use crate::signal::Packet;

pub trait Transmitter<I: NodeId> {
    fn send(&self, data: &[u8]) -> Result<(), anyhow::Error>;

    // todo retry procedure from comms
    //fn signal(&self, packet: Packet<I>, retry: Retry);
}

pub trait Receiver<I: NodeId> {
    fn receive(&self) -> Option<Packet<I>>;
}


