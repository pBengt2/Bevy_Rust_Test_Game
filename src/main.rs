#[allow(non_snake_case)]

use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy::render::camera::{Viewport};

use bevy::math::bounding::BoundingSphere;
use bevy::math::bounding::IntersectsVolume;

// https://www.reddit.com/r/bevy/comments/198fu1z/getting_dynamic_linking_debugging_to_work_in/
// ^ instructions for debug to work correctly in RustRover.

// TODO: Various Collisions (sphere / box / etc)
// - In progress (testing sphere collisions)
// TODO: Interactions
// TODO: Import animation
// TODO: Shaders


const SCREEN_MULTIPLIER: f32 = 1.5;

const MOVEMENT_SPEED: f32 = 5.0;

#[derive(Component)]
struct PlayerID(i32);

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Radius(f32); // Used for SphereCollisionBundle
#[derive(Bundle)]
struct SphereCollisionBundle<M: Material> {
    sphere: Handle<Mesh>,
    material: Handle<M>,
    transform: Transform,
    global_transform: GlobalTransform,
    radius: Radius,
    collider: Collider,
}


fn sphere_collision(cur_sphere: BoundingSphere, other_sphere: BoundingSphere) -> bool {
    if !cur_sphere.intersects(&other_sphere) {
        return false;
    }
    true
}

fn check_for_collisions(
    mut players: Query<(&mut Transform, &PlayerID), (With<Player>, Without<Collider>)>,
    collider_query: Query<(&Radius, &Transform, &PlayerID), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    const SPHERE_RADIUS: f32 = 30.;

    // TODO: Index instead to prevent double + self checking
    for (r1, collider_transform_1, p1) in &collider_query {
        for (r2, collider_transform_2, p2) in &collider_query {
            if p1.0 != p2.0
            {
                let collision = sphere_collision(
                    BoundingSphere::new(collider_transform_1.translation, r1.0),
                    BoundingSphere::new(collider_transform_2.translation, r2.0)
                );


                if collision == true {
                    collision_events.send_default();
                    // TODO: Dummy test code. This should be handled by a collision event.
                    for (mut p_trans, p_id) in &mut players {
                        if p_id.0 == p1.0
                        {
                            let x1 = collider_transform_1.translation.x;
                            let x2 = collider_transform_2.translation.x;
                            if x1 > x2
                            {
                                p_trans.translation.x = x2 + r1.0*2.0;
                            }
                            else
                            {
                                p_trans.translation.x = x2 - r1.0*2.0;
                            }
                            return;
                        }
                    }
                    // TODO: Dummy test code.
                }
            }
        }
    }
}


fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<(&mut Camera, &PlayerID), With<PlayerCamera>>
) {
    let mut width = 0;
    let mut height = 0;
    for e in resize_reader.read() {
        width = (e.width * SCREEN_MULTIPLIER).round() as u32;
        height = (e.height * SCREEN_MULTIPLIER).round() as u32;
        println!("{}, {}", width, height);
    }
    if width != 0 {
        for (mut cam, player_id) in &mut query {
            if player_id.0 == 1 {
                cam.viewport = Some(Viewport { physical_position: Default::default(), physical_size: UVec2 { x: width / 2, y: height }, depth: 0.0..1.0 });
            } else {
                cam.viewport = Some(Viewport { physical_position: UVec2 { x: width / 2, y: 0 }, physical_size: UVec2 { x: width / 2, y: height }, depth: 0.0..1.0 });
            }
        }
    }
}

fn camera_movement(
    players: Query<(&Transform, &PlayerID), (With<Player>, Without<PlayerCamera>)>,
    mut query: Query<(&mut Camera, &mut Transform, &PlayerID), With<PlayerCamera>>
) {

    let up_vector = Vec3::new(0.0, 1.0, 0.0);
    for (player, p_player_id) in &players {
        let player_trans = player.translation;
        for (_cam, mut trans, player_id) in &mut query {
            let delta = player_trans - Vec3::new(0.0, -10.0, -10.0);
            if player_id.0 == p_player_id.0 {
                trans.translation = delta;
                trans.look_at(player_trans, up_vector);
                //trans = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
            }
        }
    }
}

fn sphere_collider_movement(
    players: Query<(&Transform, &PlayerID), (With<Player>, Without<PlayerCamera>, Without<Collider>)>,
    mut query: Query<(&mut Transform, &PlayerID), With<Collider>>
) {
    for (player, p_player_id) in &players {
        let player_trans = player.translation;
        for (mut trans, player_id) in &mut query {
            let delta = player_trans - Vec3::new(0.0, 0.0, 0.0);
            if player_id.0 == p_player_id.0 {
                trans.translation = delta;
            }
        }
    }
}

