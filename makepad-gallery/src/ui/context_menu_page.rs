use crate::ui::snippets::CONTEXT_MENU_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryContextMenuPage = ShadScrollYView{
        ShadPageTitle{
            text: "Context Menu"
        }

        ShadPageSubtitle{
            text: "Right click or long press a trigger area to reveal contextual actions."
        }

        ShadHr{}

        context_menu_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            context_menu_tabs_row := View{
                width: Fit
                visible: false
                height: 0
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                context_menu_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    context_menu_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    context_menu_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                context_menu_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    context_menu_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    context_menu_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            context_menu_preview_panel := mod.widgets.ShadPanel{
                context_menu_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                    width: Fill
                    height: Fit

                    root_view +: {
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Basic" }

                        context_menu_basic := ShadContextMenu{
                            labels: ["Open" "Duplicate" "Share" "Delete"]

                            RoundedView{
                                width: 360
                                height: Fit
                                flow: Down
                                spacing: 6.0
                                padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                draw_bg +: {
                                    color: (shad_theme.color_secondary)
                                    border_size: 1.0
                                    border_radius: (shad_theme.radius)
                                    border_color: (shad_theme.color_outline_border)
                                }

                                ShadLabel{text: "Project brief.md"}
                                ShadFieldDescription{text: "Right click this card to open the menu."}
                            }
                        }

                        context_menu_status := ShadFieldDescription{
                            text: "No action selected yet."
                        }
                    }

                    code_page +: {
                        body +: {
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code_view +: { text: #(CONTEXT_MENU_PREVIEW_CODE) }
                        }
                        }
                    }
                }
            }
        }
    }
}
