# Apps

We define our app in `main.rs` which will be executed when we run our binary
after compiling.

Calling `App::run` will start your loop and begin advancing your schedule using
the default run function.

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, hello_world_system)
    .run();
```

# Plugins

Plugins are added to the `App` with the `add_plugins` method.

```rust
fn plugin(app: &mut App) {
  app.add_system(some_plugin_system)
}

fn main() {
  App::new().add_plugins(plugin)
}
```

For more advanced plugins we can construct our own by implementing the `Plugin`
trait on a struct:

```rust
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, initialize_camera);
  }
}

fn initialize_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}
```

If your plugin becomes complicated enough to demand configuration you can add
fields to the struct that implements `Plugin`.

```rust
pub struct CameraPlugin {
  debug: bool,
}

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, initialize_camera);

    if self.debug {
      // Do something
    }
  }
}

fn main() {
  App::new()
    .add_plugins(CameraPlugin { debug: true })
}
```

There is also the `PluginGroup` trait which allow us to group related plugins
together and then configure them later. This can be great for writing a plugin
that others can add to their game.

```rust
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(CameraPlugin::default())
      .add(PhysicsPlugin::default())
      .add(LogicPlugin)
  }
}
```

This will let us (or anyone consuming your plugins) configure exactly how the
set of plugins runs in the context of our app:

<!-- examples/apps.rs -->
```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(
      game::GamePlugins
        .build()
        .disable::<physics::PhysicsPlugin>()
    )
    .run();
}
```

# Schedules

You define an app and add your systems like:

```rust
app.add_systems(Update, hello_world)
```

The `Update` is a `ScheduleLabel` that will map to the `Schedule` which actually
executes the `hello_world` system.

You will mostly be adding your logic to the three main schedule labels:

1. `Update` runs once every loop
2. `FixedUpdate` runs once every fixed timestep loop
3. `Startup` runs once at startup

However, there are many other `ScheduleLabel` in the `Main` schedule we can tap
into:

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

States in Bevy are any enum or struct that implements the `States` trait.

<!-- examples/app_state.rs:6 -->
```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
  #[default]
  MainMenu,
  InGame,
  Paused,
}

fn spawn_menu() {
  // Spawn a menu
}

