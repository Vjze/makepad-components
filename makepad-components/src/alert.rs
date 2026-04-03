use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadAlert = mod.widgets.ShadSurfaceTransparent{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.0}
        spacing: 12.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
    }

    mod.widgets.ShadAlertHeader = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.0}
        spacing: 12.0
    }

    mod.widgets.ShadAlertIcon = mod.widgets.IconInfo{
        width: 16
        height: 16
        icon_walk: Walk{width: 16, height: 16}
        draw_icon.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadAlertContent = mod.widgets.View{
        width: Fill
        height: Fit
        flow: Down
        align: Align{x: 0.0, y: 0.0}
        spacing: 6.0
    }

    mod.widgets.ShadAlertTitle = mod.widgets.ShadTextPrimary{
        width: Fill
        padding: 0.
        align: Align{x: 0.0, y: 0.0}
        draw_text.text_style.line_spacing: 1.0
    }

    mod.widgets.ShadAlertDescription = mod.widgets.ShadTextMuted{
        width: Fill
        padding: 0.
        align: Align{x: 0.0, y: 0.0}
        draw_text.text_style.line_spacing: 1.2
    }

    mod.widgets.ShadAlertDestructive = mod.widgets.ShadAlert{
        draw_bg.border_color: (shad_theme.color_destructive)
    }

    mod.widgets.ShadAlertDestructiveIcon = mod.widgets.IconX{
        width: 16
        height: 16
        icon_walk: Walk{width: 16, height: 16}
        draw_icon.color: (shad_theme.color_destructive)
    }

    mod.widgets.ShadAlertDestructiveTitle = mod.widgets.ShadTextDestructive{
        width: Fill
        padding: 0.
        align: Align{x: 0.0, y: 0.0}
        draw_text.text_style.font_size: 11
        draw_text.text_style.line_spacing: 1.0
    }

    mod.widgets.ShadAlertDestructiveDescription = mod.widgets.ShadTextDestructive{
        width: Fill
        padding: 0.
        align: Align{x: 0.0, y: 0.0}
        draw_text.text_style.line_spacing: 1.2
    }
}
