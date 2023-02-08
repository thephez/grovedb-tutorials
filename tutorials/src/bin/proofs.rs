use grovedb::GroveDb;
use grovedb::{ Query, PathQuery };

const KEY1: &[u8] = b"key1";
const KEY2: &[u8] = b"key2";

fn main() {

    // Specify the path to the previously created GroveDB instance
    let path = String::from("../storage");
    // Open GroveDB as db
    let db = GroveDb::open(path).unwrap();
    // Define the path to the subtree we want to query.
    let path = vec![KEY1.to_vec(), KEY2.to_vec()];
    // Instantiate a new query.
    let mut query = Query::new();
    // Insert a range of keys to the query that we would like returned.
    query.insert_range(30_u8.to_be_bytes().to_vec()..35_u8.to_be_bytes().to_vec());
    // Put the query into a new unsized path query.
    let path_query = PathQuery::new_unsized(path, query.clone());
    // Execute the query and collect the result items in "elements".
    let (_elements, _) = db
        .query_item_value(&path_query, true, None)
        .unwrap()
        .expect("expected successful get_path_query");

    // Generate proof.
    let proof = db.prove_query(&path_query).unwrap().unwrap();

    // Get hash from query proof and print to terminal along with GroveDB root hash.
    let (hash, _result_set) = GroveDb::verify_query(&proof, &path_query).unwrap();

    // See if the query proof hash matches the GroveDB root hash
    println!("Does the hash generated from the query proof match the GroveDB root hash?");
    if hash == db.root_hash(None).unwrap().unwrap() {
        println!("Yes");
    } else { println!("No"); };
}