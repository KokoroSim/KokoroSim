use std::process::Command;

fn main() {
    // 1. Read VERSION file from repository root
    let version = std::fs::read_to_string("../VERSION")
        .unwrap_or_else(|_| "0.1.0-dev".to_string())
        .trim()
        .to_string();
    println!("cargo:rustc-env=SIMCARDIO_VERSION={version}");
    println!("cargo:rerun-if-changed=../VERSION");

    // 2. Read SIMCARDIO_BUILD_TIME from environment, or generate formatted local time
    let build_time = std::env::var("SIMCARDIO_BUILD_TIME").unwrap_or_else(|_| {
        let output = Command::new("date")
            .arg("+%Y-%m-%d %H:%M:%S")
            .output();
        match output {
            Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
            Err(_) => "unknown".to_string(),
        }
    });
    println!("cargo:rustc-env=SIMCARDIO_BUILD_TIME={build_time}");
    println!("cargo:rerun-if-env-changed=SIMCARDIO_BUILD_TIME");
}
