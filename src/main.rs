use holo_hash::{HoloHash, hash_type::Entry};

fn main() {
    let mut hash: Vec<u8> = Vec::new();
    for i in 0..36 {
        hash.push(i);
    }

    let holohash: HoloHash<Entry> = HoloHash::from_raw_36(hash);
    println!("Created a HoloHash: {}", holohash);
}
