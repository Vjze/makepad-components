# Media Recipes

## Feature flags

- Enable only required `makepad-widgets` features:
- `features = ["3d"]` for `gltf`, `splat`, `charts` 3D.
- `features = ["maps"]` for map widgets.
- `features = ["pdf"]` for pdf widgets.

## Resource paths

- Use `crate_resource("self://...")` for portable in-crate paths.
- Co-locate media in `resources/` within the example/target crate.

## GLTF and Splat

- Copy loading pattern from `examples/gltf/src/app.rs` and `examples/splat/src/app.rs`.
- Keep camera/env setup stable before tuning visuals.

## Vector/SVG custom draw

- Use `examples/vector/src/app.rs` for custom widget draw loop (`DrawVector`, parse/render flow).
- Use `Event::NextFrame` redraw only when animation is required.

## Map/PDF

- Start from their dedicated examples and keep dependency features aligned with Cargo.
