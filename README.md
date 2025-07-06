# Bevy Starter

This repo is a minimal starter for Bevy `0.16`

## Inspiration

- [`bevy_space`](https://github.com/perlindgren/bevy-space)
- [`bevy_new_2d`](https://github.com/TheBevyFlock/bevy_new_2d)
- [`sobevy`](https://codeberg.org/doomy/sobevy)
- [`Mischief in miniature`](https://github.com/alice-i-cecile/mischief-in-miniature)

## Building

You can build your game

```
cargo run
```

If you want the extra dev features then you can toggle them:

```
cargo run --features dev
```

## Features

- Cargo configured according to Bevy guide with build optimizations
- [Avian](https://github.com/Jondolf/avian) physics
- Generic set of starting plugins with your games logic inside `GamePlugin`
- `AI.md` for passing to tools like [`aider`](https://aider.chat/) and others
  that helps them get more recent context from Bevy

## Missing

- Deployment
