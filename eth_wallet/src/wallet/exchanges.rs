use web3::types::U256;

// Stolen from https://github.com/tmsdev82/rust-eth-crypto-wallet-tutorial/blob/main/src/utils.rs
pub fn wei_to_eth(wei_val: U256) -> f64 {
  let wei = wei_val.as_u128() as f64;
  wei / 1_000_000_000_000_000_000.0
}

pub fn eth_to_wei(eth_val: f64) -> U256 {
  let wei = eth_val * 1_000_000_000_000_000_000.0;
  let wei = wei as u128;
  U256::from(wei)
}
