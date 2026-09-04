load std::string::StringManip;

fn main() {
  let fruits = Vector::default(["Apple".to_lowercase(), "Banana".to_lowercase()]);

  for (i, fruit) in fruits.enumerate() {
    println("Hello, {}{}", i+1, fruit);
  }
}