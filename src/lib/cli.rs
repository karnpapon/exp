use clap::App;

pub fn main() {
  let _matches = App::new("folder-cleaner")
    .version("1.0")
    .author("Karnpapon B. <karnpapon@gmail.com>")
    .about("Does awesome things")
    .arg("-c, --config=[FILE] 'Sets a custom config file'")
    .arg("-d, --dir=[FILE] 'Set a folder to be automatically clean-up'")
    // .arg("<INPUT>              'Sets the input file to use'")
    .subcommand(
      App::new("test")
        .about("controls testing features")
        .version("1.3")
        .author("Someone E. <someone_else@other.com>")
        .arg("-d, --debug 'Print debug information'"),
    )
    .get_matches();
}
