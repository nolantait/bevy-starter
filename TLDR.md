# Bevy TLDR

Bevy is an archetype Entity-Component-System (ECS) game engine built in Rust. It
emphasizes modularity, performance, and ease of use.

## Entity and Components

An `Entity` on its own holds no data or behavior. The actual `Entity` is just an
identifier to find associated components where the real data is stored.

Each `Entity` can only have a single `Component` of each type. These components can
be added and removed dynamically over the course of the entity's lifetime.
Everything is stored inside a `World` and everything is managed by the `App`.

A good mental model to use is that entities represent a row in an in-memory
database, while components are our columns.

- **Entities** An identifier for a row
- **Components** A column in a row
- **Systems** All the behavior

We define components by deriving the `Component` trait:

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
enum Ship {
  Destroyer,
  Cruiser,
  Battleship,
}

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
#[component(on_add = on_position_added)]
struct Position {
  x: i32,
  y: i32,
}
```

Components have 5 different life-cycle hooks we can use to handle side effects
that __need__ to happen:

1. `#[component(on_add = on_add_function)]`
2. `#[component(on_insert = on_insert_function)]`
3. `#[component(on_replace = on_replace_function)]`
4. `#[component(on_remove = on_remove_function)]`
5. `#[component(on_despawn = on_despawn_function)]`

Components can also be required by other components

```rust
#[derive(Component)]
#[require(Position, Ship)]
struct Player;

fn spawn_player_with_required_components(
  mut commands: Commands
) {
  commands.spawn(Player);
}
```

When a component is spawned, if it has any required components, it will
automatically add them __unless__ we override them. The only requirement is that
each required component implements the `Default` trait.

All these required calls are __recursive__. If a component you require has
required components, they will also be added.

A `Resource` is a special kind of component that has no `Entity`. They have more
convenient accessors for systems since there is only ever one of them.

```rust
#[derive(Resource)]
struct Score(usize);

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .run();
}
```

## Systems

Systems are where we trigger side effects that change our game's state.

In Bevy, systems are simple rust functions with one rule: They can only have
parameters that implement `SystemParam`.

```rust
fn spawn_player(mut commands: Commands) {
  // Spawns a single entity with multiple components
  commands.spawn((
      Player,
      Ship::Destroyer,
      Health(100.0),
      Position { x: 1, y: 2 }
  ));
}
```

`Commands` are what we use to change the state of our `World` in a way that is
more performant than letting each system mutate the world directly.

When you use the system parameter `Commands` you are enqueuing your commands to
the `CommandQueue` which runs when we transition to the next `Schedule`.

## Apps

Everything is coordinated through an `App` which schedules our systems to run at
certain points in the game's loop:

```rust
use bevy::prelude::*;

fn main() {
   App::new()
     .add_systems(Startup, setup_everything)
     .add_systems(Update, process_input)
     .add_systems(FixedUpdate, move_player)
     .run();
}
```

You will mostly be adding your logic to the three main schedule labels:

1. `Update` runs once every loop
2. `FixedUpdate` runs once every fixed amount of time
3. `Startup` runs once at startup

Additionally there are other built-in schedule labels for more specific use:

1. `PreStartup`
2. `Startup`
3. `PostStartup`
4. `First`
5. `PreUpdate`
6. `StateTransition`
7. `RunFixedUpdateLoop` which runs `FixedUpdate` conditionally
8. `Update`
9. `PostUpdate`
10. `Last`

These types are each a `ScheduleLabel`. Labels are used to identify a `Schedule`
which contains the metadata and executor needed to run them under certain
conditions.

Bevy will try and run all systems in parallel as long as there are no mutable
data access conflicts. Archetypes are used as a performance optimization for
this process.

An `App` can be given a state enum to manage different modes of operation:

```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
  #[default]
  MainMenu,
  InGame,
  Paused,
}

fn main() {
  App::new()
    // Add our state to our app definition
    .init_state::<AppState>()
    // We can add systems to trigger during transitions
    .add_systems(OnEnter(AppState::MainMenu), spawn_menu)
    // Or we can use run conditions
    .add_systems(Update, play_game.run_if(in_state(AppState::InGame)))
    .run();
}
```

If we wanted to create explicit transitions we could implement the logic on our
state:

```rust
impl AppState {
  fn next(&self) -> Self {
    match *self {
      AppState::MainMenu => AppState::InGame,
      AppState::InGame => AppState::Paused,
      AppState::Paused => AppState::InGame,
    }
  }
}
```

## Plugins

