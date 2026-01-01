use anchor_lang::prelude::*;
use zkora_common::*;


declare_id!("ZKStore111111111111111111111111111111111");


#[program]
pub mod zk_storage {
use super::*;


pub fn prove_access(
ctx: Context<ProveAccess>,
_attestation_sig: [u8; 64]
) -> Result<()> {
let nullifier = &mut ctx.accounts.nullifier;
require!(!nullifier.used, ZKoraError::NullifierAlreadyUsed);
nullifier.used = true;
Ok(())
}
}


#[derive(Accounts)]
pub struct ProveAccess<'info> {
#[account(mut)]
pub user: Signer<'info>,


#[account(init_if_needed, payer = user, space = 8 + 1)]
pub nullifier: Account<'info, NullifierAccount>,


pub system_program: Program<'info, System>,
}
