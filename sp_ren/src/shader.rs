use sp_asset::archive::FileArchive;
use std::{
    borrow::Cow,
    path::Path,
    sync::{Arc, Mutex},
};

pub fn load_shader(
    device: &wgpu::Device,
    assets: &Arc<Mutex<FileArchive>>,
    path: &str,
) -> wgpu::ShaderModule {
    let path = Path::new(path);
    let name = path.as_os_str().to_string_lossy();
    let mut assets = assets.lock().unwrap();
    let str = assets
        .read_string(path)
        .expect(&format!("Could not read shader {:?}", &name));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(name.as_ref()),
        source: wgpu::ShaderSource::Wgsl(Cow::Owned(str)),
    })
}
