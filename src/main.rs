use std::path::PathBuf;

use filecoin_proofs_api::{
    post::generate_single_vanilla_proof, PrivateReplicaInfo, RegisteredPoStProof,
};

fn main() {
    let comm_r = [
        116, 223, 199, 29, 80, 110, 112, 120, 25, 82, 101, 133, 64, 145, 146, 193, 2, 117, 124, 22,
        202, 37, 87, 235, 199, 215, 65, 84, 34, 42, 157, 8,
    ];
    let registered_proof = RegisteredPoStProof::StackedDrgWindow32GiBV1;
    let cache_dir_path =
        PathBuf::from("/mnt/10.0.6.36/disk1/lotusminer_1005/cache/s-t024972-163653");
    let replica_path =
        PathBuf::from("/mnt/10.0.6.36/disk1/lotusminer_1005/sealed/s-t024972-163653");
    let challenges = vec![652308141];

    let rep = PrivateReplicaInfo::new(
        registered_proof.into(),
        comm_r,
        cache_dir_path,
        replica_path,
    );

    let res = generate_single_vanilla_proof(registered_proof, 163653.into(), &rep, &challenges);

    println!("{:?}", res);
}
