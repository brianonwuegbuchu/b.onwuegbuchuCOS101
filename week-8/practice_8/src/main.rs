fn main() {
   //initialize a mutable tuple
   let mut mountaint_heights = ("Everest", 8848, "Fishtail", 6993);
   
   println!("Original tuple = {:?}", mountaint_heights);

   //change 3rd and 4th elements of a mutable tuple
   mountaint_heights.2 = "Lhotse";
   mountaint_heights.3 = 8516;

   println!("Changed tuple = {:?}", mountaint_heights);
}