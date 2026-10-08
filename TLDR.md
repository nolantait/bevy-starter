Bevy is an archetypal Entity Component System (ECS) game engine built in Rust.
It emphasizes modularity, performance, and ease of use.

## Entity and Components

An `Entity` is just an identifier to find associated `Component`. A `Component`
is what actually stores the data that makes up each entity.

A good mental model to use is that entities represent a row in an in-memory
database, while components are our columns.

Each `Entity` can only have a single `Component` per type. These components can
be added and removed dynamically over the course of the entity's lifetime.
Everything is stored inside a `World` which is managed by an `App`.

We define components by deriving the `Component` trait:

```rust
// A simple marker type
#[derive(Component)]
struct Enemy;

// Or an enum
#[derive(Component, Default)]
#[component(on_add = on_ship_added)]
enum Ship {
  Destroyer,
  Frigate,
  #[default]
  Scout,
}

// A unit struct component
#[derive(Component)]
struct Health(f32);

// With fields
#[derive(Component, Default)]
struct Position {
  x: i32,
  y: i32,
}
```

Components have 5 different life-cycle hooks we can use to handle side effects
that __need__ to happen:

1. `#[component(on_add = on_add_function)]`
2. `#[component(on_insert = on_insert_function)]`
3. `#[component(on_discard = on_discard_function)]`
4. `#[component(on_remove = on_remove_function)]`
5. `#[component(on_despawn = on_despawn_function)]`

The systems we pass to these take a `DeferredWorld` and a `HookContext`.

```rust
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;

// Or an enum
#[derive(Component, Default)]
#[component(on_add = on_ship_added)]
enum Ship {
  Destroyer,
  Frigate,
  #[default]
  Scout,
}

fn on_ship_added(mut world: DeferredWorld, context: HookContext) {
  world
    .commands()
    .entity(context.entity)
    .insert(Position { x: 0, y: 0 });
}
```

The preferred way to add components based on other components is to make
them required:

```rust
#[derive(Component)]
#[require(Position, Ship)]
struct Player;

fn spawn_player_with_required_components(mut commands: Commands) {
  commands.spawn(Player);
}
```

When a component is spawned, if it has any required components, it will
automatically add them __unless__ we override them. A bare `#[require(Foo)]`
uses `Foo::default()`, so `Foo` must implement `Default`; if you provide a
constructor instead, `Default` is not required.

All these required calls are __recursive__. If a component you require has
required components, they will also be added.

A `Resource` is actually a `Component` that belongs to a singleton entity. This
means resources can be queried like any other component, or more conveniently
through the `Res` and `ResMut` system parameters. There can only ever be one
instance of a resource type per `World`.

```rust
#[derive(Resource)]
struct Score(usize);

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Score(0))
    .run();
}
```
## Systems

Systems are where we trigger side effects that change our game's state.

In Bevy, systems are simple rust functions with one rule: They must have all of
their parameters implement `SystemParam`.

```rust
fn spawn_player(mut commands: Commands) {
  commands
    .spawn_empty()
    .insert(Player)
    .insert(Ship::Destroyer)
    .insert(Position { x: 1, y: 2 });
}
```
`Commands` are what we use to change the state of our `World` in a way that is
more performant than letting each system mutate the world directly.

When you use the system parameter `Commands` you are enqueuing commands to a
`CommandQueue`. Bevy applies them at a **sync point**, an `ApplyDeferred`
system it inserts into the schedule after systems that produce commands and are
depended on, as well as at the end of every schedule.

## Apps

Everything is coordinated through an `App` which schedules our systems to run at
certain points in the game's loop:

```rust
use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_everything)
    .add_systems(Update, move_player)
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
7. `RunFixedMainLoop` which runs the `FixedMain` schedules including
   `FixedUpdate`
8. `Update`
9. `SpawnScene`
10. `PostUpdate`
11. `Last`

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
    .add_plugins(DefaultPlugins)
    // Add our state to our app definition
    .init_state::<AppState>()
    .init_resource::<Ui>()
    .add_systems(Startup, setup)
    .add_observer(on_menu_button_pressed)
    // We can add systems to trigger during transitions
    .add_systems(OnEnter(AppState::MainMenu), spawn_menu)
    .add_systems(OnExit(AppState::MainMenu), despawn_menu)
    // Or we can use run conditions
    .add_systems(Update, play_game.run_if(in_state(AppState::InGame)))
    .add_systems(Update, toggle_game_pause)
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

Plugins are a way to group related functionality together.

Almost every app will include the `DefaultPlugins` plugin which groups together
all the default functionality needed for a game.

```rust
use bevy::prelude::*;

