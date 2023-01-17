use std::{fs, path::Path};

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("./vanilla.dat");
    if !path.exists() {
        let resp = reqwest::blocking::get("https://github.com/ipfs-force-community/venus-cluster-assets/raw/master/winning-post-vanilla/vanilla.dat").unwrap();
        fs::write(path, resp.bytes().unwrap()).unwrap()
    }
}
