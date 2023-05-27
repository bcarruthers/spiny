use glam::Mat4;

#[derive(Clone, Copy)]
pub struct CameraParams {
    pub view: Mat4,
    pub proj: Mat4,
    pub fog_color: [f32; 4],
    pub fog_depth: f32,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            fog_color: [0.025, 0.05, 0.1, 1.0],
            fog_depth: 0.0,
        }
    }
}

impl CameraParams {
    pub fn proj_view(&self) -> Mat4 {
        self.proj * self.view
    }
}