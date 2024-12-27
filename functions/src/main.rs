fn main() {
    //    print_labeled_measurement(29, String::from(""));
    some_loops(25);
}

fn some_loops(num: i32) {
    let mut count = 0;
    loop {
        if count < num {
            count = count + 1;
            println!("This is a loop! {count}");
            continue;
        }
        println!("This is outside the if: {count}");
        break;
    }
}

fn print_labeled_measurement(value: i32, mut name: String) {
    println!("The measurement is: {value}{name}");
    let _ = if value == 33 {
        name = String::from("Cory");
        println!("Your name is {name}!")
    } else if value == 29 {
        name = String::from("Maliya");
        println!("Your name is {name}!")
    } else {
        println!("I don't know who you are.")
    };
}