fn main() {
  App::new().add_plugins(DefaultPlugins).run();
}
```
`DefaultPlugins` includes the following

| Plugin | Feature | Description |
| --- | --- | --- |
| **DiagnosticsPlugin** |  | Adds core diagnostics |
| **FrameCountPlugin** |  | Adds frame counting functionality |
| **InputPlugin** |  | Adds keyboard and mouse input |
| **PanicHandlerPlugin** |  | Adds sensible panic handling |
| **TaskPoolPlugin** |  | Setup of default task pools for multi-threading |
| **TimePlugin** |  | Adds time functionality |
| **TransformPlugin** |  | Handles `Transform` components |
| **AccessibilityPlugin** | `bevy_window` | Adds non-GUI accessibility functionality |
| **AnimationPlugin** | `bevy_animation` | Adds animation support |
| **AntiAliasPlugin** | `bevy_anti_alias` | Adds multi-sample anti-aliasing (MSAA) |
| **AssetPlugin** | `bevy_asset` | Adds asset server and resources to load assets |
| **AudioPlugin** | `bevy_audio` | Adds support for using sound assets |
| **CameraPlugin** | `bevy_camera` | Adds 2D and 3D camera components and systems |
| **CiTestingPlugin** | `bevy_ci_testing` | Helps instrument continuous integration |
| **ClipboardPlugin** | `bevy_clipboard` | Adds clipboard support to a Bevy app |
| **CorePipelinePlugin** | `bevy_core_pipeline` | The core rendering pipeline |
| **DefaultPickingPlugins** | `bevy_picking` | Adds picking functionality |
| **DlssInitPlugin** | `dlss` | Initializes DLSS support if available |
| **GilrsPlugin** | `bevy_gilrs` | Adds support for gamepad inputs |
| **GizmoPlugin** | `bevy_gizmos` | Provides an immediate mode drawing api for visual debugging |
| **GizmoRenderPlugin** | `bevy_gizmos_render` | Sends gizmos to the renderer |
| **GltfPlugin** | `bevy_gltf` | Adds support for loading gltf models |
| **HotPatchPlugin** | `hotpatching` | Enables hot-patching of assets |
| **ImagePlugin** | `bevy_image` | Adds the `Image` asset and prepares them to render on your GPU |
| **InputDispatchPlugin** | `bevy_input_focus` | Dispatches bubbling keyboard and gamepad button events to the focused entity |
| **InputFocusPlugin** | `bevy_input_focus` | Sets up the core input focus system |
| **LightPlugin** | `bevy_light` | Adds light components and systems |
| **LogPlugin** | `bevy_log` | Adds logging to apps |
| **MeshPlugin** | `bevy_mesh` | Adds the `Mesh` asset and prepares them to render on the GPU |
| **PbrPlugin** | `bevy_pbr` | Adds physical based rendering with `StandardMaterial` etc |
| **PipelinedRenderingPlugin** | `bevy_render` | Adds pipelined rendering |
| **PostProcessPlugin** | `bevy_post_process` | Adds post processing effects |
| **RenderDebugOverlayPlugin** | `render_dev_tools` | Adds a rendering debug overlay to visualize various renderer buffers |
| **RenderPlugin** | `bevy_render` | Sets up rendering backend powered by `wgpu` crate |
| **ScenePlugin** | `bevy_scene` | Adds support for spawning Bevy Scenes |
| **ScheduleRunnerPlugin** | `!bevy_window` | Configures an `App` to run its `Schedule` according to a given `RunMode` (added when windowing is disabled) |
| **SpritePlugin** | `bevy_sprite` | Handling of sprites (images on our entities) |
| **SpriteRenderPlugin** | `bevy_sprite_render` | Adds support for sending sprites to the renderer |
| **StatesPlugin** | `bevy_state` | Adds state management for Apps |
| **TerminalCtrlCHandlerPlugin** | `std` | Handles Ctrl-C signals in terminal applications |
| **TextPlugin** | `bevy_text` | Supports loading fonts and rendering text |
| **UiPlugin** | `bevy_ui` | Adds support for UI layouts (flex, grid, etc) |
| **UiRenderPlugin** | `bevy_ui_render` | Adds support for sending UI nodes to renderer |
| **UiWidgetsPlugins** | `bevy_ui_widgets` | Registers the observers for all of the widgets in this crate |
| **WebAssetPlugin** | `bevy_asset` | Adds the `http` and `https` asset sources to an app |
| **WindowPlugin** | `bevy_window` | Provides an interface to create and manage `Window` components |
| **WinitPlugin** | `bevy_winit` | Interface to create operating system windows (to actually display our game) |
| **WorldSerializationPlugin** | `bevy_world_serialization` | Loading and saving collections of entities and components to files |
Plugins receive a mutable reference to the `App` and can add systems, resources,
and other plugins. Plugins are built in the order they are added to the `App`.

```rust
fn plugin(app: &mut App) {
  app.add_systems(Startup, some_plugin_system);
}

fn main() {
  App::new().add_plugins(plugin).run();
}
```
If we need to manage the life-cycle of a plugin we can implement the `Plugin`
trait and hook into it.

```rust
struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn finish(&self, _app: &mut App) {
    info!("Camera plugin finished setting up");
  }

  fn build(&self, app: &mut App) {
    app.add_systems(Startup, initialize_camera);
  }
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
//     |          ---- Which have a `Player` component on the same entity
//     v          v
Query<&Transform, With<Player>>

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
|`Has<T>`|Returns a bool that describes if an entity has the component `T`|
|`Entity`|returns the entity|
|`SpawnDetails`|when paired with the `Spawned` filter, gives access to when and where an entity was spawned|

In addition to the `Query` system parameter there are other sibling system
parameters that also perform queries:

|System parameter|Description|
|----------------|-----------|
|`Single<D, F>`|Matches exactly one query item. Skips the system if more or none.|
|`Option<Single<D, F>>`|Matches zero or one query item. Yields `None` if there are none or more than one.|
|`Populated<D, F>`|matches at least one or more. Skips the system if none.|

`Single` can be useful to reduce boilerplate when you know there is only ever a
single entity with a particular component:

```rust
fn move_the_only_player(mut transform: Single<&mut Transform, With<Player>>) {
  transform.translation.x += 1.
}
```
This system would NOT be run unless there was a single entity that matched the
components we query.

The second argument in your `Query<D, F>` is the `QueryFilter`. These filters
are wrapped by a condition type:


|method|description|
|------|-----------|
|`With<T>`|only items with a `T` component|
|`Without<T>`|only items without a `T` component|
|`Or<F>`|checks if any filters in the tuple `F` apply|
|`Changed<T>`|only components of type `T` that changed since the system last ran|
|`Added<T>`|only components of type `T` that were added since the system last ran|
|`Spawned`|only entities that were spawned since the system last ran|

To retrieve components from our ECS storage our `Query` system parameter
provides an API with the following methods:

