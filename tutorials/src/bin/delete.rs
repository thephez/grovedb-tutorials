use grovedb::GroveDb;

fn main() {

   // Specify a path and open GroveDB at the path as db
   let path = String::from("../storage");
   let db = GroveDb::open(path).unwrap();

   // Check the key-values are there
   let key1 = b"hello";
   let key2 = b"grovedb";
   let result1 = db.get([], key1, None).unwrap();
   let result2 = db.get([], key2, None).unwrap();
   println!("Before deleting, we have key1: {:?}", result1);
   println!("Before deleting, we have key2: {:?}", result2);

   // Delete the values
   db.delete([], key1, None, None)
      .unwrap()
      .expect("successfully deleted key1");
   db.delete([], key2, None, None)
      .unwrap()
      .expect("successfully deleted key2");

   // Check the key-values again
   let result3 = db.get([], key1, None).unwrap();
   let result4 = db.get([], key2, None).unwrap();
   println!("After deleting, we have key1: {:?}", result3);
   println!("After deleting, we have key2: {:?}", result4);
}
