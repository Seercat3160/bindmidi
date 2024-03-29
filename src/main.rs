#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::module_name_repetitions)]
#![cfg_attr(not(test), windows_subsystem = "windows")]

use std::{cell::RefCell, fs::read_to_string, rc::Rc};

use anyhow::anyhow;
use directories::ProjectDirs;
use libui::{
    controls::{Table, TableModel, TableParameters},
    prelude::*,
};

use crate::{
    config::{
        AbsolutePos2D, Bind, BindAction, Config, KeyboardKeyBindAction, RelativePos2D,
        ScrollBindAction, TextBindAction,
    },
    note::Note,
    state::{manager::StateManager, table_data_adaptor::Adaptor, State},
};

mod bind;
mod config;
mod note;
mod state;
mod utils;

fn main() -> anyhow::Result<()> {
    // Determine path for config file
    let config_dir = ProjectDirs::from("", "", "bindmidi")
        .ok_or(anyhow!("couldn't build config directory"))?
        .config_dir()
        .to_path_buf();
    let config_file_path = config_dir.join("config.json");

    // Attempt to deserialize into a Config, or fallback on creating a new Config
    let config: Config = match read_to_string(&config_file_path) {
        Ok(config_file_contents) => match serde_json::from_str(&config_file_contents) {
            Ok(deserialized_config) => deserialized_config,
            Err(_) => Config::new(),
        },
        Err(_) => Config::new(),
    };

    let state = State::from_config(config);
    let (state_manager, state_interface) = StateManager::new(state);
    let _manager_thread = std::thread::spawn(move || {
        let mut state_manager = state_manager;

        state_manager.manage().unwrap();
    });

    let ui = UI::init()?;

    libui::layout! { &ui,
        let layout = HorizontalBox(padded: true) {
            Stretchy: let controls_wrapper = VerticalBox(padded: true) {
                Compact: let label_status = Label("Status: Not Running")
                Compact: let bt_start = Button("Start")
                Compact: let bt_stop = Button("Stop")
                Compact: let sep_controls = HorizontalSeparator()
                Compact: let form_midi = Form(padded: true) {
                    (Compact, "MIDI Input"): let combobox_midi_input = Combobox() {
                        // Filled at runtime with the available MIDI inputs
                    }
                }
            }
            Stretchy: let config_wrapper = VerticalBox(padded: true) {
                Compact: let label_table_binds = Label("Configured Binds")
                Stretchy: let container_table_binds = VerticalBox(padded: false) {
                    // Table gets added into here later as it's not possible with this build! macro
                }
                Compact: let bt_add_bind = Button("New")
                Compact: let sep_config = HorizontalSeparator()
                Compact: let label_edit_bind = Label("Edit Selected Bind")
                Compact: let form_edit_bind = Form(padded: true) {
                    (Compact, "Note"): let combobox_bind_note = Combobox(selected: 0) {
                        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
                    }
                    (Compact, "Octave"): let spinbox_bind_octave = Spinbox(-1, 8)
                    (Compact, "Action"): let combobox_bind_action = Combobox(selected: 0) {
                        "Press Key", "Hold Key", "Click", "Hold Click", "Move Mouse" /* Relative */, "Move Mouse to" /* Absolute */, "Scroll", "Text" /* Types arbitrary text */, "Debug"
                    }

                    // The following are to be shown/hidden at runtime based on selected action in `combobox_bind_action`
                    // All possible action-specific config values are defined here:

                    // Used for: Press Key, Hold Key
                    (Compact, "Key"): let text_bind_action_key = Entry()

                    // Used for: Click, Hold Click
                    (Compact, "Mouse Button"): let combobox_bind_action_mousebutton = Combobox(selected: 0) {
                        "Left", "Right", "Middle"
                    }

                    // Used for Move Mouse
                    (Compact, "x Amount (px)"): let spinbox_bind_action_xpixels = Spinbox()
                    (Compact, "y Amount (px)"): let spinbox_bind_action_ypixels = Spinbox()

                    // Used for Move Mouse to
                    (Compact, "x Position (px)"): let spinbox_bind_action_xpos = Spinbox(0, i32::MAX)
                    (Compact, "y Position (px)"): let spinbox_bind_action_ypos = Spinbox(0, i32::MAX)

                    // Used for: Scroll
                    (Compact, "Scroll Direction"): let combobox_bind_action_scrolldirection = Combobox(selected: 0) {
                        "Up", "Down", "Left", "Right"
                    }
                    (Compact, "Scroll Amount"): let spinbox_bind_action_scrollamount = Spinbox(0, i32::MAX)

                    // Used for: Text
                    (Compact, "Text"): let text_bind_action_text = Entry()
                }
                Compact: let container_bind_edit_buttons = HorizontalBox(padded: true) {
                    Stretchy: let bt_update_bind = Button("Save")
                    Stretchy: let bt_delete_bind = Button("Delete")
                }
            }
        }
    }

    let table_binds_data = Rc::new(RefCell::new(Adaptor::new(state_interface.clone())));
    let table_binds_model = Rc::new(RefCell::new(TableModel::new(table_binds_data)));
    let table_binds_params = TableParameters::new(table_binds_model.clone());
    let mut table_binds = Table::new(table_binds_params);

    table_binds.append_text_column("Note", 0, Table::COLUMN_READONLY);
    table_binds.append_text_column("Action", 1, Table::COLUMN_READONLY);

    container_table_binds.append(table_binds.clone(), LayoutStrategy::Stretchy);

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
            spinbox_bind_action_scrollamount,
            text_bind_action_text
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
                spinbox_bind_action_scrollamount: 6,
                text_bind_action_text: 7
            );
        }
    };
    (clean_bind_action_config(0)); // Run once at startup
    combobox_bind_action.on_selected(&ui, clean_bind_action_config.clone());

    // Enable bind-editing GUI only if a bind is selected, otherwise disable
    let mut enable_bind_edit_only_if_needed = {
        shadow_clone_mut!(form_edit_bind, container_bind_edit_buttons);

        move |has_active_edit_bind| {
            enable_control_only_when!(
                has_active_edit_bind,
                form_edit_bind,
                container_bind_edit_buttons
            );
        }
    };
    (enable_bind_edit_only_if_needed(false)); // Run once at startup

    // Update data and edit form when bind (de)selected in the table
    table_binds.on_selection_changed({
        shadow_clone!(state_interface);

        shadow_clone_mut!(
            combobox_bind_note,
            spinbox_bind_octave,
            combobox_bind_action,
            text_bind_action_key,
            combobox_bind_action_mousebutton,
            spinbox_bind_action_xpixels,
            spinbox_bind_action_ypixels,
            spinbox_bind_action_xpos,
            spinbox_bind_action_ypos,
            combobox_bind_action_scrolldirection,
            spinbox_bind_action_scrollamount,
            text_bind_action_text
        );

        move |x| {
            match x.selection().first() {
                Some(idx) => state_interface.set_active_bind(Some((*idx).try_into().unwrap())),
                None => state_interface.set_active_bind(None),
            }

            // In a variable so the blocking message-passing stuff in the state_channel wrapper is only called once
            let has_active_edit_bind = state_interface.has_active_bind();

            enable_bind_edit_only_if_needed(has_active_edit_bind);

            // Set contents of the bind edit controls with the relevant info for the newly-selected bind
            if has_active_edit_bind {
                use config::BindAction as Act;

                let bind = state_interface
                    .get_active_bind()
                    .expect("already checked for None with `state_channel.has_active_edit_bind()`");

                combobox_bind_note.set_selected(i32::from(bind.note.get_pitch_class_offset()));
                spinbox_bind_octave.set_value(i32::from(bind.note.get_octave()));

                let action = bind.action;

                combobox_bind_action.set_selected(i32::from(action.index()));
                clean_bind_action_config(combobox_bind_action.selected());

                match action {
                    Act::PressKey(act) | Act::HoldKey(act) => {
                        text_bind_action_key.set_value(&act.key);
                    }
                    Act::Click(act) | Act::HoldClick(act) => {
                        combobox_bind_action_mousebutton.set_selected(i32::from(act.index()));
                    }
                    Act::MoveMouseRelative(act) => {
                        spinbox_bind_action_xpixels.set_value(act.x);
                        spinbox_bind_action_ypixels.set_value(act.y);
                    }
                    Act::MoveMouseAbsolute(act) => {
                        spinbox_bind_action_xpos.set_value(act.x);
                        spinbox_bind_action_ypos.set_value(act.y);
                    }
                    Act::Scroll(act) => {
                        combobox_bind_action_scrolldirection
                            .set_selected(i32::from(act.direction.index()));
                        spinbox_bind_action_scrollamount.set_value(act.amount);
                    }
                    Act::Text(act) => {
                        text_bind_action_text.set_value(&act.text);
                    }
                    Act::Debug => {}
                }
            }
        }
    });

    // Add new binds via the GUI
    bt_add_bind.on_clicked({
        shadow_clone!(state_interface, table_binds_model, config_file_path);

        move |_| {
            // Create new bind
            let idx = state_interface.add_default_bind();

            // Notify the table of the new row
            table_binds_model
                .borrow()
                .notify_row_inserted(idx.try_into().unwrap());

            // Save config to disk
            state_interface.save_config(config_file_path.clone());
        }
    });

    // Delete binds via the GUI
    bt_delete_bind.on_clicked({
        shadow_clone!(state_interface, table_binds_model, config_file_path);

        move |_| {
            // Delete the active edit bind
            let row = state_interface.delete_active_bind();

            if let Some(row) = row {
                // Notify the table of the removed row
                table_binds_model
                    .borrow()
                    .notify_row_deleted(row.try_into().unwrap());
            }

            // Save config to disk
            state_interface.save_config(config_file_path.clone());
        }
    });

    // Update binds via the GUI
    bt_update_bind.on_clicked({
        shadow_clone!(
            state_interface,
            table_binds_model,
            combobox_bind_note,
            spinbox_bind_octave,
            combobox_bind_action,
            text_bind_action_key,
            combobox_bind_action_mousebutton,
            spinbox_bind_action_xpixels,
            spinbox_bind_action_ypixels,
            spinbox_bind_action_xpos,
            spinbox_bind_action_ypos,
            combobox_bind_action_scrolldirection,
            spinbox_bind_action_scrollamount,
            text_bind_action_text,
            config_file_path
        );

        move |_| {
            // Create a bind from the data in the GUI
            let bind = Bind {
                note: {
                    let pitch_class_offset: u8 = combobox_bind_note.selected().try_into().unwrap();
                    let octave: i8 = spinbox_bind_octave.value().try_into().unwrap();

                    Note::new(pitch_class_offset, octave)
                },
                action: {
                    match combobox_bind_action.selected() {
                        0 => BindAction::PressKey(KeyboardKeyBindAction {
                            key: text_bind_action_key.value(),
                        }),
                        1 => BindAction::HoldKey(KeyboardKeyBindAction {
                            key: text_bind_action_key.value(),
                        }),
                        2 => BindAction::Click(match combobox_bind_action_mousebutton.selected() {
                            0 => config::MouseButton::Left,
                            1 => config::MouseButton::Right,
                            2 => config::MouseButton::Middle,
                            _ => unreachable!("shouldn't be this"),
                        }),
                        3 => BindAction::HoldClick(
                            match combobox_bind_action_mousebutton.selected() {
                                0 => config::MouseButton::Left,
                                1 => config::MouseButton::Right,
                                2 => config::MouseButton::Middle,
                                _ => unreachable!("shouldn't be this"),
                            },
                        ),
                        4 => BindAction::MoveMouseRelative(RelativePos2D {
                            x: spinbox_bind_action_xpixels.value(),
                            y: spinbox_bind_action_ypixels.value(),
                        }),
                        5 => BindAction::MoveMouseAbsolute(AbsolutePos2D {
                            x: spinbox_bind_action_xpos.value(),
                            y: spinbox_bind_action_ypos.value(),
                        }),
                        6 => BindAction::Scroll(ScrollBindAction {
                            direction: match combobox_bind_action_scrolldirection.selected() {
                                0 => config::ScrollDirection::Up,
                                1 => config::ScrollDirection::Down,
                                2 => config::ScrollDirection::Left,
                                3 => config::ScrollDirection::Right,
                                _ => unreachable!("shouldn't be this"),
                            },
                            amount: spinbox_bind_action_scrollamount.value(),
                        }),
                        7 => BindAction::Text(TextBindAction {
                            text: text_bind_action_text.value(),
                        }),
                        8 => BindAction::Debug,
                        _ => unreachable!("shouldn't be this"),
                    }
                },
            };

            // Update the bind
            let row = state_interface.update_active_bind(bind);

            // Notify the table of the updated row
            if let Some(row) = row {
                table_binds_model
                    .borrow()
                    .notify_row_changed(row.try_into().unwrap());
            }

            // Save config to disk
            state_interface.save_config(config_file_path.clone());
        }
    });

    // Set input midi port to use when selected in GUI
    combobox_midi_input.on_selected(&ui, {
        shadow_clone!(state_interface);

        move |selection| {
            state_interface.set_midi_input_port(selection.try_into().unwrap());
        }
    });

    // Start from GUI
    bt_start.disable();
    bt_start.on_clicked({
        shadow_clone!(state_interface);

        move |button| {
            state_interface.start_midi_connection();
            button.disable();
        }
    });

    // Stop from GUI
    bt_stop.disable();
    bt_stop.on_clicked({
        shadow_clone!(state_interface);

        move |button| {
            state_interface.stop_midi_connection();
            button.disable();
        }
    });

    let mut window = Window::new(&ui, "bindmidi", 600, 400, WindowType::NoMenubar);

    window.set_child(layout);
    window.show();

    let mut event_loop = ui.event_loop();
    event_loop.on_tick({
        shadow_clone!(state_interface);
        shadow_clone_mut!(combobox_midi_input, bt_start, bt_stop, label_status);

        move || {
            // Fill combobox_midi_input with available Midi inputs

            let input_names = state_interface
                .get_midi_input_names()
                .expect("error getting midi input names");

            // Skip if we already have the same number - this could have an edge case where the input ports change
            // but there is the same number of them, but whatever. That seems unlikely.
            if combobox_midi_input.count() != TryInto::<i32>::try_into(input_names.len()).unwrap() {
                combobox_midi_input.clear();

                for name in input_names {
                    combobox_midi_input.append(&name);
                }
            }

            // Enable/disable start and stop buttons based on current status

            let has_midi_connection = state_interface.has_midi_connection();
            enable_control_only_when!(!has_midi_connection, bt_start);
            enable_control_only_when!(has_midi_connection, bt_stop);
            label_status.set_text(if has_midi_connection {
                "Status: Running"
            } else {
                "Status: Not Running"
            });
        }
    });
    event_loop.run_delay(500);

    state_interface.shutdown();

    Ok(())
}
