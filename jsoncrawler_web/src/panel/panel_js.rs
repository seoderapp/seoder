pub const RAW_JS: &'static str = r#"
<script>
const socket = new WebSocket("ws://127.0.0.1:8080");

socket.addEventListener("open", (event) => {
  socket.send("list-campaigns");
  socket.send("feed");
  setTimeout(() => {
    socket.send("list-campaign-stats")
  })

  // todo: allow multi outgoing messages between thread loops
  setInterval(() => {
    socket.send("list-campaigns");
  }, 3500)
  setInterval(() => {
    socket.send("feed");
  }, 1000)

  setInterval(() => {
    socket.send("list-campaign-stats")
  }, 10000)

});

const stats = document.getElementById("feed-stats");

const pathMap = {};

socket.addEventListener("message", (event) => {
  const raw = event.data;

  if (raw.startsWith("{" + '"' + "stats")) {
    stats.innerHTML = event.data;
    return;
  }

  const ptp = "{" + '"' + "path" + '"' + ":" + '"';

  if (raw.startsWith(ptp)) {
    const list = document.getElementById("campaign-list");

    const np = raw.slice(ptp.length + 20);
    const path = np.slice(0, -2);

    if(path in pathMap === false) {
        pathMap[path] = 0;
        const cell = document.createElement("li");  
        cell.className = "campaign-item";
        cell.id = "campaign_" + path;
        cell.textContent = path;
        list.appendChild(cell);
    }
    return;
  }

  const ptc = "{" + '"' + "count" + '"';
  
  if (raw.startsWith(ptc)) {
    // parse json for now
    const np = JSON.parse(raw);
    const path = np && np.path;

    if(path in pathMap) {
        pathMap[path] = np.count;
        const cell = document.getElementById("campaign_" + path);  
        cell.textContent = "( " + np.count + " ) " + path;
    }

  }


});

const cform = document.getElementById("cform");
const rform = document.getElementById("rform");

cform.addEventListener("submit", (event) => {
  const campaign = cform.querySelector('input[name="cname"]');
  if (campaign && campaign.value) {
    socket.send("create-campaign " + campaign.value);
  } else {
    window.alert("Please enter a campaign name");
  }
  event.preventDefault();
});

rform.addEventListener("submit", (event) => {
    socket.send("run-all-campaigns");
    event.preventDefault();
  });
  
</script>
"#;
