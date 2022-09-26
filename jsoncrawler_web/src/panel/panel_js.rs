pub const RAW_JS: &'static str = r#"
<script>

  const socket = new WebSocket("ws://127.0.0.1:8080");

  socket.addEventListener("open", (event) => {
    socket.send("list-campaigns");
    socket.send("list-engines");
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
    }, 5000)

  });

  const stats = document.getElementById("feed-stats");
  const cpu = document.getElementById("cpu-stats");
  const cpua = document.getElementById("cpu-stats-average");

  const pathMap = {};
  const engineMap = {};

  socket.addEventListener("message", (event) => {
    const raw = event.data;

    if (raw.startsWith("{" + '"' + "stats")) {
      const data = JSON.parse(event.data);
      const {stats} = data;

      cpu.innerHTML = stats.cpu_usage.toFixed(2);
      cpua.innerHTML = "1 min Average " + stats.load_avg_min.toFixed(2);

      return;
    }

    const ptp = "{" + '"' + "path" + '"' + ":" + '"';

    if (raw.startsWith(ptp)) {
      const list = document.getElementById("campaign-list");
      const np = JSON.parse(raw);
      const path = np && np.path;

      if(path in pathMap === false) {
          pathMap[path] = 0;
          const cell = document.createElement("li");  
          cell.className = "campaign-item";
          cell.id = "campaign_" + path;

          const cellTitle = document.createElement("div");  
          cellTitle.textContent = path;

          const cellEngine = document.createElement("div");  
          cellEngine.textContent = "engine_default";

          const cellStats = document.createElement("div"); 
          cellStats.textContent = "( 0/0 )";
          
          cell.appendChild(cellTitle);
          cell.appendChild(cellEngine);
          cell.appendChild(cellStats);

          list.appendChild(cell);
      }
      return;
    }

    const ptpe = "{" + '"' + "epath" + '"' + ":" + '"';

    if (raw.startsWith(ptpe)) {
      const list = document.getElementById("engine-list");
      const np = JSON.parse(raw);
      const path = np && np.epath;

      if(path in engineMap === false) {
          engineMap[path] = 0;
          const cell = document.createElement("li");  
          cell.className = "engine-item";
          cell.id = "engine_" + path;

          const cellTitle = document.createElement("div");  
          cellTitle.textContent = path;          
          cell.appendChild(cellTitle);
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
          cell.firstChild.nextSibling.nextSibling.textContent = "( " + np.count + " / " + 0 + " ) ";
      }

    }
  });

  const cform = document.getElementById("cform");
  const rform = document.getElementById("rform");
  const eform = document.getElementById("eform");

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
  
  eform.addEventListener("submit", (event) => {
    const engine = eform.querySelector('input[name="ename"]');
    const epaths = eform.querySelector('input[name="epaths"]');
    const epatterns = eform.querySelector('input[name="epatterns"]');

    if (engine && engine.value) {
      const m = JSON.stringify({ name: engine.value, paths: epaths.value, patterns: epatterns.value });

      socket.send("create-engine " + m);
      socket.send("list-engines");
    } else {
      window.alert("Please enter a engine name");
    }
    event.preventDefault();

  });

</script>
"#;
