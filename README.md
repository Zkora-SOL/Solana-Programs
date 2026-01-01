# Solana-Programs
This step defines the Solana native onchain layer of Z-Kora. On Solana, smart contracts are called Programs and are written in Rust using the Anchor framework.  Z-Kora uses a hybrid ZK model:  Zero-knowledge proofs are generated and verified off chain  Solana programs verify signed attestations and enforce state transitions
programs/
├── zk_pay/
│ └── src/lib.rs
├── zk_identity/
│ └── src/lib.rs
├── zk_storage/
│ └── src/lib.rs
└── common/
└── src/lib.rs
