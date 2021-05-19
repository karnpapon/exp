use notify_rust::Notification;
use std::error;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use super::utils::*;
use sha1::{Digest, Sha1};

const TEMP_FOLDER_NAME: &str = "temp_queues";
const DELETE_FOLDER_NAME: &str = "delete_queues";
const CONFIG_FILE_NAME: &str = ".exp";

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

  pub fn init(&mut self) -> Result<()> {
    if self.check_init_file_exist() == false {
      self.setup_init_file().unwrap();
      return Ok(());
    }
    self.renew();
    self.check_init_config_id().unwrap();
    Ok(())
  }

  pub fn check_init_file_exist(&self) -> bool {
    let mut init_file = String::from(&self.root);
    init_file.push_str("/");
    init_file.push_str(CONFIG_FILE_NAME);

    let path = Path::new(&init_file);
    let is_exist = path.is_file();
    return is_exist;
  }

  pub fn setup_init_file(&mut self) -> Result<()> {    
    let res = self.get_id_from_root();
    self.set_config_path();

    let mut file = std::fs::File::create(self.config_path.clone())?;
    file.write(b"[EXP CONFIG FILE]\n")?;
    file.write_fmt(format_args!("ROOT: {}\n", self.root))?;
    file.write_fmt(format_args!("ID: {:?}\n", res))?;
    
    Ok(())
  }

  fn set_config_path(&mut self) {
    let mut file_name = String::from(&self.root);
    file_name.push_str("/");
    file_name.push_str(CONFIG_FILE_NAME);
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

  fn read_init_file(&mut self) -> Result<(String, String)> {
    let file = File::open(self.config_path.clone())?;
    let mut f_reader = BufReader::new(file);
    let mut line = String::new();
    let mut line_count = 1;
    let mut root = String::new();
    let mut id = String::from("");
    while f_reader.read_line(&mut line).unwrap() > 0 {
      if line_count == 2 {
        let split_string: Vec<&str> = line.split(" ").collect();
        root = split_string.get(1).unwrap().to_string();
      }
      if line_count == 3 {
        let split_string: Vec<&str> = line.split(" ").collect();
        id = split_string.get(1).unwrap().to_string();
      }
      line_count += 1;
      line.clear();
    }

    Ok((root.trim_end().to_string(), id))
  }

  fn check_init_config_id(&mut self) -> Result<()> {
    let id_from_root = self.get_id_from_root();
    let (_, id) = self.read_init_file().unwrap();
    if id_from_root != id {
      self.setup_init_file().unwrap();
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
    dir.push_str(DELETE_FOLDER_NAME);
    self.delete_queues_dir = Some(DeleteQueues::new(dir));
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
    dir.push_str(TEMP_FOLDER_NAME);
    self.temp_queues_dir = Some(TempQueues::new(dir));
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
      if ent.file_name() == DELETE_FOLDER_NAME {
        self.delete_queues_dir.as_mut().unwrap().create_entry(ent)?;
      };
    }
    Ok(())
  }

  fn add_temp_q_entry(&mut self) -> Result<()> {
    for entry in fs::read_dir(&self.root)? {
      let ent = entry.unwrap();
      if ent.file_name() == TEMP_FOLDER_NAME {
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
      let diff = diff(&dir)?;
      let mut to = move_to.as_ref().unwrap().clone();
      to.push_str("/");
      to.push_str(file_name.to_str().unwrap());
      let to_path = Path::new(&to);
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
          let diff = diff(&dir)?;
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
