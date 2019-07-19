#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    name: String,
    atomic_mass: f64,
    boiling_point: Option<f64>,
    density: Option<f64>,
    melting_point: Option<f64>,
    molar_heat: Option<f64>,
    number: i64,
    phase: String,
    symbol: String,
    shells: Vec<i64>,
    electron_affinity: Option<f64>,
    electronegativity: Option<f64>,
    ionization_energies: Vec<f64>,
}

fn get_periodic_table() -> Result<Vec<Element>, Box<dyn Error>> {

    let file = File::open("table.json")?;
    let reader = BufReader::new(file);

    let c: Vec<Element> = serde_json::from_reader(reader)?;
    Ok(c)

}

#[get("/<number>", format = "json")]
fn number(number: i64) -> Json<Option<Element>> {

    let elements: Vec<Element> = get_periodic_table()
        .expect("Couldn't read file!");

    let desired = elements.into_iter().find(|x| x.number == number);
    Json(desired)

}

#[get("/<element>", format = "json")]
fn element(element: String) -> Json<Option<Element>> {

    let elements: Vec<Element> = get_periodic_table()
        .expect("Couldn't read file!");

    let desired = elements.into_iter().find(|x| x.name == element);
    Json(desired)

}

#[get("/<symbol>", format = "json")]
fn symbol(symbol: String) -> Json<Option<Element>> {

    let elements: Vec<Element> = get_periodic_table()
        .expect("Couldn't read file!");

    let desired = elements.into_iter().find(|x| x.symbol == symbol);
    Json(desired)

}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite()
        .mount("/number", routes![number])
        .mount("/element", routes![element])
        .mount("/symbol", routes![symbol])
        .mount("/", routes![index])
        .launch();
}
