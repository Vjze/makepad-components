use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCard = mod.widgets.ShadSurfacePanel{
        width: Fill
        height: Fit
        draw_bg.border_color: (shad_theme.color_outline_border_hover)
    }

    mod.widgets.ShadCardHeader = mod.widgets.ShadSurfaceHeader{
        spacing: 4.0
        padding: Inset{left: 20, right: 20, top: 20, bottom: 14}
    }

    mod.widgets.ShadCardTitle = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 16
    }

    mod.widgets.ShadCardDescription = mod.widgets.Label{
        width: Fill
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 11
    }

    mod.widgets.ShadCardContent = mod.widgets.ShadSurfaceContent{
        padding: Inset{left: 20, right: 20, top: 0, bottom: 20}
    }

    mod.widgets.ShadCardFooter = mod.widgets.ShadSurfaceFooter{
        align: Align{x: 0.0, y: 0.5}
    }
}
