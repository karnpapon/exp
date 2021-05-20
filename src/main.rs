use std::io::Result;

mod lib;
use lib::app::App;
use lib::cli;

fn main() -> Result<()> {
  let args = cli::main();
  let (r, c) = match args.subcommand() {
    Some(("init", _)) => { cli::init() },
    _                 => { (cli::check().unwrap(), String::from("")) }
  };

  let root = r; 
  let cmd = c;
  let mut app = App::new(root);
  app.init(&cmd).unwrap();
  app.setup_folder();
  app.check_folder();

  Ok(())
}
