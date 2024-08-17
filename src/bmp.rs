use std::fs::File;
use std::io::Write;

pub fn save_as_bmp(filename: &str, width: usize, height: usize, buffer: &[u32]) {
    let mut file = File::create(filename).unwrap();

    // Tamaño total del archivo BMP (54 bytes de cabecera + tamaño del buffer)
    let file_size = 54 + (width * height * 3);

    // Encabezado BMP
    file.write_all(&[0x42, 0x4D]).unwrap(); // 'BM'
    file.write_all(&(file_size as u32).to_le_bytes()).unwrap(); // Tamaño del archivo
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Reservado
    file.write_all(&[54, 0, 0, 0]).unwrap(); // Offset al inicio de los datos de imagen

    // Información del encabezado
    file.write_all(&[40, 0, 0, 0]).unwrap(); // Tamaño del encabezado de información
    file.write_all(&(width as u32).to_le_bytes()).unwrap(); // Ancho
    file.write_all(&(height as u32).to_le_bytes()).unwrap(); // Alto
    file.write_all(&[1, 0]).unwrap(); // Planos
    file.write_all(&[24, 0]).unwrap(); // Bits por píxel
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Compresión
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Tamaño de la imagen
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Resolución X
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Resolución Y
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Colores usados
    file.write_all(&[0, 0, 0, 0]).unwrap(); // Colores importantes

    // Escribir datos de imagen
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let blue = (pixel & 0xFF) as u8;
            let green = ((pixel >> 8) & 0xFF) as u8;
            let red = ((pixel >> 16) & 0xFF) as u8;

            file.write_all(&[blue, green, red]).unwrap();
        }

        // Rellenar con ceros si el ancho no es múltiplo de 4 (alineación BMP)
        if (width * 3) % 4 != 0 {
            let padding = 4 - (width * 3) % 4;
            file.write_all(&vec![0; padding]).unwrap();
        }
    }
}
