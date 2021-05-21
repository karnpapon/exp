use notify_rust::Notification;
use std::error;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io::prelude::*;
use std::io::{ BufReader, Write};
use std::path::{Path, PathBuf};
use sha1::{Digest, Sha1};
// use path_slash::PathExt;
use path_slash::PathBufExt;
// use std::process::Command;
// use std::process::Stdio;

use super::utils::*;
use super::logs;
use super::cli;
use super::vars;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub trait DirWalker {
  fn check(&self, move_to: Option<String>) -> Result<()>;
}

#[derive(Debug)]
pub struct App {
  pub root: String,
  pub config_path: String,
  pub temp_queues_dir: Option<TempQueues>,
  pub delete_queues_dir: Option<DeleteQueues>,
}

#[derive(Debug)]
pub struct TempQueues {
  pub path: String,
  pub entry: Option<DirEntry>,
}

#[derive(Debug)]
pub struct DeleteQueues {
  pub path: String,
  pub entry: Option<DirEntry>,
}

impl App {
  pub fn new(root_path: String) -> Self {
    Self {
      root: root_path,
      config_path: String::from(""),
      temp_queues_dir: None,
      delete_queues_dir: None,
    }
  }

  pub fn setup_folder(&mut self) {
    self.add_temp_q().unwrap();
    self.add_delete_q().unwrap();
  }

  pub fn init(&mut self, cmd: &str) -> Result<()> {
    if self.check_config_file_existed() == false {
      self.remove_prev_config_file();
      self.create_config_file().unwrap();
      return Ok(());
    }
    self.renew();
    self.check_config_id(cmd).unwrap();
    Ok(())
  }

  fn remove_prev_config_file(&self){
    let path = match cli::check_exp_path() {
      Some(p) => p,
      None => String::from("")
    };
    if path.is_empty() { return }

    let (_path, _) = cli::init();

    // path to be initilized.
    let mut init_file = String::from(&path);
    init_file.push_str("/");
    init_file.push_str(vars::CONFIG_FILE_NAME);

    // check if any path have been initialized before.
    let mut old_path = String::from(_path);
    old_path.push_str("/");
    old_path.push_str(vars::CONFIG_FILE_NAME);

    let p = PathBuf::from_slash(&init_file);
    let _p = PathBuf::from(old_path);

    // return if EXP_PATH is the same.
    if p == _p  { return }

    match fs::remove_file(p){
      Ok(()) => { logs::print_msg(String::from("old config file `.exp` is removed. please update EXP_PATH")) },
      Err(_) => { logs::print_msg("cannot remove config file".to_string()) },
    }
  }

  pub fn check_config_file_existed(&self) -> bool {
    let mut init_file = String::from(&self.root);
    init_file.push_str("/");
    init_file.push_str(vars::CONFIG_FILE_NAME);
    let p = PathBuf::from_slash(&init_file);
    // let path = Path::new(&init_file);
    let is_exist = p.is_file();
    return is_exist;
  }

  pub fn create_config_file(&mut self) -> Result<()> {    
    
    let res = self.get_id_from_root();
    self.set_config_path();

    // let path = Path::new(&self.config_path).to_slash();
    let path = PathBuf::from_slash(&self.config_path);
    let mut file = std::fs::File::create(path)?;
    file.write(b"## this file is auto generated by exp, an CLI tool for cleaning up temp. folder. \n")?;
    file.write(b"[EXP CONFIG FILE]\n")?;
    file.write_fmt(format_args!("ROOT: {}\n", self.root))?;
    file.write_fmt(format_args!("ID: {}", res))?;
    // let echo_path = format!("echo export EXP_PATH={} >> ~/.zprofile", self.root.clone());

    // let mut output = Command::new("sh")
    //   .arg("-c")
    //   .arg(echo_path)
    //   .stdout(Stdio::piped())
    //   .spawn()
    //   .expect("failed to start 'echo'");

    // output.wait().expect("Couldn't wait for echo child");

    logs::print_init_msg(self.root.clone());
    
    Ok(())
  }

  fn set_config_path(&mut self) {
    let mut file_name = String::from(&self.root);
    file_name.push_str("/");
    file_name.push_str(vars::CONFIG_FILE_NAME);
    self.config_path = file_name;
  }

  fn get_id_from_root(&self) -> String {
    let mut hasher = Sha1::new();
    hasher.update(self.root.as_bytes());
    let result = hasher.finalize();
    let id = hex::encode(result);
    return id;
  }

  fn renew(&mut self) {
    self.set_config_path();
  }

  fn read_config_file(&mut self) -> Result<(String, String)> {
    let file = File::open(self.config_path.clone())?;
    let mut f_reader = BufReader::new(file);
    let mut line = String::new();
    let mut line_count = 1;
    let mut root = String::new();
    let mut id = String::from("");
    while f_reader.read_line(&mut line).unwrap() > 0 {
      if line_count == 3 {
        let split_string: Vec<&str> = line.split(" ").collect();
        root = split_string.get(1).unwrap().to_string();
      }
      if line_count == 4 {
        let split_string: Vec<&str> = line.split(" ").collect();
        id = split_string.get(1).unwrap().to_string();
      }
      line_count += 1;
      line.clear();
    }

    Ok((root.trim_end().to_string(), id))
  }