fn spawn_object<T: Bundle>(mut commands: Commands, obj_to_spawn: T)
{
    commands.spawn(obj_to_spawn);
}

fn player_movement(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &PlayerID), With<Player>>,
    time: Res<Time>,
)
{
    for (mut trans, player_id) in &mut query {
        let mut direction = 0.0;
        if (player_id.0 == 1 && keyboard_input.pressed(KeyCode::KeyA)) ||
            (player_id.0 == 2 && keyboard_input.pressed(KeyCode::ArrowLeft)) {
            direction -= 1.0;
        }
        if (player_id.0 == 1 && keyboard_input.pressed(KeyCode::KeyD)) ||
            (player_id.0 == 2 && keyboard_input.pressed(KeyCode::ArrowRight)) {
            direction += 1.0;
        }
        if direction != 0.0 {
            trans.translation.x = trans.translation.x + direction * MOVEMENT_SPEED * time.delta_seconds();
        }

        direction = 0.0;
        if (player_id.0 == 1 && keyboard_input.pressed(KeyCode::KeyW)) ||
            (player_id.0 == 2 && keyboard_input.pressed(KeyCode::ArrowUp)) {
            direction -= 1.0;
        }
        if (player_id.0 == 1 && keyboard_input.pressed(KeyCode::KeyS)) ||
            (player_id.0 == 2 && keyboard_input.pressed(KeyCode::ArrowDown)) {
            direction += 1.0;
        }
        if direction != 0.0 {
            trans.translation.z = trans.translation.z + direction * MOVEMENT_SPEED * time.delta_seconds();
        }
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut Transform, &PlayerID), With<Player>>,
    commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    player_movement(&keyboard_input, query, time);

    if keyboard_input.pressed(KeyCode::Space) {
        spawn_object(commands, PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut resize_reader: EventReader<WindowResized>,
) {

    // Plane
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            transform: Transform::from_matrix(Mat4::IDENTITY),
            ..default()
        }
    );

    meshes.add(Cuboid::default());
    materials.add(Color::srgb(0.5, 0.5, 0.0));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.0, 0.9, 0.0)),
            transform: Transform::from_xyz(0.6, 0.5, 0.0),
            ..default()
            },
        Player,
        PlayerID(1),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));

    // Sphere collision test
    let scb = SphereCollisionBundle {
        sphere: meshes.add(Sphere::default()),
        material: materials.add(Color::srgb(0.0, 0.9, 0.0)),
        transform: Transform::from_xyz(-0.6, 0.5, 0.0),
        global_transform: GlobalTransform::from_xyz(0.0, 0.5, 0.0),
        collider: Collider,
        radius: Radius(0.5),
    };
    commands.spawn((
        scb,
        PlayerID(1),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));
    // Sphere collision test

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.0, 0.0, 0.9)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        PlayerID(2),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));

    // Sphere collision test
    let scb_2 = SphereCollisionBundle {
        sphere: meshes.add(Sphere::default()),
        material: materials.add(Color::srgb(0.0, 0.0, 0.9)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        global_transform: GlobalTransform::from_xyz(0.0, 0.5, 0.0),
        collider: Collider,
        radius: Radius(0.5),
    };
    commands.spawn((
        scb_2,
        PlayerID(2),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));
    // Sphere collision test

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


    let mut height = 100;
    let mut width = 100;
    for e in resize_reader.read() {
        width = (e.width * SCREEN_MULTIPLIER).round() as u32;
        height = (e.height * SCREEN_MULTIPLIER).round() as u32;
    }

    // Camera
    commands.spawn((
        Camera3dBundle {
           transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
           camera: Camera {
           viewport: Some(Viewport { physical_position: Default::default(), physical_size: UVec2{ x:width/2, y:height }, depth: 0.0..1.0 }),
           order: 0,
           clear_color: ClearColorConfig::Default,
           ..default()
           },
           ..default()
        },
        PlayerCamera,
        PlayerID(1)
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                viewport: Some(Viewport { physical_position: UVec2{ x:width/2, y:0 }, physical_size: UVec2{ x:width/2, y:height }, depth: 0.0..1.0 }),
                order: 1,
                clear_color: ClearColorConfig::Default,
                ..default()
            },
            ..default()
        },
        PlayerCamera,
        PlayerID(2)
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, camera_movement)
        .add_systems(Update, sphere_collider_movement)
        .add_systems(
            FixedUpdate,
            (
                handle_input,
                check_for_collisions,
            ).chain(),
        )
        .run();
}