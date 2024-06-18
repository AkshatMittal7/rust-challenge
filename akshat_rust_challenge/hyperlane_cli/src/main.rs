use clap::{Arg, Command};
use ethers::prelude::*;
use std::convert::TryFrom;
use tokio::runtime::Runtime;

fn main() {
    let matches = Command::new("Hyperlane CLI")
        .version("1.0")
        .author("Akshat Mittal")
        .about("CLI for sending and querying Hyperlane messages")
        .subcommand(
            Command::new("send")
                .about("Send a message")
                .arg(
                    Arg::new("origin_chain")
                        .required(true)
                        .help("The origin chain"),
                )
                .arg(
                    Arg::new("mailbox_address")
                        .required(true)
                        .help("The mailbox address"),
                )
                .arg(Arg::new("rpc_url").required(true).help("The RPC URL"))
                .arg(
                    Arg::new("destination_address")
                        .required(true)
                        .help("The destination address"),
                )
                .arg(
                    Arg::new("message_bytes")
                        .required(true)
                        .help("The message bytes"),
                ),
        )
        .subcommand(
            Command::new("query")
                .about("Query messages")
                .arg(
                    Arg::new("chain")
                        .required(true)
                        .help("The chain to query from"),
                )
                .arg(
                    Arg::new("matching_list")
                        .required(true)
                        .help("The matching list"),
                ),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("send") {
        println!("message sent");
    } else if let Some(_matches) = matches.subcommand_matches("query") {
        println!("message queried");
    }
}
