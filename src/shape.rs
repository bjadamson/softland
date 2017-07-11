pub fn make_cube_vertices(dimensions: &(f32, f32, f32)) -> [[f32; 4]; 8] {
    let &(w, h, l) = dimensions;

    [[-w, -h, l,  1.0], // front bottom-left
    [w,   -h, l,  1.0], // front bottom-right
    [w,   h, l,   1.0], // front top-right
    [-w,  h, l,   1.0], // front top-left

    [-w,  -h, -l, 1.0], // back bottom-left
    [w,   -h, -l, 1.0], // back bottom-right
    [w,   h, -l,  1.0], // back top-right
    [-w,  h,  -l, 1.0]] // back top-left
}

pub fn make_triangle_vertices(radius: f32) -> [[f32; 4]; 3]{
    [[-radius, -radius, 0.0, 1.0], [radius, -radius, 0.0, 1.0], [0.0, radius, 0.0, 1.0]]
}
