use macroquad::math::*;
use rapier3d::prelude::*;

pub struct PhysicsWorld {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
}

// This is a rudimentary physics set up taken straight from rapier3d docs. Maybe some things
// would need to be customized in real world, maybe not. Don't have time to dig deep into
// the docs and figure out which parameters would be ideal.
//
// Right now it features only two things: physics body creation and simulation step. Also some
// helper methods to help with 2D scene integration.
impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn step(&mut self, dt: f32) {
        let gravity = vector![0.0, 0.0, 0.0];
        let physics_hooks = ();
        let event_handler = ();

        self.integration_parameters.dt = dt;

        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }

    pub fn create_body_cuboid(&mut self, body_type: RigidBodyType, origin: Vec3, half_extent: Vec3) -> RigidBodyHandle {
        println!("adding cuboid collider: origin={origin:?} half_extent={half_extent:?}");

        let rigid_body = RigidBodyBuilder::new(body_type)
            .translation(vector![origin.x, origin.y, origin.z])
            .lock_rotations()
            .build();
        let body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(half_extent.x, half_extent.y, half_extent.z).build();

        self.collider_set
            .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);

        body_handle
    }

    // Some helper methods below for easier 2D scene integration.

    pub fn set_body_linear_velocity(&mut self, handle: RigidBodyHandle, vel: Vec3) {
        let body = &mut self.rigid_body_set[handle];
        body.set_linvel(vector![vel.x, vel.y, vel.z], true);
    }

    #[inline]
    pub fn set_body_linear_velocity_2d(&mut self, handle: RigidBodyHandle, vel: Vec2) {
        self.set_body_linear_velocity(handle, vec3(vel.x, vel.y, 0.0));
    }

    pub fn get_body_translation(&self, handle: RigidBodyHandle) -> Vec3 {
        let body = &self.rigid_body_set[handle];
        let translation = body.translation();
        vec3(translation.x, translation.y, translation.z)
    }

    #[inline]
    pub fn get_body_translation_2d(&self, handle: RigidBodyHandle) -> Vec2 {
        let translation = self.get_body_translation(handle);
        vec2(translation.x, translation.y)
    }
}
