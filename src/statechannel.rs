use std::sync::{mpsc, Arc};

use libui::controls::{TableDataSource, TableValueType};

use crate::config::Bind;

pub struct StateChannel {
    tx: mpsc::Sender<StateChannelMessage>,
}

impl StateChannel {
    pub fn new() -> (Arc<Self>, mpsc::Receiver<StateChannelMessage>) {
        let (tx, rx) = mpsc::channel::<StateChannelMessage>();

        let state_channel = StateChannel { tx };

        (Arc::new(state_channel), rx)
    }

    //TODO: write a macro to reduce boilerplate here

    pub fn len_binds(&self) -> usize {
        let (rtx, rrx) = oneshot::channel::<usize>();

        let message = StateChannelMessage::LenBinds(rtx);

        self.tx.send(message).expect("can't send");

        rrx.recv().expect("can't receive")
    }

    pub fn get_note_string(&self, idx: usize) -> anyhow::Result<String> {
        let (rtx, rrx) = oneshot::channel::<anyhow::Result<String>>();

        let message = StateChannelMessage::NoteString(rtx, idx);

        self.tx.send(message).expect("can't send");

        rrx.recv().expect("can't receive")
    }

    pub fn get_nice_action_string(&self, idx: usize) -> anyhow::Result<String> {
        let (rtx, rrx) = oneshot::channel::<anyhow::Result<String>>();

        let message = StateChannelMessage::ActionString(rtx, idx);

        self.tx.send(message).expect("can't send");

        rrx.recv().expect("can't receive")
    }

    pub fn set_active_edit_bind(&self, idx: Option<usize>) {
        let (rtx, rrx) = oneshot::channel::<()>();

        let message = StateChannelMessage::SetActiveEditBind(rtx, idx);

        self.tx.send(message).expect("can't send");

        rrx.recv().expect("can't receive");
    }

    pub fn has_active_edit_bind(&self) -> bool {
        let (rtx, rrx) = oneshot::channel::<bool>();

        let message = StateChannelMessage::HasActiveEditBind(rtx);

        self.tx.send(message).expect("can't send");

        rrx.recv().expect("can't receive")
    }

    pub fn get_active_edit_bind(&self) -> Option<Bind> {
        let (rtx, rrx) = oneshot::channel::<Option<Bind>>();

        let message = StateChannelMessage::GetActiveEditBind(rtx);

        self.tx.send(message).expect("can't send");

        rrx.recv().expect("can't receive")
    }
}

pub enum StateChannelMessage {
    LenBinds(oneshot::Sender<usize>),
    NoteString(oneshot::Sender<anyhow::Result<String>>, usize),
    ActionString(oneshot::Sender<anyhow::Result<String>>, usize),
    SetActiveEditBind(oneshot::Sender<()>, Option<usize>),
    HasActiveEditBind(oneshot::Sender<bool>),
    GetActiveEditBind(oneshot::Sender<Option<Bind>>),
}

pub struct BindsTableDataAdaptor {
    state: Arc<StateChannel>,
}

impl BindsTableDataAdaptor {
    pub fn new(statechannel: Arc<StateChannel>) -> Self {
        BindsTableDataAdaptor {
            state: statechannel,
        }
    }
}

impl TableDataSource for BindsTableDataAdaptor {
    fn num_columns(&mut self) -> i32 {
        2
    }

    fn num_rows(&mut self) -> i32 {
        self.state.len_binds().try_into().unwrap()
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
                match self.state.get_note_string(row) {
                    Ok(x) => libui::controls::TableValue::String(x),
                    Err(_) => unreachable!(
                        "binds table row index shouldn't be out of bounds of the vec<bind>"
                    ),
                }
            }
            1 => {
                // The bind's action
                match self.state.get_nice_action_string(row) {
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
        // do nothing, as this isn't supported
    }
}
