use std::ops::Mul;
use glam::Mat4;
use stereokit::*;
use stereokit::named_colors::BLUE_VIOLET;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "my-tag")))]
pub fn main() {
    println!("Hello World");
    _main();
}

pub fn _main() {
    let sk = SettingsBuilder::new().init().unwrap();
    let model = sk.model_create_mesh(Mesh::SPHERE, Material::PBR);
    sk.run(|sk| {
        sk.model_draw(
            &model,
            Mat4::IDENTITY.mul(Mat4::from_scale([0.1, 0.1, 0.1].into())),
            BLUE_VIOLET,
            RenderLayer::default()
        )
    }, |_| {});
}