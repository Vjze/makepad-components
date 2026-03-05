pub use makepad_widgets;

use makepad_widgets::*;

pub mod accordion;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::accordion::script_mod(vm);
}
