
use super::Signal;

impl Signal {

    pub fn serialize(&self) -> Result<Vec<u8>, anyhow::Error> {
        bincode::serialize(self)?
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, anyhow::Error> {
        bincode::deserialize(data)?
    }

}
