// arc website pixel trail
// Bresenham's Line Algorithm

mod trail;
use trail::Trail;
use raylib::prelude::*;


const CELL_SIZE    : i32   = 20 ;
const FIELD_WIDTH  : usize = 30 ;
const FIELD_HEIGHT : usize = 30 ;


fn mouse_pos_to_field(pos: Vector2) -> (i32, i32) {
  (
    (pos.x - (pos.x % CELL_SIZE as f32)) as i32, 
    (pos.y - (pos.y % CELL_SIZE as f32)) as i32, 
  )
}


fn bresenham() {}


enum DrawMode {
  Click,
  Hover,
}


fn main() {
  let draw_mode = DrawMode::Hover;
  let mut prev_mouse_pos_in_field = (0, 0);
  let mut trail = Trail::new();
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
    match draw_mode {
      DrawMode::Click => {
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
          // trail.push(
          //   mouse_pos_to_field(
          //     d.get_mouse_position()
          //   )
          // );
        }
      },
      DrawMode::Hover => {
        let new_pos = mouse_pos_to_field(d.get_mouse_position());
        if new_pos != prev_mouse_pos_in_field {
          prev_mouse_pos_in_field = new_pos;
          trail.set_active(true)
               .ignite(&mut d, new_pos.0, new_pos.1, time_now);                                           
        }
      },
    }
    trail.bury(&mut d, time_now)
         .color(&mut d, time_now); 
  }
}