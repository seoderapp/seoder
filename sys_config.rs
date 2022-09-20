use std::process::Command;

#[allow(missing_docs)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // update system to unlimited for crawler
    if cfg!(target_os = "linux") {
        // cpu time unlimited
        Command::new("ulimit")
            .args(["-t", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited cpu");

        // mem unlimited
        Command::new("ulimit")
            .args(["-m", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited mem");

        // blocks unlimited
        Command::new("ulimit")
            .args(["-f", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited blocks");

        // stack process
        Command::new("ulimit")
            .args(["-s", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited stacks");

        // virtual mem
        Command::new("ulimit")
            .args(["-v", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited virtual mem");

        // user process
        Command::new("ulimit")
            .args(["-u", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited process");

        // open files
        Command::new("ulimit")
            .args(["-n", "unlimited"])
            .output()
            .expect("failed to execute ulimit unlimited files");
    }

    Ok(())
}
