use rapier2d::{prelude::*, na::Vector2};

pub struct KgPhysics {
  pub rigid_body_set: RigidBodySet,
  pub collider_set: ColliderSet,
  integration_parameters: IntegrationParameters,
  physics_pipeline: PhysicsPipeline,
  island_manager: IslandManager,
  broad_phase: BroadPhase,
  narrow_phase: NarrowPhase,
  impulse_joint_set: ImpulseJointSet,
  multibody_joint_set: MultibodyJointSet,
  ccd_solver: CCDSolver,
  gravity: Vector2<f32>,
}

impl KgPhysics {
  pub fn new() -> Self {
    Self {
      rigid_body_set: RigidBodySet::new(),
      collider_set: ColliderSet::new(),
      integration_parameters: IntegrationParameters::default(),
      physics_pipeline: PhysicsPipeline::new(),
      island_manager: IslandManager::new(),
      broad_phase: BroadPhase::new(),
      narrow_phase: NarrowPhase::new(),
      impulse_joint_set: ImpulseJointSet::new(),
      multibody_joint_set: MultibodyJointSet::new(),
      ccd_solver: CCDSolver::new(),
      gravity: vector![0.0, 0.0]
    }
  }

  pub fn update(&mut self) {
    let physics_hooks = ();
    let event_handler = ();

    /* Run the game loop, stepping the simulation once per frame. */
    for _ in 0..200 {
      self.physics_pipeline.step(
        &self.gravity,
        &self.integration_parameters,
        &mut self.island_manager,
        &mut self.broad_phase,
        &mut self.narrow_phase,
        &mut self.rigid_body_set,
        &mut self.collider_set,
        &mut self.impulse_joint_set,
        &mut self.multibody_joint_set,
        &mut self.ccd_solver,
        &physics_hooks,
        &event_handler,
      );
    }
  }
}
