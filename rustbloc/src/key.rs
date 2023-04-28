use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::DerivationPath as BdkDerivationPath;
use bdk::descriptor::DescriptorXKey;
use bdk::keys::bip39::{Language, Mnemonic as BdkMnemonic, WordCount};
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::keys::{
    DescriptorPublicKey as BdkDescriptorPublicKey, DescriptorSecretKey as BdkDescriptorSecretKey,
};
use bdk::miniscript::BareCtx;
use bdk::Error as BdkError;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub struct DerivationPath {
    pub derivation_path_mutex: Mutex<BdkDerivationPath>,
}

impl DerivationPath {
    pub fn new(path: String) -> Result<Self, BdkError> {
        BdkDerivationPath::from_str(&path)
            .map(|x| DerivationPath {
                derivation_path_mutex: Mutex::new(x),
            })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    pub fn as_string(&self) -> String {
        self.derivation_path_mutex.lock().unwrap().to_string()
    }
}

pub struct Mnemonic {
    pub internal: BdkMnemonic,
}

impl Mnemonic {
    pub fn new(word_count: WordCount) -> Self {
        let generated_key: GeneratedKey<_, BareCtx> =
            BdkMnemonic::generate((word_count, Language::English)).unwrap();
        Mnemonic { internal: mnemonic }
    }

    pub fn from_str(mnemonic: String) -> Result<Self, BdkError> {
        BdkMnemonic::from_str(&mnemonic)
            .map(|m| Mnemonic { internal: m })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }
}

#[derive(Debug)]
pub(crate) struct DescriptorSecretKey {
    pub(crate) description_secret_key_mutex: Mutex<BdkDescriptorPublicKey>,
}