fn play_game() {
  // Play the game
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

When you call `App::init_state<S>`:

1. Bevy will add a resource for both `State<S>` and `NextState<S>` to your app.
2. It will also add systems for handling transitioning between states.

Your `NextState<S>` is an enum that can be in one of two states:

1. `NextState::Pending(s)`: The next state has been triggered and will
   transition
2. `NextState::Unchanged`: The next state has not been triggered

We transition from one state to another by calling `NextState::set(S)` in any of
our systems.

One of those systems that got added when we called `App::init_state` was
`apply_state_transition<S>` which is scheduled to run in the `PreUpdate` stage
of your app.

This system triggers the `OnExit(PreviousState)` and `OnEnter(YourState)`
schedules once before finally transitioning to our next state. This is assuming
it has been set and is currently `NextState::Pending(S)`.

We cannot transition back to the same state we are on. So if you accidentally
set the `NextState` to the current state nothing will happen.

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

# Commands

Each `Command` represents a mutation we want to apply. They eventually execute
a function that receives exclusive `&mut World` reference that is used to make
changes.

Bevy schedules them to all run together inside one system in the order they
are added to the `CommandQueue`, which we do through the `Commands` system
parameter.

To execute a command, first we have to schedule them.

We don't manually add them to a `CommandQueue`, instead, we use a system
parameter: `Commands` which is the public API to the queue.

```rust
fn spawn_an_entity(mut commands: Commands,) {
  commands.spawn_empty();
}
```

This is the most simple command we can write. It will simply spawn an empty
`Entity` without any components.

At a high level, an `Entity` exclusively owns zero or more `Component`
instances.

Each entity can only have a single component of each type. These types can be
added or removed dynamically over the course of the entity's lifetime.

To despawn that same entity we would also use a command:

```rust
fn despawn_an_entity(
  mut commands: Commands,
  query: Query<Entity>
) {
  for entity in query.iter() {
    commands.entity(entity).despawn();
  }
}
```

Its important to note: ___neither command actually executes here___. Instead
they were queued to run the next time all your commands run together. This will
happen any time we transition to the next schedule during the `apply_deferred`
system that was added by the `DefaultPlugins`.

All changes to your world state should come from these commands, including
spawning and despawning.

```rust
#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
  // Here we are `Commands`
  commands
    .spawn_empty()
    // We are now an `EntityCommands`
    // for the entity we just spawned
    .insert(Player)
    .insert(Transform::default());
}
```

After we `spawn_empty` we actually get back an `EntityCommands` for the
particular entity we spawned.

`EntityCommands` allow us to `insert` components onto the new entity. Insert
itself will return `EntityCommands` which lets us chain these calls together.

Spawning an entity with some kind of component is so common that there is
a shorter version of this:

```rust
fn spawn_player_shorter(mut commands: Commands) {
  commands.spawn(Player).insert(Transform::default());
}
```

Tuples of components are already `Bundle` types this can be
shortened even further to:

```rust
fn spawn_player_shortest(mut commands: Commands) {
  let bundle = (Player, Transform::from_xyz(1., 1., 1.));
  commands.spawn(bundle);
}
```

Bundles used to be the preferred way to group up components, however since
`0.15` Bevy has moved to using required components. The same bundle from above
could be written instead like:

```rust
#[derive(Component)]
#[require(Transform)]
struct Player;


fn spawn_with_bundle(mut commands: Commands) {
  commands.spawn(Player);
}
```

These required components will have their defaults added if we do not override
them by providing our own. This is much less boiler plate and much easier to
keep in our heads when trying to think how to spawn components that depend on
each other.

We can write tests for our custom commands and manually `push` them to
a `CommandQueue` and finally trigger an `apply`:

```rust
use bevy::ecs::system::Command;
use bevy::prelude::*;

struct MyCommand;

impl Command for MyCommand {
  fn apply(self, world: &mut World) {
    info!("Hello, world!");
  }
}

#[cfg(test)]
mod tests {
  use bevy::ecs::world::CommandQueue;

  #[test]
  fn test_my_command() {
    let mut world = World::default();
    let mut command_queue = CommandQueue::default();

    // We could manually add our commands to the queue
    command_queue.push(MyCommand);

    // We can apply the commands to a given world:
    command_queue.apply(&mut world);
  }
}
```

# Components

We define components as normal structs, but we tell Bevy about them by using the
derive macro:

```rust
#[derive(Component)]
struct Position {
  x: i32,
  y: i32
}
```

This derive macro will add behavior to this struct at compile time and make them
available through our queries.

When we query for a component, we can ask for mutable or read-only access and
then do whatever we want with its fields to change the state of our entities.

A `Component` may be a struct but it can also be other data types like an enum
or zero sized type:

```rust
// A simple marker type
#[derive(Component)]
struct Player;

// Or an enum
#[derive(Component)]
enum Ship {
  Destroyer,
  Frigate,
  Scout
}
```

We add components to entities through our commands.

```rust
fn spawn_player(
  mut commands: Commands
) {
  commands
    .spawn_empty()
    .insert(Player)
    .insert(Ship::Destroyer)
    .insert(Position { x: 1, y: 2 });
}
```

This schedules a series of commands to run that will add a new `Entity` and then
insert each component by placing it in the arrays we mentioned in the previous
section.

Components can require other components

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
automatically add them, unless we override them. The only requirement is that
each required component implements the `Default` trait.

If we wanted to we could change the players ship by providing one ourselves:

```rust
fn spawn_player_with_required_components(
  mut commands: Commands
) {
  commands.spawn((Player, Ship::Destroyer));
}
```

# Events

Events can be sent to one (or both) of these places:

1. The event stream
2. Observers

When we send events to the event stream its stored inside an `Events<T>`
resource.

The `EventReader` will track which systems have read which events.

Events are defined just like our resources and components.

We define events as a type that derives `Event`:

```rust
#[derive(Event)]
struct PlayerKilled;

#[derive(Event)]
struct PlayerDetected(Entity);

#[derive(Event)]
struct PlayerDamaged {
  entity: Entity,
  damage: f32,
}
```

Then we add our event to our `App`, similar to how we manage our assets.

```rust
fn main() {
  App::new()
    .add_event::<PlayerKilled>();
}
```

When we `add_event`, Bevy adds a system for handling that specific type:
`Events<T>::event_update_system`.

This system runs each frame, cleaning up any unconsumed events by calling
`Events<T>::update`. If this function were not called, then our events will grow
unbounded eventually exhausting the queue.

This also means that if your events are not consumed by the next frame then
they will be cleaned up and dropped silently.

To write events to the stream we use an `EventWriter`. Any two systems that
use the same event writer type will not be run in parallel as they both use
mutable access to `Events<T>`.

```rust
fn detect_player(
  mut events: EventWriter<PlayerDetected>,
  players: Query<(Entity, &Transform), With<Player>>
) {
  for (entity, transform) in players.iter() {
    // ...
    events.send(PlayerDetected(entity));
  }
}
```

Each `EventWriter` can only write events for one type that is known during
compile time. There may be times where you don't know this type and as a work
around you can send type erased events through your `Commands`

```rust
commands.queue(|w: &mut World| {
  w.send_event(MyEvent);
});
```

We must consume our events steadily each frame or risk losing them. We can read
events from our systems with an `EventReader` that consumes events from our
buffers:

<!-- examples/events.rs -->
```rust
fn react_to_detection(
  mut events: EventReader<PlayerDetected>
) {
  for event in events.read() {
    // Do something with each event here
  }
}
```

If you have many different types of events you want handled the same way you can
use a generic system and an `Events` resource:

<!-- examples/events.rs -->
```rust
fn handle_event<T: Event>(
  mut events: ResMut<Events<T>>
) {
  // We can clear events this frame
  events.clear();

  // Or clear events next frame (bevy default)
  events.update();

  // Or consume our events right here and now
  for event in events.drain() {
    // ...
  }
}

fn main() {
  App::new()
    .init_resource::<Events<PlayerKilled>>()
    .add_systems(Update, handle_event::<PlayerKilled>)
    .run();
}
```

An `Observer` is a system that listens for a `Trigger`. Each trigger is for
a specific event type.

It is important to note that when you send events using an `EventWriter`, they
do not automatically trigger our observers. We have to trigger them manually,
usually using `Commands`:

```rust
commands.trigger(SomeEvent)
```

This is not the same as writing an event to an event stream like with
`EventWriter`. Instead, these events are sent directly to the observer and
handled immediately. Not sent to your `Events<T>` collection.

Bevy has some built in triggers that we can use to hook into:

|Type|Description|
|----|-----------|
|`OnAdd`|Triggers when an entity is added|
|`OnInsert`|Triggers when an entity is inserted|
|`OnRemove`|Triggers when an entity is despawned|

To create an observer, we can add it to our `App` definition:

```rust
#[derive(Component, Debug)]
struct Position(Vec2);

#[derive(Component)]
struct Enemy;

fn on_respawn(
  trigger: Trigger<OnAdd, Enemy>,
  query: Query<(&Enemy, &Position)>,
) {
  let (enemy, position) = query.get(trigger.entity()).unwrap();
  println!("Enemy was respawned at {:?}", position);
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .observe(on_respawn);
}
```

The first generic argument of `Trigger` is the event, the second is optional and
you can think of like an argument to the event type. Most observers you create
will only take one.

If we want more control, we can choose to react to a type of event and only
call our system with a specific entity:

```rust
#[derive(Component)]
struct Boss;

#[derive(Event)]
struct BossSpawned;

fn on_boss_spawned(
  trigger: Trigger<BossSpawned>,
  query: Query<(&Enemy, &Position)>,
) {
  let (enemy, position) = query.get(trigger.entity()).unwrap();
  println!("Boss was spawned at {:?}", position);
}

fn spawn_boss(
  mut commands: Commands,
) {
  commands.spawn((Enemy, Boss)).add_observer(on_boss_spawned);
  commands.trigger(BossSpawned);
}
```

Observers added this way are actually created as an `EntityObserver` which will
use component hooks to only send our system specific entities.

# Queries

Queries are a declarative way of specifying what `Component` data we want in
our systems. They fetch components from your game `World`, according to their
specification, only when you iterate over them.

The `Query<Q, F>` system parameter lets us specify the data we want from each entity
using the two generic parameters:

1. The world query
2. The filter

When we request `&T` we are asking for a readonly reference to the data:

```rust
Query<&Ball>
```

This is good because then Bevy can give readonly references to many systems that
it will try and run in parallel to give us better performance.

That means we cannot change any of the values of the component unless we
ask for it to be `&mut T`:

```rust
Query<&mut Ball>
```

Each generic parameter in `Query<Q, F>` can itself be a tuple.

```rust
Query<(&Ball, &Player)>
```

When a generic parameter is a tuple then ___all___ the types in that tuple must
be satisfied by that query. So the above example is like saying:

> Fetch me all ball and player components from every entity with both

It works similarly with filters, ensuring that ___all___ conditions are met.

The first parameter is your `Query<Q, F>` is your `QueryFetch` which tells Bevy
exactly what data you want from your `World`.

```rust
#[derive(Component, Debug)]
struct Player;

fn fetch_players(query: Query<&Player>) {
  for player in &query {
    info!("Player: {:?}", player);
  }
}
```

This query is equivalent to saying:

> Fetch me the player component from each entity that has one

But a tuple will change the meaning:

```rust
fn fetch_players_with_rocket(
  query: Query<(&Player, &Rocket)>
) {
  for (player, rocket) in &query {
    info!("Player: {:?}", player);
    info!("Rocket: {:?}", rocket);
  }
}
```

Now we are saying:

> Fetch me every player and rocket from all the entities that have both player
> and rocket components

Simple tuple combinations won't be enough to specify all the complicated queries
we want to. There are convenient types that make expressing more complicated
queries easy:

|parameter|description|
|---------|-----------|
|`Option<T>`|a component but only if it exists, otherwise `None`|
|`AnyOf<T>`|fetches entities with any of the components in type T|
|`Ref<T>`|shared borrow of an entity's component `T` with access to change detection|
|`Entity`|returns the entity|

Lets say we wanted to ask our game world to:

> Fetch me all players ___or___ astroids for all entities that have either one

We can pass a tuple, but those tuples represent ___and___ operations. To
express this kind of ___or___ logic we can wrap each argument in an `Option`:

<!-- examples/queries.rs -->
```rust
fn fetch_players_or_astroids(
  query: Query<(Option<&Player>, Option<&Astroid>)>,
) {
  for (player, astroid) in &query {
    if let Some(player) = player {
      info!("Player: {:?}", player);
    }

    if let Some(astroid) = astroid {
      info!("Astroid: {:?}", astroid);
    }
  }
}
```

```rust
Query<AnyOf<(&Player, &Rocket, &mut Astroid)>>
```

Here we are saying:

> Find me an optional player, rocket, or astroid from entities that have any of
> these components

That means that expanding it would be equivalent to:

```rust
Query<(
    Option<&Player>,
    Option<&Rocket>,
    Option<&mut Astroid>
  ),
  Or<(
    With<Player>,
    With<Rocket>,
    With<Astroid>
  )>>
```

Each type is returned as an `Option<T>` as the entities will have any of the
types we specified.

If we wanted to know about how an entity has changed we can use the `Ref<T>`
parameter:

<!-- examples/queries.rs -->
```rust
fn react_to_player_spawning(query: Query<Ref<Player>>) {
  for player in &query {
    if player.is_added() {
      // Do something
    }
  }
}
```

These borrows are immutable and don't require unique access. It would be most
equivalent to a `Query<&Player>` but we get some additional change tracking
methods:

|method|description|
|------|-----------|
|`is_added`|returns true if this value was added after the system ran|
|`is_changed`|returns true if the value was added or mutably dereferenced either since the last time the system ran or, if the system never ran, since the beginning of the program|
|`last_changed`|returns the change tick recording the time this data was most recently changed|

We can, as part of our queries, request the `Entity`. You can imagine an
`Entity` as just a unique ID.

<!-- examples/queries.rs -->
```rust
fn fetch_entities(
  query: Query<Entity>
) {
  // ...
}
```

The `Entity` on its own isn't very useful. But once we have this ID we can use
certain methods on our queries to get components from **that** entity rather
than all entities:

<!-- examples/queries.rs -->
```rust
fn fetch_rocket_by_player_entity(
  players: Query<Entity, With<Player>>,
  query: Query<&Rocket>
) {
  for player in &players {
    let rocket = query.get(player).unwrap();
  }
}
```

The second argument in your `Query<Q, F>` is the `QueryFilter`. These filters
are wrapped by a condition type:

|method|description|
|------|-----------|
|`With<T>`|only items with a `T` component|
|`Without<T>`|only items without a `T` component|
|`Or<F>`|checks if all filters in the tuple `F` apply|
|`Changed<T>`|only components of type `T` that were changed this tick|
|`Added<T>`|only components of type `T` that were added this tick|

When you use a filter type for the change trackers like:

```rust
Query<Player, Added<Player>>
```

It is basically the same thing as using a `Ref<T>` and accessing the change
trackers directly:

<!-- examples/queries.rs -->
```rust
fn react_to_player_spawning(
  query: Query<Ref<Player>>
) {
  for player in &query {
    if player.is_added() {
        // Do something
    }
  }
}
```

In terms of performance these would be equivalent.

When querying for two mutable types that can contain the same components we 
must use `Without` to disjoint the set and follow Rust's borrowing rules:

<!-- examples/queries.rs -->
```rust
fn fetch_players_and_rockets(
  players: Query<&mut Player, With<Rocket>>,
  rockets: Query<&mut Player, With<Invincibility>>
) {
  // This will panic at runtime
}
```

Otherwise it would be too ambiguous whether there can exist entities which have
both `Rocket` and `Invincibility`. This would lead to duplicated borrows on the
same components.

An alternative would be to wrap the two queries into a `ParamSet`:

<!-- examples/queries.rs -->
```rust
fn fetch_with_param_set(
  query: ParamSet<(
    Query<&mut Player, With<Rocket>>,
    Query<&mut Player, With<Invincibility>>
  )>
) {
  // This will not panic at runtime
}
```

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

If we know there is **only** a single entity in a query we can use
`single`/`single_mut`:

<!-- examples/queries.rs -->
```rust
fn move_player(
  mut query: Query<&mut Transform>
) {
  let mut transform = query.single_mut();
  transform.translation.x += 1.;
}
```

However, this method would panic if there was more than one entity
containing a `Transform` component.

In cases where you are not 100% sure there will only be a single entity you
should prefer the safer access versions `get_single` which returns a `Result`
instead:

<!-- examples/queries.rs -->
```rust
fn move_player_safely(
  mut query: Query<&mut Transform>
) {
  if let Ok(mut transform) = query.get_single_mut() {
    transform.translation.x += 1.
  }
}
```

In situations where we have a particular `Entity` (which is basically an ID),
we can use `get` or `get_mut`.

A good time to use this can be when we store a resource (or anything else)
and that component is guaranteed to be available throughout our game.

<!-- examples/queries.rs -->
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

All queries over many components return an `Iterator` which will yield a tuple
of components according to the `Q` generic of our `Query`.

The most common way to provide components to our systems is to use the `iter`
method to enumerate each component that exists:

<!-- examples/queries.rs -->
```rust
fn move_players(
  mut query: Query<&mut Transform>
) {
  // We can enumerate all matches
  for mut transform in query.iter_mut() {
    transform.translation.x += 1.;
  }
}
```

Because the `QueryState` is iterable itself we can shorten the above to
enumerating the query directly, instead of calling `iter`:

<!-- examples/queries.rs -->
```rust
fn move_players_shorthand(
  mut query: Query<&mut Transform>
) {
  // We can enumerate all matches
  for mut transform in &mut query {
    transform.translation.x += 1.;
  }
}
```

When we want to enumerate two sets of components, zip them up together, and
enumerate the tuples of all the combinations, we can use `iter_combinations`:

<!-- examples/queries.rs -->
```rust
#[derive(Component)]
struct Steering(Vec2);

#[derive(Component)]
struct Avoid;

const AVOID_DISTANCE: f32 = 100.;
const AVOIDANCE_FORCE: f32 = 0.1;

fn ship_avoidance_system(
  mut query: Query<(&mut Steering, &Transform), With<Avoid>>
) {
  let mut iter = query.iter_combinations_mut();

  while let Some([
    (mut steering_a, transform_a),
    (mut steering_b, transform_b)
  ]) = iter.fetch_next() {
    let a_to_b = transform_b.translation - transform_a.translation;
    let distance = a_to_b.length_squared();

    if distance < AVOID_DISTANCE {
      // Steer the two ships away from each other
      steering_a.0 += AVOIDANCE_FORCE;
      steering_b.0 -= AVOIDANCE_FORCE;
    }
  }
}
```

The combination's that are yielded by the iterator are not guaranteed to have
any particular order.

If we wanted combinations of 3 or more we can tweak the `K` parameter of the
function:

<!-- examples/queries.rs -->
```rust
fn every_three_transforms(
  mut query: Query<&mut Transform>
) {
  // Set our `K` parameter on the function to 3
  let mut combinations = query.iter_combinations_mut::<3>();

  // Now we get all combinations of 3 items returned
  while let Some([
    transform_a,
    transform_b,
    transform_c
  ]) = combinations.fetch_next() {
    // mutably access components data
  }
}
```

If we only need readonly access we can use the syntax without querying the iter
over and over again:

<!-- examples/queries.rs -->
```rust
fn readonly_combinations(query: Query<&Transform>) {
  for [
    transform_a,
    transform_b
  ] in query.iter_combinations() {
    // ...
  }
}
```

In cases where we have a list of `Entity` and we want to iterate over only those
entity components we can use `iter_many`.

<!-- examples/queries.rs -->
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

# Systems

When systems are added to our `App` they are added to a particular `Schedule`.

These schedules contain the rules of when each system should run over the course
of each frame.

By default, Bevy is trying to schedule all systems that don't need mutable
access to the same data to run in parallel. This is all in an effort to speed up
our game.

To schedule a system we call `add_systems` and specify the schedule and the
system(s) we want to run:

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, hello_world);
}
```

Each call to `add_systems` can take a tuple of systems:

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, (defend, attack));
}
```

