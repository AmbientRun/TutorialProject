use ambient_api::{
    core::{
        physics::components::{dynamic, sphere_collider},
        primitives::concepts::Sphere,
        rendering::components::color,
        transform::{
            components::{scale, translation},
            concepts::Transformable,
        },
    },
    prelude::*,
};

use packages::my_game::components::bouncy_created;

#[main]
pub fn main() {
    fixed_rate_tick(Duration::from_secs_f32(1.0), |_| {
        Entity::new()
            .with_merge(Sphere::suggested())
            .with_merge(Transformable::suggested())
            .with(scale(), Vec3::ONE * (random::<f32>() + 0.5))
            .with(translation(), Vec3::Z * 10.)
            .with(color(), random::<Vec3>().extend(1.0))
            .with(sphere_collider(), 0.5)
            .with(dynamic(), true)
            .with(bouncy_created(), game_time())
            .spawn();
    });
}