| Method | Description |
| --- | --- |
| `iter` / `iter_mut` | returns an iterator over all items |
| `iter_many` / `iter_many_mut` | iterates over only the items matching a list of entities, yielding a `Result` for each entity |
| `iter_combinations` / `iter_combinations_mut` | returns an iterator over all combinations of a specified number of items |
| `contiguous_iter` / `contiguous_iter_mut` | returns an iterator that receives the whole table slice at once, allowing SIMD optimizations to work |
| `contiguous_par_iter` / `contiguous_par_iter_mut` | returns a parallel iterator that receives whole table slices at once, allowing SIMD optimizations to work |
| `par_iter` / `par_iter_mut` | returns a parallel iterator |
| `get` / `get_mut` | returns a query item for a given entity |
| `get_many` / `get_many_mut` | returns query items for each entity in a fixed-size array |
| `single` / `single_mut` | returns the only query item as a `Result`, `Err` if there isn't exactly one |
| `is_empty` | returns true if the query is empty |
| `contains` | returns true if the query contains a given entity |
<div></div>

Every method that returns query items has a `*_mut` variant which returns the
components with mutable ownership. This lets us change their data, instead of
just reading it. The `*_mut` methods require a mutable `Query` parameter.

In situations where we have a particular `Entity` (which is basically an ID),
we can use `get` or `get_mut`.

```rust
#[derive(Resource)]
struct PlayerRef(Entity);

fn move_player_by_component(
  mut query: Query<&mut Transform>,
  player: Res<PlayerRef>,
) {
  if let Ok(mut transform) = query.get_mut(player.0) {
    transform.translation.x += 1.;
  }
}
```

In cases where we have a list of `Entity` and we want to iterate over only those
entity components we can use `iter_many`.

```rust
#[derive(Component, Debug)]
struct Health(f32);

#[derive(Resource)]
struct Selection {
  enemies: Vec<Entity>,
}

fn attack_selected_enemies(
  mut query: Query<&mut Health>,
  selected: Res<Selection>,
) {
  let mut iter = query.iter_many_mut(&selected.enemies).matched();
  while let Some(mut health) = iter.fetch_next() {
    health.0 -= ATTACK_DAMAGE;
  }
}
```

`iter_many`/`iter_many_mut` yield a `Result` for each entity, so missing or
despawned entities don't silently disappear. Chain `.matched()` to skip entities
that don't match the query.

## Assets

To load assets we use the `AssetServer` which manages asynchronous loading
assets from a particular `AssetSource`, usually the filesystem.

All assets follow the same general process:

1. We register a new `Asset<T>` type if it's custom
2. We register an `AssetLoader` for that asset if it's custom
3. We add the asset to our `assets` folder
4. Then we call `AssetServer::load` to get a `Handle<T>` to the asset

```rust
fn load_images(
  mut bevy_image: ResMut<BevyImage>,
  asset_server: Res<AssetServer>,
) {
  // This will not block, the asset will be loaded in the background
  bevy_image.0 = asset_server.load("example_images/bevy.png");
}
```
By default Bevy looks for our assets inside the `assets` folder. The base
directory it uses depends on how the app is run: `cargo run` sets
`CARGO_MANIFEST_DIR` to our crate root, while a standalone executable falls back
to the directory containing the executable. You can change it with the
`file_path` field on `AssetPlugin`, or with the `BEVY_ASSET_ROOT` /
`CARGO_MANIFEST_DIR` environment variables.

Assets can be tracked one of two ways:

1. Through events like `AssetEvent::LoadedWithDependencies`
2. Or by querying the asset server with `AssetServer::get_load_state`.

## Messages and events

There are two kinds of events in Bevy:

1. `Message` for communication between systems
2. `Event` and `EntityEvent` for __observers__ that react when events are
   triggered

`Messages<T>` is a collection that acts as a double buffered queue.
This is done to ensure each system has an opportunity to see each
message. It helps systems not have to care about the exact ordering within
a frame.

Messages are defined by deriving the `Message` trait and need to be registered
to the `App`:

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
    .add_plugins(DefaultPlugins)
    .add_message::<PlayerKilled>()
    .add_message::<PlayerDetected>()
    .run();
}
```

If your messages are not consumed within two updates then
they will be cleaned up and dropped silently.

To write messages to a stream we use a `MessageWriter<T>`:

```rust
fn detect_player(
  mut messages: MessageWriter<PlayerDetected>,
  players: Query<(Entity, &Transform), With<Player>>,
) {
  for (entity, transform) in players {
    // ...
    messages.write(PlayerDetected(entity));
  }
}
```
We can read messages from our systems with a
`MessageReader<T>` that consumes messages from our buffers:

```rust
fn react_to_detection(mut messages: MessageReader<PlayerDetected>) {
  for message in messages.read() {
    // Do something with each event here
  }
}
```
To skip a system entirely when there are no new messages, use
`PopulatedMessageReader<T>` or the `on_message::<T>` run condition instead.

Events are the immediate version of messages. They come in two types:

1. `Event` for global events defined with a `GlobalTrigger`
2. `EntityEvent` for entity specific events defined with an `EntityTrigger`, or
   with a `PropagateEntityTrigger` when propagation is enabled

Propagation is opt-in and needs to be enabled on the event. With
`auto_propagate` it happens automatically; otherwise an observer must call
`On::propagate(true)`. The traversal defaults to `ChildOf` and can be changed
with `propagate = &'static SomeRelationship`.

```rust
#[derive(EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static ChildOf)]
struct LocationTravelled {
  #[event_target]
  entity: Entity,
}
```
These events are consumed by an `Observer` which is a callback system that
__must__ take an `On` system parameter as the __first__ argument:

