extern crate iron;
extern crate env_logger;
extern crate liquid;
extern crate iron_liquid as irl;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::default::Default;

use iron::prelude::*;
use iron::{status};

use irl::{Template, LiquidEngine};

use liquid::Value;
use liquid::Value::*;



fn make_data () -> HashMap<String, Value> {
    let mut data = HashMap::new();

    data.insert("year".to_string(), Str("2015".to_string()));

    let mut teams = vec![];

    let mut tmp = HashMap::new();
    tmp.insert("name".to_string(), Str("jiangsu sainty".to_string()));
    tmp.insert("pts".to_string(), Num(43f32));
    teams.push(Object(tmp));

    tmp = HashMap::new();
    tmp.insert("name".to_string(), Str("Beijing Guoan".to_string()));
    tmp.insert("pts".to_string(),  Num(27f32));
    teams.push(Object(tmp));

    tmp = HashMap::new();
    tmp.insert("name".to_string(), Str("Guangzhou Evergrand".to_string()));
    tmp.insert("pts".to_string(), Num(22f32));
    teams.push(Object(tmp));

    tmp = HashMap::new();
    tmp.insert("name".to_string(), Str("Shandong Luneng".to_string()));
    tmp.insert("pts".to_string(), Num(12f32));
    teams.push(Object(tmp));

    data.insert("teams".to_string(), Array(teams));
    data
}

/// the handler
fn hello_world(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    // open http://localhost:3000/
    let data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    env_logger::init().unwrap();

    let mut chain = Chain::new(hello_world);
    let muse = LiquidEngine::new("./examples", Default::default());

    chain.link_after(muse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