Almost every app will include the `DefaultPlugins` plugin which groups together
all the default functionality needed for a game.

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .run();
}
```

`DefaultPlugins` includes the following


|Plugin|Description|
|------|-----------|
|__DiagnosticsPlugin__|Adds core diagnostics|
|__DlssInitPlugin__|Initializes DLSS support if available|
|__FrameCountPlugin__|Adds frame counting functionality|
|__HierarchyPlugin__|Handles `Parent` and `Children` components|
|__InputPlugin__|Adds keyboard and mouse input|
|__PanicHandlerPlugin__|Adds sensible panic handling|
|__ScheduleRunnerPlugin__|Configures an `App` to run its `Schedule` according to a given `RunMode`|
|__TaskPoolPlugin__|Setup of default task pools for multithreading|
|__TimePlugin__|Adds time functionality|
|__TransformPlugin__|Handles `Transform` components|

Then additionally, depending on the features you enable, it will include:

|Plugin|Feature|Description|
|------|-------|-----------|
|__AccessibilityPlugin__|`bevy_window`|Adds non-GUI accessibility functionality|
|__AnimationPlugin__|`bevy_animation`|Adds animation support|
|__AntiAliasPlugin__|`bevy_anti_alias`|Adds multi-sample anti-aliasing (MSAA)|
|__AssetPlugin__|`bevy_asset`|Adds asset server and resources to load assets|
|__AudioPlugin__|`bevy_audio`|Adds support for using sound assets|
|__DefaultPickingPlugins__|`bevy_picking`|Adds picking functionality|
|__DevToolsPlugin__|`bevy_dev_tools`|Enables developer tools in an `App`|
|__CameraPlugin__|`bevy_camera`|Adds 2D and 3D camera components and systems|
|__CiTestingPlugin__|`bevy_ci_testing`|Helps instrument continuous integration|
|__CorePipelinePlugin__|`bevy_core_pipeline`|The core rendering pipeline|
|__GltfPlugin__|`bevy_gltf`|Adds support for loading gltf models|
|__GilrsPlugin__|`bevy_gilrs`|Adds support for gamepad inputs|
|__GizmoPlugin__|`bevy_gizmos`|Provides an immediate mode drawing api for visual debugging|
|__HotPatchPlugin__|`hotpatching`|Enables hot-patching of assets|
|__ImagePlugin__|`bevy_render`|Adds the `Image` asset and prepares them to render on your GPU|
|__LightPlugin__|`bevy_light`|Adds light components and systems|
|__LogPlugin__|`bevy_log`|Adds logging to apps|
|__MeshPlugin__|`bevy_mesh`|Adds the `Mesh` asset and prepares them to render on the GPU|
|__PbrPlugin__|`bevy_pbr`|Adds physical based rendering with `StandardMaterial` etc|
|__PostProcessingPlugin__|`bevy_post_process`|Adds post processing effects|
|__PipelinedRenderingPlugin__|`bevy_render`|Adds pipelined rendering|
|__RenderPlugin__|`bevy_render`|Sets up rendering backend powered by `wgpu` crate|
|__ScenePlugin__|`bevy_scene`|Loading and saving collections of entities and components to files|
|__SpritePlugin__|`bevy_sprite`|Handling of sprites (images on our entities)|
|__StatesPlugin__|`bevy_state`|Adds state management for Apps|
|__TerminalCtrlCHandlerPlugin__|`std`|Handles Ctrl-C signals in terminal applications|
|__TextPlugin__|`bevy_text`|Supports loading fonts and rendering text|
|__UiPlugin__|`bevy_ui`|Adds support for UI layouts (flex, grid, etc)|
|__UiRenderPlugin__|`bevy_ui_render`|Adds support for sending UI nodes to renderer|
|__WindowPlugin__|`bevy_window`|Provides an interface to create and manage `Window` components|
|__WinitPlugin__|`bevy_winit`|Interface to create operating system windows (to actually display our game)|


Plugins are a way to group related functionality together. They receive a
mutable reference to the `App` and can add systems, resources, and other
plugins. Plugins are run in the order they are added to the `App`.

```rust
fn plugin(app: &mut App) {
  app.add_system(some_plugin_system);
}

fn main() {
  App::new().add_plugins(plugin);
}
```

If we need to manage the life-cycle of a plugin we can implement the `Plugin`
trait and hook into it.

```rust
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn cleanup(&self, _app: &App) -> bool {
    info!("Time to clean up")
    true
  }

  fn build(&self, app: &mut App) {
    app.add_systems(Startup, initialize_camera);
  }
}

fn initialize_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}
```

## Querying

To access the components of an entity inside our systems we can use the
`Query<D, F>` system parameter:

```rust
fn fetch_players(query: Query<&Player>) {
  for player in &query {
    info!("Player: {:?}", player);
  }
}
```

The `Query` system parameter lets us specify the data we want from each entity
using the two generic parameters:

```rust
//     ------- the `QueryData`
//    |  ---- the `QueryFilter`
//    v  v
Query<D, F>

//      --------- Give us read-only access to all the `Transform` components
//     |     ---- Which have a `Player` component on the same entity
//     v     v
Query<&Ball, With<Player>>