Or, if we need fine grain control over ordering, we can use Bevy's built-in
methods like `before` and `after`:

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, (defend, attack.after(defend)));
}
```

By default systems run in parallel with each other and their order is
non-deterministic.

Normal systems cannot safely access the World instance directly because they run
in parallel. Our `World` contains all of our components, so mutating arbitrary
parts of it in parallel is not thread safe.

Ordering can be controlled with:

- The core sets like `Update`, `PostUpdate`, `FixedUpdate`, etc..
- by calling the `.before(this_system)` or `.after(that_system)` methods when 
  adding them to your schedule
- by adding them to a `SystemSet`, and then using `.configure_set(ThisSet.before(ThatSet))` 
  syntax to configure many systems at once
- through the use of `.add_systems(Update, (system_a, system_b, system_c).chain())`
- by calling `.in_schedule`
- by calling `.on_startup`

Some common `SystemParam` are:

|System parameter|Description|
|----------------|-----------|
|`Res`|A reference to a resource|
|`ResMut`|A mutable reference to a resource|
|`Local`|A local system variable that persists between invocations of the system|
|`Deferred`|A param that stores a buffer which gets applied to a `World` during an `apply_system_buffers` call|
|`NonSend/NonSendMut`|A shared borrow of a non `Send` resource, systems taking these are forced onto the main thread to avoid sending these resources between threads|
|`SystemChangeTick`|Reads the previous and current change ticks containing a `last_run` and `this_run` each holding a `Tick` which can be used to check the time the system has been run at.|
|`Query`|A query for resources, components or entities|
|`Commands`|The main interface for scheduling commands to run|
|`EventReader`|An interface for reading events of a particular type|
|`EventWriter`|An interface for writing events of a particular type|
|`&World`|A reference to the current `World`|
|`Archetypes`|Metadata about archetypes|
|`Bundles`|Metadata about bundles|
|`Components`|Metadata about components|
|`Entities`|Metadata about entities|

A `ParamSet` is a collection of potentially conflicting `SystemParam`'s.
It allows systems to safely access and interact with up to 8 mutually exclusive
params. For example: two queries that reference the same mutable data or
an event reader and writer of the same type.

We can access the params of a `ParamSet` with `p0`, `p1`, etc according to the
order they were defined in the type.

A `ParamSet` can take any `SystemParam`.

`ParamSet` can be used when mutably accessing the same component twice in one
system:

<!-- examples/systems.rs -->
```rust
// This will panic at runtime when the system gets initialized.
fn bad_system(
  mut enemies: Query<&mut Health, With<Enemy>>,
  mut allies: Query<&mut Health, With<Ally>>,
) {
  // ...
}
```

Instead `ParamSet` leverages the borrow checker to ensure that only one of 
the contained parameters are accessed at a given time.

```rust
fn good_system(
  mut set: ParamSet<(
    Query<&mut Health, With<Enemy>>,
    Query<&mut Health, With<Ally>>,
  )>,
) {
  // This will access the first `SystemParam`.
  for mut health in set.p0().iter_mut() {
    // Do your fancy stuff here...
  }
  // The second `SystemParam`.
  // This would fail to compile if the previous parameter was still borrowed.
  for mut health in set.p1().iter_mut() {
    // Do even fancier stuff here...
  }
}
```

Systems can be locally stateful by using the `Local<T>` system parameter:

```rust
fn print_at_end_round(mut counter: Local<u32>) {
  *counter += 1;
  println!("In set 'Last' for the {}th time", *counter);
  // Print an empty line between rounds
  println!();
}
```

The local `counter` variable will keep its state between invocations of the
function.

Higher order systems can even be composed of many other systems using 
the `pipe` method:

This should be used in combination with `ParamSet` to avoid `SystemParam`
collisions.

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(
      Update,
      (
        parse_message_system.pipe(handler_system),
        data_pipe_system.pipe(info),
        parse_message_system.pipe(debug),
        warning_pipe_system.pipe(warn),
        parse_error_message_system.pipe(error),
        parse_message_system.pipe(ignore),
      ),
    );
}
```

