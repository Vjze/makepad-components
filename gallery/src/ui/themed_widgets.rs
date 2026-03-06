use makepad_code_editor::code_view::CodeView;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""
        code_container := SolidView{
            width: Fill
            height: Fit
            padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
            draw_bg +: {
                color: (shad_theme.color_muted)
                border_radius: (shad_theme.radius)
            }

            code_view := CodeView{
                keep_cursor_at_end: false
                editor +: {
                    height: Fit
                }
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
        // Optimization: avoid repeated string allocations in handle_event/draw_walk loops
        // Previously: called trim().to_string() unconditionally on every frame/event
        // Now: check raw ArcStringMut reference first, only allocate if it changed
        let current_raw = self.code.as_ref();
        if current_raw != self.last_code.as_str() {
            self.last_code = current_raw.to_string();
            let trimmed = current_raw.trim();
            if let Some(mut cv) = self
                .view
                .widget(cx, ids!(code_view))
                .borrow_mut::<CodeView>()
            {
                cv.set_text(cx, trimmed);
            }
        }
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_code(cx);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_code(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}
