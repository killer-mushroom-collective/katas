use std::{time::SystemTime, net::{SocketAddr, UdpSocket, Ipv4Addr}};

use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::{ConnectionConfig, transport::{ClientAuthentication, NetcodeClientTransport}}};

use networking::*;


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
        .add_systems(Update, send_move_event)
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
    mut commands: Commands
) {
}

fn update_pc(
    mut players: Query<(&mut Transform, &mut PlayableCharacter)>,
) {
/*
    let (mut transform, pc) = players.get_single_mut().unwrap();
    transform.translation = pc.position.into();
    */
}

fn send_move_event(
    mut move_event: EventWriter<MoveEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut event = MoveEvent::default();
    if keyboard_input.pressed(KeyCode::W) {
        event.forward = 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        event.forward = -1.;
    }
    if keyboard_input.pressed(KeyCode::A) {
        event.right = -1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        event.right = 1.;
    }
    move_event.send(event);
}
