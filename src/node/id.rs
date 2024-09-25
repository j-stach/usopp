
use std::fmt::Display;
use std::hash::Hash;
use serde::{ Serialize, Deserialize };

pub trait NodeId: Clone + Display + PartialEq + Eq + Hash + Serialize + Deserialize<'static> {}


