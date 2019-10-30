// Varibles hlod promitive data or references to data
// Varibles are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Jerry";
  //* let age = 18;
  let mut age = 18;
  //* warning: value assigned to `age` is never read
  //* 为遍历赋值但没用，后面又重新赋值，会报错
  println!("My name is {} and I am {}", name, age);
  // error: cannot assign twice to immutable variable
  age = 38;

  println!("My name is {} and I am {}", name, age);


  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}", my_name, my_age);
}
