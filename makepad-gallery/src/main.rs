pub use makepad_code_editor;
pub use makepad_widgets;

mod ui;

use crate::ui::command_palette::GalleryCommandPalette;
use makepad_components::context_menu::ShadContextMenu;
use makepad_components::input_otp::ShadInputOtp;
use makepad_components::makepad_widgets::*;
use makepad_components::sheet::ShadSheet;
use makepad_components::{ShadCarousel, ShadDialog, ShadSonner};
use makepad_router::RouterWidgetWidgetRefExt;

const SIDEBAR_ROUTES: &[(LiveId, LiveId)] = &[
    (live_id!(sidebar_accordion), live_id!(accordion_page)),
    (live_id!(sidebar_alert), live_id!(alert_page)),
    (live_id!(sidebar_aspect_ratio), live_id!(aspect_ratio_page)),
    (live_id!(sidebar_avatar), live_id!(avatar_page)),
    (live_id!(sidebar_badge), live_id!(badge_page)),
    (live_id!(sidebar_breadcrumb), live_id!(breadcrumb_page)),
    (live_id!(sidebar_button), live_id!(button_page)),
    (live_id!(sidebar_button_group), live_id!(button_group_page)),
    (live_id!(sidebar_card), live_id!(card_page)),
    (live_id!(sidebar_carousel), live_id!(carousel_page)),
    (live_id!(sidebar_checkbox), live_id!(checkbox_page)),
    (live_id!(sidebar_collapsible), live_id!(collapsible_page)),
    (
        live_id!(sidebar_command_palette),
        live_id!(command_palette_page),
    ),
    (live_id!(sidebar_context_menu), live_id!(context_menu_page)),
    (live_id!(sidebar_dialog), live_id!(dialog_page)),
    (live_id!(sidebar_input), live_id!(input_page)),
    (live_id!(sidebar_input_otp), live_id!(input_otp_page)),
    (live_id!(sidebar_radio_group), live_id!(radio_group_page)),
    (live_id!(sidebar_resizable), live_id!(resizable_page)),
    (live_id!(sidebar_scroll_area), live_id!(scroll_area_page)),
    (live_id!(sidebar_select), live_id!(select_page)),
    (live_id!(sidebar_separator), live_id!(separator_page)),
    (live_id!(sidebar_sheet), live_id!(sheet_page)),
    (live_id!(sidebar_skeleton), live_id!(skeleton_page)),
    (live_id!(sidebar_switch), live_id!(switch_page)),
    (live_id!(sidebar_tabs), live_id!(tabs_page)),
    (live_id!(sidebar_textarea), live_id!(textarea_page)),
    (live_id!(sidebar_toggle), live_id!(toggle_page)),
    (live_id!(sidebar_kbd), live_id!(kbd_page)),
    (live_id!(sidebar_label), live_id!(label_page)),
    (live_id!(sidebar_progress), live_id!(progress_page)),
    (live_id!(sidebar_sidebar), live_id!(sidebar_page)),
    (live_id!(sidebar_slider), live_id!(slider_page)),
    (live_id!(sidebar_sonner), live_id!(sonner_page)),
    (live_id!(sidebar_spinner), live_id!(spinner_page)),
];

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: mod.widgets.GalleryAppUi{}
    }
}

impl App {
    const SMALL_SCREEN_WIDTH: f64 = 900.0;