```rust
fn on_respawn(event: On<Add<Enemy>>, query: Query<(&Enemy, &Position)>) {
  let (enemy, position) = query.get(event.entity).unwrap();
  println!("Enemy was respawned at {:?}", position);
}
```
Observers can be global by adding them to the `App` definition:

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_observer(on_respawn);
}
```
Or they can be local and only triggered for particular entities:

```rust
fn spawn_boss(mut commands: Commands) {
  let entity = commands
    .spawn((Enemy, Boss, Position(Vec2::ZERO)))
    .observe(on_boss_spawned)
    .id();

  commands.trigger(BossSpawned { entity });
}
```
An `EntityEvent` with propagation enabled will bubble up a hierarchy along its
`Traversal` (by default `ChildOf`, which goes from child to parent).

This table summarizes the differences between events and messages:

| |Events|Messages|
|-|---------|------|
|__Optimal event frequency__|Infrequent|Frequent|
|__Handler__|Only handles a single event|Can handle many messages together|
|__Latency__|Immediate with `World::trigger`; next sync point with `Commands::trigger`|Up to 1 frame|
|__Event propagation__|Bubbling|None|
|__Scope__|World ___or___ Entity|World|
|__Ordering__|No explicit order|Ordered|
|__Coupling__|High|Low|

## Relationships

Bevy has a built-in relationship it provides for parent/child relationships that
is made up of two components:

1. `ChildOf`: The `Relationship` we attach to other entities
2. `Children`: The `RelationshipTarget` that is kept in sync

The built-in hierarchy is wired into Bevy's transform and visibility
propagation, so a parent's `Transform` and `Visibility` are inherited by its
children automatically.

When you despawn the parent (the entity holding the `Children`) then all of its
children and their descendants are despawned automatically, because `Children`
is declared with `linked_spawn`. Use `detach_children` or `detach_all_children`
first if you want to keep them alive.

```rust
fn spawn_ship(mut commands: Commands) {
  let fleet = commands.spawn((Fleet, Name::new("Fleet A"))).id();

  commands.spawn((Ship, Name::new("Ship A"), ChildOf(fleet)));
}
```
We can spawn children from a parent with the `with_children` method:

```rust
fn spawn_fleet(mut commands: Commands) {
  commands
    .spawn((Fleet, Name::new("Fleet A")))
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
    Name::new("Fleet B"),
    children![(Ship, Name::new("Ship 3")), (Ship, Name::new("Ship 4"))],
  ));
}
```
The source of truth is the `Relationship` component. This is the component we
will be adding to __other entities__ to specify the relationship. It must contain
a reference to the entity we will be attaching ourselves to.

```rust
// This is the relationship FROM child TO parent
#[derive(Component)]
#[relationship(relationship_target = ShipAttachments)]
struct AttachedToShip(Entity);
```
The `RelationshipTarget` is the component that will automatically be kept in
sync with all our `AttachedToShip` components. It must contain a collection of
entities, using any type that implements `RelationshipSourceCollection` such as
`Vec<Entity>`, an entity hash set, or a single `Entity`.

```rust
// This is the relationship target on the parent
#[derive(Component)]
#[relationship_target(relationship = AttachedToShip, linked_spawn)]
struct ShipAttachments(Vec<Entity>);
```
The `linked_spawn` attribute means despawning the entity holding the
`ShipAttachments` will also despawn the entities that have an `AttachedToShip`
component. Even without it, despawning the ship would still remove the
`AttachedToShip` components, but leave the turret entities alive.

To create the relationship we can then spawn this `Relationship` on other
entities.

```rust
fn spawn_attached_to_ship(mut commands: Commands) {
  let ship = commands.spawn(Ship).id();

  commands.spawn((GunTurret, AttachedToShip(ship), Name::new("GunTurret 1")));
  commands.spawn((GunTurret, AttachedToShip(ship), Name::new("GunTurret 2")));
}
```
This can be shortened by using the `related!` macro to specify the relationships
from the parent entity instead:

```rust
fn build_ship(mut commands: Commands) {
  commands.spawn((
    Ship,
    Name::new("Ship A"),
    related!(ShipAttachments[
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
  turrets: Query<&Name, With<GunTurret>>,
) {
  for (ship_name, attachments) in &ships {
    info!("{} has the following attachments:", ship_name.as_str());

    for &attachment in &attachments.0 {
      if let Ok(child_name) = turrets.get(attachment) {
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
```

```rust
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

1. Reading messages emitted automatically by Bevy's input systems
2. Polling resources like `ButtonInput` or `Touches`, or querying components
   like `Gamepad`

Bevy has a different type for each kind of input:

|Type|Description|
|----|-----------|
|`ButtonInput<T>`|a "press-able" input, such as `KeyCode` or `MouseButton`|
|`Axis<T>`|stores the position data from certain input devices|
|`Touches`|a collection of `Touch`es that have happened|
|`TouchInput`|messages representing touch based input events|
|`Gamepad`|a component on each connected controller, holding its own button and axis state|
|`GamepadAxis`|an enum describing the axes available on a gamepad|


For example, to handle keyboard input we use the `ButtonInput<T>` resource which
has a set of convenient methods we can use to trigger behavior:


|Method|Description|
|------|-----------|
|`pressed`|will return `true` between a press and release event|
|`just_pressed`|will return `true` on the frame the press was processed|
|`just_released`|will return `true` on the frame the release was processed|


We can listen to keyboard events by reading `KeyboardInput` messages:

```rust
/// Track keyboard inputs — useful for debugging or keybinding tools
fn log_keyboard_input(mut keyboard_events: MessageReader<KeyboardInput>) {
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
```
```rust
/// Handle combos or hotkeys, e.g., casting special ability
fn combo_key_system(input: Res<ButtonInput<KeyCode>>) {
  let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
  let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

  if ctrl && shift && input.just_pressed(KeyCode::KeyA) {
    info!("Special ability activated! (Ctrl + Shift + A)");
  }
}
```
When you place your mouse on the screen it has two positions:

1. On-screen coordinates (the position of the pixel on a screen)
2. World coordinates (the position of the mouse projected onto our game)

The `RelativeCursorPosition` component stores the cursor position relative to
our node. If it is within the range of `(-0.5, -0.5)` to `(0.5, 0.5)` then the cursor
is currently over the node, with `(0., 0.)` being center. You can use the
`cursor_over: bool` field to figure this out.

If the cursor position is unknown (e.g we are alt+tabbed out of our game) then
the position will be `None`.

```rust
// This systems polls the relative cursor position
// and displays its value in a text component.
fn relative_cursor_position(cursor_query: Query<&RelativeCursorPosition>) {
  if let Ok(cursor) = cursor_query.single() {
    // This is an `Option<Vec2>` as an unknown cursor position
    // will return `None`
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
- Z increases coming out of the screen toward the viewer ("forward" is -Z)
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
```

```rust
use bevy::input::mouse::MouseMotion;

fn rotate_camera_to_mouse(
  mut mouse_motion: MessageReader<MouseMotion>,
  mut transform: Single<&mut Transform, With<MainCamera>>,
) {
  // The factors are just arbitrary mouse sensitivity values.
  // It's often nicer to have a faster horizontal sensitivity than vertical.
  let mouse_sensitivity = Vec2::new(0.12, 0.10);

  for motion in mouse_motion.read() {
    let delta_yaw = -motion.delta.x * mouse_sensitivity.x;
    let delta_pitch = -motion.delta.y * mouse_sensitivity.y;

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

Bevy ships optional camera controllers behind cargo features: `free_camera`,
`pan_camera` and `pan_orbit_camera`. `PanOrbitCamera` orbits, pans and zooms
around the point under the cursor, which it finds through picking. It ships
without an input mapping so we can wire it up ourselves.

```rust
use crate::pan_orbit_input::CustomInputPlugin;
use bevy::{
  camera_controller::pan_orbit_camera::prelude::*,
  prelude::*,
};

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins,
      // The controller uses picking to find the point under the cursor
      MeshPickingPlugin,
      DefaultPanOrbitCameraPlugins,
      // The controller ships without an input mapping, so we provide one
      CustomInputPlugin,
    ))
    .add_systems(Startup, (spawn_camera, spawn_cube))
    .run();
}

#[derive(Component)]
#[require(Camera3d, PanOrbitCamera)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    MainCamera,
    Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
  ));
}
```

## UI

Bevy's UI system is also done through its ECS.

A `Node` is a component that holds the layout and style properties.
Nodes are laid out with either a flexbox or CSS grid layout.

This is what a `Node` looks like:

```rust
// https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html
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
    direction: InlineDirection::Ltr,
    margin: UiRect::DEFAULT,
    padding: UiRect::DEFAULT,
    border: UiRect::DEFAULT,
    border_radius: BorderRadius::DEFAULT,
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
    scrollbar_width: 0.,
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
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    justify_content: JustifyContent::Center,
    ..default()
  };

  let square = (
    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
    Node {
      width: Val::Px(200.),
      border: UiRect::all(Val::Px(2.)),
      ..default()
    },
  );

  commands.spawn((container, children![(square)]));
}
```
All `Children` of a node will set their `UiTransform` relative to their
parent, so the `Node` we spawned as a child will be placed in the center
of its parent. You should never modify `UiTransform`/`UiGlobalTransform`
directly; position nodes through the `Node` layout and style fields.

Values are expressed with the `Val` enum, including `Val::Px`, `Val::Percent`,
`Val::Vw`, `Val::Vh`, `Val::VMin`, `Val::VMax`, `Val::Em` (relative to the
node's own font size, `EmSize`) and `Val::Rem` (relative to the root font size,
`RemSize`).

Text can be rendered in two separate ways:

1. As part of our game with `Text2d`
2. As part of our UI with `Text`

```rust
fn spawn_in_ui(mut commands: Commands, assets: Res<AssetServer>) {
  let regular_font_handle: Handle<Font> = Default::default();
  let bold_font_handle: Handle<Font> = assets.load("fonts/Roboto-Bold.ttf");

  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      bottom: Val::Px(5.0),
      right: Val::Px(5.0),
      ..default()
    },
    Text::new("Here is some text. But "),
    TextFont::from(regular_font_handle.clone()),
    children![(
      TextSpan::new("this part is bold"),
      TextFont::from(bold_font_handle.clone()),
    )],
  ));
}
```
```rust
fn spawn_in_scene(mut commands: Commands, assets: Res<AssetServer>) {
  let font = assets.load("fonts/FiraSans-Bold.ttf");

  commands.spawn((
    Text2d::new("Here is some text in the scene"),
    TextFont::from(font).with_font_size(FontSize::Px(60.0)),
    TextColor::WHITE,
    // `TextLayout::justify` only aligns the lines *within* the block of text.
    TextLayout::justify(Justify::Center),
    // Wrapping and truncation are controlled by `TextBounds`, not by the
    // `Node`.
    TextBounds::new_horizontal(300.0),
    // The `Anchor` controls where the text sits relative to its `Transform`.
    Anchor::TOP_LEFT,
    Text2dShadow::default(),
  ));
}
```
For text entry, spawn `EditableText` together with a `TextInput` from
`bevy_ui_widgets`. `EditableText` on its own is state-only, while `TextInput`
adds the working widget on top.

Adding interactivity is handled by the picking backend, which tracks hover and
press state with the `picking::hover::Hovered` and `ui::Pressed` components.
`Interaction` is a deprecated alias for `DeprecatedInteraction`, so prefer the
picking-based components in new code.

```rust
use bevy::color::palettes::css::*;

