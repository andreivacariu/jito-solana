use serde::{Deserialize, Serialize};
use {
    digest::Digest, itertools::Itertools, sha2::Sha256,
    solana_sdk::transaction::VersionedTransaction,
};

#[derive(Debug, PartialEq, Default, Eq, Clone, Serialize, Deserialize)]
pub struct VersionedBundle {
    pub transactions: Vec<VersionedTransaction>,
}

pub fn derive_bundle_id(transactions: &[VersionedTransaction]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(transactions.iter().map(|tx| tx.signatures[0]).join(","));
    format!("{:x}", hasher.finalize())
}
