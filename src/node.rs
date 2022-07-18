use anyhow::{anyhow, Result};
use async_std::{io, task};
use futures::{
    prelude::{stream::StreamExt, *},
    select,
};
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::SwarmEvent,
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};

// use crate::error::NodeError;

// We create a custom network behaviour that combines floodsub and mDNS.
// In the future, we want to improve libp2p to make this easier to do.
// Use the derive to generate delegating NetworkBehaviour impl and require the
// NetworkBehaviourEventProcess implementations below.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct MyBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,

    // Struct fields which do not implement NetworkBehaviour need to be ignored
    #[behaviour(ignore)]
    #[allow(dead_code)]
    ignored_member: bool,
}

#[derive(Debug)]
enum OutEvent {
    Floodsub(FloodsubEvent), // events related to pubsub topics
    Mdns(MdnsEvent),         // events releated to peer discovery
}

impl From<MdnsEvent> for OutEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl From<FloodsubEvent> for OutEvent {
    fn from(v: FloodsubEvent) -> Self {
        Self::Floodsub(v)
    }
}

pub struct Node {
    key: identity::Keypair,
    behaviour: MyBehaviour,
    swarm: Swarm<MyBehaviour>,
    pub id: PeerId,
    pub topics: Vec<floodsub::Topic>,
}

impl Node {
    pub async fn new() -> Result<Self> {
        // Create a random PeerId
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // Set up an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
        let transport = libp2p::development_transport(local_key.clone()).await?;

        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;

        let behaviour = MyBehaviour {
            floodsub: Floodsub::new(local_peer_id),
            mdns,
            ignored_member: false,
        };

        let swarm = {
            let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
            let behaviour = MyBehaviour {
                floodsub: Floodsub::new(local_peer_id),
                mdns,
                ignored_member: false,
            };

            Swarm::new(transport, behaviour, local_peer_id)
        };

        let n = Self {
            key: local_key,
            id: local_peer_id,
            topics: Vec::new(),
            behaviour: behaviour,
            swarm: swarm,
        };
        Ok(n)
    }

    pub fn subscribe(&mut self, topic: &str) -> bool {
        let t = floodsub::Topic::new(topic);
        self.behaviour.floodsub.subscribe(t.clone())
    }

    pub fn publish(&mut self, topic: &str, msg: impl Into<Vec<u8>>) {
        let t = floodsub::Topic::new(topic);
        self.behaviour.floodsub.publish_any(t, msg)
    }


    pub fn dial_peer(&mut self, addr: Multiaddr) -> Result<()> {
        println!("dialing {:?}", &addr);
        match self.swarm.dial(addr) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                println!("failed to dial peer: {}", e);
                return Err(anyhow!(e));
            }
        }
    }

    async fn event_loop(&mut self) -> Result<()> {
        // Kick it off
        loop {
            select! {
                // line = stdin.select_next_some() => self.swarm
                //     .behaviour_mut()
                //     .floodsub
                //     .publish(floodsub_topic.clone(), line.expect("Stdin not to close").as_bytes()),

                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {:?}", address);
                    }
                    SwarmEvent::Behaviour(OutEvent::Floodsub(
                        FloodsubEvent::Message(message)
                    )) => {
                        println!(
                            "Received: '{:?}' from {:?}",
                            String::from_utf8_lossy(&message.data),
                            message.source
                        );
                    }
                    SwarmEvent::Behaviour(OutEvent::Mdns(
                        MdnsEvent::Discovered(list)
                    )) => {
                        for (peer, _) in list {
                            self.swarm
                                .behaviour_mut()
                                .floodsub
                                .add_node_to_partial_view(peer);
                        }
                    }
                    SwarmEvent::Behaviour(OutEvent::Mdns(MdnsEvent::Expired(
                        list
                    ))) => {
                        for (peer, _) in list {
                            if !self.swarm.behaviour_mut().mdns.has_node(&peer) {
                                self.swarm
                                    .behaviour_mut()
                                    .floodsub
                                    .remove_node_from_partial_view(&peer);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    pub async fn listen(&mut self) -> Result<()> {
        // Read full lines from stdin
        // let mut stdin = io::BufReader::new(io::stdin()).lines().as_bytes();

        // Listen on all interfaces and whatever port the OS assigns
        let listen_id = self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        println!("listener id: {:?}", listen_id);
        Ok(())
        // self.event_loop().await
    }
}
