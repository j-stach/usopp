
use crate::node::NodeId;
use crate::signal::Package;

pub trait Transmitter<I: NodeId> {
    fn send(&self, signal: &[u8], target: I) -> Result<(), anyhow::Error>;
}

pub trait Receiver<I: NodeId> {
    fn receive(&self) -> Option<Package<I>>;
}


