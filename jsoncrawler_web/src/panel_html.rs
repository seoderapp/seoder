use hyper::{Body, Request, Response};
use std::convert::Infallible;

fn raw_html() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html>
      <head>
        <title>Spider program</title>
      </head>
      <body>
        <h1>Panel to monitor and manage the program</h1>
        <p>The engines available - campaigns.</p>
        <h2>CRUD Campaign options</h2>
    
        <div>
          <h3>List campaigns</h3>
          <ul id="campaign-list"></ul>
        </div>
    
        <div>
          <h3>Create Campaigns</h3>
          <form id="cform">
            <label for="cname">Campaign name</label>
            <input name="cname" placeholder="Name" type="text" />
            <button type="submit">Submit</button>
          </form>
        </div>
    
        <div>
          <h3>Run all Campaigns</h3>
          <form>
            <button type="submit">Submit</button>
          </form>
        </div>
    
        <div>
          <h3>Stats</h3>
          <div id="feed-stats"></div>
        </div>
    
        <script>
          const socket = new WebSocket("ws://127.0.0.1:8080");
    
          socket.addEventListener("open", (event) => {
            // start a new stats feed
            socket.send("feed");
          });
    
          const stats = document.getElementById("feed-stats");

          socket.addEventListener("message", (event) => {
            stats.innerHTML = event.data;
          });

          const cform = document.getElementById("cform");

          cform.addEventListener("submit", (event) => {
            const campaign = cform.querySelector('input[name="cname"]');
            if (campaign && campaign.value) {
              socket.send("create-campaign " + campaign.value);
            } else {
              window.alert("Please enter a campaign name");
            }
            event.preventDefault();
          });

        </script>
      </body>
    </html>    
    "#
}

/// generate the web panel
pub async fn panel_handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(raw_html().into()))
}