  fn check_config_id(&mut self, cmd: &str) -> Result<()> {
    let id_from_root = self.get_id_from_root();
    let (_, id) = self.read_config_file().unwrap();
    let is_matched = id_from_root.trim_end() == id.trim_end();

    if is_matched && cmd == "init" { 
      logs::print_msg(String::from("skipped: init the same path."));
      return Ok(())
    }

    if is_matched == false {
      logs::print_init_msg(self.root.clone());
      self.create_config_file().unwrap();
    }
    Ok(())
  }

  pub fn check_folder(&self) {
    self
      .temp_queues_dir
      .as_ref()
      .unwrap()
      .check(Some(
        self.delete_queues_dir.as_ref().unwrap().path.to_string(),
      ))
      .unwrap();

    self
      .delete_queues_dir
      .as_ref()
      .unwrap()
      .check(None)
      .unwrap();
  }

  fn add_delete_q(&mut self) -> Result<()> {
    let mut dir = String::from(&self.root);
    dir.push_str("/");
    dir.push_str(vars::DELETE_FOLDER_NAME);
    let p = PathBuf::from_slash(&dir).display().to_string();
    self.delete_queues_dir = Some(DeleteQueues::new(p));
    if self.delete_queues_dir.as_ref().unwrap().is_exist() {
      return self.add_delete_q_entry();
    }
    self.delete_queues_dir.as_ref().unwrap().create_dir()?;
    self.add_delete_q_entry()?;
    Ok(())
  }

  fn add_temp_q(&mut self) -> Result<()> {
    let mut dir = String::from(&self.root);
    dir.push_str("/");
    dir.push_str(vars::TEMP_FOLDER_NAME);
    let p = PathBuf::from_slash(&dir).display().to_string();
    self.temp_queues_dir = Some(TempQueues::new(p));
    if self.temp_queues_dir.as_ref().unwrap().is_exist() {
      return self.add_temp_q_entry();
    }
    self.temp_queues_dir.as_ref().unwrap().create_dir()?;
    self.add_temp_q_entry()?;
    Ok(())
  }

  fn add_delete_q_entry(&mut self) -> Result<()> {
    for entry in fs::read_dir(&self.root)? {
      let ent = entry.unwrap();
      if ent.file_name() == vars::DELETE_FOLDER_NAME {
        self.delete_queues_dir.as_mut().unwrap().create_entry(ent)?;
      };
    }
    Ok(())
  }

  fn add_temp_q_entry(&mut self) -> Result<()> {
    for entry in fs::read_dir(&self.root)? {
      let ent = entry.unwrap();
      if ent.file_name() == vars::TEMP_FOLDER_NAME {
        self.temp_queues_dir.as_mut().unwrap().create_entry(ent)?;
      };
    }
    Ok(())
  }
}

impl DirWalker for TempQueues {
  fn check(&self, move_to: Option<String>) -> Result<()> {
    for entry in fs::read_dir(&self.path)? {
      let file_name = entry.as_ref().unwrap().file_name();
      let dir = entry?.path();
      let diff = diff_between_created_and_now(&dir)?;
      let mut to = move_to.as_ref().unwrap().clone();
      to.push_str("/");
      to.push_str(file_name.to_str().unwrap());
      let p = PathBuf::from_slash(&to).display().to_string();
      let to_path = Path::new(&p);
      if is_expired(diff, 0) {
        if let Ok(_) = fs::rename(&dir, to_path) {
          Notification::new()
            .summary("CLEANED UP")
            .body("move to delete folder")
            .icon("firefox")
            .show()?;
        }
      }
    }
    Ok(())
  }
}

impl TempQueues {
  fn new(path: String) -> Self {
    Self {
      path: path,
      entry: None,
    }
  }

  fn create_dir(&self) -> std::io::Result<()> {
    let p = Path::new(&self.path);
    fs::create_dir(p)?;
    Ok(())
  }

  fn create_entry(&mut self, entry: DirEntry) -> Result<()> {
    self.entry = Some(entry);
    Ok(())
  }

  fn is_exist(&self) -> bool {
    let path = Path::new(&self.path);
    let is_exist = path.is_dir();
    return is_exist;
  }
}

impl DirWalker for DeleteQueues {
  fn check(&self, _move_to: Option<String>) -> Result<()> {
    if let Ok(entries) = fs::read_dir(&self.path) {
      for entry in entries {
        if let Ok(entry) = entry {
          let dir = entry.path();
          let diff = diff_between_created_and_now(&dir)?;
          if let Ok(file_type) = entry.file_type() {
            if is_expired(diff, 7) {
              if file_type.is_file() {
                fs::remove_file(dir.clone()).unwrap();
              }
              if file_type.is_dir() {
                if let Ok(_) = fs::remove_dir_all(dir.clone()) {
                  Notification::new()
                    .summary("CLEANED UP")
                    .body("folder has expired")
                    .icon("firefox")
                    .show()?;
                }
              }
            }
          } else {
            println!("Couldn't get file type for {:?}", entry.path());
          }
        }
      }
    }
    Ok(())
  }
}

impl DeleteQueues {
  fn new(path: String) -> Self {
    Self {
      path: path,
      entry: None,
    }
  }

  fn create_dir(&self) -> Result<()> {
    let p = Path::new(&self.path);
    fs::create_dir(p)?;
    Ok(())
  }

  fn create_entry(&mut self, entry: DirEntry) -> Result<()> {
    self.entry = Some(entry);
    Ok(())
  }

  fn is_exist(&self) -> bool {
    let path = Path::new(&self.path);
    let is_exist = path.is_dir();
    return is_exist;
  }
}
