use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{Keypair, Signer},
    },
    Client, Cluster,
};
use solana_token_cli::{init, ID};
use std::rc::Rc;

fn setup_program() -> (anchor_client::Program<Rc<Keypair>>, Rc<Keypair>) {
    let payer = Rc::new(Keypair::new());
    let cluster = Cluster::Localnet;
    let client = Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed());
    let program = client.program(ID).expect("Failed to create program client");
    (program, payer)
}

#[tokio::test]
#[ignore] // Requires local validator with deployed program and funded payer
async fn test_init() {
    let (program, payer) = setup_program();

    let result = init(&program, &payer, 9, None).await;

    assert!(result.is_ok());
}
