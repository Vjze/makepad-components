use crate::ui::snippets::KBD_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryKbdPage = GalleryComponentPage{
        GalleryPageTitle{
            text: "Kbd"
        }

        GalleryPageSubtitle{
            text: "Keyboard shortcut key caps for displaying shortcuts (e.g. ⌘ ⇧ ⌥ ⌃ or Ctrl + B)."
        }

        GalleryHr{}

        kbd_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            kbd_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                kbd_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    kbd_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    kbd_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                kbd_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    kbd_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    kbd_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            kbd_preview_panel := mod.widgets.GalleryPreviewPanel{
                kbd_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        Label{
                            text: "Modifier keys"
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                        }

                        View{
                            width: Fit
                            height: Fit
                            flow: Right
                            spacing: 6.0
                            align: Align{y: 0.5}

                            ShadKbd{ label := ShadKbdLabel{text: "⌘"} }
                            ShadKbd{ label := ShadKbdLabel{text: "⇧"} }
                            ShadKbd{ label := ShadKbdLabel{text: "⌥"} }
                            ShadKbd{ label := ShadKbdLabel{text: "⌃"} }
                        }

                        Label{
                            text: "Shortcut"
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                        }

                        View{
                            width: Fit
                            height: Fit
                            flow: Right
                            spacing: 6.0
                            align: Align{y: 0.5}

                            ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
                            ShadKbdSeparator{}
                            ShadKbd{ label := ShadKbdLabel{text: "B"} }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(KBD_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
