use ambient_api::{
    core::{
        model::components::model_from_url,
        physics::components::{cube_collider, dynamic, plane_collider, sphere_collider},
        player::components::is_player,
        primitives::{
            components::{cube, quad},
            concepts::Sphere,
        },
        rendering::components::color,
        transform::{
            components::{scale, translation},
            concepts::{Transformable, TransformableOptional},
        },
    },
    prelude::*,
};
use packages::{
    character_animation::components::basic_character_animations,
    character_controller::components::use_character_controller,
    package_manager,
    this::{
        components::bouncy_created,
        messages::{Paint, Teleport},
    },
};

#[main]
pub fn main() {
    // Set up mod manager
    entity::add_component(
        package_manager::entity(),
        package_manager::components::mod_manager_for(),
        packages::this::entity(),
    );

    // Spawn entities
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 10.0)
        .with(color(), vec4(1.0, 0.0, 0.0, 1.0))
        .with(plane_collider(), ())
        .spawn();

    for _ in 0..30 {
        Entity::new()
            .with(cube(), ())
            .with(cube_collider(), Vec3::ONE)
            .with(translation(), (random::<Vec2>() * 20.0 - 10.0).extend(1.))
            .spawn();
    }

    Entity::new()
        .with_merge(Transformable {
            local_to_world: Default::default(),
            optional: TransformableOptional {
                scale: Some(Vec3::ONE * 0.3),
                ..Default::default()
            },
        })
        .with(
            model_from_url(),
            packages::this::assets::url("AntiqueCamera.glb"),
        )
        .spawn();

    // Queries
    spawn_query(is_player()).bind(move |players| {
        for (id, _) in players {
            entity::add_components(
                id,
                Entity::new()
                    .with(use_character_controller(), ())
                    .with(
                        model_from_url(),
                        packages::base_assets::assets::url("Y Bot.fbx"),
                    )
                    .with(basic_character_animations(), id),
            );
        }
    });

    query(bouncy_created()).each_frame(|entities| {
        for (id, created) in entities {
            if (game_time() - created).as_secs_f32() > 5.0 {
                entity::despawn(id);
            }
        }
    });

    // Messages
    fixed_rate_tick(Duration::from_secs_f32(0.5), |_| {
        Entity::new()
            .with_merge(Sphere::suggested())
            .with_merge(Transformable::suggested())
            .with(scale(), Vec3::ONE * 0.2)
            .with(
                translation(),
                Vec3::X * 10. + (random::<Vec2>() * 2.0 - 1.0).extend(10.),
            )
            .with(sphere_collider(), 0.5)
            .with(dynamic(), true)
            .with(bouncy_created(), game_time())
            .spawn();
    });

    Paint::subscribe(|ctx, msg| {
        if ctx.client_user_id().is_none() {
            return;
        }

        let Some(hit) = physics::raycast_first(msg.ray_origin, msg.ray_dir) else {
            return;
        };

        Entity::new()
            .with(cube(), ())
            .with(translation(), hit.position)
            .with(scale(), Vec3::ONE * 0.1)
            .with(color(), vec4(0., 1., 0., 1.))
            .spawn();
    });

    Teleport::subscribe(|ctx, _| {
        let Some(player_id) = ctx.client_entity_id() else {
            return;
        };

        entity::mutate_component(player_id, translation(), |t| {
            *t += 10. * Vec3::Z;
        });
    });
}
