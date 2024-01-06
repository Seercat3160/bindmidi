use std::sync::Arc;

use libui::controls::{TableDataSource, TableValueType};

use super::interface::StateInterface;

/// Provides an interface between the app state and the GUI to allow for the table of binds to be displayed
pub struct Adaptor {
    interface: Arc<StateInterface>,
}

impl Adaptor {
    pub fn new(state_interface: Arc<StateInterface>) -> Self {
        Adaptor {
            interface: state_interface,
        }
    }
}

impl TableDataSource for Adaptor {
    fn num_columns(&mut self) -> i32 {
        2
    }

    fn num_rows(&mut self) -> i32 {
        self.interface.len_binds().try_into().unwrap()
    }

    fn column_type(&mut self, column: i32) -> libui::controls::TableValueType {
        match column {
            0 | 1 => TableValueType::String,
            _ => unreachable!("shouldn't be more than two columns"),
        }
    }

    fn cell(&mut self, column: i32, row: i32) -> libui::controls::TableValue {
        let row: usize = row.try_into().unwrap();

        match column {
            0 => {
                // Note of that bind
                match self.interface.get_bind_note_string(row) {
                    Ok(x) => libui::controls::TableValue::String(x),
                    Err(_) => unreachable!(
                        "binds table row index shouldn't be out of bounds of the vec<bind>"
                    ),
                }
            }
            1 => {
                // The bind's action
                match self.interface.get_bind_action_string(row) {
                    Ok(x) => libui::controls::TableValue::String(x),
                    Err(_) => unreachable!(
                        "binds table row index shouldn't be out of bounds of the vec<bind>"
                    ),
                }
            }
            _ => unreachable!("shouldn't be more than two columns"),
        }
    }

    fn set_cell(&mut self, _column: i32, _row: i32, _value: libui::controls::TableValue) {
        // do nothing, as this isn't supported (and shouldn't happen)
        unreachable!("this shouldn't happen");
    }
}
