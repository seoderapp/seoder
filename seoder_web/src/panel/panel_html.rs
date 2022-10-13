use super::panel_js::RAW_JS_TOP;

use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tera::{Context, Tera};

pub fn raw_html() -> String {
    let s: String = match std::env::var("WS_CONNECTION") {
        Ok(v) => {
            let js = format!(
                r#"<script>const sock = new WebSocket("{}");</script>"#,
                v
            );

            js.to_string()
        }
        Err(_) => RAW_JS_TOP.to_string(),
    };

    s
}

/// generate the web panel
pub async fn panel_handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    lazy_static::lazy_static! {
      pub static ref TEMPLATES: Tera = {
          let mut tera = match Tera::new("templates/**") {
              Ok(t) => t,
              Err(e) => {
                  println!("Parsing error(s): {}", e);
                  ::std::process::exit(1);
              }
          };
          tera.autoescape_on(vec![]);
          tera
      };
    }
    let js = raw_html();

    // todo: move to build step and manipulate static html with custom js target from raw html
    let mut context = Context::new();
    context.insert("js", &js);
    context.insert("title", &"Home");
    
    let templ = TEMPLATES.render("app.html", &context).unwrap();

    Ok(Response::new(templ.into()))
}
