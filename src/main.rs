mod framebuffer;
mod player;
mod maze;
mod caster;
mod bmp;

use crate::framebuffer::FrameBuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::cast_rays;
use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};

fn draw_text(fb: &mut FrameBuffer, x: usize, y: usize, text: &str, scale: usize) {
    let font = [
        [0b11111, 0b10001, 0b10001, 0b10001, 0b11111], // 0
        [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
        [0b11111, 0b00001, 0b11111, 0b10000, 0b11111], // 2
        [0b11111, 0b00001, 0b11111, 0b00001, 0b11111], // 3
        [0b10001, 0b10001, 0b11111, 0b00001, 0b00001], // 4
        [0b11111, 0b10000, 0b11111, 0b00001, 0b11111], // 5
        [0b11111, 0b10000, 0b11111, 0b10001, 0b11111], // 6
        [0b11111, 0b00001, 0b00010, 0b00100, 0b01000], // 7
        [0b11111, 0b10001, 0b11111, 0b10001, 0b11111], // 8
        [0b11111, 0b10001, 0b11111, 0b00001, 0b11111], // 9
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00000], // Space
        [0b00000, 0b00100, 0b00100, 0b00000, 0b00100], // .
    ];

    let fb_width = fb.width();
    let fb_height = fb.height();

    for (i, c) in text.chars().enumerate() {
        let digit = match c {
            '0'..='9' => c as usize - '0' as usize,
            '.' => 11,
            _ => 10, // Space
        };

        for row in 0..5 {
            for col in 0..5 {
                if font[digit][row] & (1 << (4 - col)) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let pos_x = x + i * (6 * scale) + col * scale + sx;
                            let pos_y = y + row * scale + sy;
                            // Verificación adicional antes de dibujar
                            if pos_x < fb_width && pos_y < fb_height {
                                fb.point(pos_x, pos_y);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_minimap(fb: &mut FrameBuffer, maze: &Maze, player: &Player, minimap_size: usize, offset_x: usize, offset_y: usize) {
    let scale = minimap_size / maze.width.max(maze.height);

    let fb_width = fb.width();
    let fb_height = fb.height();

    for y in 0..maze.height {
        for x in 0..maze.width {
            let color = if maze.is_wall(x, y) {
                0xFFFFFF
            } else {
                0x000000
            };

            for i in 0..scale {
                for j in 0..scale {
                    let pos_x = offset_x + x * scale + i;
                    let pos_y = offset_y + y * scale + j;
                    if pos_x < fb_width && pos_y < fb_height {
                        fb.buffer[pos_y * fb_width + pos_x] = color;
                    }
                }
            }
        }
    }

    let player_x = offset_x + (player.x * scale as f64) as usize;
    let player_y = offset_y + (player.y * scale as f64) as usize;

    let player_size = scale / 2;

    for i in 0..player_size {
        for j in 0..player_size {
            let pos_x = player_x + i;
            let pos_y = player_y + j;
            if pos_x < fb_width && pos_y < fb_height {
                fb.buffer[pos_y * fb_width + pos_x] = 0xFF0000;
            }
        }
    }
}

fn main() {
    let width: usize = 640;
    let height: usize = 480;

    let mut fb = FrameBuffer::new(width, height);
    let maze = Maze::new();
    let mut player = Player::new(3.5, 3.5, -1.0, 0.0, 0.0, 0.66);

    // Verificación de que el jugador spawnea dentro del mapa
    assert!(player.x >= 0.0 && player.x < maze.width as f64, "Player X position is out of bounds!");
    assert!(player.y >= 0.0 && player.y < maze.height as f64, "Player Y position is out of bounds!");

    fb.clear();

    let mut window = Window::new(
        "Ray Caster - ESC para salir",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut last_mouse_x = width as f64 / 2.0;

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        fb.clear();

        if window.is_key_down(Key::W) {
            player.move_forward(0.1, &maze);
        }
        if window.is_key_down(Key::S) {
            player.move_backward(0.1, &maze);
        }
        if window.is_key_down(Key::A) {
            player.rotate(-0.05);
        }
        if window.is_key_down(Key::D) {
            player.rotate(0.05);
        }

        let mouse_sensitivity: f64 = 0.02;
        let (mouse_x, _mouse_y) = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((last_mouse_x as f32, 0.0));

        let delta_x = mouse_x as f64 - last_mouse_x;
        last_mouse_x = mouse_x as f64;

        if delta_x != 0.0 {
            let rotation_angle = -delta_x * mouse_sensitivity;
            player.rotate(rotation_angle);
        }

        cast_rays(&mut fb, &maze, &player);

        render_minimap(&mut fb, &maze, &player, 100, 10, 10);

        frame_count += 1;
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time);

        if elapsed >= Duration::new(1, 0) {
            fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;
            last_time = current_time;
        }

        let fps_text = format!("FPS: {:.2}", fps);
        let text_width = fps_text.len() * 6 * 3;
        let text_x = width - text_width - 10;

        let fps_area_width = text_width;
        let fps_area_height = 5 * 3;

        fb.set_current_color(0x000000);
        for y in 0..fps_area_height {
            for x in 0..fps_area_width {
                fb.point(text_x + x, 10 + y);
            }
        }

        fb.set_current_color(0xFFFFFF);
        draw_text(&mut fb, text_x, 10, &fps_text, 3);

        // Verificación de límites antes de actualizar el buffer
        if fb.buffer.len() == width * height {
            window.update_with_buffer(&fb.buffer, width, height).unwrap();
        }
    }

    bmp::save_as_bmp("output.bmp", width, height, &fb.buffer);
}
