use std::process;

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
  process::exit(0);
}

pub fn print_msg(msg: String){
  println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
  println!("{}", msg);
  println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
}