use std::fs;
use std::io::{self, Write, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
mod lib;
use lib::app::App;
use lib::cli;

fn main() -> Result<()> {
  let args = cli::main();
  let mut root = String::from("");
  let mut export_abs_path = Command::new("sh");
  match args.subcommand().unwrap() {
    ("init", ref matches) => {
      let abs = matches.value_of("path").unwrap().to_string();
      let abs_path = fs::canonicalize(&PathBuf::from(&abs)).unwrap();
      root = abs_path.display().to_string();
      
      println!("--------------------------------------------------------------");
      println!("copy line below to .bash_profile or .bashrc and run `source .bash_profile`");
      println!("--------------------------------------------------------------");
      println!("\n");
      println!("export EXP_PATH={:?}", abs_path);
      println!("\n");
    }
    ("check", _) => {
      let output = export_abs_path
        .arg("-c")
        .arg("echo $EXP_PATH")
        .output()
        .expect("failed to execute process");
      root = String::from_utf8_lossy(&output.stdout).trim_end().to_string();
    }
    _ => {}
  }

  let mut app = App::new(root);
  app.init().unwrap();
  app.setup_folder();
  app.check_folder();
  // println!("{:#?}", app);

  Ok(())
}
