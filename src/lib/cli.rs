use clap::{App, Arg, ArgMatches};

pub fn main() -> ArgMatches {
  let matches = App::new("exp")
    .version("1.0")
    .author("Karnpapon B. <karnpapon@gmail.com>")
    .about("temp. folder cleaner")
    .subcommand(App::new("check").about("start checking folder"))
    .subcommand(
      App::new("init").about("Adds path").arg(
        Arg::new("path") 
          .about("target folder path")
          .index(1)
          .required(true),
      ),
    )
    .get_matches();

  return matches;
}
