use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCollapsiblePage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Collapsible"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Single section toggle inspired by shadcn/ui collapsible."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        ShadCollapsible{
            margin: Inset{top: 12, right: 12}
            title: "Order #4189"
            is_open: true
            body: View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 8.0

                RoundedView{
                    width: Fill
                    height: Fit
                    flow: Right
                    padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                    draw_bg +: {
                        color: #0000
                        border_size: 1.0
                        border_radius: 6.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    Label{
                        width: Fill
                        text: "Status"
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                    Label{
                        text: "Shipped"
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 10
                    }
                }

                RoundedView{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                    spacing: 4.0
                    draw_bg +: {
                        color: #0000
                        border_size: 1.0
                        border_radius: 6.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    Label{
                        text: "Shipping address"
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 10
                    }
                    Label{
                        text: "100 Market St, San Francisco"
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                }

                RoundedView{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
                    spacing: 4.0
                    draw_bg +: {
                        color: #0000
                        border_size: 1.0
                        border_radius: 6.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    Label{
                        text: "Items"
                        draw_text.color: (shad_theme.color_primary)
                        draw_text.text_style.font_size: 10
                    }
                    Label{
                        text: "2x Studio Headphones"
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                }
            }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        collapsible_example_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadCollapsible{\n    title: \"Order #4189\"\n    is_open: true\n    body: View{\n        flow: Down\n        Label{text: \"Status: Shipped\"}\n    }\n}"
        }
    }
}
