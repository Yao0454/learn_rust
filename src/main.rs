use std::io;

fn get_reverse(mut x: i32) -> i32 {
    let mut reverse: i32 = 0;
    while x > 0 {
        let digit = x % 10;
        reverse = reverse * 10 + digit;
        x /= 10;
    }
    reverse
}

fn is_prime(x: i32) -> bool {
    let i = 2;
    while i * i < x {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut m = String::new();
    io::stdin()
        .read_line(&mut m)
        .expect("Failed to read a line!");

    let m: i32 = m.parse().expect("Please enter a number \n[Error]: ");

    for i in m..2 {
        if is_prime(i) && is_prime(get_reverse(i)) {
            println!("{i}");
            return;
        }
    }

    println!("no");
}
