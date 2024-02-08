use std::fs::File;
use std::io::{self, Write};

mod vec3;
use vec3::*;

fn main() {
    let mut output = File::create("./out.ppm").unwrap();

    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;


    let header = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    let _ = output.write(header.as_bytes()).unwrap();

    for row in 0..HEIGHT {
        print!("Row {}: ", row);
        io::stdout().flush().unwrap();

        for col in 0..WIDTH {
            let color = Color3::new(row as f64 / (WIDTH-1) as f64, col as f64 / (HEIGHT-1) as f64, 0.);
            let _ = output.write(color.ppm().as_bytes()).unwrap();
        }
        
        println!("Done.");
    }
}
