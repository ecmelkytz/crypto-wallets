mod wallet;
use anyhow::Result;
use std::{env, str::FromStr};
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();

  // let crypto_wallet = wallet::EthWallet::new();
  // crypto_wallet.save_to_file("eth_wallet.json")?;

  let wallet = wallet::EthWallet::read_from_file("eth_wallet.json")?;
  println!("{:?}", wallet);

  let sepolia_ws = env::var("INFURA_SEPOLIA_WS")?;
  let web3_connect = wallet::web3_connection(&sepolia_ws).await?;
  let block_number = web3_connect.eth().block_number().await?;
  println!("Blok numarası: {}", &block_number);
  let balance = wallet.get_balance(&web3_connect).await?;
  println!("Cüzdan Bakiyesi: {} Sepolia ETH", &balance);

  // let transaction = wallet::create_transaction(
  //   Address::from_str("0x04EcE94d247AC2fCCDcE603C9881A6e9117F6B25")?, 0.01);

  // let transaction_hash =
  //   wallet::sign_and_send(&web3_connect, transaction,
  //     &wallet.get_secret_key()?).await?;
  // println!("Transaction hash: {:?}", transaction_hash);  

  Ok(())
}
