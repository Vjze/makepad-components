use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadLabel = mod.widgets.Label {
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
    }
}
