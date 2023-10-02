use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::{cube, quad},
        transform::components::{lookat_target, rotation, scale, translation},
    },
    prelude::*,
};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    for i in 1..=4 {
        Entity::new()
            .with(cube(), ())
            .with(translation(), vec3(i as f32, 0., 1.))
            .with(rotation(), Quat::from_rotation_z(i as f32))
            .with(scale(), Vec3::ONE * i as f32 * 0.2)
            .spawn();
    }

    println!("Hello, Ambient!");
}
