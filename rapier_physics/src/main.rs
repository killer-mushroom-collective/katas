use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::build()
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_materials.system())
        .add_startup_stage("physics_setup", SystemStage::single(setup_physics.system()))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(print_ball_altitude.system())
        .run();
}

struct Ground;
struct Ball;

struct Materials {
    ground_material: Handle<ColorMaterial>,
    ball_material: Handle<ColorMaterial>
}

fn setup_camera(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);
}

fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let materials = Materials {
        ground_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        ball_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    };
    commands.insert_resource(materials)
}

// Mostly taken from https://rapier.rs/docs/user_guides/rust_bevy_plugin/getting_started_bevy
fn setup_physics(mut commands: Commands, materials: Res<Materials>) {
    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 10.0),
        ..Default::default()
    };
    let ground_sprite = SpriteBundle {
        sprite: Sprite::new(Vec2::new(100.0, 10.0)),
        material: materials.ground_material.clone(),
        ..Default::default()
    };
    commands.spawn_bundle(collider).insert_bundle(ground_sprite).insert(Ground);

    /* Create the bouncing ball. */
    let ball_sprite = SpriteBundle {
        sprite: Sprite::new(Vec2::new(5.0, 5.0)),
        transform: Transform::from_xyz(0.0, 100.0, 1.0),
        material: materials.ball_material.clone(),
        ..Default::default()
    };
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0.0, 100.0).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(ball_sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(Ball);
}

fn print_ball_altitude(positions: Query<&RigidBodyPosition>, mut balls: Query<&mut Transform, With<Ball>>) {
    for rb_pos in positions.iter() {
        println!("Ball altitude: {}", rb_pos.position.translation.vector.y);
        let mut ball_pos = balls.single_mut().unwrap();
        ball_pos.translation = Transform::from_xyz(
            rb_pos.position.translation.vector.x,
            rb_pos.position.translation.vector.y,
            1.0
        ).translation
    }
}