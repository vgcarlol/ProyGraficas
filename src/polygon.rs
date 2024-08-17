use crate::framebuffer::FrameBuffer;

pub fn draw_polygon(fb: &mut FrameBuffer, points: &[(isize, isize)]) {
    let len = points.len();
    if len < 2 {
        return;
    }

    for i in 0..len {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % len];
        crate::line::draw_line(fb, x0, y0, x1, y1);
    }
}

pub fn fill_polygon(_fb: &mut FrameBuffer, _points: &[(isize, isize)]) {
    // Implementación pendiente
}
