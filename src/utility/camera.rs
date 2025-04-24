use cgmath::{Matrix4, Point3, Vector3, InnerSpace};

pub struct Camera {
    m_view_matrix: Matrix4<f32>,
    m_position: Point3<f32>,
    m_front: Vector3<f32>,
    m_up: Vector3<f32>,
    m_right: Vector3<f32>,
    m_world_up: Vector3<f32>,

    m_yaw: f32,
    m_pitch: f32,
    m_movement_speed: f32,
}

impl Camera {
    pub fn new() -> Camera {
        let m_view_matrix = Matrix4::new( 
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32,
        );
        let m_position = Point3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 };
        let m_world_up = Vector3 { x: 0.0f32, y: 1.0f32, z: 0.0f32 };
        let mut camera = Camera {
            m_view_matrix,
            m_position,
            m_front: Vector3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 },
            m_up: Vector3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 },
            m_right: Vector3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 },
            m_world_up,
            m_yaw: 90.0f32,
            m_pitch: 0.0f32,
            m_movement_speed: 5.0f32
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
}