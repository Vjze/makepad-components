use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.Accordion = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 0.0
    }

    mod.widgets.AccordionItem = FoldHeader{
        width: Fill
        height: Fit
        body_walk: Walk{width: Fill, height: Fit}

        header: View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            padding: Inset{top: 12, bottom: 12, left: 12, right: 12}
            spacing: 8.0

            title := Label{
                text: "Accordion Item"
                draw_text.color: #ddd
                draw_text.text_style.font_size: 11
            }

            View{width: Fill, height: Fit}

            fold_button := FoldButton{
                width: 16
                height: 16
            }
        }

        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
            spacing: 8.0

            body_text := Label{
                text: "Accordion content"
                draw_text.color: #b8b8b8
                draw_text.text_style.font_size: 10
            }
        }
    }
}
