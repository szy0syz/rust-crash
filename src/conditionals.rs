// Conditionals - Userd to check the condition of something and act on result

pub fn run() {
  let age: u8 = 17;
  let check_id: bool = false;
  let knows_person_of_age = true;

  // If/Else
  //* 模范生活酒吧服务员问顾客，根据年龄服务
  if age >= 21 && check_id && knows_person_of_age {
    println!("Bartender: What would you like to drink?");
  } else if age < 21 && check_id {
    println!("Battender: Sorry, you have t leave");
  } else {
        println!("Battender: I'll need to see your ID");
  }

  // Shorthand If
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of age: {}", is_of_age);
}
