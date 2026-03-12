use crate::ui::snippets::INPUT_OTP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryInputOtpPage = ShadScrollYView{
        ShadPageTitle{
            text: "Input OTP"
        }

        ShadPageSubtitle{
            text: "Segmented one-time passcode entry with numeric filtering and paste support."
        }

        ShadHr{}

        input_otp_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            input_otp_tabs_row := View{
                width: Fit
                visible: false
                height: 0
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                input_otp_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    input_otp_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    input_otp_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                input_otp_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    input_otp_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    input_otp_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            input_otp_preview_panel := mod.widgets.ShadPanel{
                input_otp_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                    width: Fill
                    height: Fit

                    root_view +: {
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Verification code" }

                        View{
                            width: Fit
                            height: Fit
                            flow: Down
                            spacing: 8.0

                            ShadLabel{text: "Enter the 6-digit code"}
                            otp_demo := ShadInputOtp{}
                            otp_status := ShadFieldDescription{
                                text: "Waiting for input."
                            }
                        }
                    }

                    code_page +: {
                        body +: {
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code_view +: { text: #(INPUT_OTP_PREVIEW_CODE) }
                        }
                        }
                    }
                }
            }
        }
    }
}
