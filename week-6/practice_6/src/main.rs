fn main() {
   let n1 = "Electrical".to_string();
   let n2 = "Electronic".to_string();
   let n3 = "Engineering".to_string();
   let n4 = n1 + &n2 + &n3; // n2 & n3 refernce is passed
   // About Electrical/Electronic
   println!("\nThe {} is informed by the aspiration to
   train electical/electronic engineering profssionals 
   in the areas of design, building and the maintenance of 
   electrical control systems,", n4);

   let w1 = "Computer".to_string();
   let w2 = "Science".to_string();
   let w3 = w1 + &w2; // reference is passed
   println!();
   println!("{} is aimed at developing competent, creative, 
    innovative euntrepreneurial and ethically-minded persons,
    capable of creating value in the diverse fields of Computer Science. ",w3);

}