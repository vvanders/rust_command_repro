use std::process::Command;
use std::env;

fn main() {
    //This is crucial, if we don't set path and instead call "./src/test.bat" everything works fine
    env::set_var("PATH", "./src");

    let cmd = Command::new("test.bat").output().unwrap();

    println!("{}", String::from_utf8_lossy(&cmd.stdout));
}