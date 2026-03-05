use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSwitchBase = #(ShadSwitch::register_widget(vm))

    mod.widgets.ShadSwitch = set_type_default() do mod.widgets.ShadSwitchBase{
        width: Fit
        height: Fit
        label: "Switch"
        checked: false
        grab_key_focus: true

        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            checked_val: instance(0.0)

            color_border: uniform(shad_theme.color_outline_border)
            color_border_hover: uniform(shad_theme.color_outline_border_hover)
            color_primary: uniform(shad_theme.color_primary)
            color_background: uniform(shad_theme.color_background)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                let sz = self.rect_size

                let border_width = 1.0
                let switch_width = sz.x
                let switch_height = sz.y
                let radius = switch_height * 0.5
                let padding = 2.0

                // Focus ring
                sdf.box(0.0, 0.0, switch_width, switch_height, radius)
                sdf.stroke(mix(vec4(0.0), self.color_border_hover, self.focus), 1.5)

                // Track
                sdf.box(1.5, 1.5, switch_width - 3.0, switch_height - 3.0, radius - 1.5)
                let track_color_unchecked = mix(self.color_border, self.color_border_hover, self.hover)
                sdf.fill(mix(track_color_unchecked, self.color_primary, self.checked_val))

                // Knob
                let knob_radius = radius - padding - 1.5
                let knob_x_unchecked = padding + 1.5 + knob_radius
                let knob_x_checked = switch_width - padding - 1.5 - knob_radius
                let knob_x = mix(knob_x_unchecked, knob_x_checked, self.checked_val)
                let knob_y = switch_height * 0.5

                sdf.circle(knob_x, knob_y, knob_radius)
                sdf.fill(self.color_background)

                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 11}
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }

            focus: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {focus: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {draw_bg: {focus: 1.0}}
                }
            }

            checked: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {checked_val: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {checked_val: 1.0}}
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadSwitchAction {
    #[default]
    None,
    Changed(bool),
}

#[derive(Script, Widget, Animator)]
pub struct ShadSwitch {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[rust]
    area: Area,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[redraw]
    #[live]
    draw_text: DrawText,

    #[live]
    label: String,
    #[live]
    checked: bool,
    #[live]
    grab_key_focus: bool,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadSwitch {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            self.animator_toggle(
                cx,
                self.checked,
                animator::Animate::No,
                ids!(checked.on),
                ids!(checked.off),
            );
        });
    }
}

impl Widget for ShadSwitch {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_checked) {
            if let Some(args_obj) = args.as_object() {
                let trap = vm.bx.threads.cur().trap.pass();
                let value = vm.bx.heap.vec_value(args_obj, 0, trap);
                if let Some(checked) = value.as_bool() {
                    vm.with_cx_mut(|cx| {
                        self.set_checked(cx, checked, animator::Animate::No);
                    });
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(checked) {
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.checked));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.area.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::KeyDown(ke) => {
                if let KeyCode::Space | KeyCode::ReturnKey = ke.key_code {
                    self.checked = !self.checked;
                    self.animator_toggle(
                        cx,
                        self.checked,
                        animator::Animate::Yes,
                        ids!(checked.on),
                        ids!(checked.off),
                    );
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ShadSwitchAction::Changed(self.checked),
                    );
                    self.area.redraw(cx);
                }
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area);
                }
                self.checked = !self.checked;
                self.animator_toggle(
                    cx,
                    self.checked,
                    animator::Animate::Yes,
                    ids!(checked.on),
                    ids!(checked.off),
                );
                cx.widget_action_with_data(
                    &self.action_data,
                    uid,
                    ShadSwitchAction::Changed(self.checked),
                );
                self.area.redraw(cx);
            }
            Hit::KeyFocus(_) => {
                self.animator_play(cx, ids!(focus.on));
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, ids!(focus.off));
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    if fe.device.has_hovers() {
                        self.animator_play(cx, ids!(hover.on));
                    } else {
                        self.animator_play(cx, ids!(hover.off));
                    }
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let switch_width = 36.0;
        let switch_height = 20.0;

        let mut layout = Layout::flow_right().with_align_y(0.5);
        layout.spacing = 8.0;

        cx.begin_turtle(walk, layout);

        self.draw_bg.draw_walk(cx, Walk::fixed(switch_width, switch_height));

        if !self.label.is_empty() {
            self.draw_text.draw_walk(
                cx,
                Walk::new(Size::fit(), Size::fit()),
                Align { x: 0.0, y: 0.5 },
                self.label.as_ref(),
            );
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl ShadSwitch {
    pub fn set_checked(&mut self, cx: &mut Cx, checked: bool, animate: animator::Animate) {
        self.checked = checked;
        self.animator_toggle(cx, checked, animate, ids!(checked.on), ids!(checked.off));
        self.area.redraw(cx);
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadSwitchAction::Changed(v) = item.cast() {
                return Some(v);
            }
        }
        None
    }
}

impl ShadSwitchRef {
    pub fn set_checked(&self, cx: &mut Cx, checked: bool, animate: animator::Animate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_checked(cx, checked, animate);
        }
    }

    pub fn is_checked(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_checked())
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(inner) = self.borrow() {
            inner.changed(actions)
        } else {
            None
        }
    }
}
