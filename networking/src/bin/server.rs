use std::{time::SystemTime, net::{SocketAddr, UdpSocket, Ipv4Addr}};

use bevy::{prelude::*, log::LogPlugin};
use bevy_replicon::{
    prelude::*, 
    renet::{ServerEvent, ConnectionConfig, transport::{ServerConfig, ServerAuthentication, NetcodeServerTransport}},
};

use networking::*;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
			LogPlugin::default(),
            ReplicationPlugins.build()
                .disable::<ClientPlugin>()
                .set(ServerPlugin::new(TickPolicy::MaxTickRate(60))),
        ))
        .replicate::<PlayableCharacter>()
        .add_client_event::<MoveEvent>(SendPolicy::Ordered)
        .add_systems(Startup, build_server)
        .add_systems(Update, server_event)
        .add_systems(Update, handle_move_events)
        .run();
}

fn build_server(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
) {
	info!("Building server");
    let server = RenetServer::new(ConnectionConfig{
        server_channels_config: network_channels.server_channels(),
        client_channels_config: network_channels.client_channels(),
        ..default()
    });

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 9090);
    let socket = UdpSocket::bind(public_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 4,
        protocol_id: 0,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    commands.insert_resource(server);
    commands.insert_resource(transport);
	info!("Built server");
}

fn handle_move_events(
    mut move_events: EventReader<FromClient<MoveEvent>>,
    mut players: Query<(&PlayerID, &mut Transform, &mut PlayableCharacter)>,
    timer: Res<Time>,
) {
    for FromClient {client_id, event } in move_events.iter() {
        for (player_id, mut transform, mut pc) in players.iter_mut() {
            if player_id.0 == client_id.clone() {
                transform.translation += 25. * Vec3::new(event.right, 0., event.forward) * timer.delta_seconds();
                pc.position = transform.translation.into();
            }
        }
    }
}

fn server_event(
    mut commands: Commands,
    mut server_events: EventReader<ServerEvent>,
    players: Query<(Entity, &PlayableCharacter, &PlayerID)>,
) {
    for event in &mut server_events {
        match event {
            ServerEvent::ClientConnected { client_id } => {
				info!("CONNECTED! {:?}", client_id);
                let pc = PlayableCharacter::default();
                commands.spawn((
					PlayerID(client_id.clone()),
                    GlobalTransform::default(),
                    Transform::default(),
					pc.clone(),
					Replication,
                ));
            },
            ServerEvent::ClientDisconnected { client_id, .. } => {
				info!("disconnected! {:?}", client_id);
                for (entity, _, player_id) in players.iter() {
                    if player_id.0 == client_id.clone() {
                        commands.entity(entity).despawn();
                    }
                }
            },
        }
    }
}
