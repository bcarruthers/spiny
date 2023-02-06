use glam::Mat4;

#[derive(Clone)]
pub struct CameraParams {
    pub view: Mat4,
    pub proj: Mat4,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
        }
    }
}

impl CameraParams {
    pub fn proj_view(&self) -> Mat4 {
        self.proj * self.view
    }
}