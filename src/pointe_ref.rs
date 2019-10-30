// Reference Pointers - Point to a resource in memory

pub fn run() {
  // Primitive Array
  let arr1 = [1,2,3];
  let arr2 = arr1;


  // Vector
  // let vec1 = [1,2,3];
  // let vec2 = vec1; // ok

  let vec1 = vec![1,2,3];
  let vec2 = &vec1;

  println!("Values: {:?}", (arr1, arr2, &vec1, vec2));
}
