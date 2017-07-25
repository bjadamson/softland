use shader;
use shader::{ColorVertex, UvVertex};

#[inline(always)]
pub fn construct_uv_cube<'a>() -> ([UvVertex; 36], &'a [u16]) {
    let vertices = make_cube_vertices();
    let normals = make_cube_normals();
    let uvs = make_cube_uvs();
    macro_rules! make_vertex {
        ($idx:expr, $uv:expr, $normal:expr) => (
            UvVertex {
                pos: [vertices[$idx][0], vertices[$idx][1], vertices[$idx][2], vertices[$idx][3]],
                uv: $uv,
                normal: normals[$normal],
            }
        )
    }
    macro_rules! make_vertex_for_face {
        ($idx:expr, $uv:expr, $normal:expr) => {{
            let v0 = make_vertex!($idx + 0, $uv[0], $normal);
            let v1 = make_vertex!($idx + 1, $uv[1], $normal);
            let v2 = make_vertex!($idx + 2, $uv[2], $normal);
            let v3 = make_vertex!($idx + 3, $uv[3], $normal);
            let v4 = make_vertex!($idx + 4, $uv[4], $normal);
            let v5 = make_vertex!($idx + 5, $uv[5], $normal);
            [v0, v1, v2, v3, v4, v5]
            }
    }};
    let v0 = make_vertex_for_face!(0, uvs[0], 0);
    let v1 = make_vertex_for_face!(6, uvs[1], 1);
    let v2 = make_vertex_for_face!(12, uvs[2], 2);
    let v3 = make_vertex_for_face!(18, uvs[3], 3);
    let v4 = make_vertex_for_face!(24, uvs[4], 4);
    let v5 = make_vertex_for_face!(30, uvs[5], 5);

    let v = [v0[0], v0[1], v0[2], v0[3], v0[4], v0[5], v1[0], v1[1], v1[2], v1[3], v1[4], v1[5],
             v2[0], v2[1], v2[2], v2[3], v2[4], v2[5], v3[0], v3[1], v3[2], v3[3], v3[4], v3[5],
             v4[0], v4[1], v4[2], v4[3], v4[4], v4[5], v5[0], v5[1], v5[2], v5[3], v5[4], v5[5]];

    const INDICES: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                              19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35];
    (v, &INDICES)
}

pub fn construct_color_cube<'a>(colors: &[[f32; 4]; 6]) -> ([ColorVertex; 36], &'a [u16]) {
    let vertices = make_cube_vertices();
    let normals = make_cube_normals();
    macro_rules! make_vertex {
        ($idx:expr, $color:expr, $normal:expr) => (
            ColorVertex {
                pos: [vertices[$idx][0], vertices[$idx][1], vertices[$idx][2], vertices[$idx][3]],
                color: colors[$color],
                normal: normals[$normal],
            }
        )
    }
    macro_rules! make_vertex_for_face {
        ($idx:expr, $color:expr, $normal:expr) => {{
            let v0 = make_vertex!($idx + 0, $color, $normal);
            let v1 = make_vertex!($idx + 1, $color, $normal);
            let v2 = make_vertex!($idx + 2, $color, $normal);
            let v3 = make_vertex!($idx + 3, $color, $normal);
            let v4 = make_vertex!($idx + 4, $color, $normal);
            let v5 = make_vertex!($idx + 5, $color, $normal);
            [v0, v1, v2, v3, v4, v5]
            }
    }};
    let v0 = make_vertex_for_face!(0, 0, 0);
    let v1 = make_vertex_for_face!(6, 1, 1);
    let v2 = make_vertex_for_face!(12, 2, 2);
    let v3 = make_vertex_for_face!(18, 3, 3);
    let v4 = make_vertex_for_face!(24, 4, 4);
    let v5 = make_vertex_for_face!(30, 5, 5);

    let v = [v0[0], v0[1], v0[2], v0[3], v0[4], v0[5], v1[0], v1[1], v1[2], v1[3], v1[4], v1[5],
             v2[0], v2[1], v2[2], v2[3], v2[4], v2[5], v3[0], v3[1], v3[2], v3[3], v3[4], v3[5],
             v4[0], v4[1], v4[2], v4[3], v4[4], v4[5], v5[0], v5[1], v5[2], v5[3], v5[4], v5[5]];

    const INDICES: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                              19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35];
    (v, &INDICES)
}