Usually we would schedule our commands with the `Command` system parameter so
they can be executed later in a system that has exclusive access to our
`World`.

However, if we use the `World` system parameter then we can manipulate the state
directly. For example we can run commands exactly when the system runs, instead
of scheduling them to run later:

```rust
fn spawn_immediately(world: &mut World) {
  world.spawn(Player);
}
```

The downside of exclusive systems is that they cannot be run in parallel if
other systems also need to mutate the world. Otherwise they work exactly the
same as your normal systems.

# Assets

In Bevy your assets are managed through two key resources:
- `Assets<T>` which is what stores the loaded assets of each type
- `AssetServer` which loads your assets from files asynchronously

To add our assets from the file system, we tell our `AssetServer` to `load` them
for us. This will return a handle to the loading asset.

A handle is like a pointer but cooler. We attach these handles to some kind
of component and Bevy's rendering systems will render them onto the screen.

```rust
#[derive(Component)]
struct Car;

fn spawn_ambulance(mut commands: Commands, asset_server: Res<AssetServer>) {
  let model = asset_server.load("models/ambulance.glb#Scene0");

  commands.spawn((
    Car,
    Transform::from_xyz(0.0, 0.0, 0.0),
    SceneRoot(model)
  ));
}
```

For convenience, if you need to pass around these assets often, you can store
these handles inside a resource.

