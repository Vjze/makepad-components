use makepad_components::makepad_widgets::*;
use makepad_code_editor::code_view::CodeViewWidgetExt;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryHr = Hr{
        draw_bg.color: (shad_theme.color_outline_border)
    }

    mod.widgets.GalleryCheckBox = CheckBox{
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
        draw_bg.color: (shad_theme.color_muted_foreground)
        draw_bg.color_hover: (shad_theme.color_secondary_hover)
    }

    mod.widgets.GalleryToggle = Toggle{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""
        code_block := SolidView{
            width: Fill
            height: Fit
            margin: Inset{top: 4, bottom: 8}
            padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
            draw_bg.color: (shad_theme.color_secondary)

            code_view := CodeView{
                width: Fill
                height: Fit
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCodeSnippet {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    code: ArcStringMut,
    #[rust]
    last_code: String,
}

impl GalleryCodeSnippet {
    fn sync_code(&mut self, cx: &mut Cx) {
        let current_code = self.code.as_ref().trim();
        if current_code != self.last_code {
            self.last_code = current_code.to_string();
            let cv = self.view.code_view(cx, ids!(code_view));
            cv.set_text(cx, &self.last_code);
        }
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_code(cx);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
