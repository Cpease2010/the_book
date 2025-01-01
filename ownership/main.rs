fn main() {
    let mut some = String::from("Hello Cor yPease!");
    something(&mut some);
}

fn something(s: &mut String) {
    s.push_str(" pushing something else!");
    println!("This is s: {s}");
}
