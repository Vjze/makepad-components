## 2024-05-24 - Accessibility improvements

**Learning:** Buttons in the Makepad component library lack `cursor: MouseCursor.Hand` by default, making them feel less interactive. Also, buttons need aria labels.
**Action:** Add `cursor: MouseCursor.Hand` to all interactive elements. Wait, buttons might have cursor by default, but I need to check.
## 2024-03-09 – [Add pointer cursor to buttons]
**Learning:** In Makepad, UI elements like buttons may lack an interactive pointer cursor by default.
**Action:** Explicitly adding `cursor: MouseCursor.Hand` to components improves discoverability and usability.

## 2025-02-17 - Missing Disabled Visual States on Button Variants
**Learning:** The `ShadButton*` variants (which wrap `ButtonFlat`) in Makepad components do not have disabled visual states by default (`color_disabled`, `border_color_disabled`, `draw_text.color_disabled`). This causes buttons to visually appear active even when their `disabled` flag is set, confusing users.
**Action:** When creating new components wrapping `ButtonFlat` or similar generic buttons in Makepad, always map the theme's muted tokens (`shad_theme.color_muted`, `shad_theme.color_muted_foreground`) to the respective `_disabled` properties to ensure accurate state communication.
