pub mod error;
pub mod node;
use async_std::{io, task};

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut node: node::Node = node::Node::new().await?;
    node.listen().await?;
    Ok(())
}
