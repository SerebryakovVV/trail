use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};
use crate::CELL_SIZE;

const TRAIL_SIZE: usize = 60;
const LIFE_TIME: f64 = 1.0;

// https://doc.rust-lang.org/std/num/type.NonZeroUsize.html

#[derive(Copy, Clone, Debug)]
struct Slot {
  x     : i32,
  y     : i32,
  birth : f64, 
}


impl Slot {
  fn new(x: i32, y: i32, birth: f64) -> Self {
    Self {x, y, birth}
  }
}

#[derive(Debug)]
pub struct Trail {
  data   : [Slot; TRAIL_SIZE],
  active : bool              ,
  start  : usize             ,
  end    : usize             ,
}

impl Trail {
  pub fn new() -> Self {
    Self {
      data   : [Slot::new(0, 0, 0.0); TRAIL_SIZE],
      active : false                             ,
      start  : 0                                 ,
      end    : 0                                 ,
    }
  }

  pub fn set_active(&mut self, status: bool) -> &mut Self {
    self.active = status;
    self
  }

  pub fn bury(&mut self, handle: &mut RaylibDrawHandle, time_now: f64) -> &mut Self {
    let Slot {x, y, birth} = self.data[self.start];
    if time_now - birth > LIFE_TIME && self.active {
      handle.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, Color::WHITE);
      if self.start == self.end {
        self.set_active(false);
        return self
      }
      if self.start >= self.data.len() - 1 {
        self.start = 0;
      } else {
        self.start += 1;
      }
      return self.bury(handle, time_now);
    }
    self
  }

  pub fn ignite(&mut self, handle: &mut RaylibDrawHandle, x: i32, y: i32, birth: f64) -> &mut Self {
    if self.end >= self.data.len() - 1 {
      self.end = 0;
    } else {
      self.end += 1;
    }
    if self.end == self.start && self.active {
      self.kill_oldest(handle);
    }
    let slot_to_enter = &mut self.data[self.end];
    slot_to_enter.x = x;
    slot_to_enter.y = y;
    slot_to_enter.birth = birth;
    self
  }

  fn kill_oldest(&mut self, handle: &mut RaylibDrawHandle) {
    let Slot {x, y, ..} = self.data[self.start];
    handle.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, Color::WHITE);
    if self.start >= self.data.len() - 1 {
      self.start = 0;
    } else {
      self.start += 1;
    }
  }

  pub fn color(&mut self, handle: &mut RaylibDrawHandle, time_now: f64) -> &mut Self {
    let mut index = self.start;
    loop { 
      let Slot { x, y, birth } = self.data[index];
      let alpha = (255.0 * (1.0 - (time_now - birth) / LIFE_TIME)) as u8;
      handle.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, Color::new(0, 0, 0, alpha));
      if index == self.end {break;}
      if index >= self.data.len() - 1 {
        index = 0;
      } else {
        index += 1;
      }
    }
    self
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ignition() {}
}