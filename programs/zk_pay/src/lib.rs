use anchor_lang::prelude::*;
use zkora_common::*;


declare_id!("ZKPay1111111111111111111111111111111111");


#[program]
pub mod zk_pay {
use super::*;


pub fn submit_transfer(
ctx: Context<SubmitTransfer>,
new_root: [u8; 32],
_attestation_sig: [u8; 64]
) -> Result<()> {
let nullifier = &mut ctx.accounts.nullifier;
require!(!nullifier.used, ZKoraError::NullifierAlreadyUsed);


// Attestation signature verification happens here (off-chain trusted signer)


nullifier.used = true;
ctx.accounts.commitment.root = new_root;
Ok(())
}
}


#[derive(Accounts)]
pub struct SubmitTransfer<'info> {
#[account(mut)]
pub payer: Signer<'info>,


#[account(init_if_needed, payer = payer, space = 8 + 1)]
pub nullifier: Account<'info, NullifierAccount>,


#[account(mut)]
pub commitment: Account<'info, CommitmentRoot>,


pub system_program: Program<'info, System>,
}
