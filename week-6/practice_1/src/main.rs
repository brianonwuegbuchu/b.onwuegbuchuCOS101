fn main() {
   let name = "Aisha Lawal";
   let uni:&str = "Pan Atlantic University";
   let add:&str = "Km 52 Lekki-Epe Expressway, Ijebu Lekki, Lagos";
   println!("Name: {}", name);
   println!("University: {}, \nAddress: {}",uni,add);

   let department:&'static str = "Computer Science";
   let school:&'static str = "School of Science and Technology";
   println!("Departnment: {}, \nSchool: {}",department,school);
}
