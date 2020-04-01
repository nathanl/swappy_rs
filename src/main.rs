mod alphagram;

fn main() {
    let string = "Mañana".to_string();
    let chars = alphagram::new(&string[..]);
    for c in chars {
        println!("char is {}", c);
    }
}
