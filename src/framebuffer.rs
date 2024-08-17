pub struct FrameBuffer {
    pub buffer: Vec<u32>,
    width: usize,
    height: usize,
    background_color: u32,
    current_color: u32,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            background_color: 0x000000, // color negro por defecto
            current_color: 0xFFFFFF, // color blanco por defecto
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        // Verificación de los límites antes de acceder al buffer
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        } else {
            println!("Attempted to draw outside framebuffer: x = {}, y = {}", x, y);
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
