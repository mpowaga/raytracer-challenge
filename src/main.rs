mod canvas;
mod tuples;

use canvas::Canvas;
use tuples::color;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let color1 = color(1.0, 0.5, 0.0);

    for y in 0..100 {
        for x in 0..100 {
            canvas.write_pixel(x, y, color1);
        }
    }

    let ppm = canvas.to_ppm();

    std::fs::write("output.ppm", ppm).unwrap();

    println!("File written to output.ppm");
}
