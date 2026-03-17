use crate::animation::{AnimationStep, AnimationTicker};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let SpinnerVisual = mod.widgets.RoundedView{
        width: Fill
        height: Fill

        draw_bg +: {
            color: uniform(shad_theme.color_primary)
            rotation_speed: uniform(1.0)
            stroke_width: uniform(2.5)
            arc_gap: uniform(0.25)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let radius = min(self.rect_size.x * 0.5, self.rect_size.y * 0.5) - self.stroke_width * 0.5
                let center = self.rect_size * 0.5
                let rotation = self.draw_pass.time * self.rotation_speed * 2.0 * PI
                let gap_radians = self.arc_gap * 2.0 * PI
                let start_angle = rotation
                sdf.arc_round_caps(
                    center.x
                    center.y
                    radius
                    start_angle
                    start_angle + 2.0 * PI - gap_radians
                    self.stroke_width
                )
                return sdf.fill(self.color)
            }
        }
    }

    mod.widgets.ShadSpinnerBase = #(ShadSpinner::register_widget(vm))

    mod.widgets.ShadSpinner = set_type_default() do mod.widgets.ShadSpinnerBase{
        width: 24
        height: 24
        animate: true
        animation_fps: 24.0

        spinner_body := SpinnerVisual{}
    }

    mod.widgets.ShadSpinnerSm = mod.widgets.ShadSpinner{
        width: 16
        height: 16
        spinner_body := SpinnerVisual{
            draw_bg +: {
                stroke_width: uniform(2.0)
            }
        }
    }

    mod.widgets.ShadSpinnerLg = mod.widgets.ShadSpinner{
        width: 32
        height: 32
        spinner_body := SpinnerVisual{
            draw_bg +: {
                stroke_width: uniform(3.0)
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadSpinner {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live(true)]
    animate: bool,
    #[live(24.0)]
    animation_fps: f64,
    #[rust]
    ticker: AnimationTicker,
}

impl ShadSpinner {
    fn is_visible(&self, cx: &Cx) -> bool {
        let area = self.view.area();
        if !area.is_valid(cx) {
            return false;
        }
        let clipped = area.clipped_rect(cx);
        clipped.size.x > 0.0 && clipped.size.y > 0.0
    }
}

impl Widget for ShadSpinner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let animate = self.animate && self.animation_fps > 0.0 && self.is_visible(cx);
        if let AnimationStep::Redraw { .. } =
            self.ticker
                .handle_event(cx, event, animate, self.animation_fps)
        {
            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let should_tick =
            self.animate && self.animation_fps > 0.0 && cx.walk_turtle_would_be_visible(walk);
        self.ticker.ensure_started(cx, should_tick);
        self.view.draw_walk(cx, scope, walk)
    }
}
