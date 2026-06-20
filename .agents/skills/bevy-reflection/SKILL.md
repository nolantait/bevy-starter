---
name: bevy-reflection
description: Reference for Bevy's reflection system — Reflect derive, dynamic field access, serialization, trait reflection, and scenes.
metadata:
  crate: bevy_reflect
  bevy: "0.19"
---

## Basics

Derive `Reflect` to enable runtime introspection, dynamic field access, and serialization:

```rust
#[derive(Reflect)]
struct Slider {
  #[reflect(@RangeInclusive::<f32>::new(0.0, 1.0))]
  value: f32,
}
```

## Registering types

```rust
app.register_type::<Slider>();
```

This adds the type to `AppTypeRegistry` (shared `TypeRegistry`).

## Dynamic field access

```rust
let mut slider = Slider { value: 0.5 };
*slider.get_field_mut("value").unwrap() = 2.0;
assert_eq!(*slider.get_field::<f32>("value").unwrap(), 2.0);
```

## Reflection subtraits

Derive macro infers: `Struct`, `TupleStruct`, `Enum`, `Tuple`, `List`, `Array`, `Map`, `Set`. Opaque types can't be broken down further.

## Foreign types that don't implement Reflect

Options:
1. Fork and derive
2. Convert to/from a reflectable type
3. Newtype wrapper
4. Reflect `Serialize`/`Deserialize` via serde:

```rust
#[derive(Reflect, Serialize, Deserialize, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
struct Trader { balance: Decimal }
```

## Serialization (RON)

```rust
fn serialize(type_registry: Res<AppTypeRegistry>) {
  let reg = type_registry.read();
  let serializer = ReflectSerializer::new(&slider, &reg);
  let ron = ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
}
```

## Trait reflection

```rust
#[reflect_trait]
trait DoThing { fn do_thing(&self) -> String; }

#[derive(Reflect)]
#[reflect(DoThing)]
struct MyType { value: String }

impl DoThing for MyType { fn do_thing(&self) -> String { format!("{} World!", self.value) } }

// Dynamic access:
let reflect_do_thing = type_registry.get_type_data::<ReflectDoThing>(reflect_value.type_id()).unwrap();
let my_trait: &dyn DoThing = reflect_do_thing.get(&*reflect_value).unwrap();
```

## Scenes

Reflection powers scene serialization. Register types with `register_type`, then `DynamicWorld::from_world(world).serialize(&type_registry)` to produce RON.
