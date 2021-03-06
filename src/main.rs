#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;

use std::mem;

const MEANING_OF_LIFE: u8 = 42; // has no fixed address

static mut GLOBAL_STATIC: i32 = 123;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

fn main() {
    fundamental_data_types();
    operators();
    scope_and_shadowing();

    println!("The meaning of life is {}", MEANING_OF_LIFE);

    unsafe {
        GLOBAL_STATIC = 777;
        println!("This is mutable unsafe static = {}", GLOBAL_STATIC);
    }

    sh::stack_and_heap();

    if_statement();

    while_and_loop();
    for_loop();
    match_statement();

    structures();
    enums();
}

fn enums() {
    // let c: Color = Color::RgbColor(10, 0, 0);
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
    println!(
        "Line starts at ({}, {}) and ends at ({}, {})",
        myline.start.x, myline.start.y, myline.end.x, myline.end.y
    );
}

fn if_statement() {
    let temp = 15;

    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("quiet cold");
    } else {
        println!("temperature is OK");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}

fn match_statement() {
    let country_code = 7777;

    let country = match country_code {
        44 => "UK",
        49 => "Germany",
        7 => "Russia",
        1...999 => "unknown",
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country);
}

fn for_loop() {
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            println!("Skipping {}", x);
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;

    loop
    // = while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 20 {
            break;
        }
    }
}

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        println!("inside before declaration, a = {}", a);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
}

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("{}", a);

    a = a + 1;
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, a % 3);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1 | 2 = {}", c);

    let two_to_ten = 1 << 10;
    println!("2^10 = {}", two_to_ten);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("It's {} that pi is less than 4.0", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5;
    println!("It's {} that x is equal to 5", x_is_5);
}

fn fundamental_data_types() {
    // unsigned 0..255
    let a: u8 = 123; // 8 bits
    println!("a = {}", a);

    let mut b: i8 = 0; // mutable
    println!("b = {}", b);
    b = 23;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32 value
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {} after modification", c);

    let z: isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    let f = 4 > 0; // true
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}