//                     --- NOTE: Each parameter can be a tuple as well
//                    |
//                    v
Query<&mut Transform, (With<Player>, With<Living>)>
```

When one of the generic parameters is a tuple then ___all___ the types in that
tuple must be satisfied by that query.

There are convenient types that make expressing more complicated queries easier:

|parameter|description|
|---------|-----------|
|`Option<T>`|a component but only if it exists, otherwise `None`|
|`AnyOf<T>`|fetches entities with any of the components in type T|
|`Ref<T>`|shared borrow of an entity's component `T` with access to change detection|
|`Entity`|returns the entity|

In addition to the `Query` system parameter there are other sibling system
parameters that also perform queries:

|System parameter|Description|
|----------------|-----------|
|`Single<D, F>`|Matches exactly one query item. Skips the system if more or none.|
|`Option<Single<D, F>>`|Matches zero or one query item. Skips the system if more.|
|`Populated<D, F>`|matches at least one or more. Skips the system if none.|

`Single` can be useful to reduce boilerplate when you know there is only ever a
single entity with a particular component:

```rust
fn move_the_only_player(mut transform: Single<&mut Transform, With<Player>>) {
  transform.translation.x += 1.
}
```

The second argument in your `Query<D, F>` is the `QueryFilter`. These filters
are wrapped by a condition type:


|method|description|
|------|-----------|
|`With<T>`|only items with a `T` component|
|`Without<T>`|only items without a `T` component|
|`Or<F>`|checks if all filters in the tuple `F` apply|
|`Changed<T>`|only components of type `T` that were changed this tick|
|`Added<T>`|only components of type `T` that were added this tick|

To retrieve components from our ECS storage our `Query` system parameter
provides several methods:

|method|description|
|------|-----------|
|`iter`|returns an iterator over all items|
|`for_each`|runs the given function in parallel for each item|
|`iter_many`|runs a given function for each item matching a list of entities|
|`iter_combinations`|returns an iterator over all combinations of a specified number of items|
|`par_iter`|returns a parallel iterator|
|`get`|returns a query item for a given entity|
|`get_component<T>`|returns the component for a given entity|
|`many`|returns a query item for a given list of entities|
|`get_single`|the safe version of `single` which returns a `Result<T>`|
|`single`|returns the query item while panicking if there are others|
|`is_empty`|returns true if the query is empty|
|`contains`|returns true if query contains a given entity|

Each method also has a corresponding `*_mut` variant which will return the
components with mutable ownership. This lets us change their data, instead of
just reading it.

In situations where we have a particular `Entity` (which is basically an ID),
we can use `get` or `get_mut`.

```rust
#[derive(Resource)]
struct PlayerRef(Entity);

fn move_player_by_component(
  mut query: Query<&mut Transform>,
  player: Res<PlayerRef>
) {
  if let Ok(mut transform) = query.get_mut(player.0) {
    transform.translation.x += 1.;
  }
}
```

In cases where we have a list of `Entity` and we want to iterate over only those
entity components we can use `iter_many`.

```rust
#[derive(Component)]
struct Health(pub f32);

#[derive(Resource)]
struct Selection {
  enemies: Vec<Entity>
}

const ATTACK_DAMAGE: f32 = 10.;

fn attack_selected_enemies(
  mut query: Query<&mut Health>,
  selected: Res<Selection>
) {
  let mut iter = query.iter_many_mut(&selected.enemies);
  while let Some(mut health) = iter.fetch_next() {
    health.0 -= ATTACK_DAMAGE;
  }
}
```

## Assets

To load assets we use the `AssetServer` which manages asynchronous loading
assets from a particular `AssetSource`, usually the filesystem.

All assets follow the same general process:

1. We register a new `Asset<T>` type if its custom
2. We Register an `AssetLoader` for that asset if its custom
3. We add the asset to our `assets` folder
4. Then we call `AssetServer::load` to get a `Handle<T>` to the asset

```rust
fn load_images(asset_server: Res<AssetServer>, mut commands: Commands) {
  // This will not block, the asset will be loaded in the background
  let image_handle: Handle<Image> = asset_server.load("images/bevy.png");

  commands.spawn(Sprite {
    image: image_handle,
    ..default()
  });
}
```

By default it will expect our assets to be inside the `assets` folder inside
the root directory of our application controlled by the `BEVY_ASSET_ROOT`
environment variable.

Assets can be tracked one of two ways:

1. Through events like `AssetEvent::LoadedWithDependencies`
2. Or by querying the asset server with `AssetServer::get_load_state`.

## Messages and events

There are two kind of events in Bevy:

1. `Message` for communication between systems
2. `Event` and `EntityEvent` for __observers__ that trigger immediate behavior

`Messages<T>` is a collection that acts as a double buffered queue.
This is done to ensure each system has an opportunity to see each
message. It is helping systems not have to care about the exact ordering within
a frame.

Messages are defined by deriving the `Message` trait:

```rust
// With a marker message
#[derive(Message)]
struct PlayerKilled;

// With a unit type
#[derive(Message)]
struct PlayerDetected(Entity);

// With fields
#[derive(Message)]
struct PlayerDamaged {
  entity: Entity,
  damage: f32,
}

