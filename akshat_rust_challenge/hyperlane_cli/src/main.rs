use clap::{Arg, Command};
use ethers::prelude::*;
use hex::decode;
use std::convert::TryFrom;

#[tokio::main]
async fn main() {
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
                .arg(
                    Arg::new("rpc_url")
                        .required(true)
                        .help("The RPC URL"),
                )
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

    if let Some(matches) = matches.subcommand_matches("send") {
        let origin_chain = matches.get_one::<String>("origin_chain").unwrap();
        let mailbox_address = matches.get_one::<String>("mailbox_address").unwrap();
        let rpc_url = matches.get_one::<String>("rpc_url").unwrap();
        let destination_address = matches.get_one::<String>("destination_address").unwrap();
        let message_bytes = matches.get_one::<String>("message_bytes").unwrap();

        println!("Sending message...");
        println!("Origin Chain: {}", origin_chain);
        println!("Mailbox Address: {}", mailbox_address);
        println!("RPC URL: {}", rpc_url);
        println!("Destination Address: {}", destination_address);
        println!("Message Bytes: {}", message_bytes);

        // Call the function to send a message
        if let Err(e) = send_message(mailbox_address, rpc_url, destination_address, message_bytes).await {
            eprintln!("Error sending message: {}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("query") {
        let chain = matches.get_one::<String>("chain").unwrap();
        let matching_list = matches.get_one::<String>("matching_list").unwrap();

        println!("Querying messages...");
        println!("Chain: {}", chain);
        println!("Matching List: {}", matching_list);

        // Call the function to query messages
        if let Err(e) = query_messages(chain, matching_list).await {
            eprintln!("Error querying messages: {}", e);
        }
    }
}

async fn send_message(mailbox_address: &str, rpc_url: &str, destination_address: &str, message_bytes: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Ethereum provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Load the private key
    let private_key = "mour_private_key";  // Make sure to replace this with your actual private key
    let wallet: LocalWallet = private_key.parse()?;
    let client = SignerMiddleware::new(provider, wallet.clone());

    // Interact with the Mailbox contract to send the message
    let mailbox: Address = mailbox_address.parse()?;
    let _destination: Address = destination_address.parse()?;  // Prefix with underscore to suppress unused variable warning
    let data = decode(message_bytes)?;

    let tx: TransactionRequest = TransactionRequest::new()
        .to(mailbox)
        .data(data)
        .from(wallet.address())
        .into();

    let pending_tx = client.send_transaction(tx, None).await?;
    let receipt = pending_tx.await?;

    println!("Message sent with transaction hash: {:?}", receipt.unwrap().transaction_hash);
    Ok(())
}

async fn query_messages(chain: &str, matching_list: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Ethereum provider
    let rpc_url = "rpc_url_for_chain";  // Replace with the actual RPC URL for the chain
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Define the Mailbox contract address and the event signature
    let mailbox_address: Address = "mailbox_contract_address".parse()?;  // Replace with actual contract address
    let event_signature = "event_signature";  // Replace with actual event signature

    // Fetch events from the Mailbox contract
    let filter = Filter::new()
        .address(mailbox_address)
        .event(event_signature);

    let logs = provider.get_logs(&filter).await?;

    // Filter logs based on the MatchingList
    for log in logs {
        // Implement filtering logic based on matching_list
        println!("Log: {:?}", log);
    }

    Ok(())
}