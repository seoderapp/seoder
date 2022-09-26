use super::panel_css::RAW_CSS;
use super::panel_js::RAW_JS;
use const_format::concatcp;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub fn raw_html() -> &'static str {
    const TOP: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Spider program</title>"#;

    const BTOM: &str = r#"
  </head>
  <body>
    <h1>Panel to monitor and manage the program</h1>
    <p>The engines available - campaigns.</p>
    <h2>CRUD Campaign options</h2>

    <div class="card">
      <div class="card-body">
        <h3>List campaigns</h3>
        <ul id="campaign-list"></ul>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Create Campaign</h3>
        <form id="cform">
          <label for="cname">Campaign name</label>
          <input name="cname" placeholder="Name" type="text" class="form-control" />
          <button type="submit" class="button btn-primary">Submit</button>
        </form>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Run all Campaigns</h3>
        <form id="rform">
          <button type="submit" type="submit" class="button">Submit</button>
        </form>
      </div>
    </div>

    <div class="seperator"></div>

    <div class="card">
      <div class="card-body">
        <h3>Stats</h3>
        <div id="feed-stats"></div>
      </div>
    </div>  
"#;

    const ENDB: &str = "</body>
</html>";

    concatcp!(TOP, RAW_CSS, BTOM, RAW_JS, ENDB)
}

/// generate the web panel
pub async fn panel_handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(raw_html().into()))
}
