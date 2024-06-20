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
                        .help("The message bytes (hex encoded)"),
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

        // Print the arguments to ensure they are parsed correctly
        println!("Parsed arguments: origin_chain={}, mailbox_address={}, rpc_url={}, destination_address={}, message_bytes={}",
            origin_chain, mailbox_address, rpc_url, destination_address, message_bytes);

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
    println!("Initializing Ethereum provider...");
    let provider = match Provider::<Http>::try_from(rpc_url) {
        Ok(provider) => provider,
        Err(e) => {
            eprintln!("Failed to initialize provider: {}", e);
            return Err(Box::new(e));
        }
    };

    // Load the private key
    println!("Loading private key...");
    let private_key = "0x96fd32b1facd7d859dc3c6dfaa730694ea34c00741f2729a5df236233788077d";
    let wallet: LocalWallet = match private_key.parse() {
        Ok(wallet) => wallet,
        Err(e) => {
            eprintln!("Failed to parse private key: {}", e);
            return Err(Box::new(e));
        }
    };
    let client = SignerMiddleware::new(provider, wallet.clone());

    // Interact with the Mailbox contract to send the message
    println!("Parsing mailbox address...");
    let mailbox: Address = match mailbox_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to parse mailbox address: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Parsing destination address...");
    let _destination: Address = match destination_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to parse destination address: {}", e);
            return Err(Box::new(e));
        }
    };

    // Decode the message bytes independently
    println!("Decoding message bytes...");
    let data = match decode(message_bytes) {
        Ok(d) => {
            println!("Decoded message bytes: {:?}", d);
            d
        }
        Err(e) => {
            eprintln!("Failed to decode message bytes: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Creating transaction request...");
    let tx: TransactionRequest = TransactionRequest::new()
        .to(mailbox)
        .data(data)
        .from(wallet.address())
        .into();

    println!("Sending transaction...");
    let pending_tx = match client.send_transaction(tx, None).await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to send transaction: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Awaiting transaction receipt...");
    let receipt = match pending_tx.await {
        Ok(receipt) => receipt,
        Err(e) => {
            eprintln!("Failed to get transaction receipt: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Message sent with transaction hash: {:?}", receipt.unwrap().transaction_hash);
    Ok(())
}

async fn query_messages(_chain: &str, _matching_list: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Implement the logic to query messages from the blockchain
    // Example: Fetch events from the Mailbox contract and filter based on the MatchingList
    println!("Querying messages for chain: {} with matching list: {}", _chain, _matching_list);
    // TODO: Implement the query logic
    Ok(())
}

//0x4ac7A40722277121045B119b81AC69AC8577319b
//cargo run -- send origin_chain 0x4ac7A40722277121045B119b81AC69AC8577319b http://127.0.0.1:8545 0x5AD9E93A5eE9F33cc4c4d94e1a186f61D7e1CB35 68656c6c6f20776f726c64