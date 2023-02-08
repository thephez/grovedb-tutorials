use grovedb::GroveDb;
use grovedb::Element;

fn main() {

   // Specify a path and open GroveDB at the path as db
   let path = String::from("../storage");
   let db = GroveDb::open(path).unwrap();

   // Define key-values for insertion
   let KEY1 = b"hello";
   let VAL1 = b"world";
   let KEY2 = b"grovedb";
   let VAL2 = b"rocks";

   // Insert key-value 1 into the root tree
   db.insert([], KEY1, Element::Item(VAL1.to_vec(), None), None, None)
       .unwrap()
       .expect("successful root tree leaf insert");

   // Insert key-value 2 into the root tree
   db.insert([], KEY2, Element::Item(VAL2.to_vec(), None), None, None)
       .unwrap()
       .expect("successful root tree leaf 2 insert");

   // At this point the Items are fully inserted into the database.
   // No other steps are required.

   // To show that the Items are there, we will use the get()
   // function to get them from the RocksDB backing store.

   // Get value 1
   let result1 = db.get([], KEY1, None).unwrap();

   // Get value 2
   let result2 = db.get([], KEY2, None).unwrap();

   // Print the values to terminal
   println!("{:?}", result1);
   println!("{:?}", result2);
}