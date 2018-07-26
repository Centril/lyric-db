extern crate treexml;

mod database;
use database::Database;

fn main() {
    let mut database = Database::from("testfiles/test.ldb").unwrap();

    for (k, v) in database.entries.iter() {
        println!("artist: {:?}", k);
        for (album, tracks) in v.iter() {
            println!("album: {:?}", album);
            tracks.iter().for_each(|t| println!("{:?}", t));
        }
    }
}
