// arc website pixel trail
// Bresenham's Line Algorithm
// use |stack - circular buffer| for the pixels with color
// on update run old to new, so if a pixel is revisited, we just have the latest value
// and we have two pointers, the oldest time-ended value will move pointer on its death

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

fn draw_field(handle: &mut RaylibDrawHandle, trail: &Vec<(i32, i32)>) {
  for rec in trail.iter() {
    handle.draw_rectangle(rec.0, rec.1, CELL_SIZE, CELL_SIZE, Color::BLACK);
  }
}

fn bresenham() {}

enum DrawMode {
  Click,
  Hover,
}


fn main() {
  let mut draw_mode = DrawMode::Hover;
  let mut prev_mouse_pos_in_field = (0, 0);
  // let mut trail: Vec<(i32, i32)> = Vec::new();
  let mut trail = Trail::new();
  let (mut rl, thread) = raylib::init()
                                .size(
                                  FIELD_WIDTH as i32 * CELL_SIZE, 
                                  FIELD_HEIGHT as i32 * CELL_SIZE)
                                .title("Game of Life")
                                .build();
  rl.set_target_fps(60);
  while !rl.window_should_close() {
 
    //    if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
    //     edit_flag = if edit_flag {false} else {true};
    // }

    
 
    let mut d = rl.begin_drawing(&thread);
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
          prev_mouse_pos_in_field = new_pos;                         // the trail grows when hovering over the same pixels, but thats what i need, actually
          trail.set_active(true)
               .ignite(&mut d, new_pos.0, new_pos.1, time_now)
               .bury(&mut d, time_now)
               .color(&mut d);                                         // the better way would be to check for mouse move separately
        }                                                              // and to bury on each frame
      },
    }

    


    d.clear_background(Color::WHITE);
    // draw_field(&mut d, &trail);

    // let x = d.get_mouse_position();
 
  
    // if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {}

    
    

  }
}