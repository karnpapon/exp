use chrono::{DateTime, TimeZone, Utc};
use std::fs;
use std::io::{Result};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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

// Pseudo-code.
pub fn is_expired(diff: i64, offset: i64) -> bool {
  if diff == (0 + offset) {
    return true;
  }
  return false;
}

pub fn diff(path: &PathBuf) -> Result<i64> {
  let created_time = get_metadata(path)?;
  let as_date_time = system_time_to_date_time(created_time);
  let diff = difference_as_day(as_date_time);
  return Ok(diff);
}

pub fn get_metadata(path: &PathBuf) -> Result<SystemTime> {
  let metadata = fs::metadata(&path)?;
  return metadata.created();
}
