/**
 *
 */
use std::path::PathBuf;
use std::io::Read;
use std::fs::File;
use std::sync::Mutex;
use std::collections::HashMap;

use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use iron::headers::ContentType;

use plugin::Plugin as PluginFor;

use liquid::{self, Value, parse, Context, LiquidOptions, Renderable};

#[derive(Clone)]
pub struct Template {
    name: String,
    value: HashMap<String, Value>,
}

impl Template {

    pub fn new(name: &str, value: HashMap<String, Value>) -> Template  {
        Template {
            name: name.to_string(),
            value: value
        }
    }
}

pub struct LiquidEngine<'a> {
    source: String,
    options: Mutex<LiquidOptions<'a>>,
}

impl<'a> typemap::Key for LiquidEngine<'static> {
    type Value = Template;
}

impl Modifier<Response> for Template {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<LiquidEngine>(self);
    }
}

impl PluginFor<Response> for LiquidEngine<'static> {
    type Error = ();

    fn eval(resp: &mut Response) -> Result<Template, ()> {
        match resp.extensions.get::<LiquidEngine>(){
            Some(t) => Ok(t.clone()),
            None => Err(())
        }
    }
}

impl<'a> LiquidEngine<'a> {

    pub fn new(src: &str, options: LiquidOptions<'a>) -> LiquidEngine<'a> {
        LiquidEngine {
            source: src.to_string(),
            options: Mutex::new(options),
        }
    }

    pub fn render(&self, filename: &ToString, data: &HashMap<String, Value>) -> Result<String, liquid::Error> {

        let mut pathbuf = PathBuf::from(&self.source);
        pathbuf.push(filename.to_string() + ".liquid");

        let mut text = String::new();
        let mut file = match File::open(pathbuf) {
            Ok(f) => f,
            Err(e) => return Err(liquid::Error::from(e.to_string())),
        };
        file.read_to_string(&mut text).ok();

        let mut options = self.options.lock().unwrap();
        let template = parse(&text, &mut options).unwrap();

        let mut data = Context::with_values(data.clone());

        match template.render(&mut data) {
            Ok(Some(s)) => Ok(s),
            Ok(None) => Ok("".to_string()),
            Err(x) => Err(x),
        }
    }
}

impl AfterMiddleware for LiquidEngine<'static>  {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        let page_wrapper = resp.extensions.get::<LiquidEngine>().as_ref()
            .and_then(|h| {
                Some(self.render(&h.name, &h.value))
            });

        match page_wrapper {
            Some(page_result) => {
                match page_result {
                    Ok(page) => {
                        if !resp.headers.has::<ContentType>() {
                            resp.headers.set(ContentType::html());
                        }
                        resp.set_mut(page);
                        Ok(resp)
                    }
                    Err(e) => {
                        info!("{}", e);
                        Err(IronError::new(e, status::InternalServerError))
                    }
                }
            }
            None => {
                Ok(resp)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use iron::prelude::*;
    use middleware::*;

    use liquid::Value;

    fn hello_world() -> IronResult<Response> {
        let resp = Response::new();

        let mut data = HashMap::new();
        data.insert("title".to_string(), Value::Str("Mustache on Iron".to_string()));

        Ok(resp.set(Template::new("index", data)))
    }

    #[test]
    fn test_resp_set() {
        let mut resp = hello_world().ok().expect("response expected");

        // use response plugin to retrieve a cloned template for testing
        match resp.get::<LiquidEngine>() {
            Ok(h) => {
                assert_eq!(h.name, "index".to_string());
                let titleval = match h.value.get("title").unwrap() {
                    &Value::Str(ref s) => s.clone(),
                    _ => String::from("fail"),
                };

                assert_eq!(titleval, "Mustache on Iron");

            },
            _ => panic!("template expected")
        }
    }

}
