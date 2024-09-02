pub struct Person {
    pub name: String,
    pub age: u8,
}
// トレイト実装して、ストラテジーパターンを実装する
impl Person {
    pub fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    pub fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    pub fn introduction(&self) {
        println!("I am {} years old.", self.age);
    }
}
