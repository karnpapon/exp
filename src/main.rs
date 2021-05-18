use std::io::{Result};

mod lib;
use lib::app::App;
use lib::cli;


fn main() -> Result<()> {
  cli::main();
  let root = String::from(".");
  let mut app = App::new(root);
  app.setup_folder();
  app.check_folder();
  println!("{:#?}", app);

  Ok(())
}
