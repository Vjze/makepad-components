use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

const MAX_OTP_SLOTS: usize = 6;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadInputOtpSlot = RoundedView{
        width: 48
        height: 56
        align: Align{x: 0.5, y: 0.5}
        draw_bg +: {
            color: #0000
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadInputOtpBase = #(ShadInputOtp::register_widget(vm))

    mod.widgets.ShadInputOtp = set_type_default() do mod.widgets.ShadInputOtpBase{
        width: Fit
        height: Fit
        cell_count: 6
        value: ""

        slot_wrap := View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 8.0

            slot_0 := mod.widgets.ShadInputOtpSlot{
                label_0 := Label{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_1 := mod.widgets.ShadInputOtpSlot{
                label_1 := Label{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_2 := mod.widgets.ShadInputOtpSlot{
                label_2 := Label{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_3 := mod.widgets.ShadInputOtpSlot{
                label_3 := Label{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_4 := mod.widgets.ShadInputOtpSlot{
                label_4 := Label{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
            slot_5 := mod.widgets.ShadInputOtpSlot{
                label_5 := Label{
                    draw_text.color: (shad_theme.color_primary)
                    draw_text.text_style.font_size: 22.0
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadInputOtpAction {
    Changed(String),
    Completed(String),
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadInputOtp {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    cell_count: u32,
    #[live]
    value: String,

    #[rust]
    last_completed: Option<String>,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadInputOtp {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.value = self.sanitize(&self.value);
            self.sync_slots(cx);
        });
    }
}

impl ShadInputOtp {
    fn visible_cells(&self) -> usize {
        self.cell_count.clamp(1, MAX_OTP_SLOTS as u32) as usize
    }

    fn sanitize(&self, value: &str) -> String {
        value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .take(self.visible_cells())
            .collect()
    }

    fn slot_ref(&self, cx: &Cx, index: usize) -> ViewRef {
        match index {
            0 => self.view.view(cx, ids!(slot_0)),
            1 => self.view.view(cx, ids!(slot_1)),
            2 => self.view.view(cx, ids!(slot_2)),
            3 => self.view.view(cx, ids!(slot_3)),
            4 => self.view.view(cx, ids!(slot_4)),
            _ => self.view.view(cx, ids!(slot_5)),
        }
    }

    fn set_slot_label(&self, cx: &mut Cx, index: usize, value: &str) {
        match index {
            0 => self.view.label(cx, ids!(label_0)).set_text(cx, value),
            1 => self.view.label(cx, ids!(label_1)).set_text(cx, value),
            2 => self.view.label(cx, ids!(label_2)).set_text(cx, value),
            3 => self.view.label(cx, ids!(label_3)).set_text(cx, value),
            4 => self.view.label(cx, ids!(label_4)).set_text(cx, value),
            _ => self.view.label(cx, ids!(label_5)).set_text(cx, value),
        }
    }

    fn sync_slots(&mut self, cx: &mut Cx) {
        let chars: Vec<char> = self.value.chars().collect();
        let visible_cells = self.visible_cells();

        for index in 0..MAX_OTP_SLOTS {
            let slot = self.slot_ref(cx, index);
            let is_visible = index < visible_cells;
            slot.set_visible(cx, is_visible);
            if !is_visible {
                continue;
            }

            let digit = chars.get(index).map(|c| c.to_string()).unwrap_or_default();
            self.set_slot_label(cx, index, &digit);
        }
    }

    fn emit_completed_if_needed(&mut self, cx: &mut Cx) {
        if self.value.len() == self.visible_cells() {
            if self.last_completed.as_deref() != Some(self.value.as_str()) {
                self.last_completed = Some(self.value.clone());
                cx.widget_action_with_data(
                    &self.action_data,
                    self.widget_uid(),
                    ShadInputOtpAction::Completed(self.value.clone()),
                );
            }
        } else {
            self.last_completed = None;
        }
    }

    fn push_input(&mut self, cx: &mut Cx, input: &TextInputEvent) {
        let mut next = self.value.clone();
        if input.replace_last && !next.is_empty() {
            next.pop();
        }
        next.push_str(&input.input);
        self.set_value(cx, self.sanitize(&next));
    }

    fn pop_digit(&mut self, cx: &mut Cx) {
        if self.value.is_empty() {
            return;
        }
        let mut next = self.value.clone();
        next.pop();
        self.set_value(cx, next);
    }

    fn set_value(&mut self, cx: &mut Cx, next: String) {
        if next != self.value {
            self.value = next.clone();
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadInputOtpAction::Changed(next),
            );
        }
        self.emit_completed_if_needed(cx);
        self.sync_slots(cx);
    }

    pub fn changed(&self, actions: &Actions) -> Option<String> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadInputOtpAction::Changed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }

    pub fn completed(&self, actions: &Actions) -> Option<String> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadInputOtpAction::Completed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }
}

impl Widget for ShadInputOtp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let area = self.view.area();

        match event.hits(cx, area) {
            Hit::FingerDown(_) => {
                cx.set_key_focus(area);
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Text);
            }
            _ => {}
        }

        if cx.has_key_focus(area) {
            match event {
                Event::KeyDown(ke) => match ke.key_code {
                    KeyCode::Backspace => self.pop_digit(cx),
                    KeyCode::Escape => cx.set_key_focus(Area::Empty),
                    _ => {}
                },
                Event::TextInput(input) => {
                    self.push_input(cx, input);
                }
                _ => {}
            }
        }

        self.sync_slots(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShadInputOtpRef {
    pub fn changed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn completed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.completed(actions))
    }
}
