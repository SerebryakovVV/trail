const RING_SIZE: usize = 5;



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
    if self.data[self.start].birth < time { // can use recursion here, but need to be careful, cause it will loop infinitely; maybe should add did_ignite flag, so this would quaranty there is at leas one cell alive
      self.start += 1;                      // well, i have the end index, so there is actually nothing to be afraid of with recursion
      
      if self.start != self.end {
        self.bury(time);
      }
      
    }
  }

  fn ignite(&mut self, body: (i32, i32, f64)) {
    self.end += 1;
    let slot = &mut self.data[self.end];
    slot.x = body.0;
    slot.y = body.1;
    slot.birth = body.2;
  }

  fn color() {}

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
    println!("{:?}", ring);
    ring.bury(0.1);
    println!("{:?}", ring);
    ring.bury(0.2);
    println!("{:?}", ring);
    ring.bury(0.6);
    println!("{:?}", ring);
    ring.bury(2.2);
    println!("{:?}", ring);
    

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