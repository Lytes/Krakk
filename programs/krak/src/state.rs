use anchor_lang::prelude::*;

#[account]
pub struct HashBounty {
    pub amt: u64,           // 8 Amount of lamports placed as bounty
    pub creator: Pubkey,    // 32 Public key of the bounty creator
    pub hash: [u8; 32],     // 32 Stored hash
    pub time_created: u64,  // 8 Unix timestamp of creation
    pub extra_info: String, // 96 Additional information, max 96 bytes
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum HashType {
    BLAKE3,
    SHA256,
    SHA3,
}
