use anchor_lang::prelude::*;


#[account]
pub struct NullifierAccount {
pub used: bool,
}


#[account]
pub struct CommitmentRoot {
pub root: [u8; 32],
pub index: u64,
}


#[account]
pub struct AuthorityConfig {
pub authority: Pubkey,
}


#[error_code]
pub enum ZKoraError {
#[msg("Nullifier already used")]
NullifierAlreadyUsed,


#[msg("Invalid attestation signature")]
InvalidAttestation,


#[msg("Unauthorized")]
Unauthorized,
}
```rust
use anchor_lang::prelude::*;


#[account]
pub struct NullifierAccount {
pub used: bool,
}


#[account]
pub struct CommitmentRoot {
pub root: [u8; 32],
}


#[error_code]
pub enum ZKoraError {
#[msg("Nullifier already used")]
NullifierAlreadyUsed,
}
