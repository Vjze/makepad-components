use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySwitchPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Switch"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Toggle between on and off states."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        Label{
            text: "Default"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadSwitch{label: "Enable notifications"}
            ShadSwitch{label: "Dark mode" checked: true}
            ShadSwitch{label: "Use cellular data"}
        }

        GalleryHr{}

        Label{
            text: "Inline with label"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 24.0
            align: Align{y: 0.5}

            ShadSwitch{label: "Email alerts" checked: true}
            ShadSwitch{label: "SMS alerts"}
        }

        GalleryHr{}

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        switch_example_snippet := GalleryCodeSnippet{
            code: "ShadSwitch{label: \"Enable notifications\"}\nShadSwitch{label: \"Dark mode\" checked: true}"
        }
    }
}

