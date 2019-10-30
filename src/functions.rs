// Functions - Used to store blacks of code for re-use

pub fn run() {
  greeting("Jerry", "Shi");

  // Bind function values to varuables
  let get_sum = add(5, 5);
  println!("Sum: {}", get_sum);

  // 闭包
  let n3: i32 = 11; //* 这好比 js 中作用域链向上查找吗？
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("C Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, Nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
