# iron-liquid
Bring liquid template to Iron : https://github.com/iron/iron

Use liquid : https://github.com/FerarDuanSednan/liquid-rust.git

Inspired by https://github.com/sunng87/handlebars-iron.git


#Example
```rust

use liquid::Value;
use std::default::Default;

/// the handler
fn hello_world(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data : HashMap<String, Value> = make_data();
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
```

