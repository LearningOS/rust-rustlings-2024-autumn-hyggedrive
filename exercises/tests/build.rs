use std::env;
use std::time::SystemTime;

fn main() {
    let timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    env::set_var("TEST_FOO", timestamp.to_string());
    println!("cargo:rerun - if - changed=build.rs");
}