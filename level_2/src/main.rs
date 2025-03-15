fn string_to_string(input: String) -> String {
    let mut s = input;
    s.push_str("!");
    s
}

fn str(input: &String) -> &String {
    input
}

fn main() {
    let input = String::from("Привет, мир!");
    let s = String::from("Hello, world!");
    println!("{input}");
    println!("{}",string_to_string(input)); // Передача владения, после вызова функции input перестанет существовать
    println!("{}",str(&s)); // Передача ссылки, после вызова функции s все еще существует
}