fn main() {
  App::new()
    .add_message::<PlayerKilled>();
    .add_message::<PlayerDetected>();
    .add_message::<PlayerDamaged>();
}
```

If your messages are not consumed by 2 frames from now then
they will be cleaned up and dropped silently.

To write messages to a stream we use a `MessageWriter<T>`:

```rust
fn detect_player(
  mut messages: MessageWriter<PlayerDetected>,
  players: Query<(Entity, &Transform), With<Player>>,
) {
  for (entity, transform) in players {
    messages.write(PlayerDetected(entity));
  }
}
```

We can read messages from our systems with an
`MessageReader<T>` that consumes messages from our buffers:

```rust
fn react_to_detection(mut messages: MessageReader<PlayerDetected>) {
  for message in messages.read() {
    // Do something with each event here
  }
}
```

Events are the immediate version of messages. They come in two types:

1. `Event` for global events defined with a `GlobalTrigger`
2. `EntityEvent` for entity specific events defined with an `EntityTrigger`

These events are consumed by an `Observer` which is a callback system that takes
an `On` system parameter:

```rust
fn on_respawn(
  event: On<Add, Enemy>,
  query: Query<(&Enemy, &Position)>,
) {
  let (enemy, position) = query.get(event.entity).unwrap();
  println!("Enemy was respawned at {:?}", position);
}
```

Observers can be global by adding them to the `App` definition:

```rust
fn main() {
  App::new().add_plugins(DefaultPlugins).add_observer(on_respawn);
}
```

Or they can be local and only triggered for particular entities:

```rust
fn spawn_boss(mut commands: Commands) {
  let entity = commands.spawn((Enemy, Boss)).observe(on_boss_spawned).id();

  // Later, or potentially in another system
  commands.trigger(BossSpawned { entity });
}
```

These entity events will bubble up a hierarchy of `ChildOf` attached components.

This table summarizes the differences between events and messages:

| |Events|Messages|
|-|---------|------|
|__Optimal event frequency__|Infrequent|Frequent|
|__Handler__|Only handles a single event|Can handle many messages together|
|__Latency__|Immediate|Up to 1 frame|
|__Event propagation__|Bubbling|None|
|__Scope__|World ___or___ Entity|World|
|__Ordering__|No explicit order|Ordered|
|__Coupling__|High|Low|

## Relationships

Bevy has a built-in relationship it provides for parent/child relationships that
is made up of two components:

1. `ChildOf`: The `Relationship` we attach to other entities
2. `Children`: The `RelationshipTarget` that is kept in sync

These will propagate `Transform` and `GlobalTransform` of the parent to its
children to keep them in sync.

When you despawn the parent (the entity holding the `Children`) then all the
`ChildOf` components are removed automatically.

```rust
fn spawn_ship(mut commands: Commands) {
  let fleet = commands.spawn(Fleet).id();

  commands.spawn(Ship, ChildOf(fleet));
}
```

We can spawn children from a parent with the `with_children` method:

```rust
fn spawn_fleet(mut commands: Commands) {
  commands
    .spawn(Fleet)
    .with_children(|parent| {
      parent.spawn((Ship, Name::new("Ship 1")));
      parent.spawn((Ship, Name::new("Ship 2")));
    });
}
```

Instead of the closure we can pass a bundle of children to the `children!` macro.

```rust
fn spawn_fleet_with_sugar(mut commands: Commands) {
  commands.spawn((
    Fleet,
    children![
      (Ship, Name::new("Ship 3")),
      (Ship, Name::new("Ship 4")),
    ]
  ));
}
```

The source of truth is the `Relationship` component. This is the component we
will be adding to __other entities__ to specify the relationship. It must contain
a reference to the entity we will be attaching ourselves to.

```rust
#[derive(Component)]
#[relationship(relationship_target = ShipAttachments)]
struct AttachedToShip(pub Entity);
```

The `RelationshipTarget` is the component that will automatically be kept in
sync with all our `AttachedToShip` components. It must contain a list of
entities to store them.

```rust
#[derive(Component)]
#[relationship_target(relationship = AttachedToShip, linked_spawn)]
struct ShipAttachments(Vec<Entity>);
```

The `linked_spawn` will allow us to remove the `ShipAttachments` and Bevy will
automatically despawn any `AttachedToShip` components on our other entities.

To create the relationship we can then spawn this `Relationship` on other
entities.

```rust
fn spawn_ship(mut commands: Commands) {
  // Spawn the parent Ship
  let ship = commands.spawn((Ship, Name::new("Ship"))).id();

  // Spawn a GunTurret and attach it to the Ship using the new Relationship
  // component
  commands.spawn((GunTurret, AttachedToShip(ship), Name::new("GunTurret 1")));
  commands.spawn((GunTurret, AttachedToShip(ship), Name::new("GunTurret 2")));
}
```

This can be shortened by using the `related!` macro to specify the relationships
from the parent entity instead:

```rust
fn build_ship(mut commands: Commands) {
  // Spawn a Ship entity
  commands.spawn((
    Ship,
    Name::new("Ship A"),
    related!(ShipAttachments[
      // Attach GunTurrets to the Ship using the relationship
      (GunTurret, Name::new("GunTurret 1")),
      (GunTurret, Name::new("GunTurret 2")),
    ]),
  ));
}
```

Relationships are stored as components so we can query them:

```rust
fn log_ship_report(
  ships: Query<(&Name, &ShipAttachments), With<Ship>>,
  turrents: Query<&Name, With<GunTurret>>,
) {
  for (ship_name, attachments) in &ships {
    info!("{} has the following attachments:", ship_name.as_str());

    for &attachment in &attachments.0 {
      if let Ok(child_name) = turrents.get(attachment) {
        info!(" - {}", child_name.as_str());
      }
    }
  }
}
```

You can iterate the association from either side of the relationship:

```rust
fn iterate_from_turrets_to_ships(
  ships: Query<Entity, With<Ship>>,
  turrets: Query<Entity, With<AttachedToShip>>,
  attachments: Query<&AttachedToShip>,
) {
  for turret in &turrets {
    for attached in attachments.iter_ancestors(turret) {
      let ship = ships.get(attached);

      info!("Turret {:?} is attached to Ship {:?}", turret, ship);
    }
  }
}

