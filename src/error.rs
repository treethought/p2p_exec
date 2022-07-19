use std::fmt;

#[derive(Debug)]
pub enum NodeError {
    SwarmNotInitialized,
    InvalidExecFunction,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeError::SwarmNotInitialized => write!(f, "swarm not initialized"),
            NodeError::InvalidExecFunction => write!(f, "invalid exec function name"),
        }
    }
}

// Implement the StdError trait
impl std::error::Error for NodeError {}
