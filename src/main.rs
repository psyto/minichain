use minichain::blockchain::Blockchain;
use minichain::wallet::Wallet;

fn main() {
    let my_blockchain_address: String = "my_blockchain_address".to_string();
    let mut bc = Blockchain::new_blockchain(my_blockchain_address);

    bc.add_transaction("A".to_string(), &"B".to_string(), 1.0);
    //    let mut previous_hash = bc.last_block().hash();
    //    let mut nonce = bc.proof_of_work();
    //    bc.create_block(nonce, previous_hash);
    bc.mining();

    bc.add_transaction("C".to_string(), &"D".to_string(), 2.0);
    bc.add_transaction("X".to_string(), &"Y".to_string(), 3.0);
    //    previous_hash = bc.last_block().hash();
    //    nonce = bc.proof_of_work();
    //    bc.create_block(nonce, previous_hash);
    bc.mining();

    println!("{:#?}", bc);

    println!(
        "Total amount of my_blockchain_address: {:.1}",
        bc.calculate_total_amount("my_blockchain_address".to_string())
    );
    println!(
        "Total amount of C: {:.1}",
        bc.calculate_total_amount("C".to_string())
    );
    println!(
        "Total amount of D: {:.1}",
        bc.calculate_total_amount("D".to_string())
    );

    let wallet = Wallet::new_wallet();
    println!("Private Key: {}", wallet.private_key);
    println!("Public Key: {:?}", wallet.public_key);
    println!("Blockchain Address: {:?}", wallet.blockchain_address);
    let transaction = Wallet::new_transaction(
        wallet.private_key,
        wallet.public_key,
        wallet.blockchain_address,
        "B".to_string(),
        1.0,
    );
    println!("Signature: {:?}", transaction.generate_signature());
}
