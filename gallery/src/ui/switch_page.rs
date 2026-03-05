use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use makepad_components::switch::*
    use crate::ui::themed_widgets::*

    mod.widgets.switch_page = {
        flow: Down
        spacing: 20
        padding: 20
        align: {x: 0.0, y: 0.0}

        Label {
            text: "Switch"
            draw_text: {
                text_style: {font_size: 24.0}
            }
        }

        Label {
            text: "A control that allows the user to toggle between checked and not checked."
            draw_text: {
                color: (shad_theme.color_muted_foreground)
                text_style: {font_size: 14.0}
            }
        }

        View {
            flow: Right
            spacing: 20

            View {
                flow: Down
                spacing: 15
                align: {x: 0.0, y: 0.0}

                Label {
                    text: "Default"
                    draw_text: {
                        text_style: {font_size: 16.0}
                    }
                }

                ShadSwitch {
                    label: "Airplane Mode"
                }
                ShadSwitch {
                    label: "Wi-Fi"
                    checked: true
                }
                ShadSwitch {
                    label: "Bluetooth"
                }
            }
        }
    }
}
