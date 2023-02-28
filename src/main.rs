use complex_numbers::Complex;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

const STEP: f64 = 0.1;

fn main() {
    let file = File::create("output.dat").unwrap();
    let mut file = LineWriter::new(file);
    let mut im = -2.0;
    while im <= 2.0 + STEP {
        let mut re = -2.0;
        while re <= 2.0 + STEP {
            let c = Complex::new(re, im);
            if c.compare_abs(2.0) {
                continue;
            }
            let mut z = Complex::new(0.0, 0.0);
            for _ in 0..1000 {
                z = z * z + c;
            }
            if z.abs().is_finite() {
                file.write(b"0 ").unwrap();
            } else {
                file.write(b"1 ").unwrap();
            }
            re += STEP;
        }
        im += STEP;
        file.write(b"\n").unwrap();
    }
}