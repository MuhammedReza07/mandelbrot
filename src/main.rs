use complex_numbers::Complex;

fn main() {
    let c = Complex {real: 0.0, imaginary: 0.0};
    let mut s = Complex{real: 0.0, imaginary: 0.0};
    for _ in 0..1000 {
        s = s * s + c;
    }
    if s.abs().is_finite() {
        println!("{} is in the Mandelbrot set.", c);
    } else {
        println!("{} is not in the Mandelbrot set.", c);
    }
}