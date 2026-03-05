# Network Recipes

## Protocol boundaries

- Define protocol constants and endpoints near file top.
- Encapsulate encoding/decoding separately from UI event handlers.

## State separation

- Keep mutable runtime state in Rust fields or script state blocks.
- Keep render code declarative; avoid embedding protocol side effects in draw paths.

## Timers and loops

- Use periodic events for polling/stream status.
- Redraw only affected widgets after state transitions.

## Action handling

- Route button/input actions to protocol functions.
- Log transitions (connect, request, response, error) with concise status labels.

## Stability checks

- Verify one integration path at a time (single backend, single endpoint).
- Add fallback/default handling for missing responses and parse failures.
