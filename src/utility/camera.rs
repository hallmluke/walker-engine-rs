use cgmath::{Matrix4, Point3, Vector3, InnerSpace};
use winit::event::MouseButton;
use winit::keyboard::{Key, KeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Camera {
    pub m_view_matrix: Matrix4<f32>,
    m_position: Point3<f32>,
    m_front: Vector3<f32>,
    m_up: Vector3<f32>,
    m_right: Vector3<f32>,
    m_world_up: Vector3<f32>,

    m_yaw: f32,
    m_pitch: f32,
    m_keyboard_movement_speed: f32,
    m_mouse_movement_speed: f32
}

impl Camera {
    pub fn new() -> Camera {
        let m_view_matrix = Matrix4::new( 
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
        );
        let m_position = Point3 { x: -3.0f32, y: 0.5f32, z: 0.0f32 };
        let m_world_up = Vector3 { x: 0.0f32, y: 1.0f32, z: 0.0f32 };
        let mut camera = Camera {
            m_view_matrix,
            m_position,
            m_front: Vector3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 },
            m_up: Vector3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 },
            m_right: Vector3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 },
            m_world_up,
            m_yaw: 0.0f32,
            m_pitch: 0.0f32,
            m_keyboard_movement_speed: 5.0f32,
            m_mouse_movement_speed: 5.0
        };
        camera.update_camera_vectors();
        camera.update_view_matrix();
        camera
    }

    fn update_camera_vectors(&mut self) {
        let mut front: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        front.x = f32::cos(self.m_yaw.to_radians()) * f32::cos(self.m_pitch.to_radians());
        front.y = f32::sin(self.m_pitch.to_radians());
        front.z = f32::sin(self.m_yaw.to_radians()) * f32::cos(self.m_pitch.to_radians());
        self.m_front = front;
        self.m_front.normalize();
        self.m_right = self.m_front.cross(self.m_world_up);
        self.m_right.normalize();
        self.m_up = self.m_right.cross(self.m_front).normalize();
    }

    fn update_view_matrix(&mut self) {
        self.m_view_matrix = Matrix4::look_at(
            self.m_position,
            self.m_position + self.m_front,
            self.m_up
        );
    }

    pub fn update(&mut self, delta_time: f32, input: &WinitInputHelper) {
        if input.mouse_held(MouseButton::Right) {
            let cursor_diff = input.cursor_diff();
            self.m_yaw += cursor_diff.0 * delta_time * self.m_mouse_movement_speed;
            self.m_pitch -= cursor_diff.1 * delta_time * self.m_mouse_movement_speed;
            self.m_pitch = f32::min(self.m_pitch, 89.0);
            self.m_pitch = f32::max(self.m_pitch, -89.0);

            let velocity = self.m_keyboard_movement_speed * delta_time;

            if input.key_held(KeyCode::KeyW) {
                self.m_position += self.m_front * velocity;
            }
            if input.key_held(KeyCode::KeyA) {
                self.m_position -= self.m_right * velocity;
            }
            if input.key_held(KeyCode::KeyS) {
                self.m_position -= self.m_front * velocity;
            }
            if input.key_held(KeyCode::KeyD) {
                self.m_position += self.m_right * velocity;
            }

            println!("Position: {}, {}, {}", self.m_position.x, self.m_position.y, self.m_position.z);
            println!("Front: {}, {}, {}", self.m_front.x, self.m_front.y, self.m_front.z);
            println!("Yaw: {}, Pitch: {}", self.m_yaw, self.m_pitch);
        }

        self.update_camera_vectors();
        self.update_view_matrix();
    }
}