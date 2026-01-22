use anchor_client::{
    anchor_lang::{self, declare_id},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
    Client, Cluster, Program,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs, rc::Rc};

mod generated;

declare_id!("48WQW8ZMQKJhV1FKnGrYVDMEoqc8XutQmvKuqcmRrKux");

#[derive(Parser)]
#[command(name = "solana-token-cli")]
#[command(about = "CLI for interacting with the Solana Token program", long_about = None)]
struct Cli {
    /// RPC URL for the Solana cluster
    #[arg(short, long, default_value = "http://localhost:8899")]
    rpc_url: String,

    /// Path to the payer keypair file
    #[arg(short, long, default_value = "~/.config/solana/id.json")]
    keypair: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new token mint
    Init {
        /// Number of decimal places for the token
        #[arg(short, long)]
        decimals: u8,

        /// Path to mint keypair (will generate new if not provided)
        #[arg(short, long)]
        mint_keypair: Option<String>,
    },
    /// Create a token account for a user
    CreateAccount {
        /// Address of the token mint
        #[arg(short, long)]
        mint: String,

        /// Owner of the token account (defaults to payer if not provided)
        #[arg(short, long)]
        owner: Option<String>,
    },
    /// Mint tokens to a token account
    Mint {
        /// Address of the token mint
        #[arg(short, long)]
        mint: String,

        /// Owner address to mint tokens to
        #[arg(short, long)]
        to: String,

        /// Amount of tokens to mint
        #[arg(short, long)]
        amount: u64,
    },
    /// Transfer tokens between accounts
    Transfer {
        /// Address of the token mint
        #[arg(short, long)]
        mint: String,

        /// Recipient owner address
        #[arg(short, long)]
        to: String,

        /// Amount of tokens to transfer
        #[arg(short, long)]
        amount: u64,
    },
    /// Burn tokens from a token account
    Burn {
        /// Address of the token mint
        #[arg(short, long)]
        mint: String,

        /// Amount of tokens to burn
        #[arg(short, long)]
        amount: u64,
    },
    /// Get token account balance
    Balance {
        /// Address of the token mint
        #[arg(short, long)]
        mint: String,

        /// Owner address (defaults to payer if not provided)
        #[arg(short, long)]
        owner: Option<String>,
    },
    /// Get mint information
    MintInfo {
        /// Address of the token mint
        #[arg(short, long)]
        mint: String,
    },
}

fn load_keypair(path: &str) -> Result<Keypair> {
    let expanded_path = shellexpand::tilde(path);
    let keypair_bytes = fs::read(expanded_path.as_ref())?;
    let keypair = Keypair::try_from(&keypair_bytes[..])?;
    Ok(keypair)
}

fn derive_token_account(owner: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"token", owner.as_ref(), mint.as_ref()], &ID)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load payer keypair
    let payer = load_keypair(&cli.keypair)?;
    let payer = Rc::new(payer);

    // Create client and program
    let cluster = Cluster::Custom(cli.rpc_url.clone(), cli.rpc_url.clone());
    let client = Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed());
    let program = client.program(ID)?;

    match cli.command {
        Commands::Init {
            decimals,
            mint_keypair,
        } => {
            println!("TODO: implement init command");
            println!("  decimals: {}", decimals);
            println!("  mint_keypair: {:?}", mint_keypair);
        }
        Commands::CreateAccount { mint, owner } => {
            println!("TODO: implement create-account command");
            println!("  mint: {}", mint);
            println!("  owner: {:?}", owner);
        }
        Commands::Mint { mint, to, amount } => {
            println!("TODO: implement mint command");
            println!("  mint: {}", mint);
            println!("  to: {}", to);
            println!("  amount: {}", amount);
        }
        Commands::Transfer { mint, to, amount } => {
            println!("TODO: implement transfer command");
            println!("  mint: {}", mint);
            println!("  to: {}", to);
            println!("  amount: {}", amount);
        }
        Commands::Burn { mint, amount } => {
            println!("TODO: implement burn command");
            println!("  mint: {}", mint);
            println!("  amount: {}", amount);
        }
        Commands::Balance { mint, owner } => {
            println!("TODO: implement balance command");
            println!("  mint: {}", mint);
            println!("  owner: {:?}", owner);
        }
        Commands::MintInfo { mint } => {
            println!("TODO: implement mint-info command");
            println!("  mint: {}", mint);
        }
    }

    Ok(())
}
