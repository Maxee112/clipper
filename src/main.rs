extern crate ndarray;
extern crate gnuplot;

use std::f64::consts::PI;
use gnuplot::{Figure, Caption, Color};
use ndarray::Array1;


fn main() {

    let fs: f64 = 48000.0;
    let ts: f64 = 1.0/fs;
    let f: f64 = 4.0;
    let n = 96000;

    let mut t:ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>  = Array1::zeros(n);
    let mut x:ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::zeros(n);
    x[0] = f64::sin(2.0*PI*f*0.0);

    for n in 1..n {
        t[n] = t[n-1] + ts;
        x[n] = f64::sin(2.0*PI*f*t[n]);
    }
    println!("{:?}", t);
    println!("{:?}", x);

    // Procesado
    let mut y = x.clone();
    clipper((&mut y,"atan"));
    // Representación
    let mut fg = Figure::new();
    fg.axes2d().lines(&t, &x, &[Caption("Sine Wave"), Color("black")]).lines(&t, &y, &[Caption("Clipped Sine Wave"), Color("orange")]);
//    fg.axes2d().lines(&t, &y, &[Caption("Clipped Sine Wave"), Color("orange")]);
    fg.show();

}

pub fn clipper((buffer,cliptype): (&mut Array1<f64>,&str)) {

    for n in 0..buffer.len() {

        match cliptype{
            "fullwave"=> {
                buffer[n] = buffer[n].abs()
            },
            "halfwave"=> {
                if buffer[n]<0.0 {
                    buffer[n]=0.0
                }
            },
            "hard"=> {
                let thr: f64 = 0.5;
                if buffer[n] > thr {
                    buffer[n] = thr;
                } else if buffer[n] < -thr {
                    buffer[n] = -thr;
                }
            },
            "cubic"=> {
                buffer[n] = buffer[n] - ((1.0/3.0) * buffer[n].powi(3));
            },
            "atan"=> {
                buffer[n] = 2.0/PI * f64::atan(3.0 * buffer[n])
            },
            &_ => println!("Clipping type wasnt spelled correctly")
        }

    }

}