use clap::{App, Arg, ArgMatches};
use std::process::Command;

pub fn main() -> ArgMatches {
  let matches = App::new("exp")
    .version("1.0")
    .author("Karnpapon B. <karnpapon@gmail.com>")
    .about("temp. folder cleaner")
    .subcommand(App::new("init").about("Adds path"))
    .get_matches();

  return matches;
}

pub fn init() -> String {
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

pub fn check() -> Result<String, String> {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "echo %EXP_PATH%"])
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
    true => Err("error: no exported EXP_PATH".to_string()),
    false => Ok(path)
  }
}

pub fn print_init_msg(path: String) {
  println!("--------------------------------------------------------------------------");
  println!("copy line below to .bash_profile or .bashrc and run `source .bash_profile`");
  println!("--------------------------------------------------------------------------");
  println!("\n");
  println!("export EXP_PATH={:?}", path);
  println!("\n");
}

pub fn print_check_err_msg(e: String){
  println!("--------------------------------------------------------------------------");
  println!("{}", e);
  println!("please navigate to your expected path and run `exp init` first.");
  println!("--------------------------------------------------------------------------");
  panic!();
}
