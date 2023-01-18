use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::PathBuf,
};

use anyhow::Result;
use filecoin_proofs_api::{
    post::generate_single_vanilla_proof, Commitment, PrivateReplicaInfo, RegisteredPoStProof,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct X {
    comm_r: Commitment,
    cache_dir_path: PathBuf,
    replica_path: PathBuf,
    challenges: Vec<u64>,
    sector_id: u64,
}

fn main() -> Result<()> {
    let registered_proof = RegisteredPoStProof::StackedDrgWindow32GiBV1;

    let f = File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("1.txt"))?;

    let lines = io::BufReader::new(f).lines();

    for line in lines {
        let x: X = serde_json::from_str(line.unwrap().as_str())?;

        let rep =
            PrivateReplicaInfo::new(registered_proof, x.comm_r, x.cache_dir_path, x.replica_path);

        let res = generate_single_vanilla_proof(
            registered_proof,
            x.sector_id.into(),
            &rep,
            &x.challenges,
        );

        println!("{:?}", res);
    }

    Ok(())
}
