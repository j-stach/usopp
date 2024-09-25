
use std::fmt::Display;
use serde::{ Serialize, Deserialize };

pub trait NodeId: Clone + Display + Serialize + Deserialize<'static> {}


