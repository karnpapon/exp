use chrono::{DateTime, TimeZone, Utc};
use std::fs;
use std::fs::DirEntry;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait DirWalker {
  fn is_test(&self) -> bool;
  fn get_metadata(path: &PathBuf) -> Result<SystemTime>;
  fn check(&self, move_to: Option<String>) -> Result<()>;
}

#[derive(Debug)]
pub struct App {
  pub root: String,
  pub temp_queues_dir: Option<TempQueues>,
  pub delete_queues_dir: Option<DeleteQueues>,
}

impl App {
  fn new(root_path: String) -> Self {
    Self {
      root: root_path,
      temp_queues_dir: None,
      delete_queues_dir: None,
    }
  }

  fn setup_folder(&mut self) {
    self.add_temp_q().unwrap();
    self.add_delete_q().unwrap();
  }

  fn check_folder(&self) {
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
    dir.push_str("/delete_queues");
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
    dir.push_str("/temp_queues");
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
      if ent.file_name() == "delete_queues" {
        self.delete_queues_dir.as_mut().unwrap().create_entry(ent)?;
      };
    }
    Ok(())
  }

  fn add_temp_q_entry(&mut self) -> Result<()> {
    for entry in fs::read_dir(&self.root)? {
      let ent = entry.unwrap();
      if ent.file_name() == "temp_queues" {
        self.temp_queues_dir.as_mut().unwrap().create_entry(ent)?;
      };
    }
    Ok(())
  }
}

#[derive(Debug)]
pub struct TempQueues {
  pub path: String,
  pub entry: Option<DirEntry>,
}

impl DirWalker for TempQueues {
  fn is_test(&self) -> bool {
    self
      .entry
      .as_ref()
      .unwrap()
      .file_name()
      .to_str()
      .map(|s| s.starts_with("test-"))
      .unwrap_or(false)
  }

  fn get_metadata(path: &PathBuf) -> Result<SystemTime> {
    let metadata = fs::metadata(&path)?;
    return metadata.created();
  }

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
        fs::rename(&dir, to_path).unwrap();
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

#[derive(Debug)]
pub struct DeleteQueues {
  pub path: String,
  pub entry: Option<DirEntry>,
}

impl DirWalker for DeleteQueues {
  fn is_test(&self) -> bool {
    // self
    //   .dir
    //   .file_name()
    //   .to_str()
    //   .map(|s| s.starts_with("test-"))
    //   .unwrap_or(false)
    return false;
  }

  fn get_metadata(path: &PathBuf) -> Result<SystemTime> {
    let metadata = fs::metadata(&path)?;
    return metadata.created();
  }

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
                fs::remove_dir_all(dir.clone()).unwrap();
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

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
  let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
    Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
    Err(e) => {
      // unlikely but should be handled
      let dur = e.duration();
      let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
      if nsec == 0 {
        (-sec, 0)
      } else {
        (-sec - 1, 1_000_000_000 - nsec)
      }
    }
  };
  Utc.timestamp(sec, nsec)
}

fn difference_as_day(target_time: DateTime<Utc>) -> i64 {
  let now = Utc::now();
  let duration = target_time.signed_duration_since(now);
  return duration.num_days();
}

// Pseudo-code.
fn is_expired(diff: i64, offset: i64) -> bool {
  if diff == (0 + offset) {
    return true;
  }
  return false;
}

fn diff(path: &PathBuf) -> Result<i64> {
  let created_time = get_metadata(path)?;
  let as_date_time = system_time_to_date_time(created_time);
  let diff = difference_as_day(as_date_time);
  return Ok(diff);
}

fn get_metadata(path: &PathBuf) -> Result<SystemTime> {
  let metadata = fs::metadata(&path)?;
  return metadata.created();
}

fn create_dir(path: &str) {
  std::fs::create_dir_all(path).unwrap_or_else(|e| panic!("Error creating dir: {}", e));
}

fn main() -> Result<()> {
  let root = String::from(".");
  let mut app = App::new(root);
  app.setup_folder();
  app.check_folder();
  println!("{:#?}", app);

  Ok(())
}
