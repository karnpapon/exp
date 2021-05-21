use std::process;

pub fn print_init_msg(path: String) {
  println!("--------------------------------------------------------------------------");
  println!("copy lines below to ~/.profile or ~/.zprofile (zsh) depends on your shell.");
  println!("--------------------------------------------------------------------------");
  println!("\n");
  println!("export EXP_PATH={:?}", path);
  println!("exp");
  println!("\n");
}

pub fn print_check_err_msg(e: String){
  println!("--------------------------------------------------------------------------");
  println!("{}", e);
  println!("please navigate to your expected path and run `exp init` first.");
  println!("--------------------------------------------------------------------------");
  process::exit(0);
}

pub fn print_msg(msg: String){
  println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
  println!("{}", msg);
  println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
}