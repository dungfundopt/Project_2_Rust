// Không cần lifetime annotation, vì trình biên dịch tự suy ra được (Luật 1)
fn print(s: &str) {
    println!("{}", s);
}

// Cần lifetime annotation, vì có nhiều lifetime đầu vào và trả về một tham chiếu
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Owner {
  name: String
}

// Luật 3, &self sẽ được gán lifetime
impl Owner {
  fn get_name(&self) -> &str {
    &self.name
  }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    let owner = Owner{name: String::from("Test")};
    println!("owner name {}", owner.get_name());

}