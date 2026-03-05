---
name: makepad-networked-apps
description: Build Makepad apps with networking, runtime state, and external integrations using repository examples. Trigger when tasks involve chat backends, UDP/DMX control, async file/network workflows, gamepad/wheel input loops, or real-time UI updates tied to external systems.
---

# Makepad Networked Apps

Use this skill when the UI is coupled to network protocols, timers, external services, or hardware-like inputs.

## Workflow

1. Classify integration: chat API, local service workflow, UDP control, or input-loop telemetry.
2. Select a baseline from [references/network-routing.md](references/network-routing.md).
3. Apply [references/network-recipes.md](references/network-recipes.md).
4. Keep protocol/state logic separate from rendering sections.
5. Validate behavior with focused runs and targeted logging.

## Commands

```bash
cargo run -p makepad-example-<name> --release
cargo run -p cargo-makepad -- check script -p makepad-example-<name>
```
