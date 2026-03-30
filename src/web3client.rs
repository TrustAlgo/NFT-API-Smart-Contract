use std::{env, sync::Arc};

use web3::{
    contract::Contract,
    ethabi,
    transports::Http,
    Web3,
};

use crate::errors::AppError;

#[derive(Clone)]
pub struct Web3Client {
    pub web3: Web3<Http>,
    pub contract: Arc<Contract<Http>>,
}

impl Web3Client {
    pub fn new(contract_address: &str) -> Result<Self, AppError> {
        // Load environment variables
        let node_url = env::var("ETH_NODE_URL")
            .map_err(|_| AppError::InternalServerError)?;

        let abi_path = env::var("CONTRACT_ABI_PATH")
            .map_err(|_| AppError::InternalServerError)?;

        // Initialize transport
        let http = Http::new(&node_url)
            .map_err(|e| AppError::GenericError(e.to_string()))?;

        let web3 = Web3::new(http);

        // Load ABI once
        let abi_bytes = std::fs::read(&abi_path)
            .map_err(|e| AppError::GenericError(e.to_string()))?;

        let contract_abi: ethabi::Contract = serde_json::from_slice(&abi_bytes)
            .map_err(AppError::from)?;

        // Parse contract address
        let address = contract_address
            .parse()
            .map_err(|_| AppError::BadRequest("Invalid contract address".into()))?;

        let contract = Contract::new(web3.eth(), address, contract_abi);

        Ok(Self {
            web3,
            contract: Arc::new(contract),
        })
    }
}