<!-- examples/assets.rs -->
```rust
#[derive(Resource)]
struct CarAssets {
  body: Option<Handle<Mesh>>,
}

fn load_car_body(
  asset_server: Res<AssetServer>,
  mut car: ResMut<CarAssets>
) {
  let car_handle = asset_server.load("models/cars/basic.gltf#Mesh0/Primitive0");
  car.body = Some(car_handle);
}
```

Which makes them much easier to access in some other system we can access this
resource.

<!-- examples/assets.rs -->
```rust
fn spawn_car(car: Res<CarAssets>, mut commands: Commands) {
  if let Some(body) = car.body.as_ref() {
    commands.spawn(Mesh3d(body.clone()));
  }
}
```

Often during prototype you will want to create simple assets procedurally. There
are many shape primitives you can use for meshes as well as basic colors for
your materials.

<!-- examples/assets.rs -->
```rust
fn spawn_ball(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let circle = Circle::new(BALL_SIZE);
  let color = Color::BLACK;

  // `Assets::add` will load these into memory and return a `Handle` (an ID)
  // to these assets. When all references to this `Handle` are cleaned up
  // the asset is cleaned up.
  let mesh = meshes.add(circle);
  let material = materials.add(color);

  commands.spawn((
    Mesh2d(mesh),
    MeshMaterial2d(material)
  ));
}
```

