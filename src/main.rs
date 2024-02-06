use std::fs::File;
use std::io::{self, Write};


fn main() {
    let mut output = File::create("./out.ppm").unwrap();

    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 1000;


    let header = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    let _ = output.write(header.as_bytes()).unwrap();

    for row in 0..HEIGHT {
        print!("Row {}: ", row);
        io::stdout().flush().unwrap();

        for col in 0..WIDTH {
            let triplet: String = format!("{} {} {}\n", 
                ((row as f32 / (WIDTH-1) as f32) * 255.999).floor() as u32, 
                ((col as f32 / (HEIGHT-1) as f32) * 255.999).floor() as u32, 
                0
            );
            let _ = output.write(triplet.as_bytes()).unwrap();
        }
        
        println!("Done.");
    }
}