fn make_cube_vertices() -> [[f32; 4]; 36] {
    let (w, h, l) = (1.0, 1.0, 1.0);

    [// front face
     [w, -h, l, 1.0],
     [w, h, l, 1.0],
     [-w, h, l, 1.0],
     [w, -h, l, 1.0],
     [-w, h, l, 1.0],
     [-w, -h, l, 1.0],
     // back face
     [w, h, -l, 1.0],
     [w, -h, -l, 1.0],
     [-w, -h, -l, 1.0],
     [w, h, -l, 1.0],
     [-w, -h, -l, 1.0],
     [-w, h, -l, 1.0],
     // top face
     [w, h, -l, 1.0],
     [-w, h, -l, 1.0],
     [-w, h, l, 1.0],
     [w, h, -l, 1.0],
     [-w, h, l, 1.0],
     [w, h, l, 1.0],
     // bottom face
     [w, -h, -l, 1.0],
     [w, -h, l, 1.0],
     [-w, -h, l, 1.0],
     [w, -h, -l, 1.0],
     [-w, -h, l, 1.0],
     [-w, -h, -l, 1.0],
     // left face
     [-w, -h, l, 1.0],
     [-w, h, l, 1.0],
     [-w, h, -l, 1.0],
     [-w, -h, l, 1.0],
     [-w, h, -l, 1.0],
     [-w, -h, -l, 1.0],
     // right face
     [w, -h, -l, 1.0],
     [w, h, -l, 1.0],
     [w, h, l, 1.0],
     [w, -h, -l, 1.0],
     [w, h, l, 1.0],
     [w, -h, l, 1.0]]
}

fn make_cube_uvs() -> [[[f32; 2]; 6]; 6] {
    // let uv = [[1.0, 1.0], [1.0, 0.0], [0.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0]];
    let uv = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]];
    [uv, uv, uv, uv, uv, uv]
}

fn make_cube_normals() -> [[f32; 3]; 36] {
    [// front face
     [0.0, 0.0, -1.0],
     [0.0, 0.0, -1.0],
     [0.0, 0.0, -1.0],
     [0.0, 0.0, -1.0],
     [0.0, 0.0, -1.0],
     [0.0, 0.0, -1.0],
     // back face
     [0.0, 0.0, 1.0],
     [0.0, 0.0, 1.0],
     [0.0, 0.0, 1.0],
     [0.0, 0.0, 1.0],
     [0.0, 0.0, 1.0],
     [0.0, 0.0, 1.0],

     // top
     [0.0, 1.0, 0.0],
     [0.0, 1.0, 0.0],
     [0.0, 1.0, 0.0],
     [0.0, 1.0, 0.0],
     [0.0, 1.0, 0.0],
     [0.0, 1.0, 0.0],
     // bottom
     [0.0, -1.0, 0.0],
     [0.0, -1.0, 0.0],
     [0.0, -1.0, 0.0],
     [0.0, -1.0, 0.0],
     [0.0, -1.0, 0.0],
     [0.0, -1.0, 0.0],

     // left face
     [-1.0, 0.0, 0.0],
     [-1.0, 0.0, 0.0],
     [-1.0, 0.0, 0.0],
     [-1.0, 0.0, 0.0],
     [-1.0, 0.0, 0.0],
     [-1.0, 0.0, 0.0],
     // right face
     [1.0, 0.0, 0.0],
     [1.0, 0.0, 0.0],
     [1.0, 0.0, 0.0],
     [1.0, 0.0, 0.0],
     [1.0, 0.0, 0.0],
     [1.0, 0.0, 0.0]]
}

fn make_triangle_vertices(radius: f32) -> [[f32; 4]; 3] {
    [[-radius, -radius, 0.0, 1.0], [radius, -radius, 0.0, 1.0], [0.0, radius, 0.0, 1.0]]
}

pub fn make_triangle2d(length: f32, colors: &[[f32; 4]; 3]) -> [ColorVertex; 3] {
    let vertices = make_triangle_vertices(length);

    let normal = [0.0, 0.0, 0.0];
    let a = ColorVertex {
        pos: [vertices[0][0], vertices[0][1], vertices[0][2], vertices[0][3]],
        color: colors[0],
        normal: normal,
    };
    let b = ColorVertex {
        pos: [vertices[1][0], vertices[1][1], vertices[1][2], vertices[1][3]],
        color: colors[1],
        normal: normal,
    };
    let c = ColorVertex {
        pos: [vertices[2][0], vertices[2][1], vertices[2][2], vertices[2][3]],
        color: colors[2],
        normal: normal,
    };
    [a, b, c]
}
