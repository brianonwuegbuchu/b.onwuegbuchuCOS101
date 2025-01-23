fn main() {

   let b:(i32,bool,f64) = (30,true,4.9);
   println!("{}", b);

}

fn print(x:(i32,bool,f64)) {

   println!("Inside print method");
   //assaigns a tuple to distinct variables
   let (age,is_male,cgpa) = x;
   println!("Age is {}, is_Male? {}, cgpa is {}", age,is_male,cgpa);

}