fn iterate_from_ships_to_turrets(
  ships: Query<Entity, With<Ship>>,
  turrets: Query<Entity, With<GunTurret>>,
  ship_attachments: Query<&ShipAttachments>,
) {
  for ship in &ships {
    for attachment in ship_attachments.iter_descendants(ship) {
      let turret_entity = turrets.get(attachment);
      info!("Ship {:?} has Turret {:?}", ship, turret_entity);
    }
  }
}
```

You must be careful not to use this if your relationships contain loops as this
will run infinitely.

Bevy does not currently have a native way of representing many-to-many
relationships. `ChildOf` can only point to a single entity.

## Input

There are two ways to handle input in Bevy:

1. Reacting to the events emitted automatically by Bevy's input systems
2. Querying a resource like `ButtonInput`, `Axis`, `Touches` or `Gamepads`

Bevy has a different resource for each type of input:

|Resource|Description|
|--------|-----------|
|`Axis`|stores the position data from certain input devices|
|`ButtonInput`|a "press-able" input|
|`GamepadAxis`|An axis of a gamepad|
|`GamepadButton`|represents a single button of a gamepad just like a keyboard|
|`Gamepads`|represents a collection of connected game controllers|
|`TouchInput`|represents touch based input events|
|`Touches`|a collection of `Touch`es that have happened|


For example, to handle keyboard input we use the `ButtonInput<T>` resource which
has a set of convenient methods we can use to trigger behavior:


|Method|Description|
|------|-----------|
|`pressed`|will return `true` between a press and release event|
|`just_pressed`|will return `true` for one frame after a press event|
|`just_released`|will return `true` for one frame after a release event|


We can read these events in general by listening to `KeyboardInput` events:

```rust
/// Track keyboard inputs — useful for debugging or keybinding tools
fn log_keyboard_input(mut keyboard_events: EventReader<KeyboardInput>) {
    for event in keyboard_events.read() {
        println!(
            "Key pressed: {:?}, logical key: {:?}",
            event.key_code, event.logical_key
        );
    }
}
```

Or we can use the resource to check for a more specific state:

```rust
/// Handle player jump
fn jump_input_system(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        info!("Jump!");
    }
}

fn combo_key_system(input: Res<ButtonInput<KeyCode>>) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if ctrl && shift && input.just_pressed(KeyCode::KeyA) {
        info!("Special ability activated! (Ctrl + Shift + A)");
    }
}
```

When you place your mouse on the screen it would two positions:

1. On-screen coordinates (the position of the pixel on a screen)
2. World coordinates (the position of the mouse projected onto our game)

The `RelativeCursorPosition` component stores the cursor position relative to
our node. If it is within the range of `(-0.5, -0.5)` to `(0.5, 0.5)` then the cursor
is currently over the node, with `(0., 0.)` being center. You can use the
`cursor_over: bool` field to figure this out.

If the cursor position is unknown (e.g we are alt+tabbed out of our game) then
the position will be `None`.

```rust
use bevy::ui::RelativeCursorPosition;

fn relative_cursor_position(cursor_query: Query<&RelativeCursorPosition>) {
  if let Ok(cursor) = cursor_query.single() {
    if let Some(cursor) = cursor.normalized {
      info!("({:.1}, {:.1})", cursor.x, cursor.y)
    }
  }
}
```

## Cameras

Each `Camera` is responsible for 3 main things:

1. The __render target__ which is the region of the screen to draw something
1. The __projection__ which determines how to transform 3D into 2D (our screen)
1. The __position__ of the view in our scene to capture and transform

Each frame, Bevy will start by drawing the `ClearColor` over the camera's
viewport and then draw things from scratch on the screen.

The coordinate system in Bevy is __right handed__ so:

- X increases going to the right
- Y increases going up
- Z increases coming towards the screen
- The default center of the screen is (0, 0)

When we spawn a camera we use `Camera2d` or `Camera3d` depending on
our game.

```rust
// Useful for marking the "main" camera if we have many
#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn initialize_camera(mut commands: Commands) {
  commands.spawn(MainCamera);
}

fn move_camera(
  mut camera: Single<&mut Transform, With<MainCamera>>,
  player: Single<&Transform, With<Player>>,
  time: Res<Time>,
) {
  let direction = Vec3::new(
    player.translation.x,
    player.translation.y,
    camera.translation.z,
  );

  camera.translation =
    camera.translation.lerp(direction, time.delta_secs() * 2.);
}

