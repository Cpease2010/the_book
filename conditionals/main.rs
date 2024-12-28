fn main() {
    let mut count = 0;
    let something = [1, 2, 3, 5, 6, 7, 8, 9, 10];
    for num in something {
        println!("this is num {num}!");
    }
    'my_loop: loop {
        if count < 10 {
            count += 2;
            println!("Count is {count}");
            continue;
        } else {
            println!("Count is over or equal to 10!, Count is {count}")
        }
        println!("Outside the conditional! Count is {count}");
        break;
    }
}
