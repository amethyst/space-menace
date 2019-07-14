use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy,
    network::{
        NetworkSimulationTimeSystem,
        laminar::{LaminarNetworkBundle, LaminarNetworkResource}
    },
    Application,
    GameDataBuilder,
    GameData,
    SimpleState,
    StateData
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // Probably want to get this from a command line arg.
    let socket_addr = "0.0.0.0:23440".parse()?;

    let game_data = GameDataBuilder::default()
        .with_bundle(LaminarNetworkBundle)?
        .with(NetworkSimulationTimeSystem, "network_sim_time", &[]);

    let mut game = Application::build("./", ServerState)?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        .with_resource(LaminarNetworkResource::new(socket_addr))
        .build(game_data)?;

    game.run();
    Ok(())
}

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
