use anchor_lang::prelude::*;

declare_id!("48WQW8ZMQKJhV1FKnGrYVDMEoqc8XutQmvKuqcmRrKux");

#[program]
pub mod solana_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> Result<()> {
        let mint = &mut ctx.accounts.mint;

        mint.authority = ctx.accounts.authority.key();
        mint.total_supply = 0;
        mint.decimals = decimals;

        msg!(
            "Token mint initialized! Authority: {}. Decimals: {}",
            mint.authority,
            decimals
        );

        Ok(())
    }

    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;

        token_account.owner = ctx.accounts.owner.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.amount = 0;

        msg!("Token account created for owner: {}", token_account.owner);

        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let token_account = &mut ctx.accounts.token_account;

        // Verify the token account belongs to the correct mint.
        require!(token_account.mint == mint.key(), ErrorCode::MintMismatch);

        // Update total supply.
        mint.total_supply = mint
            .total_supply
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        // Update token account balance.
        token_account.amount = token_account
            .amount
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        msg!("Minted {} tokens to {}", amount, token_account.owner);

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;

        // Check sufficient balance.
        require!(from.amount >= amount, ErrorCode::InsufficientFunds);

        // Verify both accounts belong to the same mint.
        require!(from.mint == to.mint, ErrorCode::MintMismatch);

        // Update balances.
        from.amount = from.amount.checked_sub(amount).ok_or(ErrorCode::Overflow)?;

        to.amount = to.amount.checked_add(amount).ok_or(ErrorCode::Overflow)?;

        msg!(
            "Transferred {} tokens from {} to {}",
            amount,
            from.owner,
            to.owner
        );

        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let token_account = &mut ctx.accounts.token_account;

        // Check sufficient balance.
        require!(token_account.amount >= amount, ErrorCode::InsufficientFunds);

        // Verify token accounts belongs to this mint.
        require!(token_account.mint == mint.key(), ErrorCode::MintMismatch);

        // Update balances.
        token_account.amount = token_account
            .amount
            .checked_sub(amount)
            .ok_or(ErrorCode::Overflow)?;

        mint.total_supply = mint
            .total_supply
            .checked_sub(amount)
            .ok_or(ErrorCode::Overflow)?;

        msg!("Burned {} tokens from {}", amount, token_account.owner);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 1)]
    pub mint: Account<'info, TokenMint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub mint: Account<'info, TokenMint>,

    #[account(init, payer = payer, space = 8 + 32 + 32 + 8, seeds = [b"token", owner.key().as_ref(), mint.key().as_ref()], bump)]
    pub token_account: Account<'info, TokenAccount>,

    // This is the owner of the token account we're creating.
    pub owner: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut, has_one = authority)]
    pub mint: Account<'info, TokenMint>,

    #[account(mut, seeds = [b"token", token_account.owner.as_ref(), mint.key().as_ref()], bump)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, seeds = [b"token", owner.key().as_ref(),], bump, has_one = owner)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub mint: Account<'info, TokenMint>,

    #[account(mut, seeds = [b"token", owner.key().as_ref(),], bump, has_one = owner)]
    pub token_account: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
}

#[account]
pub struct TokenMint {
    pub authority: Pubkey,
    pub total_supply: u64,
    pub decimals: u8,
}

#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Token account mint does not match the provided mint")]
    MintMismatch,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Sender has insufficient funds")]
    InsufficientFunds,
}
