use super::panel_css::RAW_CSS;
use super::panel_js::RAW_JS;
use super::panel_js::RAW_JS_TOP;

use crate::string_concat_impl;
use const_format::concatcp;
use hyper::{Body, Request, Response};
use jsoncrawler_lib::string_concat::string_concat;
use std::convert::Infallible;

pub fn raw_html() -> String {
    const TOP: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Spider program</title>"#;

    const BTOM: &str = r#"
  </head>
  <body>
    <div class="box sr-only">
      <h1>Spider Panel Management</h1>
      <p>Fast insight generation.</p>
      <h2>Realtime custom engine and db handling</h2>
    </div>
  

    <form id="rform" class="bar">
      <button type="submit" type="submit" class="button">Run all Campaigns</button>
    </form>

    <div class="card">
      <div class="card-body">
        <h3>Stats</h3>
        <div id="feed-stats">
          <div class="stats-box">
            <h4 class="gutter">CPU</h4>
            <div id="cpu-stats"></div>
            <div class="seperator"></div>
            <div id="cpu-stats-average"></div>
          </div>
          <div class="stats-box">
            <h4 class="gutter">Network</h4>
            <h5>Received, Transmited, Total Transmited</h5>
            <div id="network-stats"></div>
          </div>
          <div class="stats-box">
            <h4 class="gutter">Memory</h4>
            <h5>Free, Used, Total</h5>
            <div id="memory-stats"></div>
          </div>
        </div>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Create Engine</h3>
        <form id="eform">
          <label for="ename">Name</label>
          <input name="ename" placeholder="Vanalla, starship, and etc" type="text" class="form-control" />

          <label for="epaths">Paths</label>
          <input name="epaths" placeholder="/,/welcome,/about" type="text" class="form-control" />

          <label for="epatterns"Patterns</label>
          <input name="epatterns" placeholder="wild thunder,holy,pizza,*cats*" type="text" class="form-control" />

          <button type="submit" class="button btn-primary">Submit</button>
        </form>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Create Campaign</h3>
        <form id="cform">
          <div>
            <label for="cname">Campaign name</label>
            <input name="cname" placeholder="Big event" type="text" class="form-control" />
          </div>

          <div class="box">
            <fieldset class="form-control" id="engine-select">
            <legend>Select an engine:</legend>
            </fieldset>
          </div>

          <div class="seperator"></div>
          <div class="seperator"></div>
          <div class="seperator"></div>

          <button type="submit" class="button btn-primary">Submit</button>
        </form>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Engines</h3>
        <ul id="engine-list"></ul>
      </div>
    </div> 

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Campaigns</h3>
        <ul id="campaign-list"></ul>
      </div>
    </div>

    <div class="seperator"></div>

"#;

    const ENDB: &str = "</body>
</html>";

    let s: String = match std::env::var("WS_CONNECTION") {
        Ok(v) => {
            let js = format!(
                r#"
          <script>
          
            const socket = new WebSocket("{}");
          "#,
                v
            );

            let base = concatcp!(TOP, RAW_CSS, BTOM).to_string();

            string_concat!(base, js, RAW_JS, ENDB).to_string()
        }
        Err(_) => concatcp!(TOP, RAW_CSS, BTOM, RAW_JS_TOP, RAW_JS, ENDB).to_string(),
    };

    s
}

/// generate the web panel
pub async fn panel_handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(raw_html().into()))
}
