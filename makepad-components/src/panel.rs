use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadPanel = mod.widgets.ShadSurfaceTransparent{
        width: Fill
        height: Fit
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
    }
}
