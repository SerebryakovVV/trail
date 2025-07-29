const RING_SIZE: usize = 5;

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
pub struct Ring {
  data  : [Slot; RING_SIZE],
  start : usize            ,
  end   : usize            ,
}

impl Ring {
  fn new() -> Self {
    Self { data: [Slot::new(0, 0, 0.0); RING_SIZE], start: 0, end: 0 }
  }

  fn bury(&mut self, time: f64) {
    if self.data[self.start].birth < time 
    // && self.active == True
    { // can use recursion here, but need to be careful, cause it will loop infinitely; maybe should add did_ignite flag, so this would quaranty there is at leas one cell alive
      // paint cell white
      // check for overflow
      if self.start >= self.data.len() - 1 {
        self.start = 0;
      } else {
        self.start += 1;
      }
                      // well, i have the end index, so there is actually nothing to be afraid of with recursion
      
      if self.start != self.end {
        self.bury(time);
      }

    }
  }

  // if there is too many new cells, then end can reach start
  // need to implement a kill_oldest method
  fn ignite(&mut self, body: (i32, i32, f64)) {
    if self.end >= self.data.len() - 1 {           // if the length of the ring is zero, this panics, because usize
      self.end = 0;
    } else {
      self.end += 1;
    }

    if self.end == self.start {
      self.kill_oldest();
    }

    let slot = &mut self.data[self.end];
    slot.x = body.0;
    slot.y = body.1;
    slot.birth = body.2;
  }

  fn kill_oldest(&mut self) {
      if self.start >= self.data.len() - 1 {
        self.start = 0;
      } else {
        self.start += 1;
      }
  }

  fn color() {
    // this will run at the end
    // check the time on each slot
    // color accordingly
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ignition() {
    let mut ring = Ring::new();
    ring.ignite((0,0,0.5));
    ring.ignite((0,0,1.0));
    ring.ignite((0,0,1.5));
    ring.ignite((0,0,2.0));
    ring.ignite((0,0,2.5));
    ring.ignite((0,0,3.0));
    println!("{:#?}", ring);
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