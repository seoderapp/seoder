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

          const cellContentBlock = document.createElement("div");  
          cellContentBlock.className = "flex";

          const cellTitle = document.createElement("div");  
          cellTitle.textContent = path;

          const cellEngine = document.createElement("div");  
          cellEngine.textContent = "engine_default";

          const cellStats = document.createElement("div"); 
          cellStats.textContent = "( 0/0 )";
          
          const cellBtnBlock = document.createElement("div");  
          cellBtnBlock.className = "row";

          const cellBtnRunButton = document.createElement("button");  
          const cellBtnDeleteButton = document.createElement("button");  

          cellBtnDeleteButton.textContent = "Delete";
          cellBtnRunButton.textContent = "Run";

          cellBtnRunButton.addEventListener("click", (event) => {
            const name = event.path[2].firstChild.firstChild.textContent;
            socket.send("run-campaign " + name)
            event.preventDefault();
          });

          cellBtnDeleteButton.addEventListener("click", (event) => {
            const name = event.path[2].firstChild.firstChild.textContent;
            socket.send("delete-campaign " + name)
            event.preventDefault();
          });
        
          // actions
          cellBtnBlock.appendChild(cellBtnRunButton);
          cellBtnBlock.appendChild(cellBtnDeleteButton);

          // content
          cellContentBlock.appendChild(cellTitle);
          cellContentBlock.appendChild(cellEngine);
          cellContentBlock.appendChild(cellStats);

          cell.appendChild(cellContentBlock);
          cell.appendChild(cellBtnBlock);

          list.appendChild(cell);
      }
      return;
    }

    // todo delete pipe message on delete


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

          const engineSelect = document.getElementById("engine-select");

          const eKeys = Object.keys(engineMap);

          eKeys.forEach((key) => {
            const inputName = "eselect" + key;
            const kid = "ekeys_" + key;
            const item = document.getElementById(kid);

            if (!item) {
              const cellContainer = document.createElement("div");
              const cellLabel = document.createElement("label");
              const cellSelect = document.createElement("input");

              cellContainer.id = kid;
              cellLabel.htmlFor = kid          
              cellSelect.name = "eselect";
              cellSelect.value = key;
              cellSelect.type = "radio";
  
              cellLabel.innerText = key;
              cellContainer.appendChild(cellLabel);
              cellContainer.appendChild(cellSelect);
              engineSelect.appendChild(cellContainer);
            }
          });


     
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
          if (cell && cell.firstChild && cell.firstChild.firstChild.nextSibling) {
            cell.firstChild.firstChild.nextSibling.nextSibling.textContent = "( " + np.count + " / " + 0 + " ) ";
          }
      }

    }
  });

  const cform = document.getElementById("cform");
  const rform = document.getElementById("rform");
  const eform = document.getElementById("eform");

  cform.addEventListener("submit", (event) => {
    const campaign = cform.querySelector('input[name="cname"]');
    const cengine = cform.querySelector('input[name="eselect"]');

    if (campaign && campaign.value) {
      const m = JSON.stringify({ name: campaign.value, engine: cengine.value });

      socket.send("create-campaign " + m);
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
