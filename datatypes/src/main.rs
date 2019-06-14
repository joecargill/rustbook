fn main() {
    let bit8_signed: i8 = -128;
    let bit8_unsign: u8 = 255;
    println!(
        " 8-bit   signed = {}\n       unsigned =  {}\n",
        bit8_signed, bit8_unsign
    );

    let bit16_signed: i16 = -32_768;
    let bit16_unsign: u16 = 65_535;
    println!(
        "16-bit   signed = {}\n       unsigned =  {}\n",
        bit16_signed, bit16_unsign
    );

    let bit32_signed: i32 = -2_147_483_648;
    let bit32_unsign: u32 = 4_294_967_295;
    println!(
        "32-bit   signed = {}\n       unsigned =  {}\n",
        bit32_signed, bit32_unsign
    );

    let bit64_signed: i64 = -9_223_372_036_854_775_808;
    let bit64_unsign: u64 = 18_446_744_073_709_551_615;
    println!(
        "64-bit   signed = {}\n       unsigned =  {}\n",
        bit64_signed, bit64_unsign
    );

    let arch_signed: isize = -9_223_372_036_854_775_808;
    let arch_unsign: usize = 18_446_744_073_709_551_615;
    println!(
        " arch    signed = {}\n       unsigned =  {}\n",
        arch_signed, arch_unsign
    );

    let x = 2.999999999999999; // f64
    let y: f32 = 3.999999; // f32

    println!("\nf64 = {}\nf32 = {}\n", x, y);

    // addition
    let sum = 5 + 10;
    println!("     5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!(" 95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("     4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("     43 % 5 = {}", remainder);

    let t = true;
    let f: bool = false; // explicit type annotation
    println!("\ntrue: {},   false: {}", t, f);

    let c = 'z';
    let z = '𝚭';
    println!("\n{} {}", c, z);

    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup1;
    println!("\nx={} y={} z={}", x, y, z);

    let tup2 = (1.2, 999, 2);
    let (a, b, c) = tup2;
    println!("\na={} b={} c={}", a, b, c);

    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    let one = tup1.2;
    println!("\nx={} y={} z={}", five_hundred, six_point_four, one);

    let years = [1993, 1994, 1995, 1996, 1997];

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
        "Nov", "Dec",
    ];

    let years: [i32; 5] = [1993, 1994, 1995, 1996, 1997];

    let array_of_tens = [10; 5];

    println!("\n{} {} {}", array_of_tens[0], months[2], years[2]);

    let i = 10;
    let e = years[i];
    println!("\n{}", e); // runtime error
}
