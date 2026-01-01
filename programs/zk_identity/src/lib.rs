use anchor_lang::prelude::*;
use zkora_common::*;


declare_id!("ZKID11111111111111111111111111111111111");


#[program]
pub mod zk_identity {
use super::*;


pub fn prove_membership(
ctx: Context<ProveMembership>,
_attestation_sig: [u8; 64]
) -> Result<()> {
let nullifier = &mut ctx.accounts.nullifier;
require!(!nullifier.used, ZKoraError::NullifierAlreadyUsed);
nullifier.used = true;
Ok(())
}
}


#[derive(Accounts)]
pub struct ProveMembership<'info> {
#[account(mut)]
pub user: Signer<'info>,


#[account(init_if_needed, payer = user, space = 8 + 1)]
pub nullifier: Account<'info, NullifierAccount>,


pub system_program: Program<'info, System>,
}
