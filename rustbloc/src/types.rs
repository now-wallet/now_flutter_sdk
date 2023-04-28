use bdk::bitcoin::blockdata::script::Script as BdkScript;
use bdk::bitcoin::hashes::hex::{FromHex, ToHex};
use bdk::bitcoin::{Address as BdkAddress, OutPoint as BdkOutPoint, Txid};
use bdk::{Balance as BdkBalance, Error as BdkError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

pub struct TxOut {
    pub value: u64,
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub(crate) txid: String,
    pub(crate) vout: u32,
}

impl From<&OutPoint> for BdkOutPoint {
    fn from(x: &OutPoint) -> BdkOutPoint {
        BdkOutPoint {
            txid: Txid::from_str(&x.clone().txid).unwrap(),
            vout: x.clone().vout(),
        }
    }
}

#[derive(Deserialize)]
pub struct Balance {
    pub immature: u64,
    pub trusted_pending: u64,
    pub untrested_pending: u64,
    pub total: u64,
}
