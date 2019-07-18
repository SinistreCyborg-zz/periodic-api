#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Element {
    name: String,
    atomic_mass: f64,
    boiling_point: f64,
    density: f64,
    melting_point: f64,
    molar_heat: f64,
    number: i64,
    phase: String,
    symbol: String,
    shells: Vec<i64>,
    electron_affinity: f64,
    electronegativity: f64,
    ionization_energies: Vec<i64>,
}

fn get_periodic_table() -> Result<Vec<Element>, Box<dyn Error>> {

    let file = File::open(Path::new("./table.json"))?;
    let reader = BufReader::new(file);

    let c: Vec<Element> = serde_json::from_reader(reader)?;
    Ok(c)

}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