fn button_system(
  mut interactions: Query<
    (
      &Hovered,
      Has<Pressed>,
      &mut BackgroundColor,
      &mut BorderColor,
      &Children,
    ),
    With<Button>,
  >,
  mut texts: Query<&mut Text>,
) {
  for (hovered, pressed, mut color, mut border_color, children) in
    &mut interactions
  {
    if let Ok(mut text) = texts.get_mut(children[0]) {
      match (hovered.get(), pressed) {
        (_, true) => {
          text.0 = "Press".to_string();
          *color = PRESSED_BUTTON.into();
          border_color.set_all(BLUE);
        }
        (true, false) => {
          text.0 = "Hover".to_string();
          *color = HOVERED_BUTTON.into();
          border_color.set_all(WHITE);
        }
        (false, false) => {
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

However the first node is also the last to receive any interactions so it's
actually the final node that would be interacted with.

Bevy ships first-party widgets. `bevy_ui_widgets` (buttons, checkboxes, sliders,
radio groups, scrollbars, menus, popovers, lists, tabs and text inputs) is
enabled by default as part of the `ui` feature collection, while the larger
styled and themed `bevy_feathers` set requires enabling the `bevy_feathers`
feature.

## Picking

To enable clicking, dragging and other interactions we use bevy's built in
picking library.

A picking backend takes `PointerLocation` components and produces `PointerHits`
events that contain these pointers and the entities they are hitting.

An app can have multiple picking backends active at once.

The `PointerHits` event contains information about what entities a pointer is
currently hitting:

```rust
pub struct PointerHits {
    pub pointer: PointerId,
    pub picks: Vec<(Entity, HitData)>,
    pub order: f32,
}
```

These `PointerHits` are then used by the generic picking plugins to produce
higher level pointer events such as `PointerPress` or `PointerDrag`, which we can
react to with either an `Observer` or `MessageReader`.

Every pointer interaction has its own concrete event type rather than being
wrapped in a generic one. Each event carries a `pointer: Pointer` field with
metadata such as its `id`, `target` and `position`. A handler can work for any
pointer event by taking the `PointerEvent` trait.

A pointer's target is a `NormalizedRenderTarget`, which can be one of:

1. Windows
2. Images
3. GPU Texture Views

Bevy's `DefaultPlugins` includes the `DefaultPickingPlugins` if you have the
`bevy_picking` feature enabled which contains:


|Plugin|Description|
|------|-----------|
|`InteractionPlugin`|Generates pointer events and handles event bubbling|
|`PickingPlugin`|Sets up the core picking infrastructure and `PickingSettings`|
|`PointerInputPlugin`|Mouse and touch events for picking pointers|


Picking can be disabled for individual entities by adding `Pickable::IGNORE`.

Sprites and UI elements have a picking backend already enabled. Sprites still
need a `Pickable` component to opt in, while UI elements are pickable by
default. To enable picking meshes we need to add the `MeshPickingPlugin`.

Within a single frame, the order of picking events is:

1. `PointerOut` -> `PointerLeave` -> `PointerDragLeave`
2. `PointerDragEnter` -> `PointerEnter` -> `PointerOver`

Then any of the following can happen in any order:

- For each movement: `PointerDragStart` -> `PointerDrag` -> `PointerDragOver` ->
  `PointerMove`
- For each button press: `PointerPress` or `PointerClick` -> `PointerRelease` ->
  `PointerDragDrop` -> `PointerDragEnd` -> `PointerDragLeave`
- For each pointer cancellation: `PointerCancel`

Between frames things are managed by the interaction state machine:

- When a pointer moves over the target: `PointerOver` -> `PointerEnter` ->
  `PointerMove` -> `PointerLeave` -> `PointerOut`
- When a pointer presses buttons on the target: `PointerPress` -> `PointerClick`
  -> `PointerRelease`
- When a pointer drags the target: `PointerDragStart` -> `PointerDrag` ->
  `PointerDragEnd`
- When a pointer drags something over the target: `PointerDragEnter` ->
  `PointerDragOver` -> `PointerDragDrop` -> `PointerDragLeave`
- When a pointer is canceled: No other events will follow the `PointerCancel`
  event for that pointer.

## Timers

Timers come in two modes:

1. `TimerMode::Once` which stays finished after reaching the end until it is
   reset manually
2. `TimerMode::Repeating` which resets itself automatically when it reaches the
   end

In Bevy, timers don't tick down from their initial value. Instead they tick up
from zero until they reach their `Duration`.

We can then call `is_finished` or `just_finished` to switch behavior when they
are done. For a `TimerMode::Once` timer `is_finished` stays `true` after the
tick that finished it, while `just_finished` is only `true` on that tick. For a
`TimerMode::Repeating` timer the two are equivalent. A repeating timer can
finish more than once in a single tick, so use `times_finished_this_tick` to
know how many times it finished.

```rust
#[derive(Resource)]
pub struct MatchTime(Timer);

impl MatchTime {
  pub fn new() -> Self {
    Self(Timer::from_seconds(60.0, TimerMode::Once))
  }
}

impl Default for MatchTime {
  fn default() -> Self {
    Self::new()
  }
}

fn countdown(time: Res<Time>, mut match_time: ResMut<MatchTime>) {
  match_time.0.tick(time.delta());
}

fn end_match(match_time: Res<MatchTime>) {
  if match_time.0.is_finished() {
    // Here we would reset our game
  }
}
```

Implement `Default` by hand so that `init_resource` builds a real duration.
A derived `Default` would create a zero-length timer because that is what
`Timer::default` returns.

Bevy has a built in `Time` resource we can use to get the `delta` between this
tick and the last. The delta is a `Duration` that we can pass straight to
`tick`. If you need the seconds use `delta_secs` or `delta_secs_f64`.

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
      // Add a cooldown component to the player entity
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
/// `Local` values are initialized from `Default`, and a default `Timer` has a
/// zero-length duration. This wrapper builds a real timer on first access.
struct LocalTimer(Timer);

impl FromWorld for LocalTimer {
  fn from_world(_world: &mut World) -> Self {
    Self(Timer::from_seconds(5.0, TimerMode::Once))
  }
}

fn local_timer(time: Res<Time>, mut timer: Local<LocalTimer>) {
  timer.0.tick(time.delta());

  if timer.0.just_finished() {
    info!("The timer is finished");
  }
}
```

`Local` values are initialized from `Default`, and a default `Timer` has a
zero-length duration, so wrap it in a type that builds a real timer through
`FromWorld`.

Handy `Timer` methods: `pause`, `unpause`, `reset`, `finish`, `duration`,
`set_duration`, `elapsed`, `remaining`, `fraction`, and `fraction_remaining`.

For running a system on an interval, prefer the run conditions `on_timer`,
`on_real_timer`, `once_after_delay`, and `repeating_after_delay`.

`Res<Time>` is `Time<Virtual>` (affected by pause/speed). Use `Time<Real>` for
unscaled time and `Time<Fixed>` in `FixedUpdate`.

## Audio

`AudioSource` holds the audio data and is connected to an `AudioSink` which is
usually done by spawning an `AudioPlayer`.

The data must be one of the file formats supported by Bevy:

- `wav`
- `ogg` (and the related `oga` and `spx`)
- `flac`
- `mp3`

A sink is a destination for the sound data. This is the place where sources will
send their data and will be emitted to the global listener.

```rust
use bevy::prelude::*;
use std::time::Duration;

fn play_pitch(mut pitch_assets: ResMut<Assets<Pitch>>, mut commands: Commands) {
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
|`PlaybackSettings::REMOVE`|Will play the audio once then remove the audio components from the entity|

We can trigger our sounds to play by spawning an `AudioPlayer` on any entity.

```rust
fn play_background_audio(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
) {
  let audio = asset_server.load("background_audio.ogg");

  // Create an entity dedicated to playing our background music
  commands.spawn((AudioPlayer::new(audio), PlaybackSettings::LOOP));
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
|`volume`|Get the current volume|
|`set_volume`|Set the volume|
|`mute`|Mute the playback|
|`unmute`|Unmute the playback|
|`toggle_playback`|Toggle the playback|
|`toggle_mute`|Toggle muting the playback|
|`is_paused`|Returns true if the sink is paused|
|`is_muted`|Returns true if the sink is muted|
|`speed`|Get the speed of the sound|
|`set_speed`|Control the speed of the playback|
|`position`|Get the current playback position|
|`empty`|Returns true if the sink has no more sounds to play|
|`try_seek`|Seek to a certain point in the source sound|

These methods come from the `AudioSinkPlayback` trait, which is implemented by
both `AudioSink` and `SpatialAudioSink`.

There are two separate sources of volume for our apps:

1. Global volume
2. Audio sink volume

To change the global volume we modify the `GlobalVolume` resource:

```rust
use bevy::{
  audio::{GlobalVolume, Volume},
  prelude::*,
};

fn change_global_volume(mut volume: ResMut<GlobalVolume>) {
  volume.volume = Volume::Linear(0.5);
}
```

Changing `GlobalVolume` does __not__ affect audio that is already playing; the
new value is only applied when a sink is created. The same is true for changing
`PlaybackSettings` on a playing entity.

By default sounds are flat and unaffected by position. Set `spatial: true` on
`PlaybackSettings` (and give the entity a `Transform`) to make a sound spatial,
then use a `SpatialListener` component to control how it is heard. Only a single
`SpatialListener` should exist at a time.

## Scenes

Bevy ships two scene systems: the in-code Bevy Scene Notation (BSN) and
file-based world serialization. This section covers world serialization; BSN is
covered further down.

The older world serialization format stores whole worlds in a file that is
reinitialized when it is loaded. Worlds are saved into a `.scn` or `.scn.ron`
file. The format of the file is based on
[Rusty Object Notation (RON)](https://crates.io/crates/ron).

We save worlds to a file by using the `DynamicWorld::serialize` method:

```rust
fn save_scene_system(world: &mut World) {
  let dynamic_world = DynamicWorld::from_world(world);

  // Dynamic worlds can be serialized like this:
  let type_registry = world.resource::<AppTypeRegistry>();
  let type_registry = type_registry.read();
  let serialized_world = dynamic_world.serialize(&type_registry).unwrap();

  // Showing the serialized world in the console
  info!("{}", serialized_world);

  // Writing the world to a new file. Using a task to avoid calling the
  // filesystem APIs in a system as they are blocking This can't work in WASM as
  // there is no filesystem access
  #[cfg(not(target_arch = "wasm32"))]
  IoTaskPool::get()
    .spawn(async move {
      // Write the world RON data to file
      File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
        .and_then(|mut file| file.write(serialized_world.as_bytes()))
        .expect("Error while writing world to file");
    })
    .detach();
}
```
When Bevy loads the world file, it needs to deserialize it into actual
components and entities that it loads into your world.

There are two ways to instantiate a `DynamicWorld`:

1. Using the `WorldInstanceSpawner` resource with `spawn_dynamic` (deferred),
   `spawn_dynamic_sync` (immediate), or `spawn_dynamic_as_child`
2. Adding the `DynamicWorldRoot` component to an entity, which spawns the
   world's entities as children of that entity

The easiest of these is simply spawning a `DynamicWorldRoot`. It uses the
`WorldAssetLoader` to deserialize everything:

```rust
const SCENE_FILE_PATH: &str = "scene.scn.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  // Spawning a `DynamicWorldRoot` creates a new entity and spawns instances
  // of the world's entities as children of that entity.
  commands.spawn(DynamicWorldRoot(asset_server.load(SCENE_FILE_PATH)));
}
```

Once the world has loaded, a `WorldInstance` component is added to the entity
holding the scene root. It dereferences to the `InstanceId` of the spawned
world, which can be passed to methods such as
`WorldInstanceSpawner::despawn_instance_sync` to interact with it.

For example we could despawn all our loaded scenes:

```rust
fn despawn_scene(
  trigger: On<bevy::world_serialization::WorldInstanceReady>,
  mut spawner: ResMut<WorldInstanceSpawner>,
  world: &mut World,
) {
  spawner.despawn_instance_sync(world, &trigger.instance_id);
}
```
When a component is deserialized, Bevy constructs it by trying, in order:

1. Reflected `FromReflect` (generated by `#[derive(Reflect)]`)
2. Reflected `Default` plus `apply`
3. Finally reflected `FromWorld`

`FromWorld` is a fallback that lets you customize initialization using the
current world's resources, and it only participates if you add
`#[reflect(FromWorld)]` to the type:

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

## Bevy Scene Notation (BSN)

To spawn scenes inline it's best to use the `bsn!` macro

```rust
#[derive(Component, Clone, Default)]
struct Ship;

#[derive(Component, Clone, Default)]
struct Player {
  score: usize,
}

fn spawn_scene(mut commands: Commands) {
  commands.spawn_scene(bsn! {
      Player
      Ship
  });
}
```
We can make our own functions that return scenes:

```rust
fn button() -> impl Scene {
  bsn! {
      Button
      Node { width: px(100) }
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, my_button.spawn())
    .run();
}
```
Field values can be arbitrary Rust expressions:

```rust
fn increment_score(current_points: usize) -> impl Scene {
  bsn! {
      Player { score: {current_points + 10} }
  }
}
```
A `bsn!` expression implements the `Scene` trait and describes a single root
entity, while a `bsn_list!` (or a scene list inside `Children [...]`) implements
`SceneList` where each entry becomes its own entity.

Any kind of scene reference must be prefixed with `@`: `@my_scene_variable`,
`@my_scene_function()` or `@{some_scene_expression}`. The `@` tells BSN we mean a
scene rather than a component; a plain component value or function is written
without it.

Enums used as components work directly as long as they implement `Default +
Clone`. BSN does not patch individual variant fields, so every field of the
variant we use must be specified:

```rust
bsn! {
  Foo::A { x: 1, y: 0 }
}
```

Two scenes that define the same component are combined, even down to the various
fields:

```rust
fn button() -> impl Scene {
  bsn! {
      Button
      Node { width: px(100) }
  }
}

fn my_button() -> impl Scene {
  bsn! {
      @button()
      Node { height: px(100) }
  }
}
```
So `my_button` would create a `Node` with ___both___ a width and height of
`100px`.

Inside a scene list (`Children [...]`, `bsn_list![...]`), entities are separated
by `--`. Whitespace separates components on the same entity. For `bsn_list!`
prefer the brace form, which `rustfmt` leaves alone:

```rust
// One child with A and B
Children [ A B ]

// Two children, one with A, one with B
Children [ A -- B ]

// Preferred for bsn_list!
bsn_list! {
  A
  --
  B
}
```

We can make `SceneComponent` aggregates which must respond to `scene`:

```rust
#[derive(SceneComponent, Default, Clone)]
#[scene(CarConfig)]
struct Car {
  boost: f32,
}

impl Car {
  fn scene(props: CarConfig) -> impl Scene {
    let wheels: Box<dyn Scene> = match props.wheels {
      WheelSize::Standard => Box::new(bsn! { SlimWheels }),
      WheelSize::Wide => Box::new(bsn! { WideWheels }),
    };

    bsn! {
      Transform { translation: Vec3 { x: 10. } }
      @wheels
      Children [
        FrontWheel
        --
        BackWheel
      ]
    }
  }
}
```
Scene components are spawned with the `@` prefix:

```rust
fn spawn_car(mut commands: Commands) {
  commands.spawn_scene(bsn! {
    @Car {
      @wheels: WheelSize::Wide,
      boost: 100.,
    }
  });
}
```
Scene components can also accept "props" for dynamic behavior. A props struct is
declared on the component with `#[scene(CarConfig)]`, passed into its scene
function, and used to change what gets spawned. Props are set with the `@` prefix
to distinguish them from the component's own fields:

```rust
#[derive(Default)]
struct CarConfig {
  wheels: WheelSize,
}

#[derive(Default)]
enum WheelSize {
  #[default]
  Standard,
  Wide,
}
```

In `spawn_car` above, `@wheels` is a prop (consumed by `Car::scene`), while
`boost` is a normal field on the `Car` component.

BSN supports [relationships](/bevy/relationships)

```rust
fn spawn_scene(mut commands: Commands) {
  commands.spawn_scene(bsn! {
    Player
    Children [
      Sword
      --
      Shield
    ]
  });
}
```
It even works for custom relationships we define ourselves.

BSN supports referencing entities via a provided `Name` component by prefixing a
`#` as a shorthand.

```rust
fn player() -> impl Scene {
  bsn! {
      #Joe
      Player
  }
}
```
This lets us reference entities from the same scene by name:

```rust
#[derive(Component, FromTemplate)]
struct EmployedBy(Entity);

fn boss() -> impl Scene {
  bsn! {
      #Boss
      Children [
          #Joe EmployedBy(#Boss)
      ]
  }
}
```
This works the same way for lists:

```rust
#[derive(Component, FromTemplate)]
struct ReportsTo(Entity);

fn employees() -> impl SceneList {
  bsn_list! {
      #Joe ReportsTo(#Jane)
      --
      #Jane ReportsTo(#Joe)
  }
}
```
Entities we make through scenes can be hooked up to observe events

```rust
fn button() -> impl Scene {
  bsn! {
      Node { width: px(100), height: px(50) }
      on(|press: On<PointerPress>| {
          info!("button pressed!")
      })
  }
}
```
The `Add` event fires before children exist. `Ready` fires for each entity in a
scene once the whole scene has been spawned, bottom-up, which makes it the right
hook for logic that needs the complete hierarchy:

```rust
fn ready_button() -> impl Scene {
  bsn! {
      Node { width: px(100), height: px(50) }
      on(|ready: On<Ready>| {
          info!("the full scene is ready!")
      })
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
  commands.spawn((RigidBody::Dynamic, Collider::circle(0.5), Sensor));
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
