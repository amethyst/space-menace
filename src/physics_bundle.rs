use amethyst::{
    core::{Float, SystemBundle, Transform},
    ecs::DispatcherBuilder,
    error::Error,
};
use specs_physics::{
    systems::{
        PhysicsStepperSystem,
        SyncBodiesFromPhysicsSystem,
        SyncBodiesToPhysicsSystem,
        SyncCollidersToPhysicsSystem,
        SyncParametersToPhysicsSystem,
    },
    SimplePosition,
};

/// Bundle containing all `System`s relevant to the game physics.
#[derive(Default)]
pub struct PhysicsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle {
    fn build(self, builder: &mut DispatcherBuilder) -> Result<(), Error> {
        // builder.add(
        //     SyncBodiesToPhysicsSystem::<Float, Transform>::default(),
        //     "sync_bodies_to_physics_system",
        //     &[],
        // );
        // builder.add(
        //     SyncCollidersToPhysicsSystem::<Float, Transform>::default(),
        //     "sync_colliders_to_physics_system",
        //     &["sync_bodies_to_physics_system"],
        // );
        // builder.add(
        //     SyncParametersToPhysicsSystem::<Float>::default(),
        //     "sync_parameters_to_physics_system",
        //     &[],
        // );
        // builder.add(
        //     PhysicsStepperSystem::<Float>::default(),
        //     "physics_stepper_system",
        //     &[
        //         "sync_bodies_to_physics_system",
        //         "sync_colliders_to_physics_system",
        //         "sync_parameters_to_physics_system",
        //     ],
        // );
        // builder.add(
        //     SyncBodiesFromPhysicsSystem::<Float, Transform>::default(),
        //     "sync_bodies_from_physics_system",
        //     &["physics_stepper_system"],
        // );
        specs_physics::register_physics_systems::<Float, Transform>(builder);

        Ok(())
    }
}