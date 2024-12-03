fn main() {
   let a:i32 = 2;  // Bit presentation 10
   let b:i32 = 3;  // Bit presentation 11

   let mut _result:i32;

   let reult = a&b;
   println!("(a & b) => {} ",reult);

   let reult = a | b;
   println!("(a | b) => {} ",reult);

   let result = a ^ b;
   println!("(a ^ b) => {} ",result);

   let result = !b;
   println!("(!b) => {} ",result);

   let result = a << b;
   println!("(a << b) => {} ",result);

   let result = a >> b;
   println!("(a >> b) => {} ",result);
}
