pub struct Grid2D<T> {
    width: usize,
    height: usize,
    size: usize,
    data: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn new(width: usize, height: usize, default_val: T) -> Self
    where
        T: Clone,
    {
        let size = width * height;
        Self {
            width,
            height,
            size,
            data: vec![default_val; size],
        }
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }


    pub fn get(&self, x: usize, y: usize) -> &T {
        return &self.data[y * self.width + x];
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.data[y * self.width + x] = val;
    }

    pub fn get_at_index(&self, i: usize) -> &T {
        return &self.data[i];
    }

    pub fn set_at_index(&mut self, i: usize, val: T) {
        self.data[i] = val;
    }


    pub fn resize(&mut self, width: usize, height: usize, default_val: T)
    where
        T: Clone,
    {
        self.size = width * height;
        self.width = width;
        self.height = height;
        self.data = vec![default_val; self.size];
    }
}

pub fn make_uniform_grid(
    out_grid: &mut Grid2D<glam::Vec2>,
    width: usize,
    height: usize,
) {
    out_grid.resize(width, height, glam::Vec2::ZERO);
    let mut index = 0;
    for y in 0..height {
        for x in 0..width {
            out_grid.set_at_index(
                index,
                glam::Vec2::new(
                    x as f32 / (width as f32 - 1.0),
                    y as f32 / (height as f32 - 1.0),
                ),
            );
            index += 1;
        }
    }
}

pub fn make_grid_indices(out_indices: &mut Vec<u32>, width: usize, height: usize) {
    out_indices.clear();
    out_indices.reserve((width - 1) * (height - 1) * 6);

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let p00 = y * width + x;
            let p10 = p00 + 1;
            let p01 = p00 + width;
            let p11 = p10 + width;

            // Add the two faces...
            out_indices.push(p00 as u32);
            out_indices.push(p11 as u32);
            out_indices.push(p10 as u32);
            out_indices.push(p00 as u32);
            out_indices.push(p01 as u32);
            out_indices.push(p11 as u32);
        }
    }
}