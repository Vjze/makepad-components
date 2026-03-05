use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryButtonGroupPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Button Group"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "A container that groups related actions with consistent segmented styling"
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
            width: Fit
            height: Fit
            flow: Right
            spacing: 10.0
            align: Align{y: 0.5}

            ShadButtonIcon{text: "←"}

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Archive"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Report"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Snooze"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemIcon{text: "⋯"}
            }
        }

        Label{
            text: "Sizes"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0

            ShadButtonGroup{
                ShadButtonGroupItemSm{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemSm{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemSm{text: "Month"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Month"}
            }

            ShadButtonGroup{
                ShadButtonGroupItemLg{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemLg{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemLg{text: "Month"}
            }
        }

        Label{
            text: "Toolbar"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 10.0
            align: Align{y: 0.5}

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Bold"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Italic"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Underline"}
            }

            ShadButtonGroup{
                ShadButtonGroupItemIcon{text: "A-"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemIcon{text: "A+"}
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        button_group_example_snippet := GalleryCodeSnippet{
            code: "View{\n    width: Fit\n    height: Fit\n    flow: Right\n    spacing: 10.0\n    ShadButtonIcon{text: \"←\"}\n    ShadButtonGroup{\n        ShadButtonGroupItem{text: \"Archive\"}\n        ShadButtonGroupSeparator{}\n        ShadButtonGroupItem{text: \"Report\"}\n    }\n    ShadButtonGroup{\n        ShadButtonGroupItem{text: \"Snooze\"}\n        ShadButtonGroupSeparator{}\n        ShadButtonGroupItemIcon{text: \"⋯\"}\n    }\n}"
        }
    }
}
