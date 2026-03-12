use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.draw.KeyCode
    use mod.widgets.*

    let GalleryPageStackView = StackNavigationView{
        full_screen: false
        draw_bg +: {
            color: instance(shad_theme.color_background)
        }
        header +: {
            visible: false
            height: 0
        }
        body +: {
            width: Fill
            height: Fill
            flow: Overlay
            margin: Inset{top: 0, right: 0, bottom: 0, left: 0}
        }
    }

    mod.widgets.GalleryContentFlip = StackNavigation{
        width: Fill
        height: Fill

        root_view +: {
            width: Fill
            height: Fill
            flow: Overlay
            show_bg: true
            draw_bg.color: (shad_theme.color_background)
        }

        accordion_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryAccordionPage{} }
        }
        alert_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryAlertPage{} }
        }
        aspect_ratio_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryAspectRatioPage{} }
        }
        avatar_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryAvatarPage{} }
        }
        badge_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryBadgePage{} }
        }
        breadcrumb_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryBreadcrumbPage{} }
        }
        button_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryButtonPage{} }
        }
        button_group_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryButtonGroupPage{} }
        }
        card_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryCardPage{} }
        }
        carousel_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryCarouselPage{} }
        }
        checkbox_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryCheckboxPage{} }
        }
        collapsible_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryCollapsiblePage{} }
        }
        command_palette_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryCommandPalettePage{} }
        }
        context_menu_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryContextMenuPage{} }
        }
        dialog_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryDialogPage{} }
        }
        input_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryInputPage{} }
        }
        input_otp_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryInputOtpPage{} }
        }
        radio_group_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryRadioGroupPage{} }
        }
        resizable_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryResizablePage{} }
        }
        scroll_area_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryScrollAreaPage{} }
        }
        select_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySelectPage{} }
        }
        separator_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySeparatorPage{} }
        }
        sheet_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySheetPage{} }
        }
        skeleton_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySkeletonPage{} }
        }
        switch_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySwitchPage{} }
        }
        tabs_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryTabsPage{} }
        }
        textarea_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryTextareaPage{} }
        }
        toggle_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryTogglePage{} }
        }
        kbd_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryKbdPage{} }
        }
        label_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryLabelPage{} }
        }
        progress_page := GalleryPageStackView{
            body +: { mod.widgets.GalleryProgressPage{} }
        }
        sidebar_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySidebarPage{} }
        }
        slider_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySliderPage{} }
        }
        sonner_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySonnerPage{} }
        }
        spinner_page := GalleryPageStackView{
            body +: { mod.widgets.GallerySpinnerPage{} }
        }
    }

    mod.widgets.GalleryAppUi = Root{
        main_window := Window{
            window.inner_size: vec2(1400 900)
            window.title: "Makepad Components Gallery"
            pass +: { clear_color: (shad_theme.color_background) }
            window_menu +: {
                command_palette_menu := MenuItem.Item {
                    name: "Command Palette"
                    key: KeyCode.KeyK
                    enabled: true
                }
                view_menu := MenuItem.Sub {
                    name: "View"
                    items: [@zoom_in, @zoom_out, @line9, @command_palette_menu, @fullscreen]
                }
            }
            body +: {
                width: Fill
                height: Fill
                flow: Overlay
                draw_bg.color: (shad_theme.color_background)

                app_shell := View{
                    width: Fill
                    height: Fill
                    flow: Right
                    sidebar := mod.widgets.GallerySidebar{}
                    main_content := View{
                        width: Fill
                        height: Fill
                        flow: Down

                        mobile_header := View{
                            width: Fill
                            height: Fit
                            visible: false
                            flow: Right
                            align: Align{y: 0.5}
                            padding: Inset{left: 16, right: 16, top: 12, bottom: 12}
                            spacing: 12.0
                            draw_bg.color: (shad_theme.color_background)

                            mobile_sidebar_button := ShadButtonGhost{
                                width: 36
                                height: 36
                                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                                text: "☰"
                                draw_text.text_style.font_size: 16
                            }

                            ShadLabel{
                                text: "Components"
                                draw_text.text_style.font_size: 11
                                draw_text.color: (shad_theme.color_muted_foreground)
                            }
                        }

                        content_flip := mod.widgets.GalleryContentFlip{}
                    }
                }

                command_palette := mod.widgets.GalleryCommandPalette{}
            }
        }
    }
}
