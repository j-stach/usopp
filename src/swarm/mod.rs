

use std::collections::HashMap;
use crate::node::{ Node, NodeId };

pub struct Register<I: NodeId>(HashMap<I, f32>);

impl Register<I: NodeId> {

    pub fn assign_freq(&mut self, node: &Node) -> Result<(), anyhow::Error> {
        if let Some(freq) = open_freq(&self) {
            *node.freq = freq;
            self.insert(node.id.clone(), freq);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No frequencies available!"))
        }
    }

    pub fn open_freq(&self) -> Option<f32> {
        // Use total bandwidth/range, 
        // divided by bandwidth for protocol & 
        // account for guard width
    }
}

