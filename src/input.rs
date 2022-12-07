use crate::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::input::mouse::MouseWheel;
use crate::camera::MainCamera;

#[derive(Resource)]
pub struct MousePosition(pub Vec2);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MousePosition(Vec2::default()))
            .add_system(cursor_system)
            .add_system(input_avoidance_system)
            .add_system(input_spawn_system)
            .add_system(input_shooting_system)
            .add_system(input_stance_system);
    }
}

fn cursor_system(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_position = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
        commands.insert_resource(MousePosition(world_position));
    }
}

fn input_avoidance_system(
    mut events: EventReader<MouseWheel>,
    mut avoidance_factor: ResMut<AvoidanceFactor>
) {
    for event in events.iter() {
        avoidance_factor.0 += event.y * 100.;
        avoidance_factor.0 = avoidance_factor.0.clamp(0., MAX_AVOIDANCE);
    }
}

fn input_spawn_system(
    keys: Res<Input<KeyCode>>,
    mouse_position: Res<MousePosition>,
    mut events: EventWriter<BoidSpawned>
) {
    if keys.just_pressed(KeyCode::Space) {
        let spawn_event = BoidSpawned(mouse_position.0);
        events.send(spawn_event);
    }
}

fn input_shooting_system(
    keys: Res<Input<MouseButton>>,
    mut events: EventWriter<Shoot>
) {
    if keys.just_pressed(MouseButton::Left) {
        let shoot_event = Shoot;
        events.send(shoot_event);
    }
}

fn input_stance_system(
    buttons: Res<Input<MouseButton>>,
    mut events: EventWriter<StanceChanged>,
    stance: Res<PlayerStance>
) {
    if buttons.just_pressed(MouseButton::Right) {
        match stance.0 {
            Stance::Follow => events.send(StanceChanged(Stance::Evade)),
            Stance::Evade => events.send(StanceChanged(Stance::Follow))
        }
    }
}

