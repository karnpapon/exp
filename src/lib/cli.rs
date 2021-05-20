use clap::{App, ArgMatches};
use std::process::Command;
use super::logs;

pub fn main() -> ArgMatches {
  let matches = App::new("exp")
    .version("1.0")
    .author("Karnpapon B. <karnpapon@gmail.com>")
    .about("temp. folder cleaner")
    .subcommand(App::new("init").about("Adds path"))
    .get_matches();

  return matches;
}

pub fn init() -> ( String, String ) {
  let path = _init();
  let cmd = String::from("init");
  return ( path, cmd )
}

pub fn check() -> Option<String>{
  match check_exp_path() {
    Some(path) => return Some(path),
    None => { 
      logs::print_check_err_msg("no exported EXP_PATH".to_string()); 
      return None
    },
  }
}

fn _init() -> String {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "cd"])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("pwd")
      .output()
      .expect("failed to execute process")
  };
  let path = String::from_utf8_lossy(&output.stdout)
    .trim_end()
    .to_string();

  return path;
}

pub fn check_exp_path() -> Option<String> {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "$Env:Path"])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo $EXP_PATH")
      .output()
      .expect("failed to execute process")
  };
  let path = String::from_utf8_lossy(&output.stdout)
    .trim_end()
    .to_string();

  match path.is_empty() {
    false => Some(path),
    true => None,
  }
}