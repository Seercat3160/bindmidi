use crate::config::{Bind, Config};

pub mod interface;
pub mod manager;
pub mod table_data_adaptor;

/// App data used at runtime
pub struct State {
    /// Persistent app data stored in a file
    config: Config,
    /// Index of the bind currently being edited in the GUI
    active_bind: Option<usize>,
}

impl State {
    /// Create a new State by providing the existing persistent data
    pub fn from_config(config: Config) -> Self {
        State {
            config,
            active_bind: None,
        }
    }

    /// Updates which bind is currently being edited in the GUI, so
    /// we can access it more easily without first having to know anything about it
    fn set_active_bind(&mut self, idx: Option<usize>) {
        self.active_bind = idx;
    }

    /// Returns true if there is an active bind, false if not
    fn has_active_bind(&self) -> bool {
        self.active_bind.is_some()
    }

    /// Returns a clone of the current active bind, if there is one
    fn get_active_bind(&self) -> Option<Bind> {
        self.active_bind.map(|idx| {
            self.config
                .get_bind(idx)
                .expect("active bind shouldn't be out-of-bounds")
        })
    }

    /// Delete the current active bind, if there is one set, returning it's previous index
    fn delete_active_bind(&mut self) -> Option<usize> {
        if let Some(idx) = self.active_bind {
            // Delete bind
            self.config
                .delete_bind(idx)
                .expect("active bind shouldn't be out-of-bounds");

            // Unset active bind
            self.set_active_bind(None);

            // Return old index of the deleted bind
            return Some(idx);
        }

        None
    }

    /// Set the current active bind, if there is one, to the given bind
    fn update_active_bind(&mut self, bind: Bind) -> Option<usize> {
        if let Some(idx) = self.active_bind {
            // Set bind
            self.config
                .set_bind(idx, bind)
                .expect("active bind shouldn't be out-of-bounds");

            // Return it's index
            return Some(idx);
        }

        None
    }
}
