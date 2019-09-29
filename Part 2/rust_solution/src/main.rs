use csv::{ReaderBuilder,StringRecord};
use std::error::Error;
use std::process;
use std::io;

extern crate exchange;
use exchange::{order, Exchange};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut exchange = Exchange::new();

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(io::stdin());
    let mut record = StringRecord::new();
    let mut gen = 0;
    while rdr.read_record(&mut record)? {
        gen+=1;
        let instrument = &record[1];
        let order = order::read(&record[0], &record[2], &record[3], gen);
        let trades = exchange.execute(instrument, order.unwrap());
        for t in trades {
            println!("{}", t);
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
