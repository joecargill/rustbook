fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    const MAX_POINTS2: u32 = 100000;

    println!("MAX_POINTS:  {}", MAX_POINTS);
    println!("MAX_POINTS2: {}", MAX_POINTS2);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
}
