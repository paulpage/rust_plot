extern crate gnuplot;
extern crate clap;

use clap::{Arg, App};
use gnuplot::{Figure, AxesCommon};
use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn main() {

    let matches = App::new("Rust Plot")
        .version("0.1")
        .author("Paul Page")
        .about("Plot graphs from a pipe")
        .arg(Arg::with_name("OUTPUT")
             .short("o")
             .value_name("OUTPUT")
             .help("File to direct output to"))
        .arg(Arg::with_name("INPUT")
             .help("File to use as input (defaults to stdin)"))
        .get_matches();

    let stdin = io::stdin();

    let rdr: Box<dyn io::BufRead> = match matches.value_of("INPUT") {
        Some(f) => Box::new(BufReader::new(File::open(f).unwrap())),
        _ => Box::new(stdin.lock()),
    };
    let outfile = matches.value_of("OUTPUT").unwrap_or("out.png");

    let mut xvals: Vec<f64> = Vec::new();
    let mut yvals: Vec<f64> = Vec::new();

    let mut fg = Figure::new();
    let axes = fg.axes2d();

    for (i, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        let mut vals = line.split_whitespace();
        match (vals.next(), vals.next()) {
            (Some(x), Some(y)) => {
                match x.parse::<f64>() {
                    Ok(n) => xvals.push(n),
                    Err(_) => {
                        if i == 0 {
                            axes.set_x_label(x, &[]);
                        }
                    },
                };
                match y.parse::<f64>() {
                    Ok(n) => yvals.push(n),
                    Err(_) => {
                        if i == 0 {
                            axes.set_y_label(y, &[]);
                        }
                    },
                }
            },
            _ => {
                println!("Error reading line");
            }
        };
    }

    axes.lines(&xvals, &yvals, &[]);
    fg.set_terminal(&format!("pngcairo size {}, {}", 640, 480), outfile);
    fg.show().close();
}
