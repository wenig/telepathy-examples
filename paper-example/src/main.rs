mod example;
use clap::Parser;
use std::net::SocketAddr;

fn main() {
    let args = Parameters::parse();
    example::main(args.get_own_addr(), args.get_seed_nodes());
}

/// Example to show the easy transition to distributed applications with Actix-Telepathy
#[derive(Parser, Debug)]
struct Parameters {
    /// Own public IP address to listen to
    #[clap(long)]
    own_addr: String,

    /// Addresses of already known cluster nodes
    #[clap(long)]
    seed_nodes: Vec<String>
}

impl Parameters {
    pub fn get_own_addr(&self) -> SocketAddr {
        self.own_addr.parse().unwrap()
    }

    pub fn get_seed_nodes(&self) -> Vec<SocketAddr> {
        self.seed_nodes.iter().map(|addr| addr.parse().unwrap()).collect()
    }
}
