use crate::framebuffer::FrameBuffer;
use crate::maze::Maze;
use crate::player::Player;
use image::GenericImageView;

pub fn cast_rays(fb: &mut FrameBuffer, maze: &Maze, player: &Player) {
    let wall_texture = image::open("assets/textures/wall_texture.jpg").expect("No se pudo abrir la textura de pared");
    let wall_texture = wall_texture.to_rgb8();
    let floor_texture = image::open("assets/textures/floor_texture.jpg").expect("No se pudo abrir la textura de piso");
    let floor_texture = floor_texture.to_rgb8();
    let ceiling_texture = image::open("assets/textures/ceiling_texture.png").expect("No se pudo abrir la textura de techo");
    let ceiling_texture = ceiling_texture.to_rgb8();

    let wall_width = wall_texture.width();
    let wall_height = wall_texture.height();
    let floor_width = floor_texture.width();
    let floor_height = floor_texture.height();
    let ceiling_width = ceiling_texture.width();
    let ceiling_height = ceiling_texture.height();

    let width = fb.width();
    let height = fb.height() as i32;
    let half_height = height / 2;

    for x in 0..width {
        let camera_x = 2.0 * x as f64 / width as f64 - 1.0;
        let ray_dir_x = player.dir_x + player.plane_x * camera_x;
        let ray_dir_y = player.dir_y + player.plane_y * camera_x;

        let mut map_x = player.x as i32;
        let mut map_y = player.y as i32;

        let delta_dist_x = (1.0 / ray_dir_x).abs();
        let delta_dist_y = (1.0 / ray_dir_y).abs();

        let step_x;
        let step_y;

        let mut side_dist_x;
        let mut side_dist_y;

        if ray_dir_x < 0.0 {
            step_x = -1;
            side_dist_x = (player.x - map_x as f64) * delta_dist_x;
        } else {
            step_x = 1;
            side_dist_x = (map_x as f64 + 1.0 - player.x) * delta_dist_x;
        }

        if ray_dir_y < 0.0 {
            step_y = -1;
            side_dist_y = (player.y - map_y as f64) * delta_dist_y;
        } else {
            step_y = 1;
            side_dist_y = (map_y as f64 + 1.0 - player.y) * delta_dist_y;
        }

        let mut hit = 0;
        let mut side = 0;

        while hit == 0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }

            if maze.is_wall(map_x as usize, map_y as usize) {
                hit = 1;
            }
        }

        let perp_wall_dist = if side == 0 {
            (map_x as f64 - player.x + (1 - step_x) as f64 / 2.0) / ray_dir_x
        } else {
            (map_y as f64 - player.y + (1 - step_y) as f64 / 2.0) / ray_dir_y
        };

        let line_height = (height as f64 / perp_wall_dist) as i32;
        let draw_start = (-line_height / 2 + half_height).max(0);
        let draw_end = (line_height / 2 + half_height).min(height - 1);

        let mut wall_x = if side == 0 {
            player.y + perp_wall_dist * ray_dir_y
        } else {
            player.x + perp_wall_dist * ray_dir_x
        };
        wall_x -= wall_x.floor();

        let tex_x = (wall_x * wall_width as f64) as u32;

        let fb_width = fb.width();

        // Renderizar el techo (líneas por encima del muro)
        for y in 0..draw_start {
            let current_dist = height as f64 / (2.0 * y as f64 - height as f64);
            
            let floor_x = player.x + current_dist * ray_dir_x;
            let floor_y = player.y + current_dist * ray_dir_y;

            let ceiling_tex_x = ((floor_x * ceiling_width as f64) as u32 % ceiling_width) as u32;
            let ceiling_tex_y = ((floor_y * ceiling_height as f64) as u32 % ceiling_height) as u32;

            let ceiling_pixel_index = (ceiling_tex_y * ceiling_width + ceiling_tex_x) as usize * 3;
            let ceiling_color = ((ceiling_texture.as_raw()[ceiling_pixel_index] as u32) << 16)
                | ((ceiling_texture.as_raw()[ceiling_pixel_index + 1] as u32) << 8)
                | (ceiling_texture.as_raw()[ceiling_pixel_index + 2] as u32);

            if (y as usize) < height as usize && x < fb_width {
                fb.buffer[y as usize * fb_width + x] = ceiling_color;
            }
        }

        // Renderizar el muro
        for y in draw_start..draw_end {
            let d = y * 256 - height * 128 + line_height * 128;
            let tex_y = ((d * wall_height as i32) / line_height / 256) as u32;

            let pixel_index = (tex_y * wall_width + tex_x) as usize * 3;
            let color = ((wall_texture.as_raw()[pixel_index] as u32) << 16)
                | ((wall_texture.as_raw()[pixel_index + 1] as u32) << 8)
                | (wall_texture.as_raw()[pixel_index + 2] as u32);

            if (y as usize) < height as usize && x < fb_width {
                fb.buffer[y as usize * fb_width + x] = color;
            }
        }

        // Renderizar el piso (líneas por debajo del muro)
        for y in draw_end..height {
            let current_dist = height as f64 / (2.0 * y as f64 - height as f64);
            
            let floor_x = player.x + current_dist * ray_dir_x;
            let floor_y = player.y + current_dist * ray_dir_y;

            let floor_tex_x = ((floor_x * floor_width as f64) as u32 % floor_width) as u32;
            let floor_tex_y = ((floor_y * floor_height as f64) as u32 % floor_height) as u32;

            let pixel_index = (floor_tex_y * floor_width + floor_tex_x) as usize * 3;
            let floor_color = ((floor_texture.as_raw()[pixel_index] as u32) << 16)
                | ((floor_texture.as_raw()[pixel_index + 1] as u32) << 8)
                | (floor_texture.as_raw()[pixel_index + 2] as u32);

            if (y as usize) < height as usize && x < fb_width {
                fb.buffer[y as usize * fb_width + x] = floor_color;
            }
        }
    }
}
