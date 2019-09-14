use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy,
    network::simulation::{
        NetworkSimulationResource,
        udp::UdpNetworkBundle
    },
    Application,
    GameDataBuilder,
    GameData,
    SimpleState,
    StateData
};
use std::net::{UdpSocket, SocketAddr};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // Probably want to get this from a command line arg.
    let socket_addr: SocketAddr = "0.0.0.0:23440".parse()?;

    let game_data = GameDataBuilder::default()
        .with_bundle(UdpNetworkBundle::new(10240))?;

    let socket = UdpSocket::bind(socket_addr)?;

    let mut net = NetworkSimulationResource::new_server();
    net.set_socket(socket);

    let mut game = Application::build("./", ServerState)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        .with_resource(net)
        .build(game_data)?;

    game.run();
    Ok(())
}

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
