use avian2d::prelude::*;
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels. The
    // unit allows the engine to tune its parameters for the scale of the world, improving
    // stability.
    app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0));
}

// Example of a test for a plugin
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, plugin));
        app
    }

    #[test]
    fn app_plugin_registers_plugins() {
        let app = setup();

        // Check if the plugins are registered
        assert!(app.world().contains_resource::<PhysicsLengthUnit>());
    }
}
