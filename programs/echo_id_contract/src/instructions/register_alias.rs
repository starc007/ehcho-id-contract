use anchor_lang::prelude::*;

use crate::{error::EchoIDError as ErrorCode, state::*, merkle};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterAliasParams {
    pub username: String,
    pub public_key: [u8; 32],
    pub initial_chain_mapping: ChainMapping,
}

#[derive(Accounts)]
#[instruction(params: RegisterAliasParams)]
pub struct RegisterAlias<'info> {
    #[account(mut)]
    pub product_owner: Signer<'info>,
    #[account(
        seeds = [b"product_owner", product_owner.key().as_ref()],
        bump,
        constraint = product_owner_account.address == product_owner.key() @ ErrorCode::Unauthorized,
        constraint = product_owner_account.is_active @ ErrorCode::ProductOwnerNotActive
    )]
    pub product_owner_account: Account<'info, ProductOwner>,
    #[account(
        init,
        payer = product_owner,
        space = 8 + 32 + 4 + params.username.len() + 4 + product_owner_account.suffix.len() + 32 + 4 + 8 + 8 + 32,
        seeds = [params.username.as_bytes(), b"@", product_owner_account.suffix.as_bytes()],
        bump,
    )]
    pub alias_account: Account<'info, AliasAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAlias>, params: RegisterAliasParams) -> Result<()> {
    require!(!params.username.contains('@'), ErrorCode::InvalidUsername);
    
    let alias_account = &mut ctx.accounts.alias_account;
    let product_owner_account = &ctx.accounts.product_owner_account;
    
    alias_account.owner = *ctx.accounts.product_owner.key;
    alias_account.username = params.username;
    alias_account.product_suffix = product_owner_account.suffix.clone();
    alias_account.public_key = params.public_key;

    
    let leaf = merkle::hash_chain_mapping(&params.initial_chain_mapping);
    alias_account.chain_mappings_root = merkle::compute_merkle_root(&[leaf]);
    alias_account.chain_mapping_count = 1;
    
    alias_account.reputation = 10;
    alias_account.reputation_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}