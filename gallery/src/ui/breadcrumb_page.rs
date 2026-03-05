use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBreadcrumbPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Breadcrumb"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Displays the path to the current resource using a hierarchy of links."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        Label{
            text: "Default"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        ShadBreadcrumb{
            ShadBreadcrumbLink{ text: "Home" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbLink{ text: "Components" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbPage{ text: "Breadcrumb" }
        }

        GalleryHr{}

        Label{
            text: "Custom Separator"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        ShadBreadcrumb{
            ShadBreadcrumbLink{ text: "Home" }
            ShadBreadcrumbSeparator{ text: "/" }
            ShadBreadcrumbLink{ text: "Components" }
            ShadBreadcrumbSeparator{ text: "/" }
            ShadBreadcrumbPage{ text: "Breadcrumb" }
        }

        GalleryHr{}

        Label{
            text: "Collapsed / Ellipsis"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        ShadBreadcrumb{
            ShadBreadcrumbLink{ text: "Home" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbEllipsis{}
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbLink{ text: "Components" }
            ShadBreadcrumbSeparator{}
            ShadBreadcrumbPage{ text: "Breadcrumb" }
        }

        Label{
            text: "Preview + Source"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        breadcrumb_code_snippet := GalleryCodeSnippet{
            code: "mod.widgets.ShadBreadcrumb{\n    ShadBreadcrumbLink{ text: \"Home\" }\n    ShadBreadcrumbSeparator{}\n    ShadBreadcrumbLink{ text: \"Components\" }\n    ShadBreadcrumbSeparator{}\n    ShadBreadcrumbPage{ text: \"Breadcrumb\" }\n}"
        }
    }
}
