use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTextarea = mod.widgets.ShadInput{
        height: 120
        is_multiline: true
        empty_text: "Type your message here."
        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
    }

    mod.widgets.ShadTextareaSm = mod.widgets.ShadTextarea{
        height: 96
    }

    mod.widgets.ShadTextareaLg = mod.widgets.ShadTextarea{
        height: 160
    }
}
