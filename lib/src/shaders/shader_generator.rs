pub struct ShaderGenerator {}

impl ShaderGenerator {
    pub fn generate_plane(detail: usize) -> (Vec<f32>, Vec<u32>) {
        let mut points = vec![];
        let mut indecies = vec![];
        let detailf32 = detail as f32;

        for y in 0..(detail + 1) {
            for x in 0..(detail + 1) {
                // normalizes X and Y between -1. and 1.
                let normal_x = (2. * (x as f32 / detailf32) - 1.) / 2.;
                let normal_y = (2. * (y as f32 / detailf32) - 1.) / 2.;

                // pushes to points
                points.push(normal_x);
                points.push(normal_y);
                points.push(0.);
            }
        }

        let to_index = |x: usize, y: usize| (y * (detail + 1) + x) as u32;
        for x in 0..detail {
            for y in 0..detail {
                // top triangle
                indecies.push(to_index(x, y));
                indecies.push(to_index(x + 1, y));
                indecies.push(to_index(x, y + 1));

                // bottom triangle
                indecies.push(to_index(x, y + 1));
                indecies.push(to_index(x + 1, y));
                indecies.push(to_index(x + 1, y + 1));
            }
        }

        (points, indecies)
    }

}
