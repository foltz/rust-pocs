use std::fmt;
use serde::{Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum ImportType {
    History,
    Realtime,
}

impl fmt::Display for ImportType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}