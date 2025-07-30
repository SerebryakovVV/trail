use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::CELL_SIZE;

const TRAIL_SIZE: usize = 5;

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

  pub fn bury(&mut self, handle: &mut RaylibDrawHandle, time: f64) -> &mut Self {
    let Slot {x, y, birth} = self.data[self.start];

    if birth < time && self.active { // can use recursion here, but need to be careful, cause it will loop infinitely; maybe should add did_ignite flag, so this would quaranty there is at leas one cell alive
      
      handle.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, Color::WHITE);
      if self.start == self.end {
        self.set_active(false);
        
        return self         // rewrite to non-recursive
      }

      
      // paint cell white
      // check for overflow

      if self.start >= self.data.len() - 1 {
        self.start = 0;
      } else {
        self.start += 1;
      }
                      // well, i have the end index, so there is actually nothing to be afraid of with recursion
      
      
        return self.bury(handle, time);         // rewrite to non-recursive
      

    }
    self
  }

  // if there is too many new cells, then end can reach start
  // need to implement a kill_oldest method
  pub fn ignite(&mut self, handle: &mut RaylibDrawHandle, x: i32, y: i32, birth: f64) -> &mut Self {
    if self.end >= self.data.len() - 1 {                                // if the length of the ring is zero, this panics, because usize
      self.end = 0;
    } else {
      self.end += 1;
    }
    if self.end == self.start && self.active {                          // also check for active here, will run when the trail is empty
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


  // it would be better to separate this into another function, outside of ring struct
  // pub fn color(mut self) -> Self {
  pub fn color(&mut self, handle: &mut RaylibDrawHandle) -> &mut Self {
    // this will run at the end
    // check the time on each slot
    // color accordingly
    self
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ignition() {
    // let mut ring = Ring::new();
    // ring.ignite((0,0,0.5));
    // ring.ignite((0,0,1.0));
    // ring.ignite((0,0,1.5));
    // ring.ignite((0,0,2.0));
    // ring.ignite((0,0,2.5));
    // ring.ignite((0,0,3.0));
    // println!("{:#?}", ring);
    // ring.bury(0.1);
    // println!("{:?}", ring);
    // ring.bury(0.2);
    // println!("{:?}", ring);
    // ring.bury(0.6);
    // println!("{:?}", ring);
    // ring.bury(2.2);
    // println!("{:?}", ring);
    

  }
}

// on each frame we check the oldest link
// if dead, move the start further, check next one, cycle
// then we check if mouse moved out of previous cell
// if it did, then the main part of the program begins and we extrapolate the pixels
// push new pixel into ring, with smoothed birth timestamp
// move end too
// and the indexes can move around the ring, thats why its called ring btw

// additions:
// color the slot that just died white
//    this implies that we bury first and color second



// whats the state of the field, if end=start, or if end=0






