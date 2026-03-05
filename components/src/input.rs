use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadInput = mod.widgets.TextInput {
        width: Fill
        height: Fit
        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
        empty_text: "Enter text..."

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0

            color: #0000
            color_hover: #0000
            color_focus: #0000
            color_down: #0000
            color_empty: #0000
            color_disabled: (shad_theme.color_muted)

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
            border_color_down: (shad_theme.color_primary)
            border_color_empty: (shad_theme.color_outline_border)
            border_color_disabled: (shad_theme.color_outline_border)
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_empty: (shad_theme.color_muted_foreground)
            color_disabled: (shad_theme.color_muted_foreground)
        }
        draw_text.text_style.font_size: 11.0
        
        draw_cursor +: {
            color: (shad_theme.color_primary)
        }
        
        draw_selection +: {
            color: (shad_theme.color_muted)
        }
    }
}
