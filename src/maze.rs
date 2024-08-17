pub struct Maze {
    pub map: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    pub fn new() -> Self {
        // Mapa más complejo con múltiples caminos y áreas abiertas
        let map = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        // Asegúrate de que `width` y `height` coincidan con las dimensiones del mapa
        let width = map[0].len();
        let height = map.len();

        Self { map, width, height }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.map[y][x] == 1
    }
}
