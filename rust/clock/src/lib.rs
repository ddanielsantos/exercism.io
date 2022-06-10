use std::fmt::Debug;
use std::string::ToString;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl ToString for Clock {
  fn to_string(&self) -> String {
    let mut result = "".to_string();
    let formated_hour = if self.hours < 10 {
      format!("0{}", self.hours)
    } else {
      format!("{}", self.hours)
    }
    .to_string();
    let formated_minutes = if self.minutes < 10 {
      format!("0{}", self.minutes)
    } else {
      format!("{}", self.minutes)
    }
    .to_string();

    result.push_str(&formated_hour);
    result.push_str(":");
    result.push_str(&formated_minutes);

    result
  }
}

pub fn sanitize_time(hours: i32, minutes: i32) -> (i32, i32) {
  let mut hours = hours;
  let mut minutes = minutes;

  loop {
    minutes = match minutes {
      i if i < 0 => {
        hours += if i <= -60 { i / 60 } else { 60 / i };
        60 - i * -1 % 60
      }
      i if i >= 60 => {
        hours += i / 60;
        i % 60
      }
      _ => minutes,
    };
    hours = match hours {
      i if i < 0 => 24 - i * -1 % 24,
      i if i >= 24 => i % 24,
      _ => hours,
    };

    if minutes >= 0 && minutes < 60 && hours >= 0 && hours < 24 {
      break;
    }
  }

  (hours, minutes)
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    let (hours, minutes) = sanitize_time(hours, minutes);
    Self {
      hours: hours,
      minutes: minutes,
    }
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    unimplemented!("Add {} minutes to existing Clock time", minutes);
  }
}
