use std::io;

fn main() {
   let mut input = String::new();
   let stdin = io::stdin();

   let weight_kg: f64;
   let height_m: f64;
   let height_cm: i32;

   println!("Enter you weight in KG");
   stdin.read_line(&mut input)
   .expect("Read fails");
   weight_kg = input.trim().parse().unwrap();

   input.clear();

   println!("Enter you height in CM.");
   stdin.read_line(&mut input)
   .expect("Read fails");
   height_cm = input.trim().parse().unwrap();
   height_m = height_cm as f64 / 100 as f64;
   println!("Height in M: {height_m}");

   let bmi = weight_kg / (height_m * height_m );
   println!("You BMI is {:.2}", bmi);
}