fn rotate_camera_to_mouse(
  time: Res<Time>,
  mut mouse_motion: MessageReader<MouseMotion>,
  mut transform: Single<&mut Transform, With<Camera>>,
) {
  let dt = time.delta_secs();
  // The factors are just arbitrary mouse sensitivity values.
  // It's often nicer to have a faster horizontal sensitivity than vertical.
  let mouse_sensitivity = Vec2::new(0.12, 0.10);

  for motion in mouse_motion.read() {
    let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
    let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

    // Add yaw which is turning left/right (global)
    transform.rotate_y(delta_yaw);

    // Add pitch which is looking up/down (local)
    const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    // Apply the rotation
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
  }
}
```

## UI

Bevy's UI system is also done through its ECS.

A `Node` is a component that holds the layout and style properties.
Nodes are laid out with either a flexbox or CSS grid layout.

This is what a `Node` looks like:

```rust
impl Node {
  pub const DEFAULT: Self = Self {
    display: Display::DEFAULT,
    box_sizing: BoxSizing::DEFAULT,
    position_type: PositionType::DEFAULT,
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,
    flex_direction: FlexDirection::DEFAULT,
    flex_wrap: FlexWrap::DEFAULT,
    align_items: AlignItems::DEFAULT,
    justify_items: JustifyItems::DEFAULT,
    align_self: AlignSelf::DEFAULT,
    justify_self: JustifySelf::DEFAULT,
    align_content: AlignContent::DEFAULT,
    justify_content: JustifyContent::DEFAULT,
    margin: UiRect::DEFAULT,
    padding: UiRect::DEFAULT,
    border: UiRect::DEFAULT,
    flex_grow: 0.0,
    flex_shrink: 1.0,
    flex_basis: Val::Auto,
    width: Val::Auto,
    height: Val::Auto,
    min_width: Val::Auto,
    min_height: Val::Auto,
    max_width: Val::Auto,
    max_height: Val::Auto,
    aspect_ratio: None,
    overflow: Overflow::DEFAULT,
    overflow_clip_margin: OverflowClipMargin::DEFAULT,
    row_gap: Val::ZERO,
    column_gap: Val::ZERO,
    grid_auto_flow: GridAutoFlow::DEFAULT,
    grid_template_rows: Vec::new(),
    grid_template_columns: Vec::new(),
    grid_auto_rows: Vec::new(),
    grid_auto_columns: Vec::new(),
    grid_column: GridPlacement::DEFAULT,
    grid_row: GridPlacement::DEFAULT,
  };
}
```

Which we can use to spawn a simple UI box centered on the screen:

```rust
fn spawn_box(mut commands: Commands) {
  let container = Node {
    width: percent(100.0),
    height: percent(100.0),
    justify_content: JustifyContent::Center,
    ..default()
  };

  let square = (
    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
    Node {
      width: px(200.),
      border: UiRect::all(px(2.)),
      ..default()
    },
  );

  commands.spawn((container, children![(square)]));
}
```

All `Children` of a node will set their position to be relative to their
parent, so the `Node` we spawned as a child will be placed in the center
of its parent.

Text can be rendered in two separate ways:

1. As part of our game with `Text2d`
2. As part of our UI with `Text`

```rust
fn spawn_text_in_ui(mut commands: Commands, assets: Res<AssetServer>) {
  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      bottom: px(5.0),
      right: px(5.0),
      ..default()
    },
    Text::new("Here is some text"),
    TextColor(Color::BLACK),
    TextLayout::new_with_justify(Justify::Center),
  ));
}

fn spawn_text_in_scene(
  asset_server: ResMut<AssetServer>,
  mut commands: Commands,
) {
  commands.spawn((
    TextFont {
      font: asset_server.load("fonts/FiraSans-Bold.ttf"),
      font_size: 100.0,
      ..default()
    },
    TextColor(Color::WHITE),
    Text2d::new("Hello, Bevy!"),
    TextLayout::new_with_justify(Justify::Center),
    Transform::from_xyz(0., 0., 0.),
  ));
}
```

Adding interactivity happens through an `Interaction` component.

```rust
fn button_system(
  mut interactions: Query<
    (
      &Interaction,
      &mut BackgroundColor,
      &mut BorderColor,
      &Children,
    ),
    (Changed<Interaction>, With<Button>),
  >,
  mut texts: Query<&mut Text>,
) {
  for (interaction, mut color, mut border_color, children) in &mut interactions
  {
    if let Ok(mut text) = texts.get_mut(children[0]) {
      match *interaction {
        Interaction::Pressed => {
          text.0 = "Press".to_string();
          *color = PRESSED_BUTTON.into();
          border_color.set_all(BLUE);
        }
        Interaction::Hovered => {
          text.0 = "Hover".to_string();
          *color = HOVERED_BUTTON.into();
          border_color.set_all(WHITE);
        }
        Interaction::None => {
          text.0 = "Button".to_string();
          *color = NORMAL_BUTTON.into();
          border_color.set_all(BLACK);
        }
      }
    }
  }
}
```

The `UiStack` orders the UI nodes so that we can have stacking windows. The
first entry is the furthest node and the first to get rendered on the screen.

However the first node is also the last to receive any interactions so its
actually the final node that would be interacted with.

## Timers

Timers come in two modes:

1. `TimerMode::Once` which will tick down to 0 once, and only resets manually
2. `TimerMode::Repeat` which will tick down to 0 then reset itself automatically

In Bevy, timers don't tick down from their initial value. Instead they tick up
from zero until they reach their `Duration`.

We can then call `finished` or `just_finished` to switch behavior when they are
done. The difference between them is that `just_finished` only returns `true` if
the timer finished in the last tick.

```rust
#[derive(Resource, Default)]
pub struct MatchTime(Timer);

impl MatchTime {
  pub fn new() -> Self {
    Self(Timer::from_seconds(60.0, TimerMode::Once))
  }
}

fn countdown(time: Res<Time>, mut match_time: ResMut<MatchTime>) {
  match_time.0.tick(time.delta());
}

