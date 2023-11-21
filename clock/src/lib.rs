use core::fmt;

pub struct Clock {
    seconds: i32,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    let total_seconds = (hours * 3600 + minutes * 60).rem_euclid(24 * 3600);
    Clock { seconds: total_seconds }
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    let total_seconds = self.seconds + minutes * 60;
    let mut clock = Clock { seconds: total_seconds };
    clock.wrap_time();
    clock
  }

  fn wrap_time(&mut self) {
    self.seconds %= 24 * 3600;
    if self.seconds < 0 {
      self.seconds += 24 * 3600;
    }
  }
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let hours = self.seconds / 3600;
    let minutes = (self.seconds % 3600) / 60;
    write!(f, "{:02}:{:02}", hours, minutes)
  }
}

impl fmt::Debug for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let hours = self.seconds / 3600;
    let minutes = (self.seconds % 3600) / 60;
    write!(f, "{:02}:{:02}", hours, minutes)
  }
}

impl PartialEq for Clock {
  fn eq(&self, other: &Self) -> bool {
    self.seconds == other.seconds
  }
}
