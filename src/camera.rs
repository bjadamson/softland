use cgmath;
use cgmath::*;
use state;
use state::{MouseState, MouseSensitivity};

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
            front: [0.0, 0.0, -1.0].into(),
            up: [0.0, 1.0, 0.0].into(),
            pitch: Default::default(),
            roll: Default::default(),
            yaw: Default::default(),
            orientation: Quaternion::one(),
        }
    }

    pub fn compute_view(&self) -> Mat4 {
        let rotation: Mat4 = Mat4::from(self.orientation);

        let f = -self.front;
        let translate = Mat4::from_translation(f.into());
        rotation * translate
    }

    fn right_vector(&self) -> Vec3 {
        Vec3::normalize(Vec3::cross(self.front, self.up))
    }

    fn move_dir(&mut self, s: f32, dir: &Vec3) {
        self.front += dir * s;
        // self.skybox_.model.translation = this->front;
    }

    fn move_z(&mut self, s: f32) {
        let mat = self.compute_view();
        let forward = Vec3::new(mat[0][2], mat[1][2], mat[2][2]);
        let forward = -forward;
        self.move_dir(s, &forward)
    }

    fn move_x(&mut self, s: f32) {
        let mat = self.compute_view();
        let strafe = Vec3::new(mat[0][0], mat[1][0], mat[2][0]);
        self.move_dir(s, &strafe);
    }

    fn move_y(&mut self, s: f32) {
        let mat = self.compute_view();
        let updown = Vec3::new(mat[0][1], mat[1][1], mat[2][1]);
        self.move_dir(s, &updown)
    }

    pub fn move_forward(&mut self, s: f32) {
        self.move_z(s);
    }

    pub fn move_backward(&mut self, s: f32) {
        self.move_z(-s);
    }

    pub fn move_left(&mut self, s: f32) {
        self.move_x(-s);
    }

    pub fn move_right(&mut self, s: f32) {
        self.move_x(s);
    }

    pub fn move_up(&mut self, s: f32) {
        self.move_y(-s);
    }

    pub fn move_down(&mut self, s: f32) {
        self.move_y(s);
    }

    pub fn pan_x(&mut self, s: f32) {
        self.move_dir(s, &Vector3::new(1.0, 0.0, 0.0));
    }

    pub fn pan_y(&mut self, s: f32) {
        self.move_dir(s, &Vector3::new(0.0, 1.0, 0.0));
    }

    pub fn rotate_to(&mut self,
                     (xnew, ynew): (f32, f32),
                     cursor_pos: (f32, f32),
                     sensitivity: MouseSensitivity)
                     -> &mut Self {
        let delta: Vector2<f32> = {
            let (xnew, ynew) = (xnew, ynew);
            let (x, y) = cursor_pos;
            let (xpos, ypos) = (xnew - x, ynew - y);
            Vector2::new(xpos, ypos)
        };

        let yaw = sensitivity.x * delta.x;
        let pitch = sensitivity.y * delta.y;
        let roll = self.roll;

        self.yaw += yaw;
        self.pitch += pitch;

        let quaternion = {
            let euler = Euler {
                x: Rad(pitch),
                y: Rad(yaw),
                z: Rad(roll),
            };
            Quaternion::from(euler)
        };

        self.orientation = (quaternion * self.orientation).normalize();
        self
    }

    pub fn position(&self) -> Vec3 {
        self.front
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        self.orientation
    }
}
