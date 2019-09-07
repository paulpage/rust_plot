extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs::File;

fn main() {

    let stdin = io::stdin();

    let args: Vec<String> = env::args().collect();
    let rdr: Box<dyn io::BufRead> = match args.len() {
        2 => Box::new(BufReader::new(File::open(&args[1]).unwrap())),
        _ => Box::new(stdin.lock()),
    };

    let mut xvals: Vec<f64> = Vec::new();
    let mut yvals: Vec<f64> = Vec::new();

    for line in rdr.lines().skip(1) {
        let line = line.unwrap();
        let mut vals = line.split_whitespace();
        match (vals.next(), vals.next()) {
            (Some(x), Some(y)) => {
                xvals.push(x.parse::<f64>().unwrap());
                yvals.push(y.parse::<f64>().unwrap());
            },
            _ => {
                println!("Error reading line");
            }
        };
    }

    let mut fg = Figure::new();
    fg.set_terminal("pngcairo", "out.png");
    fg.axes2d()
        .lines(&xvals, &yvals, &[Caption("A line"), Color("black")]);
    fg.show();
    fg.close();

}
