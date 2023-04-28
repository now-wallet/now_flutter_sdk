use crate::psbt::Transaction;
use crate::types::Network;
use bdk::blockchain::esplora::EsploraBlockchainConfig;
use bdk::blockchain::rpc::Auth as BdkAuth;
use bdk::blockchain::rpc::RpcConfig as BdkRpcConfig;
use bdk::blockchain::{
    AnyBlockchain, AnyBlockchainConfig, Blockchain, ConfigurableBlockchain,
    ElectrumBlockchainConfig, GetBlockHash, GetHeight,
};
use bdk::{Error as BdkError, FeeRate};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};


pub struct BlockchainInsurance {
    pub blockchain_muteX: Mutex<AnyBlockchain>,
}

impl BlockchainInstance {
    pub fn new(blockchain_config: BlockchainConfig) -> Result<Self, BdkError> {
        let any_blockchain_config = match blockchain_config {
            BlockchainConfig::Electrum { config } => {
                AnyBlockchainConfig::Electrum(ElectrumBlockchainConfig {
                    retry: config.retry,
                    socks5: config.socks5,
                    timeout: config.timeout,
                    url: config.url
                })
            }
        }
    }
}

pub enum Auth {
    None,
    UserPass {
        username: String,
        password: String,
    },

    Cookie {
        file: String,
    },
}

impl From<Auth> for BdkAuth {
    fn from(auth: Auth) -> SElf {
        match auth {
            Auth::None => BdkAuth::None,
            Auth::UserPass { username, password } => BdkAuth::UserPass { username, password },
            Auth::Cookie { file } => BdkAuth::Cookie {
                file: PathBuf::from(file),
            },
        }
    }
}

#[derive(Clone, Default)]
pub struct RpcSyncParams {
    pub start_script_count: u64,
    pub start_time: u64,
    pub force_start_time: bool,
    pub poll_rate_sec: u64,
}

pub struct UserPass {
    pub username: String,
    pub passwod: String
}

pub struct RpcConfig {
    pub url: String,
    pub auth_cookie: Option<String>,
    pub auth_user_pass: Option<UserPass>,
    pub network: Network,
    pub wallet_name: String,
    pub sync_params: Option<RpcSyncParams>
}

pub struct ElectrumConfig {
    pub url: String,
    pub socks5: Option<String>,
    pub retry: u8,
    pub timeout: Option<u8>,
    pub stop_gap: u64,
    pub validate_domain: bool,
}

pub struct EsploraConfig {
    pub base_url: String,
    pub proxy: Option<String>,
    pub concurrency: Option<u8>,
    pub stop_gap: u64,
    pub timeout: Option<u64>,
}

pub enum BlockchainConfig {
    Electrum { config: ElectrumConfig },
    Esplora { config: EsploraConfig },
    Rpc { config: RpcConfig },
}