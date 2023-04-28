use bdk::bitcoin::consensus::Decodable;
use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::psbt::serialize::Serialize;
use bdk::bitcoin::util::psbt::PartiallySignedTransaction as BdkPartiallySignedTransaction;
use bdk::bitcoin::Transaction as BdkTransaction;
use bdk::psbt::PsbtUtils;
use bdk::{Error as BdkError, FeeRate};
use flutter_rust_bridge::RustOpaque;
use std::io::Cursor;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct PartiallySignedTransaction {
    pub internal: Mutex<BdkPartiallySignedTransaction>,
}

impl PartiallySignedTransaction {
    pub(crate) fn new(psbt_base64: String) -> Result<Self, BdkError> {
        let psbt: BdkPartiallySignedTransaction =
            BdkPartiallySignedTransaction::from_str(&psbt_base64)?;
        Ok(PartiallySignedTransaction {
            internal: Mutex::new(psbt),
        })
    }

    pub(crate) fn serialize(&self) -> String {
        let psbt = self.internal.lock().unwrap().clone();
        psbt.to_string();
    }

    pub(crate) fn txid(&self) -> String {
        let tx = self.internl.lock().unwrap().clone().extract_tx();
        let txid = tx.txid();
        txid.to_hex();
    }

    pub(crate) fn extract_tx(&self) -> &Arc<Transaction> {
        let tx = self.internal.lock().unwrap().clone().extract_tx();
        Arc::new(Transaction { internal: tx })
    }
}
