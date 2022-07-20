pub mod error;
pub mod event;
pub mod node;
use async_std::{io, task};

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut node: node::Node = node::Node::new().await?;
    node.listen().await?;
    // node.subscribe(node::TOPIC);
    let e = node.start().await;
    if let Err(err) = e {
        println!("{}", err);
        return Err(err)
    }

    Ok(())
}
