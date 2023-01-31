use grovedb::GroveDb;
use grovedb::Element;

fn main() {
   // specify the path where you want to set up GroveDB
   let path = String::from("../storage");
  
   // open a new GroveDB at the path
   let db = GroveDb::open(path).unwrap();

   // define key-values for insertion
   let key1: &[u8] = b"hello";
   let val1 = vec![1,2,3];
   let key2: &[u8] = b"grovedb";
   let val2 = vec![4,5,6];

   // insert key-value 1 into the root tree
   db.insert([], key1, Element::new_item(val1), None, None)
       .unwrap()
       .expect("successful root tree leaf insert");

   // insert key-value 2 into the root tree
   db.insert([], key2, Element::new_item(val2), None, None)
       .unwrap()
       .expect("successful root tree leaf 2 insert");

   // get value 1
   let result1 = db.get([], key1, None).unwrap();

   // get value 2
   let result2 = db.get([], key2, None).unwrap();

   // print values to terminal
   println!("{:?}", result1);
   println!("{:?}", result2);
}
