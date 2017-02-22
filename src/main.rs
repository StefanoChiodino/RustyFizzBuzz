fn main() {
    fizzbuzz_to(10000000);
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        print!("{:?} : ", n);

        match n {
            _ if is_divisible_by(n, 15) => println!("Fizzbuzz"),
            _ if is_divisible_by(n, 3) => println!("fizz"),
            _ if is_divisible_by(n, 5) => println!("buzz"),
            _ => println!(),
        }
    }
}

fn is_divisible_by(i: u32, j: u32) -> bool {
    if i == 0 {
        return false;
    }

    i % j == 0
}

#[test]
fn test_is_divisible_by(){
    assert!(is_divisible_by(10, 5))
}