extern crate cgmath;
use cgmath::*;

// extern crate num_traits;
// use self::num_traits::identities::Zero;

type Vec3 = Vector3<f32>;
type Vec4 = Vector4<f32>;
type Mat4 = Matrix4<f32>;

#[derive(Debug)]
pub struct Camera {
    front: Vec3,
    up: Vec3,

    pitch: f32,
    roll: f32,
    yaw: f32,

    orientation: Quaternion<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            front: Vec3::zero(),
            up: Vec3::zero(),
            pitch: Default::default(),
            roll: Default::default(),
            yaw: Default::default(),
            orientation: Quaternion::one(),
        }
    }

    pub fn compute_view(&self) -> Mat4 {
        let rotate: Mat4 = Mat4::from(self.orientation);

        let f = -self.front;
        let translate: Mat4 = Mat4::from_translation(Vec3::new(f.x, f.y, f.z));
        let x: Mat4 = rotate * translate;
        rotate * translate
    }

    fn right_vector(&self) -> Vec3 {
        Vec3::normalize(Vec3::cross(self.front, self.up))
    }

    // fn xpan(&d: f32) -> Vec3 {
    // Vec3::new(d, 0.0, 0.0);
    // }

    // fn ypan(d: f32) -> Vec3 {
    // Vec3::new(0.0, -d, 0.0);
    // }

    fn move_dir(&mut self, s: f32, dir: &Vec3) {
        self.front += dir * s;
        // self.skybox_.model.translation = this->front;
    }

    fn move_z(&mut self, s: f32) {
        let mat = self.compute_view();
        let forward = -mat[2];
        let forward = Vec3::new(forward.x, forward.y, forward.z);
        self.move_dir(s, &forward)
    }

    fn move_x(&mut self, s: f32) {
        let mat = self.compute_view();
        let strafe = -mat[0];
        let strafe = Vec3::new(strafe.x, strafe.y, strafe.z);
        self.move_dir(s, &strafe)
    }

    fn move_y(&mut self, s: f32) {
        let mat = self.compute_view();
        let updown = -mat[1];
        let updown = Vec3::new(updown.x, updown.y, updown.z);
        self.move_dir(s, &updown)
    }

    pub fn move_forward(&mut self, s: f32) {
        self.move_z(s);
    }

    pub fn move_backward(&mut self, s: f32) {
        self.move_z(-s);
    }

    pub fn move_left(&mut self, s: f32) {
        self.move_x(s);
    }

    pub fn move_right(&mut self, s: f32) {
        self.move_x(-s);
    }

    pub fn move_up(&mut self, s: f32) {
        self.move_y(-s);
    }

    pub fn move_down(&mut self, s: f32) {
        self.move_y(s);
    }

    pub fn position(&self) -> Vec3 {
        self.front
    }
}
