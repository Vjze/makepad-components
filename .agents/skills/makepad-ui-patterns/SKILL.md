---
name: makepad-ui-patterns
description: Build or refactor Makepad v2 UI using script_mod! patterns from in-repo examples. Trigger when tasks involve app scaffolding, widget composition, Dock layouts, PortalList/FileTree, StackNavigation, TextInput behaviors, or modular script registration.
---

# Makepad UI Patterns

Use this skill to implement UI-first Makepad tasks by copying proven patterns from `examples/uizoo`, `examples/splash`, `examples/todo`, and `examples/text_input`.

## Workflow

1. Identify UI capability: app shell, lists, dock tabs, navigation, inputs, custom widget, or module split.
2. Open [references/ui-routing.md](references/ui-routing.md) and pick the best source file.
3. Apply [references/ui-recipes.md](references/ui-recipes.md) with minimal changes.
4. Keep Makepad v2 syntax strict: `script_mod!`, `Name: value`, `:=` ids, `crate_resource("self://...")`.
5. Verify by running only the touched example or target crate first.

## Commands

```bash
cargo run -p makepad-example-<name> --release
cargo run -p cargo-makepad -- check script -p makepad-example-<name>
```
