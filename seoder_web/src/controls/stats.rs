use crate::serde_json::{json, Value};
use crate::SYSTEM;
use sysinfo::CpuExt;
use sysinfo::NetworkExt;
use sysinfo::SystemExt;

/// get the current stats of the system
pub fn stats() -> Value {
    SYSTEM.lock().unwrap().refresh_all();

    let mut net_total_received = 0;
    let mut net_total_transmited = 0;

    let s = SYSTEM.lock().unwrap();
    let networks = s.networks();

    for (_, data) in networks {
        net_total_received += data.received();
        net_total_transmited += data.transmitted();
    }

    let v = json!({
        "stats": {
            // network
            "network_received": net_total_received,
            "network_transmited": net_total_transmited,
            "network_total_transmitted": net_total_received + net_total_transmited,
            // cpu
            "load_avg_min": s.load_average().one,
            "cpu_usage": s.global_cpu_info().cpu_usage(),
            // memory
            "memory_total": s.total_memory(),
            "memory_used": s.used_memory(),
            "memory_available": s.available_memory(),
            "memory_free": s.free_memory()
        }
    });

    v
}
