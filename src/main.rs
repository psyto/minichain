use minichain::blockchain::Blockchain;

fn main() {
    let my_blockchain_address: String = "my_blockchain_address".to_string();
    let mut bc = Blockchain::new_blockchain(my_blockchain_address);

    bc.add_transaction("A".to_string(), &"B".to_string(), 1.0);
 //   let mut previous_hash = bc.last_block().hash();
 //   let mut nonce = bc.clone().proof_of_work();
 //   bc.create_block(nonce, previous_hash);
    bc.mining();

    bc.add_transaction("C".to_string(), &"D".to_string(), 2.0);
    bc.add_transaction("X".to_string(), &"Y".to_string(), 3.0);
//    previous_hash = bc.last_block().hash();
//    nonce = bc.clone().proof_of_work();
//    bc.create_block(nonce, previous_hash);
    bc.mining();

    println!("{:#?}", bc);
}