    fn build_script_mod(vm: &mut ScriptVm, is_light_theme: bool) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        makepad_components::theme::script_mod(vm);
        if is_light_theme {
            script_eval!(vm, {
                mod.widgets.shad_theme = mod.widgets.shad_themes.light
            });
        } else {
            script_eval!(vm, {
                mod.widgets.shad_theme = mod.widgets.shad_themes.dark
            });
        }
        makepad_components::script_mod_without_theme(vm);
        makepad_code_editor::script_mod(vm);
        makepad_router::script_mod(vm);
        crate::ui::script_mod(vm);
        self::script_mod(vm)
    }

    fn set_current_page(&mut self, cx: &mut Cx, page: LiveId) {
        self.current_page = page;
        self.ui
            .router_widget(cx, ids!(content_flip))
            .go_to_route(cx, page);
        if self.is_small_screen {
            self.sidebar_open = false;
            self.apply_responsive_visibility(cx);
        }
    }

    fn toggle_command_palette(&mut self, cx: &mut Cx) {
        if let Some(mut palette) = self
            .ui
            .widget_flood(cx, ids!(command_palette))
            .borrow_mut::<GalleryCommandPalette>()
        {
            palette.toggle(cx);
        }
    }

    fn open_command_palette(&mut self, cx: &mut Cx) {
        if let Some(mut palette) = self
            .ui
            .widget_flood(cx, ids!(command_palette))
            .borrow_mut::<GalleryCommandPalette>()
        {
            palette.open(cx);
        }
    }

    fn sync_mobile_sidebar_button(&self, cx: &mut Cx) {
        self.ui.button(cx, ids!(mobile_sidebar_button)).set_text(
            cx,
            if self.is_small_screen && self.sidebar_open {
                "X"
            } else {
                "☰"
            },
        );
    }

    fn sync_theme_toggle_labels(&self, cx: &mut Cx) {
        let button_text = if self.is_light_theme {
            "Dark mode"
        } else {
            "Light mode"
        };
        self.ui
            .button(cx, ids!(sidebar_theme_toggle))
            .set_text(cx, button_text);
        self.ui
            .button(cx, ids!(mobile_theme_toggle))
            .set_text(cx, button_text);
    }

    fn reload_ui_for_theme(&mut self, cx: &mut Cx) {
        cx.with_vm(|vm| {
            let value = Self::build_script_mod(vm, self.is_light_theme);
            <Self as ScriptApply>::script_apply(
                self,
                vm,
                &Apply::Reload,
                &mut Scope::empty(),
                value,
            );
        });
        self.sync_theme_toggle_labels(cx);
        self.apply_responsive_visibility(cx);
        self.set_current_page(cx, self.current_page);
        self.ui.redraw(cx);
    }

    fn set_theme(&mut self, cx: &mut Cx, is_light_theme: bool) {
        self.is_light_theme = is_light_theme;
        self.reload_ui_for_theme(cx);
    }

    fn apply_responsive_visibility(&mut self, cx: &mut Cx) {
        self.ui
            .view(cx, ids!(mobile_header))
            .set_visible(cx, self.is_small_screen);
        self.ui
            .view(cx, ids!(sidebar))
            .set_visible(cx, !self.is_small_screen || self.sidebar_open);
        self.ui
            .view(cx, ids!(main_content))
            .set_visible(cx, !self.is_small_screen || !self.sidebar_open);
        self.sync_mobile_sidebar_button(cx);
    }

    fn update_screen_mode(&mut self, cx: &mut Cx, window_width: f64) {
        let is_small_screen = window_width < Self::SMALL_SCREEN_WIDTH;
        if self.is_small_screen != is_small_screen {
            self.is_small_screen = is_small_screen;
            self.sidebar_open = !is_small_screen;
        }
        self.apply_responsive_visibility(cx);
    }

    fn handle_sidebar_navigation(&mut self, cx: &mut Cx, actions: &Actions) {
        for (button_id, page_id) in SIDEBAR_ROUTES {
            if self.ui.button(cx, &[*button_id]).clicked(actions) {
                self.set_current_page(cx, *page_id);
                break;
            }
        }
    }

    fn set_gallery_flip_page(ui: &WidgetRef, cx: &mut Cx, flip: &[LiveId], page: LiveId) {
        let widget = ui.widget_flood(cx, flip);
        if let Some(mut page_flip) = widget.borrow_mut::<PageFlip>() {
            page_flip.set_active_page(cx, page);
            return;
        }
        ui.router_widget(cx, flip).go_to_route(cx, page);
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    is_small_screen: bool,
    #[rust]
    sidebar_open: bool,
    #[rust]
    is_light_theme: bool,
    #[rust]
    current_page: LiveId,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let palette = self.ui.widget_flood(cx, ids!(command_palette));
        if let Some(page) = actions
            .find_widget_action(palette.widget_uid())
            .and_then(|action| match action.cast() {
                ui::command_palette::GalleryCommandPaletteAction::Selected(page) => Some(page),
                _ => None,
            })
        {
            self.set_current_page(cx, page);
        }
        if self
            .ui
            .button(cx, ids!(mobile_sidebar_button))
            .clicked(actions)
            && self.is_small_screen
        {
            self.sidebar_open = !self.sidebar_open;
            self.apply_responsive_visibility(cx);
        }
        if self
            .ui
            .button(cx, ids!(sidebar_theme_toggle))
            .clicked(actions)
            || self
                .ui
                .button(cx, ids!(mobile_theme_toggle))
                .clicked(actions)
        {
            self.set_theme(cx, !self.is_light_theme);
        }

        self.handle_sidebar_navigation(cx, actions);
        if let Some(mut carousel) = self
            .ui
            .widget_flood(cx, ids!(carousel_demo))
            .borrow_mut::<ShadCarousel>()
        {
            carousel.handle_actions(cx, actions);
        }
        if self
            .ui
            .button(cx, ids!(open_command_palette_btn))
            .clicked(actions)
        {
            self.open_command_palette(cx);
        }
        if self.ui.button(cx, ids!(open_dialog_btn)).clicked(actions) {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(default_dialog))
                .borrow_mut::<ShadDialog>()
            {
                d.set_open(true);
            }
        }
        if self.ui.button(cx, ids!(open_default_btn)).clicked(actions) {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(alert_default_dialog))
                .borrow_mut::<ShadDialog>()
            {
                d.set_open(true);
            }
        }
        if self
            .ui
            .button(cx, ids!(open_destructive_btn))
            .clicked(actions)
        {
            if let Some(mut d) = self
                .ui
                .widget_flood(cx, ids!(destructive_dialog))
                .borrow_mut::<ShadDialog>()
            {
                d.set_open(true);
            }
        }
        let dialog_ref = self.ui.widget_flood(cx, ids!(default_dialog));
        if !dialog_ref.is_empty() && dialog_ref.button(cx, ids!(close_btn)).clicked(actions) {
            if let Some(mut d) = dialog_ref.borrow_mut::<ShadDialog>() {
                d.set_open(false);
            }
        }
        if self.ui.button(cx, ids!(toast_event_btn)).clicked(actions) {
            if let Some(mut s) = self
                .ui
                .widget_flood(cx, ids!(toast_event))
                .borrow_mut::<ShadSonner>()
            {
                s.set_open(true);
            }
        }
        if self.ui.button(cx, ids!(toast_desc_btn)).clicked(actions) {
            if let Some(mut s) = self
                .ui
                .widget_flood(cx, ids!(toast_desc))
                .borrow_mut::<ShadSonner>()
            {
                s.set_open(true);
            }
        }
        if self.ui.button(cx, ids!(toast_close_btn)).clicked(actions) {
            if let Some(mut s) = self
                .ui
                .widget_flood(cx, ids!(toast_close))
                .borrow_mut::<ShadSonner>()
            {
                s.set_open(true);
            }
        }
        if let Some(index) = self
            .ui
            .widget_flood(cx, ids!(context_menu_basic))
            .borrow::<ShadContextMenu>()
            .and_then(|inner| inner.selected(actions))
        {
            let label = match index {
                0 => "Open",
                1 => "Duplicate",
                2 => "Share",
                3 => "Delete",
                _ => "Unknown",
            };
            self.ui
                .label(cx, ids!(context_menu_status))
                .set_text(cx, &format!("Selected: {}", label));
        }
        let otp_demo = self.ui.widget_flood(cx, ids!(otp_demo));
        let otp_state = otp_demo.borrow::<ShadInputOtp>().map(|inner| {
            (
                inner.completed(actions),
                inner.changed(actions),
                inner.value().to_string(),
            )
        });
        if let Some((completed, changed, current_value)) = otp_state {
            let status = if let Some(value) = completed {
                format!("Completed: {}", value)
            } else if let Some(value) = changed {
                format!("Current value: {}", value)
            } else if current_value.len() >= 6 {
                format!("Completed: {}", current_value)
            } else if !current_value.is_empty() {
                format!("Current value: {}", current_value)
            } else {
                "Waiting for input.".to_string()
            };
            self.ui.label(cx, ids!(otp_status)).set_text(cx, &status);
        }

        if self
            .ui
            .button(cx, ids!(tabs_overview_trigger))
            .clicked(actions)
        {
            Self::set_gallery_flip_page(
                &self.ui,
                cx,
                ids!(tabs_content_flip),
                live_id!(overview_page),
            );
            self.ui
                .view(cx, ids!(tabs_overview_indicator))
                .set_visible(cx, true);
            self.ui
                .view(cx, ids!(tabs_usage_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_settings_indicator))
                .set_visible(cx, false);
        }
        if self
            .ui
            .button(cx, ids!(tabs_usage_trigger))
            .clicked(actions)
        {
            Self::set_gallery_flip_page(
                &self.ui,
                cx,
                ids!(tabs_content_flip),
                live_id!(usage_page),
            );
            self.ui
                .view(cx, ids!(tabs_overview_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_usage_indicator))
                .set_visible(cx, true);
            self.ui
                .view(cx, ids!(tabs_settings_indicator))
                .set_visible(cx, false);
        }
        if self
            .ui
            .button(cx, ids!(tabs_settings_trigger))
            .clicked(actions)
        {
            Self::set_gallery_flip_page(
                &self.ui,
                cx,
                ids!(tabs_content_flip),
                live_id!(settings_page),
            );
            self.ui
                .view(cx, ids!(tabs_overview_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_usage_indicator))
                .set_visible(cx, false);
            self.ui
                .view(cx, ids!(tabs_settings_indicator))
                .set_visible(cx, true);
        }

        if self
            .ui
            .button(cx, ids!(open_right_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(right_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self
            .ui
            .button(cx, ids!(open_left_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(left_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self
            .ui
            .button(cx, ids!(open_top_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(top_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        if self
            .ui
            .button(cx, ids!(open_bottom_sheet_btn))
            .clicked(actions)
        {
            let sheet = self.ui.widget_flood(cx, ids!(bottom_sheet));
            if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                s.set_open(true);
            }
            sheet.redraw(cx);
        }
        for (path, button) in [
            (ids!(right_sheet), ids!(close_right_sheet_btn)),
            (ids!(left_sheet), ids!(close_left_sheet_btn)),
            (ids!(top_sheet), ids!(close_top_sheet_btn)),
            (ids!(bottom_sheet), ids!(close_bottom_sheet_btn)),
        ] {
            if self.ui.button(cx, button).clicked(actions) {
                let sheet = self.ui.widget_flood(cx, path);
                if let Some(mut s) = sheet.borrow_mut::<ShadSheet>() {
                    s.set_open(false);
                }
                sheet.redraw(cx);
            }
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        Self::build_script_mod(vm, false)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.sidebar_open = true;
                self.current_page = live_id!(accordion_page);
                self.is_light_theme = false;
                self.sync_theme_toggle_labels(cx);
                self.apply_responsive_visibility(cx);
                self.set_current_page(cx, self.current_page);
            }
            Event::MacosMenuCommand(command) => {
                if *command == live_id!(command_palette_menu) {
                    self.open_command_palette(cx);
                    return;
                }
            }
            Event::WindowGeomChange(geom) => {
                self.update_screen_mode(cx, geom.new_geom.inner_size.x)
            }
            Event::KeyDown(key_event) => {
                if key_event.key_code == KeyCode::KeyK
                    && (key_event.modifiers.logo || key_event.modifiers.control)
                {
                    self.toggle_command_palette(cx);
                    return;
                }
            }
            _ => {}
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
