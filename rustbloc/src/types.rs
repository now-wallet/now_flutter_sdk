use bdk::bitcoin::{Address as BdkAddress, OutPoint as BdkOutPoint,  Txid};
use bdk::{Balance as BdkBalance, Error as BdkError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use bdk::bitcoin::blockdata::script::Script as BdkScript;
use bdk::bitcoin::hashes::hex::{FromHex, ToHex};

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
            vout: x.clone().vout,
        }
    }
}
impl From<BdkOutPoint> for OutPoint {
    fn from(x: BdkOutPoint) -> OutPoint {
        OutPoint {
            txid: x.txid.to_string(),
            vout: x.clone().vout,
        }
    }
}

#[derive(Deserialize)]
pub struct Balance {
    pub immature: u64,
    pub trusted_pending: u64,
    pub untrusted_pending: u64,
    pub confirmed: u64,
    pub spendable: u64,
    pub total: u64,
}
impl From<BdkBalance> for Balance {
    fn from(bdk_balance: BdkBalance) -> Self {
        Balance {
            immature: bdk_balance.immature,
            trusted_pending: bdk_balance.trusted_pending,
            untrusted_pending: bdk_balance.untrusted_pending,
            confirmed: bdk_balance.confirmed,
            spendable: bdk_balance.get_spendable(),
            total: bdk_balance.get_total(),
        }
    }
}

pub enum AddressIndex {
    New,
    LastUnused,
    Peek {
        index: u32,
    },
    Reset { index: u32 },
}
impl From<AddressIndex> for bdk::wallet::AddressIndex {
    fn from(x: AddressIndex) -> bdk::wallet::AddressIndex {
        match x {
            AddressIndex::New => bdk::wallet::AddressIndex::New,
            AddressIndex::LastUnused => bdk::wallet::AddressIndex::LastUnused,
            AddressIndex::Peek { index } => bdk::wallet::AddressIndex::Peek(index),
            AddressIndex::Reset { index} =>  bdk::wallet::AddressIndex::Reset(index)
        }
    }
}

pub struct AddressInfo {
    pub index: u32,
    pub address: String,
}
impl From<bdk::wallet::AddressInfo> for AddressInfo {
    fn from(x: bdk::wallet::AddressInfo) -> AddressInfo {
        AddressInfo {
            index: x.index,
            address: x.address.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct TransactionDetails {
    pub txid: String,
    pub received: u64,
    pub sent: u64,
    pub fee: Option<u64>,
    pub confirmation_time: Option<BlockTime>,
}

impl From<&bdk::TransactionDetails> for TransactionDetails {
    fn from(x: &bdk::TransactionDetails) -> TransactionDetails {
        TransactionDetails {
            fee: x.clone().fee,
            txid: x.clone().txid.to_string(),
            received: x.clone().received,
            sent: x.clone().sent,
            confirmation_time: set_block_time(x.confirmation_time.clone()),
        }
    }
}

fn set_block_time(time: Option<bdk::BlockTime>) -> Option<BlockTime> {
    if let Some(time) = time {
        Some(time.into())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlockTime {
    pub height: u32,
    pub timestamp: u64,
}

impl From<bdk::BlockTime> for BlockTime {
    fn from(x: bdk::BlockTime) -> Self {
        BlockTime {
            height: x.height,
            timestamp: x.timestamp,
        }
    }
}

pub struct ScriptAmount {
    pub script: String,
    pub amount: u64,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum RbfValue {
    RbfDefault,
    Value(u32),
}

pub struct BdkTxBuilderResult (pub String, pub TransactionDetails);


pub enum KeychainKind {
    External,
    Internal,
}
impl From<bdk::KeychainKind> for KeychainKind {
    fn from(e: bdk::KeychainKind) -> Self {
        match e {
            bdk::KeychainKind::External => KeychainKind::External,
            bdk::KeychainKind::Internal => KeychainKind::Internal,
        }
    }
}
impl From<KeychainKind> for bdk::KeychainKind {
    fn from(kind: KeychainKind) -> Self {
        match kind {
            KeychainKind::External => bdk::KeychainKind::External,
            KeychainKind::Internal => bdk::KeychainKind::Internal,
        }
    }
}

#[derive(Clone)]
pub enum Network {
    Testnet,
    Regtest,
    Bitcoin,
    Signet,
}
impl Default for Network {
    fn default() -> Self {
        Network::Testnet
    }
}
impl From<Network> for bdk::bitcoin::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Signet => bdk::bitcoin::Network::Signet,
            Network::Testnet => bdk::bitcoin::Network::Testnet,
            Network::Regtest => bdk::bitcoin::Network::Regtest,
            Network::Bitcoin => bdk::bitcoin::Network::Bitcoin,
        }
    }
}
impl From<bdk::bitcoin::Network> for Network {
    fn from(network: bdk::bitcoin::Network) -> Self {
        match network {
            bdk::bitcoin::Network::Signet => Network::Signet,
            bdk::bitcoin::Network::Testnet => Network::Testnet,
            bdk::bitcoin::Network::Regtest => Network::Regtest,
            bdk::bitcoin::Network::Bitcoin => Network::Bitcoin,
        }
    }
}

pub enum WordCount {
    Words12,
    Words18,
    Words24,
}
impl From<WordCount> for bdk::keys::bip39::WordCount {
    fn from(word_count: WordCount) -> Self {
        match word_count {
            WordCount::Words12 => bdk::keys::bip39::WordCount::Words12,
            WordCount::Words18 => bdk::keys::bip39::WordCount::Words18,
            WordCount::Words24 => bdk::keys::bip39::WordCount::Words24,
        }
    }
}
pub struct Address {
    pub address: BdkAddress,
}
impl Address {
    pub fn new(address: String) -> Result<Self, BdkError> {
        BdkAddress::from_str(address.as_str())
            .map(|a| Address { address: a })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    pub fn script_pubkey(&self) -> Arc<Script> {
        Arc::new(Script {
            script: self.address.script_pubkey(),
        })
    }
}
#[derive(Clone)]
pub struct Script {
    pub script: BdkScript,
}


impl Script {
    pub fn from_hex(script: String) -> Result<Self, BdkError> {
        let script = BdkScript::from_hex(script.as_str()).unwrap();
        Ok(Script { script })
    }
    pub fn new(raw_output_script: Vec<u8>) -> Result<Self, BdkError> {
        let script = raw_output_script.as_slice().to_hex();
        Script::from_hex(script)
    }
}