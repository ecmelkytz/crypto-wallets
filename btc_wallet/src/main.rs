mod wallet;

fn main() {
  let compressed_wallet = wallet::BtcWallet::new();
  println!("Compressed: {:?}", &compressed_wallet);

  let uncompressed_wallet = wallet::BtcWallet::new_uncompressed();
  println!("Uncompressed: {:?}", &uncompressed_wallet);
}
