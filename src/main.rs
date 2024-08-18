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

enum GameState {
    WelcomeScreen,
    LevelSelection,
    Playing,
    LevelCompleted,
}

fn draw_text(fb: &mut FrameBuffer, x: usize, y: usize, text: &str, scale: usize) {
    let font = [
        [0b01110, 0b10001, 0b11111, 0b10001, 0b10001], // A
        [0b11110, 0b10001, 0b11110, 0b10001, 0b11110], // B
        [0b01111, 0b10000, 0b10000, 0b10000, 0b01111], // C
        [0b11110, 0b10001, 0b10001, 0b10001, 0b11110], // D
        [0b11111, 0b10000, 0b11110, 0b10000, 0b11111], // E
        [0b11111, 0b10000, 0b11110, 0b10000, 0b10000], // F
        [0b01111, 0b10000, 0b10111, 0b10001, 0b01111], // G
        [0b10001, 0b10001, 0b11111, 0b10001, 0b10001], // H
        [0b01110, 0b00100, 0b00100, 0b00100, 0b01110], // I
        [0b00111, 0b00010, 0b00010, 0b10010, 0b01100], // J
        [0b10001, 0b10010, 0b11100, 0b10010, 0b10001], // K
        [0b10000, 0b10000, 0b10000, 0b10000, 0b11111], // L
        [0b10001, 0b11011, 0b10101, 0b10001, 0b10001], // M
        [0b10001, 0b11001, 0b10101, 0b10011, 0b10001], // N
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // O
        [0b11110, 0b10001, 0b11110, 0b10000, 0b10000], // P
        [0b01110, 0b10001, 0b10001, 0b10011, 0b01111], // Q
        [0b11110, 0b10001, 0b11110, 0b10010, 0b10001], // R
        [0b01111, 0b10000, 0b01110, 0b00001, 0b11110], // S
        [0b11111, 0b00100, 0b00100, 0b00100, 0b00100], // T
        [0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // U
        [0b10001, 0b10001, 0b10001, 0b01010, 0b00100], // V
        [0b10001, 0b10001, 0b10101, 0b11011, 0b10001], // W
        [0b10001, 0b01010, 0b00100, 0b01010, 0b10001], // X
        [0b10001, 0b01010, 0b00100, 0b00100, 0b00100], // Y
        [0b11111, 0b00010, 0b00100, 0b01000, 0b11111], // Z
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
        [0b00000, 0b00000, 0b00000, 0b00100, 0b00100], // .
    ];

    let fb_width = fb.width();
    let fb_height = fb.height();

    for (i, c) in text.chars().enumerate() {
        let digit = match c {
            'A'..='Z' => c as usize - 'A' as usize,
            '0'..='9' => c as usize - '0' as usize + 26,
            '.' => 36,
            ' ' => 37,
            _ => continue, // Ignorar caracteres no soportados
        };

        // Posición inicial del carácter
        let start_x = x + i * (7 * scale); // Ajuste del espaciado horizontal

        for row in 0..5 {
            for col in 0..5 {
                if font[digit][row] & (1 << (4 - col)) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let pos_x = start_x + col * scale + sx;
                            let pos_y = y + row * scale + sy;
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

fn draw_welcome_screen(fb: &mut FrameBuffer) {
    fb.set_background_color(0x0000FF); // Establece el color de fondo a azul
    fb.clear(); // Limpia la pantalla con el color de fondo

    fb.set_current_color(0xFFFFFF); // Establece el color del texto a blanco

    // Dibuja el texto de bienvenida
    draw_text(fb, 50, 200, "BIENVENIDO AL JUEGO", 3);
    draw_text(fb, 50, 300, "PRESIONA ENTER PARA COMENZAR A JUGAR", 2);
}



fn draw_level_selection_screen(fb: &mut FrameBuffer) {
    fb.set_background_color(0x0000FF); // Establece el color de fondo a azul
    fb.clear(); // Limpia la pantalla con el color de fondo

    fb.set_current_color(0xFFFFFF); // Establece el color del texto a blanco

    // Dibuja el texto de selección de nivel
    draw_text(fb, 50, 200, "SELECCIONA UN NIVEL", 3);
    draw_text(fb, 50, 300, "1. NIVEL 1", 2);
    draw_text(fb, 50, 350, "2. NIVEL 2", 2);
}

fn draw_level_completed_screen(fb: &mut FrameBuffer) {
    fb.set_background_color(0x00FF00); // Establece el color de fondo a verde
    fb.clear(); // Limpia la pantalla con el color de fondo

    fb.set_current_color(0xFFFFFF); // Establece el color del texto a blanco

    // Dibuja el texto de nivel completado
    draw_text(fb, 50, 200, "NIVEL COMPLETADO", 3);
    draw_text(fb, 50, 300, "PRESIONA ENTER PARA VOLVER AL MENU", 2);
}


fn render_minimap(fb: &mut FrameBuffer, maze: &Maze, player: &Player, minimap_size: usize, offset_x: usize, offset_y: usize) {
    let scale = minimap_size / maze.width.max(maze.height);

    let fb_width = fb.width();
    let fb_height = fb.height();

    // Dibujar el mapa
    for y in 0..maze.height {
        for x in 0..maze.width {
            let color = if maze.is_wall(x, y) {
                0xFFFFFF // Muros en blanco
            } else {
                0x000000 // Caminos en negro
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

    // Dibujar el punto de ganar (goal) en verde
    let goal_x = offset_x + maze.goal_x * scale;
    let goal_y = offset_y + maze.goal_y * scale;

    for i in 0..scale {
        for j in 0..scale {
            let pos_x = goal_x + i;
            let pos_y = goal_y + j;
            if pos_x < fb_width && pos_y < fb_height {
                fb.buffer[pos_y * fb_width + pos_x] = 0x00FF00; // Verde
            }
        }
    }

    // Dibujar al jugador en rojo
    let player_x = offset_x + (player.x * scale as f64) as usize;
    let player_y = offset_y + (player.y * scale as f64) as usize;

    let player_size = scale / 2;

    for i in 0..player_size {
        for j in 0..player_size {
            let pos_x = player_x + i;
            let pos_y = player_y + j;
            if pos_x < fb_width && pos_y < fb_height {
                fb.buffer[pos_y * fb_width + pos_x] = 0xFF0000; // Rojo
            }
        }
    }
}


fn main() {
    let width: usize = 640;
    let height: usize = 480;

    let mut fb = FrameBuffer::new(width, height);

    let mut current_level = 1;
    let mut maze = Maze::new(current_level);
    let mut player = Player::new(maze.start_x as f64 + 0.5, maze.start_y as f64 + 0.5, -1.0, 0.0, 0.0, 0.66);

    let mut game_state = GameState::WelcomeScreen;

    let mut window = Window::new(
        "Ray Caster - ESC para salir",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;

    let mut last_mouse_x = width as f64 / 2.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        fb.clear();

        match game_state {
            GameState::WelcomeScreen => {
                draw_welcome_screen(&mut fb);
                if window.is_key_down(Key::Enter) {
                    game_state = GameState::LevelSelection;
                }
            },
            GameState::LevelSelection => {
                draw_level_selection_screen(&mut fb);
                if window.is_key_down(Key::Key1) {
                    current_level = 1;
                    maze = Maze::new(current_level);
                    player = Player::new(maze.start_x as f64 + 0.5, maze.start_y as f64 + 0.5, -1.0, 0.0, 0.0, 0.66);
                    game_state = GameState::Playing;
                } else if window.is_key_down(Key::Key2) {
                    current_level = 2;
                    maze = Maze::new(current_level);
                    player = Player::new(maze.start_x as f64 + 0.5, maze.start_y as f64 + 0.5, -1.0, 0.0, 0.0, 0.66);
                    game_state = GameState::Playing;
                }
            },
            GameState::Playing => {
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

                // Limitar la rotación con el mouse
                let mouse_sensitivity: f64 = 0.02;
                let (mouse_x, _) = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((width as f32 / 2.0, 0.0));
                let delta_x = mouse_x as f64 - last_mouse_x;
                last_mouse_x = mouse_x as f64;

                if delta_x != 0.0 {
                    let rotation_angle = -delta_x * mouse_sensitivity;
                    player.rotate(rotation_angle);
                }

                cast_rays(&mut fb, &maze, &player);
                render_minimap(&mut fb, &maze, &player, 100, 10, 10);

                // Verificar si el jugador ha alcanzado la meta
                if maze.is_goal(player.x as usize, player.y as usize) {
                    game_state = GameState::LevelCompleted;
                }
            },
            GameState::LevelCompleted => {
                draw_level_completed_screen(&mut fb);
                if window.is_key_down(Key::Enter) {
                    game_state = GameState::WelcomeScreen;
                }
            }
        }

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

        fb.set_current_color(0x000000);
        for y in 0..15 {
            for x in 0..text_width {
                fb.point(text_x + x, 10 + y);
            }
        }

        fb.set_current_color(0xFFFFFF);
        draw_text(&mut fb, text_x, 10, &fps_text, 3);

        if fb.buffer.len() == width * height {
            window.update_with_buffer(&fb.buffer, width, height).unwrap();
        }
    }

    bmp::save_as_bmp("output.bmp", width, height, &fb.buffer);
}
