# UI Recipes

## Bootstrap

- Start from `examples/counter/src/app.rs`.
- Keep `app_main!(App)`, `script_mod!`, `App::run`, `WidgetRef` fields unchanged structurally.

## Modular script registration

- Start from `examples/uizoo/src/app.rs` and `examples/uizoo/src/lib.rs`.
- Register all module `script_mod(vm)` calls before `App::from_script_mod(...)`.

## Dock tabs

- Start from `examples/uizoo/src/app.rs`.
- Define `Dock`, `DockSplitter`, `DockTabs`, `DockTab`; route `kind` to content templates.

## PortalList

- Start from `examples/uizoo/src/tab_portallist.rs` or `examples/git/src/app.rs`.
- Define row template in DSL; bind rows in custom widget draw iteration.

## FileTree

- Start from `examples/uizoo/src/demofiletree.rs` or `examples/splash/src/app.rs`.
- Maintain explicit open-state and selection handling.

## Inputs

- Start from `examples/text_input/src/app.rs`.
- Use `returned(actions)` for Enter and set per-field input modes intentionally.
