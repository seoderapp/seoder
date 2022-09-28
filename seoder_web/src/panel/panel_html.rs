use super::panel_css::RAW_CSS;
use super::panel_js::{RAW_JS, RAW_JS_CPU_STATS, RAW_JS_TOP};

use crate::string_concat_impl;
use const_format::concatcp;
use hyper::{Body, Request, Response};
use seoder_lib::string_concat::string_concat;
use std::convert::Infallible;

pub fn raw_html() -> String {
    const TOP: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Seoder SEO Panel</title>"#;

    const BTOM: &str = r#"
  </head>
  <body>
    <div class="box sr-only">
      <h1>Spider Panel Management</h1>
      <p>Fast insight generation.</p>
      <h2>Realtime custom engine and DB handling</h2>
    </div>
  
    <div class="row">

      <div class="row center">
        <div class="text-center ph cpu-box">
          <div class="gutter stats-head">CPU</div>
          <canvas id="cpu-stats" width="200" height="200"></canvas>
          <div id="cpu-stats-average" class="center flex-col ph"></div>
        </div>
        <div class="stats-box ph">
          <div class="gutter stats-head">Network</div>
          <div class="tild">Received, Transmited, Total Transmited</div>
          <div id="network-stats" class="stats-bar gutter"></div>
          <div class="seperator gutter"></div>
          <div class="gutter stats-head">Memory</div>
          <div class="tild">Used, Free, Total</div>
          <div id="memory-stats" class="stats-bar"></div>
      </div>

      </div>

      <div class="row bar">
        <form id="uploadform" class="ph frame" enctype="multipart/form-data" method="post">
          <label for="file" class="ph">Set crawl list</label>
          <input type="file" class="" accept="text/plain" name="file"/>
          <button class="btn-primary button">Upload</button>
        </form>
        <form id="rform" class="ph">
          <button type="submit" class="button btn-primary">Run Campaigns</button>
        </form>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Create Engine</h3>
        <form id="eform">
          <label for="ename">Name</label>
          <input name="ename" placeholder="engine name" type="text" class="form-control" />

          <label for="epaths">Paths</label>
          <input name="epaths" placeholder="/home,/welcome,/about" type="text" class="form-control" />

          <label for="epatterns"Patterns</label>
          <input name="epatterns" placeholder="wild cat,holy,pizza,*cats*" type="text" class="form-control" />

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
            <input name="cname" placeholder="_rainmain" type="text" class="form-control" />
          </div>

          <div class="box gutter">
            <fieldset class="form-control" id="engine-select">
              <legend>Select an engine:</legend>
            </fieldset>
          </div>

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

            let base = concatcp!(TOP, RAW_CSS, BTOM, RAW_JS_CPU_STATS).to_string();

            string_concat!(base, js, RAW_JS, ENDB).to_string()
        }
        Err(_) => concatcp!(
            TOP,
            RAW_CSS,
            BTOM,
            RAW_JS_CPU_STATS,
            RAW_JS_TOP,
            RAW_JS,
            ENDB
        )
        .to_string(),
    };

    s
}

/// generate the web panel
pub async fn panel_handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(raw_html().into()))
}
