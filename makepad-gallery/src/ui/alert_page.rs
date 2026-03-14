use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAlertPage,
    page: alert_page,
    title: "Alert",
    subtitle: "Inline callouts for status, guidance, and destructive messaging. Compose an alert from the outer shell, an icon, and a ShadAlertContent text stack.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Anatomy" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Use ShadAlert or ShadAlertDestructive as the outer shell. They provide the border, spacing, and base colors."}
            ShadFieldDescription{text: "Render the icon as a sibling of ShadAlertContent, not inside the text stack. The outer alert row handles the horizontal layout."}
            ShadFieldDescription{text: "Put ShadAlertTitle and the description inside ShadAlertContent so the text stays vertically stacked while the icon remains top-aligned beside it."}
        }

        ShadHr{}

        ShadSectionHeader{ text: "Default" }

        ShadAlert{
            width: Fill
            ShadAlertIcon{}
            ShadAlertContent{
                ShadAlertTitle{text: "Heads up!"}
                ShadAlertDescription{
                    text: "You can add components and dependencies to your app using the cli."
                }
            }
        }

        ShadSectionHeader{ text: "Destructive" }

        ShadAlertDestructive{
            width: Fill
            ShadAlertDestructiveIcon{}
            ShadAlertContent{
                ShadAlertDestructiveTitle{text: "Error"}
                ShadAlertDestructiveDescription{
                    text: "Your session has expired. Please log in again."
                }
            }
        }

        ShadHr{}

        ShadSectionHeader{ text: "Usage" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 6.0

            ShadFieldDescription{text: "Alerts are inline status UI. They do not own modal state, dismissal logic, or background overlays like dialogs and popovers do."}
            ShadFieldDescription{text: "Use the default variant for neutral guidance, tips, or non-blocking status updates. Use the destructive variant when the copy itself should read as an error or dangerous condition."}
            ShadFieldDescription{text: "If you need actions, add them below the description or as another row inside ShadAlertContent so the page keeps ownership of the actual behavior."}
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Choose the shell first: ShadAlert for neutral information, or ShadAlertDestructive when the whole callout should carry destructive emphasis."}
        mod.widgets.GalleryActionFlowStep{text: "2. Place the icon beside ShadAlertContent, not inside it. ShadAlertIcon is the default info glyph, but any icon widget can be dropped in."}
        mod.widgets.GalleryActionFlowStep{text: "3. Stack the title and description inside ShadAlertContent so the text block stays compact and the icon can align to the top of that block."}
        mod.widgets.GalleryActionFlowStep{text: "4. Handle any real behavior in page or app code. The alert primitive is a styled layout surface, not a state machine."}
    },
}
