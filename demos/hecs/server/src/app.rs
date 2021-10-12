use std::collections::HashSet;

use hecs::World;

use naia_hecs_server::{Entity, RoomKey, Server as NaiaServer, ServerAddrs, ServerConfig};

use naia_hecs_demo_shared::{get_server_address, get_shared_config, protocol::Protocol};

use super::systems::{
    events::process_events,
    scopes::update_scopes,
    startup::app_init,
    tick::{march_and_mark, send_messages, send_updates},
};

pub type Server = NaiaServer<Protocol, Entity>;

pub struct App {
    pub server: Server,
    pub world: World,
    pub main_room_key: RoomKey,
    pub tick_count: u32,
    pub has_marker: HashSet<Entity>,
}

impl App {
    pub fn new() -> Self {
        info!("Naia Hecs Server Demo started");

        let server_addresses = ServerAddrs::new(
            get_server_address(),
            // IP Address to listen on for UDP WebRTC data channels
            "127.0.0.1:14192"
                .parse()
                .expect("could not parse WebRTC data address/port"),
            // The public WebRTC IP address to advertise
            "127.0.0.1:14192"
                .parse()
                .expect("could not parse advertised public WebRTC data address/port"),
        );

        app_init(
            ServerConfig::default(),
            get_shared_config(),
            server_addresses,
        )
    }

    pub fn update(&mut self) {
        process_events(self);
    }

    pub fn tick(&mut self) {
        march_and_mark(self);
        send_messages(self);
        update_scopes(self);
        send_updates(self);
    }
}
