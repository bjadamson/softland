pub fn make_cube_vertices(dimensions: &(f32, f32, f32)) -> [[f32; 4]; 36] {
    let &(w, h, l) = dimensions;

    // bottom face
    [[w, -h, -l, 1.0],
     [w, -h, l, 1.0],
     [-w, -h, l, 1.0],
     [w, -h, -l, 1.0],
     [-w, -h, l, 1.0],
     [-w, -h, -l, 1.0],
     // top face
     [w, h, -l, 1.0],
     [-w, h, -l, 1.0],
     [-w, h, l, 1.0],
     [w, h, -l, 1.0],
     [-w, h, l, 1.0],
     [w, h, l, 1.0],
     // right face
     [w, -h, -l, 1.0],
     [w, h, -l, 1.0],
     [w, h, l, 1.0],
     [w, -h, -l, 1.0],
     [w, h, l, 1.0],
     [w, -h, l, 1.0],
     // front face
     [w, -h, l, 1.0],
     [w, h, l, 1.0],
     [-w, h, l, 1.0],
     [w, -h, l, 1.0],
     [-w, h, l, 1.0],
     [-w, -h, l, 1.0],
     // left face
     [-w, -h, l, 1.0],
     [-w, h, l, 1.0],
     [-w, h, -l, 1.0],
     [-w, -h, l, 1.0],
     [-w, h, -l, 1.0],
     [-w, -h, -l, 1.0],
     // back face
     [w, h, -l, 1.0],
     [w, -h, -l, 1.0],
     [-w, -h, -l, 1.0],
     [w, h, -l, 1.0],
     [-w, -h, -l, 1.0],
     [-w, h, -l, 1.0]]
}

pub fn make_triangle_vertices(radius: f32) -> [[f32; 4]; 3] {
    [[-radius, -radius, 0.0, 1.0], [radius, -radius, 0.0, 1.0], [0.0, radius, 0.0, 1.0]]
}
