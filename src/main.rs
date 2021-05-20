use std::io::{Result};

mod lib;
use lib::app::App;
use lib::cli;

fn main() -> Result<()> {
  let args = cli::main();
  let mut root = String::from("");
  let mut cmd = ""; 
  match args.subcommand() {
    Some(("init", _)) => {
      let path = cli::init();
      root = path.clone();
      cli::print_init_msg(path);
      cmd = "init";
    },
    _ => {
      match cli::check(){
        Ok(path) => { root = path },
        Err(e) => { cli::print_check_err_msg(e) }
      }      
    }
  }

  let mut app = App::new(root);
  app.init(cmd).unwrap();
  app.setup_folder();
  app.check_folder();
  // println!("{:#?}", app);

  Ok(())
}
