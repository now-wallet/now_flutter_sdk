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
        let mnemonic = BdkMnemonic::parse_in(Language::English, generated_key.to_string()).unwrap();
        Mnemonic { internal: mnemonic }
    }

    pub fn from_str(mnemonic: String) -> Result<Self, BdkError> {
        BdkMnemonic::from_str(&mnemonic)
            .map(|m| Mnemonic { internal: m })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    pub fn from_entropy(entropy: Vec<u8>) -> Result<Self, BdkError> {
        BdkMnemonic::from_entropy(entropy.as_slice())
            .map(|m| Mnemonic { internal: m })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    pub fn as_string(&self) -> String {
        self.internal.to_string()
    }
}

#[derive(Debug)]
pub(crate) struct DescriptorSecretKey {
    pub(crate) descriptor_secret_key_mutex: Mutex<BdkDescriptorSecretKey>,
}
impl DescriptorSecretKey {
    pub fn new(
        network: bdk::bitcoin::Network,
        mnemonic: Mnemonic,
        password: Option<String>,
    ) -> Result<Self, BdkError> {
        let mnemonic = mnemonic.internal.clone();
        let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();
        let descriptor_secret_key = BdkDescriptorSecretKey::XPrv(DescriptorXKey {
            origin: None,
            xkey: xkey.into_xprv(network).unwrap(),
            derivation_path: BdkDerivationPath::master(),
            wildcard: bdk::descriptor::Wildcard::Unhardened,
        });
        Ok(Self {
            descriptor_secret_key_mutex: Mutex::new(descriptor_secret_key),
        })
    }

    pub fn derive(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let secp = Secp256k1::new();
        let descriptor_secret_key = self.descriptor_secret_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();
        match descriptor_secret_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derived_xprv = descriptor_x_key.xkey.derive_priv(&secp, &path)?;
                let key_source = match descriptor_x_key.origin.clone() {
                    Some((fingerprint, origin_path)) => (fingerprint, origin_path.extend(path)),
                    None => (descriptor_x_key.xkey.fingerprint(&secp), path),
                };
                let derived_descriptor_secret_key = BdkDescriptorSecretKey::XPrv(DescriptorXKey {
                    origin: Some(key_source),
                    xkey: derived_xprv,
                    derivation_path: BdkDerivationPath::default(),
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_secret_key_mutex: Mutex::new(derived_descriptor_secret_key),
                }))
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        }
    }
    pub fn extend(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let descriptor_secret_key = self.descriptor_secret_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();
        match descriptor_secret_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let extended_path = descriptor_x_key.derivation_path.extend(path);
                let extended_descriptor_secret_key = BdkDescriptorSecretKey::XPrv(DescriptorXKey {
                    origin: descriptor_x_key.origin.clone(),
                    xkey: descriptor_x_key.xkey,
                    derivation_path: extended_path,
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_secret_key_mutex: Mutex::new(extended_descriptor_secret_key),
                }))
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        }
    }
    pub fn as_public(&self) -> Result<Arc<DescriptorPublicKey>, BdkError> {
        let secp = Secp256k1::new();
        let descriptor_public_key = self
            .descriptor_secret_key_mutex
            .lock()
            .unwrap()
            .to_public(&secp)
            .unwrap();
        Ok(Arc::new(DescriptorPublicKey {
            descriptor_public_key_mutex: Mutex::new(descriptor_public_key),
        }))
    }

    pub fn secret_bytes(&self) -> Result<Vec<u8>, BdkError> {
        let descriptor_secret_key = self.descriptor_secret_key_mutex.lock().unwrap();
        let secret_bytes: Vec<u8> = match descriptor_secret_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                descriptor_x_key.xkey.private_key.secret_bytes().to_vec()
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        };

        Ok(secret_bytes)
    }

    pub fn from_string(key_str: String) -> Result<Arc<Self>, BdkError> {
        let key = BdkDescriptorSecretKey::from_str(&*key_str).unwrap();
        Ok(Arc::new(Self {
            descriptor_secret_key_mutex: Mutex::new(key),
        }))
    }
    pub fn as_string(&self) -> String {
        self.descriptor_secret_key_mutex.lock().unwrap().to_string()
    }
}

pub struct DescriptorPublicKey {
    pub descriptor_public_key_mutex: Mutex<BdkDescriptorPublicKey>,
}

impl DescriptorPublicKey {
    pub fn from_string(key: String) -> Result<Arc<Self>, BdkError> {
        let key = BdkDescriptorPublicKey::from_str(&*key).unwrap();
        Ok(Arc::new(Self {
            descriptor_public_key_mutex: Mutex::new(key),
        }))
    }
    pub fn derive(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let secp = Secp256k1::new();
        let descriptor_public_key = self.descriptor_public_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();

        match descriptor_public_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derived_xpub = descriptor_x_key.xkey.derive_pub(&secp, &path)?;
                let key_source = match descriptor_x_key.origin.clone() {
                    Some((fingerprint, origin_path)) => (fingerprint, origin_path.extend(path)),
                    None => (descriptor_x_key.xkey.fingerprint(), path),
                };
                let derived_descriptor_public_key = BdkDescriptorPublicKey::XPub(DescriptorXKey {
                    origin: Some(key_source),
                    xkey: derived_xpub,
                    derivation_path: BdkDerivationPath::default(),
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_public_key_mutex: Mutex::new(derived_descriptor_public_key),
                }))
            }
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
        }
    }
    pub fn extend(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let descriptor_public_key = self.descriptor_public_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();
        match descriptor_public_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let extended_path = descriptor_x_key.derivation_path.extend(path);
                let extended_descriptor_public_key = BdkDescriptorPublicKey::XPub(DescriptorXKey {
                    origin: descriptor_x_key.origin.clone(),
                    xkey: descriptor_x_key.xkey,
                    derivation_path: extended_path,
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_public_key_mutex: Mutex::new(extended_descriptor_public_key),
                }))
            }
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
        }
    }
    pub fn as_string(&self) -> String {
        self.descriptor_public_key_mutex.lock().unwrap().to_string()
    }
}
#[cfg(test)]
mod test {
    use crate::key::{DerivationPath, DescriptorPublicKey, DescriptorSecretKey, Mnemonic};
    use bdk::bitcoin::hashes::hex::ToHex;
    use bdk::bitcoin::Network;
    use bdk::Error as BdkError;
    use std::sync::Arc;

    fn get_descriptor_secret_key() -> Result<DescriptorSecretKey, BdkError> {
        let mnemonic = Mnemonic::from_str("chaos fabric time speed sponsor all flat solution wisdom trophy crack object robot pave observe combine where aware bench orient secret primary cable detect".to_string()).unwrap();
        DescriptorSecretKey::new(Network::Testnet, mnemonic, None)
    }

    fn derive_dsk(
        key: &DescriptorSecretKey,
        path: &str,
    ) -> Result<Arc<DescriptorSecretKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.derive(path)
    }

    fn extend_dsk(
        key: &DescriptorSecretKey,
        path: &str,
    ) -> Result<Arc<DescriptorSecretKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.extend(path)
    }

    fn derive_dpk(
        key: &DescriptorPublicKey,
        path: &str,
    ) -> Result<Arc<DescriptorPublicKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.derive(path)
    }

    fn extend_dpk(
        key: &DescriptorPublicKey,
        path: &str,
    ) -> Result<Arc<DescriptorPublicKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.extend(path)
    }
}
