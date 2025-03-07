use solana_sdk::signer::keypair::Keypair;

fn main() {
    // Generate a new Solana key pair
    let keypair = Keypair::new();

    // Extract public key (Base58 string)
    let public_key = keypair.pubkey().to_string();

    // Extract private key (Base58-encoded 64-byte array)
    let private_key = keypair.to_base58_string();

    println!("Public Key: {}", public_key);
    println!("Private Key: {}", private_key);
}
