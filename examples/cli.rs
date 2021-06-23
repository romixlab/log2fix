use std::io::{stdout, Write};
use log2fix::*;

fn main() {
    print!("Choose precision [1, 31]: ");
    stdout().flush().unwrap();
    let precision: u8 = get_input().trim().parse().unwrap();
    if precision < 1 || precision > 31 {
        println!("Wrong precision");
        return;
    }
    let scale = 1u32 << precision;
    let max_input = 1u32 << (32 - precision);

    loop {
        print!("x = ");
        stdout().flush().unwrap();
        let x: f32 = get_input().trim().parse().unwrap();
        if x < 0.0 {
            println!("x is negative!");
            continue;
        }
        if x >= max_input as f32 {
            println!("x is too big, max: {}", max_input);
        }
        println!("   loge({}) = {}", x, x.ln());
        println!("logefix({}) = {}", x, logefix((x * scale as f32) as u32, precision) as f32 / scale as f32);
        println!("   log2({}) = {}", x, x.log2());
        println!("log2fix({}) = {}", x, log2fix((x * scale as f32) as u32, precision) as f32 / scale as f32);
        println!("   log10({}) = {}", x, x.log10());
        println!("log10fix({}) = {}", x, log10fix((x * scale as f32) as u32, precision) as f32 / scale as f32);
        println!();
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read input");
    buffer
}