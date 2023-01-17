use filecoin_proofs_api::{
    post::generate_winning_post_with_vanilla, ProverId, RegisteredPoStProof,
};
use fvm_shared::address::Address;

fn main() {
    let vanilla_proof = include_bytes!("../vanilla.dat").to_vec();
    let miner_id = 25873u64;
    let mut randomness = [0u8; 32];
    randomness[31] &= 0x3f;

    // let prover_id: ProverId = Address::new_id(miner_id).to_bytes().try_into().unwrap();
    let vanilla_proofs = vec![vanilla_proof];
    let res = generate_winning_post_with_vanilla(
        RegisteredPoStProof::StackedDrgWinning32GiBV1,
        &randomness,
        [0; 32],
        &vanilla_proofs,
    )
    .unwrap();
    println!("{:?}", res);
}
