use std::{time::SystemTime, net::{SocketAddr, UdpSocket, Ipv4Addr}};

use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::{ConnectionConfig, transport::{ClientAuthentication, NetcodeClientTransport}}};

use networking::*;

pub struct MySelf(u64);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ReplicationPlugins.build()
                .disable::<ServerPlugin>(),
        ))
        .replicate::<PlayableCharacter>()
        .add_client_event::<MoveEvent>(SendPolicy::Ordered)
        .add_systems(Startup, build_client)
        .add_systems(Startup, build_world)
        .add_systems(Update, send_move_event)
        .add_systems(Update, handle_create_event)
        .add_systems(Update, update_pc)
        .run();
}

fn build_client(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
) {
    let client = RenetClient::new(ConnectionConfig{
        server_channels_config: network_channels.server_channels(),
        client_channels_config: network_channels.client_channels(),
        ..default()
    });


    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 9090);
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 0,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    commands.insert_resource(client);
    commands.insert_resource(transport);
}

fn build_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Plane::from_size(50.0).into());
    let material = materials.add(Color::SILVER.into());

    commands.spawn(PbrBundle {
        mesh,
        material,
        ..default()
    });

    commands.spawn(PointLightBundle{
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn handle_create_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_player: Query<(Entity, &PlayableCharacter), Added<PlayableCharacter>>,
) {
    for (entity, _) in new_player.iter() {
        let mesh = meshes.add(shape::Cube::default().into());
        let material = materials.add(Color::GREEN.into());
        commands
            .entity(entity)
            .insert(PbrBundle {
                mesh,
                material,
                ..default()
            });
    }
}

fn update_pc(
    mut players: Query<(&mut Transform, &mut PlayableCharacter)>,
) {
    for (mut transform, player) in players.iter_mut() {
        transform.translation = player.position.into();
    }
}

fn send_move_event(
    mut move_event: EventWriter<MoveEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut event = MoveEvent::default();
    let mut send_something = false;
    if keyboard_input.pressed(KeyCode::W) {
        event.forward = 1.;
        send_something = true;
    }
    if keyboard_input.pressed(KeyCode::S) {
        event.forward = -1.;
        send_something = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        event.right = -1.;
        send_something = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        event.right = 1.;
        send_something = true;
    }
    if send_something {
        move_event.send(event);
    }
}
