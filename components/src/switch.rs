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

            // Track + thumb colors
            color_track_off: uniform(shad_theme.color_background)
            color_track_on: uniform(shad_theme.color_primary)
            color_thumb_off: uniform(shad_theme.color_muted_foreground)
            color_thumb_on: uniform(shad_theme.color_secondary_foreground)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                let sz = self.rect_size

                let switch_width = sz.x
                let switch_height = sz.y
                let radius = switch_height * 0.5
                let border_size = 1.0
                let inset = 1.0

                // Track
                sdf.box(inset, inset, switch_width - inset * 2.0, switch_height - inset * 2.0, radius - inset)
                let track_color = mix(self.color_track_off, self.color_track_on, self.checked_val)
                let border_col = mix(self.color_border, self.color_border_hover, self.hover)
                sdf.fill_keep(track_color)
                sdf.stroke(border_col, border_size)

                // Thumb: ring when unchecked, solid when checked
                let thumb_radius = radius - 4.5
                let thumb_x_unchecked = radius - 0.5
                let thumb_x_checked = switch_width - radius + 0.5
                let thumb_x = mix(thumb_x_unchecked, thumb_x_checked, self.checked_val)
                let thumb_y = switch_height * 0.5

                sdf.circle(thumb_x, thumb_y, thumb_radius)
                let thumb_color = mix(self.color_thumb_off, self.color_thumb_on, self.checked_val)
                sdf.fill(thumb_color)

                // Inner cutout gives off-state ring; fades out to solid on-state thumb.
                let inner_color = mix(track_color, thumb_color, self.checked_val)
                sdf.circle(thumb_x, thumb_y, thumb_radius * 0.52)
                sdf.fill(inner_color)

                // Focus ring
                sdf.box(0.0, 0.0, switch_width, switch_height, radius)
                sdf.stroke(mix(vec4(0.0), self.color_border_hover, self.focus), 1.5)

                return sdf.result
            }
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            text_style: theme.font_regular{font_size: 10}
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
        let switch_width = 34.0;
        let switch_height = 20.0;

        let mut layout = Layout::flow_right().with_align_y(0.5);
        layout.spacing = 10.0;

        cx.begin_turtle(walk, layout);

        self.draw_bg
            .draw_walk(cx, Walk::fixed(switch_width, switch_height));

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
