mod trail;
use trail::Trail;
use raylib::prelude::*;

const CELL_SIZE    : i32   = 20 ;
const FIELD_WIDTH  : usize = 60 ;
const FIELD_HEIGHT : usize = 40 ;


fn mouse_pos_to_field(pos: Vector2) -> (i32, i32) {
  (
    (pos.x / CELL_SIZE as f32) as i32,
    (pos.y / CELL_SIZE as f32) as i32,
  )
}


fn bresenham(
  (x0, y0) : (i32, i32), 
  (x1, y1) : (i32, i32), 
  buffer: &mut Vec<(i32, i32)>) 
  -> &[(i32, i32)] {
    buffer.clear();
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    loop {
      buffer.push((x, y));
      if x == x1 && y == y1 {
          break;
      }
      let e2 = 2 * err;
      if e2 > -dy {
          err -= dy;
          x += sx;
      }
      if e2 < dx {
          err += dx;
          y += sy;
      }
    }
    &buffer[1..]
}


fn main() {
  let mut prev_mouse_pos_in_field = (0, 0);
  let mut trail = Trail::new();
  let mut bresenham_buffer: Vec<(i32, i32)> = Vec::with_capacity(100); 
  let (mut rl, thread) = raylib::init()
                                .size(
                                  FIELD_WIDTH as i32 * CELL_SIZE, 
                                  FIELD_HEIGHT as i32 * CELL_SIZE)
                                .title("Game of Life")
                                .build();
  rl.set_target_fps(60);
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);
    let time_now = d.get_time();
    let new_pos = mouse_pos_to_field(d.get_mouse_position());
    if new_pos != prev_mouse_pos_in_field {
      let cells = bresenham(prev_mouse_pos_in_field, new_pos, &mut bresenham_buffer);
      prev_mouse_pos_in_field = new_pos;
      trail.set_active(true);  
      for cell in cells {
        trail.ignite(
          &mut d,
          cell.0 * CELL_SIZE,
          cell.1 * CELL_SIZE,
          time_now
        );
      }  
    }
    trail.bury(&mut d, time_now)
         .color(&mut d, time_now); 
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_line() {
    let mut bresenham_buffer: Vec<(i32, i32)> = Vec::with_capacity(100); 
    println!("{:#?}", bresenham((0, 0), (4, 7), &mut bresenham_buffer));
  }
}