use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{Keypair, Signer},
    },
    Client, Cluster,
};
use solana_streamer::socket::SocketAddrSpace;
use solana_test_validator::{TestValidatorGenesis, UpgradeableProgramInfo};
use solana_token_cli::{init, ID};
use std::path::PathBuf;
use std::rc::Rc;

fn setup_validator() -> (solana_test_validator::TestValidator, Keypair) {
    let payer = Keypair::new();

    // Path to the compiled program
    let program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("target/deploy/solana_token.so");

    let mut genesis = TestValidatorGenesis::default();
    genesis.add_upgradeable_programs_with_path(&[UpgradeableProgramInfo {
        program_id: ID,
        loader: solana_sdk::bpf_loader_upgradeable::id(),
        upgrade_authority: Keypair::new().pubkey(),
        program_path,
    }]);

    let validator = genesis
        .start_with_mint_address(payer.pubkey(), SocketAddrSpace::Unspecified)
        .expect("Failed to start test validator");

    (validator, payer)
}

fn setup_program(
    validator: &solana_test_validator::TestValidator,
    payer: Rc<Keypair>,
) -> anchor_client::Program<Rc<Keypair>> {
    let cluster = Cluster::Custom(validator.rpc_url(), validator.rpc_pubsub_url());
    let client = Client::new_with_options(cluster, payer.clone(), CommitmentConfig::confirmed());
    client.program(ID).expect("Failed to create program client")
}

#[test]
fn test_init() {
    let (validator, payer) = setup_validator();
    let payer = Rc::new(payer);
    let program = setup_program(&validator, payer.clone());

    let result = init(&program, &payer, 9, None);

    assert!(result.is_ok(), "init failed: {:?}", result.err());
}
