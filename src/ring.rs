const RING_SIZE: usize = 50;



#[derive(Copy, Clone)]
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


pub struct Ring {
  data  : [Slot; RING_SIZE],
  start : usize            ,
  end   : usize            ,
}

impl Ring {
  fn new() -> Self {
    Self { data: [Slot::new(0, 0, 0.0); RING_SIZE], start: 0, end: 0 }
  }

  fn bury() {}

  fn ignite(&mut self, body: (i32, i32, f64)) {
    self.end += 1;
    let slot = &mut self.data[self.end];
    slot.x = body.0;
    slot.y = body.1;
    slot.birth = body.2;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ignition() {
    let ring = Ring::new();

  }
}

// on each frame we check the oldest link
// if dead, move the start further, check next one, cycle
// then we check if mouse moved out of previous cell
// if it did, then the main part of the program begins and we extrapolate the pixels
// push new pixel into ring, with smoothed birth timestamp
// move end too
// and the indexes can move around the ring, thats why its called ring btw