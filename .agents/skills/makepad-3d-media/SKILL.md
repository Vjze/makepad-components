---
name: makepad-3d-media
description: Implement Makepad 3D and media features using in-repo examples. Trigger when tasks involve GLTF scenes, splat rendering, chart3D views, SVG/vector drawing, image/video widgets, map rendering, PDF viewing, or related makepad-widgets feature flags.
---

# Makepad 3D Media

Use this skill for rendering-heavy features and media/resource integration.

## Workflow

1. Identify target media path: 3D model, splat, chart, vector/svg, image/video, map, or pdf.
2. Open [references/media-routing.md](references/media-routing.md) and choose the nearest example.
3. Apply [references/media-recipes.md](references/media-recipes.md), preserving feature flags and resource paths.
4. Validate the exact example crate before integrating into other modules.

## Commands

```bash
cargo run -p makepad-example-<name> --release
cargo run -p cargo-makepad -- check script -p makepad-example-<name>
```
