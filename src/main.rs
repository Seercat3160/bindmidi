#![warn(clippy::pedantic)]

use libui::prelude::*;

mod utils;

fn main() -> anyhow::Result<()> {
    let ui = UI::init()?;

    libui::build! { &ui,
        let layout = HorizontalBox(padded: true) {
            Stretchy: let controls_wrapper = VerticalBox(padded: true) {
                Compact: let label_status = Label("Status: Not Running")
                Compact: let bt_start = Button("Start")
                Compact: let bt_stop = Button("Stop")
            }
            Stretchy: let config_wrapper = VerticalBox(padded: true) {
                Compact: let label_table_binds = Label("Configured Binds")
                // TODO: add table here to show existing binds.
                // Selected table item will have values filled to the below bind-editing inputs,
                // clicking update or delete will update the bind from the values of the inputs or delete the bind
                Compact: let bt_add_bind = Button("New")
                Compact: let sep_config = HorizontalSeparator()
                Compact: let label_edit_bind = Label("Edit Selected Bind")
                Compact: let form_edit_bind = Form(padded: true) {
                    (Compact, "Note"): let combobox_bind_note = Combobox(selected: 3) {
                        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G"
                    }
                    (Compact, "Octave"): let spinbox_bind_octave = Spinbox(-1, 8)
                    (Compact, "Action"): let combobox_bind_action = Combobox(selected: 0) {
                        "Press Key", "Hold Key", "Click", "Hold Click", "Move Mouse" /* Relative */, "Move Mouse to" /* Absolute */, "Scroll"
                    }

                    // The following are to be shown/hidden at runtime based on selected action in `combobox_bind_action`
                    // All possible action-specific config values are defined here:

                    // Used for: Press Key, Hold Key
                    (Compact, "Key"): let text_bind_action_key = Entry()

                    // Used for: Click, Hold Click
                    (Compact, "Mouse Button"): let combobox_bind_action_mousebutton = Combobox(selected: 0) {
                        "Left", "Right"
                    }

                    // Used for Move Mouse
                    (Compact, "x Amount (px)"): let spinbox_bind_action_xpixels = Spinbox(0, i32::MAX)
                    (Compact, "y Amount (px)"): let spinbox_bind_action_ypixels = Spinbox(0, i32::MAX)

                    // Used for Move Mouse to
                    (Compact, "x Position (px)"): let spinbox_bind_action_xpos = Spinbox(0, i32::MAX)
                    (Compact, "y Position (px)"): let spinbox_bind_action_ypos = Spinbox(0, i32::MAX)

                    // Used for: Scroll
                    (Compact, "Scroll Direction"): let combobox_bind_action_scrolldirection = Combobox(selected: 0) {
                        "Up", "Down", "Left", "Right"
                    }
                    (Compact, "Scroll Amount"): let spinbox_bind_action_scrollamount = Spinbox(0, i32::MAX)
                }
                Compact: let container_bind_edit_buttons = HorizontalBox(padded: true) {
                    Stretchy: let bt_update_binding = Button("Save")
                    Stretchy: let bt_delete_binding = Button("Delete")
                }
            }
        }
    }

    // Show/hide action-specific config based on selected bind action
    let mut clean_bind_action_config = {
        shadow_clone_mut!(
            text_bind_action_key,
            combobox_bind_action_mousebutton,
            spinbox_bind_action_xpixels,
            spinbox_bind_action_ypixels,
            spinbox_bind_action_xpos,
            spinbox_bind_action_ypos,
            combobox_bind_action_scrolldirection,
            spinbox_bind_action_scrollamount
        );

        move |selected| {
            show_control_only_when!(selected,
                text_bind_action_key: 0 | 1,
                combobox_bind_action_mousebutton: 2 | 3,
                spinbox_bind_action_xpixels: 4,
                spinbox_bind_action_ypixels: 4,
                spinbox_bind_action_xpos: 5,
                spinbox_bind_action_ypos: 5,
                combobox_bind_action_scrolldirection: 6,
                spinbox_bind_action_scrollamount: 6
            );
        }
    };
    let _ = &(clean_bind_action_config(0));
    combobox_bind_action.on_selected(&ui, clean_bind_action_config);

    let mut window = Window::new(&ui, "midi2key", 600, 400, WindowType::NoMenubar);

    window.set_child(layout);
    window.show();

    let mut event_loop = ui.event_loop();
    event_loop.on_tick(move || {});
    event_loop.run();

    Ok(())
}
