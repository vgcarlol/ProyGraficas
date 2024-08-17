use crate::maze::Maze;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub dir_x: f64,
    pub dir_y: f64,
    pub plane_x: f64,
    pub plane_y: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, dir_x: f64, dir_y: f64, plane_x: f64, plane_y: f64) -> Self {
        Self { x, y, dir_x, dir_y, plane_x, plane_y }
    }

    pub fn move_forward(&mut self, distance: f64, maze: &Maze) {
        if !maze.is_wall((self.x + self.dir_x * distance) as usize, self.y as usize) {
            self.x += self.dir_x * distance;
        }
        if !maze.is_wall(self.x as usize, (self.y + self.dir_y * distance) as usize) {
            self.y += self.dir_y * distance;
        }
    }

    pub fn move_backward(&mut self, distance: f64, maze: &Maze) {
        if !maze.is_wall((self.x - self.dir_x * distance) as usize, self.y as usize) {
            self.x -= self.dir_x * distance;
        }
        if !maze.is_wall(self.x as usize, (self.y - self.dir_y * distance) as usize) {
            self.y -= self.dir_y * distance;
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * angle.cos() - self.dir_y * angle.sin();
        self.dir_y = old_dir_x * angle.sin() + self.dir_y * angle.cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * angle.cos() - self.plane_y * angle.sin();
        self.plane_y = old_plane_x * angle.sin() + self.plane_y * angle.cos();
    }
}