Your assets fire certain events when they are created, modified or removed:

- `AssetEvent::Created`
- `AssetEvent::LoadedWithDependencies`
- `AssetEvent::Modified`
- `AssetEvent::Removed`
- `AssetEvent::Unused`


This lets us react to changes to our assets:

<!-- examples/assets.rs:115 -->
```rust
use bevy::asset::AssetEvent;

fn react_to_images(mut events: EventReader<AssetEvent<Image>>) {
  for event in events.read() {
    match event {
      AssetEvent::Added { id } => {
        // React to the image being created
      }
      AssetEvent::LoadedWithDependencies { id } => {
        // React to the image being modified
      }
      AssetEvent::Modified { id } => {
        // React to the image being modified
      }
      AssetEvent::Removed { id } => {
        // React to the image being removed
      }
      AssetEvent::Unused { id } => {
        // React to the last strong handle for the asset being dropped
      }
    }
  }
}
```

The `AssetServer` is a resource that uses the file system to load assets in the
asynchronously in the background. Its responsible for tracking the loading state
of assets it manages.

<!-- examples/assets.rs -->
```rust
fn spawn_boid(mut commands: Commands, asset_server: Res<AssetServer>) {
  let sprite = Sprite {
    // The asset server will return a `Handle<Image>`
    // but that does not mean the asset has
    // been fully loaded yet.
    image: asset_server.load("images/bevy.png"),
    ..default()
  };
  // Spawns the bevy logo in the center of the screen
  commands.spawn((sprite, Transform::default()));
}
```

