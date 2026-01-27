use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_lang::{Discriminator, InstructionData, ToAccountMetas};

// Import the generated modules
#[path = "../src/generated.rs"]
mod generated;

#[test]
fn test_initialize_module_exists() {
    // Verify the initialize module was generated
    use generated::initialize;

    // Create an instance
    let init = initialize::Initialize { decimals: 9 };

    // Verify it has the correct discriminator
    assert_eq!(
        initialize::Initialize::DISCRIMINATOR,
        &[175, 175, 109, 31, 13, 152, 155, 237]
    );

    // Verify InstructionData trait is implemented
    let _data = init.data();
}

#[test]
fn test_initialize_accounts() {
    use generated::initialize;

    let mint = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let system_program = Pubkey::new_unique();

    let accounts = initialize::Accounts {
        mint,
        authority,
        system_program,
    };

    // Verify ToAccountMetas is implemented
    let metas = accounts.to_account_metas(None);

    assert_eq!(metas.len(), 3);
    assert_eq!(metas[0].pubkey, mint);
    assert_eq!(metas[1].pubkey, authority);
    assert_eq!(metas[2].pubkey, system_program);

    // Verify account properties
    assert!(metas[0].is_writable);
    assert!(metas[0].is_signer);
    assert!(metas[1].is_writable);
    assert!(metas[1].is_signer);
    assert!(!metas[2].is_writable);
    assert!(!metas[2].is_signer);
}

#[test]
fn test_create_token_account_module() {
    use generated::create_token_account;

    let create = create_token_account::CreateTokenAccount {};

    // Verify discriminator
    assert_eq!(
        create_token_account::CreateTokenAccount::DISCRIMINATOR,
        &[147, 241, 123, 100, 244, 132, 174, 118]
    );

    let _data = create.data();
}

#[test]
fn test_mint_tokens_module() {
    use generated::mint_tokens;

    let mint = mint_tokens::MintTokens { amount: 1000 };

    // Verify discriminator from IDL
    assert_eq!(
        mint_tokens::MintTokens::DISCRIMINATOR,
        &[59, 132, 24, 246, 122, 39, 8, 243]
    );

    // Verify the amount field is accessible
    assert_eq!(mint.amount, 1000);

    let _data = mint.data();
}

#[test]
fn test_transfer_module() {
    use generated::transfer;

    let transfer_ix = transfer::Transfer { amount: 500 };

    assert_eq!(
        transfer::Transfer::DISCRIMINATOR,
        &[163, 52, 200, 231, 140, 3, 69, 186]
    );

    assert_eq!(transfer_ix.amount, 500);
}

#[test]
fn test_burn_module() {
    use generated::burn;

    let burn_ix = burn::Burn { amount: 250 };

    assert_eq!(
        burn::Burn::DISCRIMINATOR,
        &[116, 110, 29, 56, 107, 219, 42, 93]
    );

    assert_eq!(burn_ix.amount, 250);
}

#[test]
fn test_all_instructions_have_unique_discriminators() {
    use generated::*;

    let discriminators = vec![
        initialize::Initialize::DISCRIMINATOR,
        create_token_account::CreateTokenAccount::DISCRIMINATOR,
        mint_tokens::MintTokens::DISCRIMINATOR,
        transfer::Transfer::DISCRIMINATOR,
        burn::Burn::DISCRIMINATOR,
    ];

    // Verify all discriminators are unique
    for i in 0..discriminators.len() {
        for j in (i + 1)..discriminators.len() {
            assert_ne!(
                discriminators[i], discriminators[j],
                "Discriminators at indices {} and {} are not unique",
                i, j
            );
        }
    }
}

#[test]
fn test_discriminators_are_8_bytes() {
    use generated::*;

    assert_eq!(initialize::Initialize::DISCRIMINATOR.len(), 8);
    assert_eq!(create_token_account::CreateTokenAccount::DISCRIMINATOR.len(), 8);
    assert_eq!(mint_tokens::MintTokens::DISCRIMINATOR.len(), 8);
    assert_eq!(transfer::Transfer::DISCRIMINATOR.len(), 8);
    assert_eq!(burn::Burn::DISCRIMINATOR.len(), 8);
}

#[test]
fn test_instruction_data_serialization() {
    use generated::initialize;

    let init = initialize::Initialize { decimals: 9 };
    let data = init.data();

    // Data should contain discriminator (8 bytes) + decimals (1 byte) = 9 bytes minimum
    assert!(data.len() >= 9, "Serialized data should be at least 9 bytes");

    // First 8 bytes should be the discriminator
    assert_eq!(&data[0..8], initialize::Initialize::DISCRIMINATOR);
}
