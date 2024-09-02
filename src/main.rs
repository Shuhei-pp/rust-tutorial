mod person;
use person::Person;

fn main() {
    let (name, age) = collect_name_and_age();

    // // 配列を定義,そのあと表示
    // let a = vec!["a", "1", "22", "3"];
    // for i in &a {
    //     println!("{}", i);
    // }

    let person = Person::new(name.to_string(), age);
    person.greet();
    person.introduction();
}

fn collect_name_and_age() -> (String, u8) {
    println!("please input your name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}! How old are you?:", name.trim());
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    (name, age.trim().parse().unwrap())
}