Assets are loaded async. This means that when we first spawn this `Sprite`
even though the asset server gave us back a `Handle<Image>`, the actual asset
might not yet be available.

We can use `AssetServer::get_load_state` to check if a asset
has been loaded and is ready to use in the `Assets` collection.

First we can load our image from a file using the asset server.

<!-- examples/assets.rs -->
```rust
#[derive(Resource)]
struct BevyImage(Handle<Image>);

fn load_sprites(
  mut bevy_image: ResMut<BevyImage>,
  asset_server: Res<AssetServer>,
) {
  bevy_image.0 = asset_server.load("images/bevy.png");
}
```

Here we load the image and store the `Handle<Image>` in a resource. Then in
another system we can query the loading state of that asset and only spawn our
entity when we know the asset is loaded.

<!-- examples/assets.rs -->
```rust
use bevy::asset::LoadState;

fn on_asset_event(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  bevy_image: Res<BevyImage>,
) {
  match asset_server.get_load_state(&bevy_image.0) {
    Some(LoadState::NotLoaded) => {}
    Some(LoadState::Loading) => {}
    Some(LoadState::Loaded) => {
      commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Sprite {
          image: bevy_image.0.clone(),
          ..default()
        },
      ));
    }
    Some(LoadState::Failed(_)) => {}
    None => {}
  }
}
```

If we have a great number of assets we can make things easier by loading all of
them in a single system:

<!-- examples/assets.rs:17 -->
```rust
use bevy::asset::LoadedFolder;

fn load_models(asset_server: Res<AssetServer>) {
  // You can load all assets in a folder like this. They will be loaded in
  // parallel without blocking
  let _scenes: Handle<LoadedFolder> = asset_server.load_folder("models/monkey");
}
```