fn end_match(match_time: Res<MatchTime>) {
  if match_time.0.is_finished() {
    // Here we would rest our game
  }
}
```

Bevy has a built in `Time` resource we can use to get the `delta` in seconds of
the time between this tick and the last.

```rust
fn time_passed(time: Res<Time>) {
  info!("Duration passed: {:?}", time.delta());
  info!("Seconds passed: {:?}", time.delta_secs_f64());
  info!("Total time since startup: {:?}", time.elapsed());
}
```

Now lets say we wanted to have a `Cooldown` for one of our abilities. This
wouldn't make sense as a `Resource` because the cooldown would be specific to
one of our players.

```rust
#[derive(Component)]
struct Cooldown(Timer);

#[derive(Component)]
struct Player;

fn cast_spell(
  mut commands: Commands,
  mut player_query: Query<Entity, With<Player>>,
  cooldowns: Query<&Cooldown, With<Player>>,
) {
  if let Ok(player) = player_query.single() {
    if let Ok(cooldown) = cooldowns.get(player) {
      info!(
        "You cannot cast yet. Your cooldown is {:0.0}% complete!",
        cooldown.0.fraction() * 100.0
      )
    } else {
      // Add an entity to the world with a timer
      commands
        .entity(player)
        .insert(Cooldown(Timer::from_seconds(5.0, TimerMode::Once)));

      // Cast the spell here
    }
  }
}

fn tick_cooldowns(
  mut commands: Commands,
  mut cooldowns: Query<(Entity, &mut Cooldown)>,
  time: Res<Time>,
) {
  for (entity, mut cooldown) in &mut cooldowns {
    cooldown.0.tick(time.delta());

    if cooldown.0.is_finished() {
      commands.entity(entity).remove::<Cooldown>();
    }
  }
}
```

Sometimes you will want a timer that is only ever used in a single system. For
these cases a `Resource` would be too public and you might prefer a `Local`
system parameter instead:

```rust
fn local_timer(time: Res<Time>, mut timer: Local<Timer>) {
  timer.tick(time.delta());

  if timer.just_finished() {
    info!("The timer is finished");
  }
}
```

## Audio

`AudioSource` holds the audio data and is connected to an `AudioSink` which is
usually done by spawning an `AudioPlayer`.

The data must be one of the file formats supported by Bevy:

- `wav`
- `ogg`
- `flac`
- `mp3`

A sink is a destination for the sound data. This is the place where sources will
send their data and will be emitted to the global listener.

```rust
fn play_pitch(
    mut pitch_assets: ResMut<Assets<Pitch>>,
    mut commands: Commands,
) {
    info!("playing pitch with frequency: {}", 220.0);
    commands.spawn((
        AudioPlayer(pitch_assets.add(Pitch::new(220.0, Duration::new(1, 0)))),
        PlaybackSettings::DESPAWN,
    ));
}
```

There are a few different playback settings that are built in:

|Setting|Description|
|-------|-----------|
|`PlaybackSettings::ONCE`|Will play the associated audio only once|
|`PlaybackSettings::LOOP`|Will loop the audio|
|`PlaybackSettings::DESPAWN`|Will play the audio once then despawn the entity|
|`PlaybackSettings::REMOVE`|Will play the audio once then despawn the component|

We can trigger our sounds to play by spawning an `AudioPlayer` on any entity.

```rust
fn play_background_audio(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
) {
  let audio = asset_server.load("background_audio.ogg");

  // Create an entity dedicated to playing our background music
  commands.spawn((
    AudioPlayer::new(audio),
    PlaybackSettings::LOOP,
  ));

  // Spawn our listener
  commands.spawn((
    SpatialListener::new(100.), // Gap between the ears
    Transform::default(),
  ));
}
```

Once the asset is loaded the music will start playing in a loop until this
entity we spawned is despawned or the component is removed.

To control the playback of our `AudioPlayer` we can use the `AudioSink` which
was added by the `AudioPlugin` automatically when we spawned our entity:

```rust
fn pause(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  music_controller: Query<&AudioSink, With<MusicBox>>,
) {
  let Ok(sink) = music_controller.single() else {
    return;
  };

  if keyboard_input.just_pressed(KeyCode::Space) {
    sink.toggle_playback();
  }
}
```

The `AudioSink` is our public API to:

|Method|Description|
|------|-----------|
|`play`|Resumes playback|
|`pause`|Pause playback|
|`stop`|Stop the playback, cannot be restarted after|
|`mute`|Mute the playback|
|`unmute`|Unmute the playback|
|`toggle_playback`|Toggle the playback|
|`toggle_mute`|Toggle muting the playback|
|`is_paused`|Returns true if the sink is paused|
|`is_muted`|Returns true if the sink is muted|
|`speed`|Get the speed of the sound|
|`set_speed`|Control the speed of the playback|
|`empty`|Returns true if the sink has no more sounds to play|
|`try_seek`|Seek to a certain point in the source sound|

There are two separate sources of volume for our apps:

1. Global volume
2. Audio sink volume

To change the global volume we modify the `GlobalVolume` resource:

```rust
use bevy::audio::Volume;

fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
  volume.volume = Volume::Linear(0.5);
}
```

## Scenes

Scenes can be serialized into file based representations. Everything is
serialized into a file and then reinitialized when the scene is loaded.

Scenes are saved into a `.scn` or `.scn.ron`. The format of the file is based on
[Rusty Object Notation (RON)](https://crates.io/crates/ron).

We save scenes to a file by using the `DynamicScene::serialize` method:

```rust
fn save_scene_system(world: &mut World) {
  let scene = DynamicScene::from_world(world);

  // Scenes can be serialized like this:
  let type_registry = world.resource::<AppTypeRegistry>();
  let type_registry = type_registry.read();
  let serialized_scene = scene.serialize(&type_registry).unwrap();

  // Showing the scene in the console
  info!("{}", serialized_scene);

  // Writing the scene to a new file. Using a task to avoid calling the
  // filesystem APIs in a system as they are blocking This can't work in WASM as
  // there is no filesystem access
  #[cfg(not(target_arch = "wasm32"))]
  IoTaskPool::get()
    .spawn(async move {
      // Write the scene RON data to file
      File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
        .and_then(|mut file| file.write(serialized_scene.as_bytes()))
        .expect("Error while writing scene to file");
    })
    .detach();
}
```

When Bevy loads the scene file, it needs to deserialize it into actual
components and entities that it loads into your world.

There are 3 ways to spawn scenes:

1. Using `SceneSpawner::spawn_dynamic`
2. Adding the `DynamicSceneRoot` component to an entity
3. Using the `DynamicSceneBuilder` to construct a `DynamicScene` from a `World`

The easiest of these is simply spawning a `DynamicSceneRoot`. It uses the
`SceneLoader` to deserialize everything:

```rust
const SCENE_FILE_PATH: &str = "scene.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  // "Spawning" a scene bundle creates a new entity and spawns new instances
  // of the given scene's entities as children of that entity.
  let scene = asset_server.load(SCENE_FILE_PATH);
  commands.spawn(DynamicSceneRoot(scene));
}
```

Once the scene has been loaded, a `SceneInstance` component is added to the
component which can be used with the `SceneSpawner` to interact with the scene.

For example we could despawn all our loaded scenes:

```rust
use bevy::scene::SceneInstance;

fn despawn_all_scenes(
  query: Query<&SceneInstance>,
  mut spawner: ResMut<SceneSpawner>,
  world: &mut World,
) {
  // Despawning the scene root entity will also despawn all of its children
  for instance in &query {
    spawner.despawn_instance_sync(world, instance);
  }
}
```

The `FromWorld` trait determines how your component is constructed when it
loads into the `World`.

Implementing `FromWorld` on a component will let you customize initialization
using the current Worlds resources:

```rust
impl FromWorld for ComponentB {
  fn from_world(world: &mut World) -> Self {
    let time = world.resource::<Time>();
    ComponentB {
      _time_since_startup: time.elapsed(),
      value: "Default Value".to_string(),
    }
  }
}
```

## Physics

Bevy does not have a built-in physics engine. The most native to Bevy is
`avian`.

Your position, in the eyes of Bevy's renderer, is dictated by an entity's
`Transform` component.

In Avian we can use a somewhat more convenient `Position` component. This is
kept in sync with the `Transform` automatically by Avian's `SyncPlugin`.

```rust
fn move_things_with_position(mut query: Query<&mut Position>) {
  for mut position in &mut query {
    position.x += 1.;
  }
}
```

Just like a `Position` Avian provides a `Rotation` component.

```rust
fn rotate_things(mut query: Query<&mut Rotation>) {
  for mut rotation in &mut query {
    *rotation = rotation.add_angle_fast(0.1);
  }
}
```

Rigid bodies come in 3 different components, each specialized for something:

1. `RigidBody::Dynamic` are similar to real life objects and are affected by
   forces and contacts.
2. `RigidBody::Kinematic` can only be moved programmatically, which is useful
   for things like player character controllers and moving platforms.
3. `RigidBody::Static` can not move, so they can be good for objects in the
   environment like the ground and walls.

To move things we can control the `Position` directly or use a `LinearVelocity`
component:

```rust
fn spawn_ball(mut commands: Commands) {
  commands.spawn((
    RigidBody::Dynamic,
    LinearVelocity(Vec2::new(0.0, 0.0)),
    Collider::circle(0.5),
    Mass(5.0),
    CenterOfMass::new(0.0, -0.5),
  ));
}
```

Avian is going to use the size of this collider to determine how much mass your
body has.

Adding a `Sensor` component will let you detect collisions without affecting the
entity's mass properties or interacting with other physical bodies.

```rust
fn spawn_sensor(mut commands: Commands) {
  commands.spawn((
    RigidBody::Dynamic,
    Collider::circle(0.5),
    Sensor,
  ));
}
```

For processing a large number of collisions at once you would use the
`MessageReader`:

```rust
fn react_to_collisions(mut collision_events: MessageReader<CollisionStart>) {
  for event in collision_events.read() {
    info!(
      "Collision started between {:?} and {:?}",
      event.collider1, event.collider2
    );
  }
}
```

However if we want entity-specific collisions then we can use observers:

```rust
#[derive(Component)]
struct SecurityCamera;

#[derive(Component)]
struct Enemy;

fn setup_security_cameras(mut commands: Commands) {
  commands
    .spawn((
      SecurityCamera,
      Collider::circle(3.0), // Detection radius
      Sensor,
      CollisionEventsEnabled, // So we receive collision events
    ))
    .observe(|trigger: On<CollisionStart>, enemy_query: Query<&Enemy>| {
      let camera = trigger.collider1;
      let intruder = trigger.collider2;
      if enemy_query.contains(intruder) {
        println!("Security camera {camera} detected enemy {intruder}!");
      }
    });
}
```
