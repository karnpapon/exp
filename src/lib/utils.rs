use chrono::{DateTime, TimeZone, Utc};
use std::fs;
use std::io::{Result};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::vars;

pub fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
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

pub fn difference_as_day(target_time: DateTime<Utc>) -> i64 {
  let now = Utc::now();
  let duration = target_time.signed_duration_since(now);
  return duration.num_days();
}

pub fn is_expired(diff: i64, offset: i64) -> bool {
  let validate_range = vars::VALIDATE_DAYS + (offset as u8);
  let file_age = diff as u8;
  if file_age > validate_range { return true; }
  return false;
}

pub fn diff_between_created_and_now(path: &PathBuf) -> Result<i64> {
  let since_last_opened = get_metadata(path)?;
  let as_date_time = system_time_to_date_time(since_last_opened);
  let diff = difference_as_day(as_date_time);
  return Ok(diff);
}

pub fn get_metadata(path: &PathBuf) -> Result<SystemTime> {
  let metadata = fs::metadata(&path)?;
  return metadata.accessed();
}
