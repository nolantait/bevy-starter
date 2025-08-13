#![allow(unused)]
use bevy::prelude::*;

#[derive(Resource)]
pub struct MousePosition(Vec2);

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(MousePosition(Vec2::default()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, plugin));
        app
    }

    #[test]
    fn plugin_registers_resources() {
        let mut app = setup();

        // Check if the plugins are registered
        assert!(app.world().contains_resource::<MousePosition>());
    }
}
