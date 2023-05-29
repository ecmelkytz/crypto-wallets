mod exchanges;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, fs::File, io::BufReader};
use secp256k1::{rand::rngs::OsRng, Secp256k1, PublicKey, SecretKey};
use web3::{
  signing::keccak256,
  transports::ws::WebSocket,
  types::{Address, TransactionParameters, H256},
  Web3,
};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
  let secp = Secp256k1::new();
  let mut rng = OsRng::new().expect("OsRng");
  secp.generate_keypair(&mut rng)
}

pub fn wallet_address(public_key: &PublicKey) -> Address {
  let public_key = public_key.serialize();
  let hash = keccak256(&public_key);
  Address::from_slice(&hash[12..])
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EthWallet {
  pub secret_key: String,
  pub public_key: String,
  pub address: String,
}

impl EthWallet {
  pub fn new() -> Self {
    let (sk, pk) = generate_keypair();
    let address: Address = wallet_address(&pk);
    EthWallet {
      secret_key: sk.to_string(),
      public_key: pk.to_string(),
      address: format!("{:?}", address),
    }
  }

  pub fn save_to_file(&self, file_path: &str) -> Result<()> {
    std::fs::write(
      file_path,
      serde_json::to_string_pretty(self).unwrap()
    )?;
    Ok(())
  }

  pub fn read_from_file(file_path: &str) -> Result<EthWallet> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let wallet: EthWallet = serde_json::from_reader(reader)?;
    Ok(wallet)
  }

  pub fn get_secret_key(&self) -> Result<SecretKey> {
    Ok(SecretKey::from_str(&self.secret_key)?)
  }

  pub async fn get_balance(&self, web3: &Web3<WebSocket>) -> Result<f64> {
    let wallet_address = Address::from_str(&self.address)?;
    let balance = web3.eth().balance(wallet_address, None).await?;
    Ok(exchanges::wei_to_eth(balance))
  }
}

pub async fn web3_connection(ws: &str) -> Result<Web3<WebSocket>> {
  let transport = web3::transports::ws::WebSocket::new(ws).await?;
  Ok(web3::Web3::new(transport))
}

pub fn create_transaction(to: Address, eth_value: f64) -> TransactionParameters {
  TransactionParameters { to: Some(to), value: exchanges::eth_to_wei(eth_value), ..Default::default() }
}

pub async fn sign_and_send(
  web3: &Web3<WebSocket>,
  transaction: TransactionParameters,
  secret_key: &SecretKey,
) -> Result<H256> {
  let signed = web3.accounts().sign_transaction(transaction, secret_key).await?;
  let transaction_result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
  Ok(transaction_result)
}
