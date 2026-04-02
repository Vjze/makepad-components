use crate::ui::registry::gallery_page_entries;
use makepad_components::makepad_widgets::*;

macro_rules! define_gallery_sidebar {
    (
        $(
            {
                title: $title:literal,
                route: $route:literal,
                page: $page:ident,
                widget: $widget:ident,
                sidebar_id: $sidebar_id:ident,
                sidebar_label: $sidebar_label:literal,
                section: $section:literal,
                shortcut: $shortcut:literal,
                snippet: $snippet:ident,
                $(transition: $transition:ident,)?
            }
        )*
    ) => {
        script_mod! {
            use mod.prelude.widgets.*
            use mod.widgets.*

            mod.widgets.GalleryMobileSidebarMenuButton = IconButtonMenu{
                width: 36
                height: 36
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                spacing: 0.0
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                    border_color_hover: (shad_theme.color_outline_border_hover)
                    border_color_down: (shad_theme.color_outline_border_hover)
                    border_color_focus: (shad_theme.color_primary)
                }
                draw_icon.color: (shad_theme.color_primary)
            }

            mod.widgets.GalleryMobileSidebarCloseButton = IconButtonX{
                width: 36
                height: 36
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                spacing: 0.0
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                    border_color_hover: (shad_theme.color_outline_border_hover)
                    border_color_down: (shad_theme.color_outline_border_hover)
                    border_color_focus: (shad_theme.color_primary)
                }
                draw_icon.color: (shad_theme.color_primary)
            }

            mod.widgets.GallerySidebar = ShadSidebar{
                width: 280

                mobile_sidebar_close_button := mod.widgets.GalleryMobileSidebarCloseButton{
                    visible: false
                }

                ShadLabel{
                    text: "Makepad Component\nGallery"
                    draw_text.text_style.font_size: 13
                }

                ShadSidebarSectionLabel{text: "Catalog"}

                ShadScrollYView{
                    width: Fill
                    height: Fill
                    flow: Down

                    $(
                        $sidebar_id := ShadSidebarItem{text: $sidebar_label}
                    )*
                }
            }
        }
    };
}

gallery_page_entries!(define_gallery_sidebar);
