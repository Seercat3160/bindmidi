use std::sync::{mpsc, Arc};

use oneshot;

use crate::{bind::BindExecuteState, config::Bind, note::Note};

pub struct StateInterface {
    channel: mpsc::SyncSender<StateMessage>,
}

impl StateInterface {
    pub fn new() -> (Arc<Self>, mpsc::Receiver<StateMessage>) {
        let (send_channel, recv_channel) = mpsc::sync_channel(0);

        let interface = Self {
            channel: send_channel,
        };

        (Arc::new(interface), recv_channel)
    }

    pub fn request(&self, req: StateMessageRequest) -> StateMessageResponse {
        let (response_sender, response_receiver) = oneshot::channel();

        self.channel
            .send(StateMessage {
                response_channel: response_sender,
                request: req,
            })
            .expect("couldn't send");

        response_receiver.recv().expect("Can't receive")
    }
}

impl StateInterface {
    pub fn len_binds(&self) -> usize {
        match self.request(StateMessageRequest::LenBinds) {
            StateMessageResponse::LenBinds(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn get_bind_note_string(&self, bind_idx: usize) -> anyhow::Result<String> {
        match self.request(StateMessageRequest::NoteString(bind_idx)) {
            StateMessageResponse::NoteString(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn get_bind_action_string(&self, bind_idx: usize) -> anyhow::Result<String> {
        match self.request(StateMessageRequest::ActionString(bind_idx)) {
            StateMessageResponse::ActionString(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn set_active_bind(&self, bind: Option<usize>) {
        match self.request(StateMessageRequest::SetActiveBind(bind)) {
            StateMessageResponse::SetActiveBind => (),
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn has_active_bind(&self) -> bool {
        match self.request(StateMessageRequest::HasActiveBind) {
            StateMessageResponse::HasActiveBind(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn get_active_bind(&self) -> Option<Bind> {
        match self.request(StateMessageRequest::GetActiveBind) {
            StateMessageResponse::GetActiveBind(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn add_default_bind(&self) -> usize {
        match self.request(StateMessageRequest::AddDefaultBind) {
            StateMessageResponse::AddDefaultBind(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn delete_active_bind(&self) -> Option<usize> {
        match self.request(StateMessageRequest::DeleteActiveBind) {
            StateMessageResponse::DeleteActiveBind(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn update_active_bind(&self, bind: Bind) -> Option<usize> {
        match self.request(StateMessageRequest::UpdateActiveBind(bind)) {
            StateMessageResponse::UpdateActiveBind(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn get_midi_input_names(&self) -> anyhow::Result<Vec<String>> {
        match self.request(StateMessageRequest::MidiInputNames) {
            StateMessageResponse::MidiInputNames(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn set_midi_input_port(&self, idx: usize) {
        match self.request(StateMessageRequest::SetMidiInputPort(idx)) {
            StateMessageResponse::SetMidiInputPort => (),
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn start_midi_connection(&self) {
        match self.request(StateMessageRequest::StartMidiConnection) {
            StateMessageResponse::StartMidiConnection => (),
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn stop_midi_connection(&self) {
        match self.request(StateMessageRequest::StopMidiConnection) {
            StateMessageResponse::StopMidiConnection => (),
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn has_midi_connection(&self) -> bool {
        match self.request(StateMessageRequest::HasMidiConnection) {
            StateMessageResponse::HasMidiConnection(x) => x,
            _ => unimplemented!("wrong response type"),
        }
    }

    pub fn execute_binds(&self, note: Note, vel: u8, state: BindExecuteState) {
        match self.request(StateMessageRequest::ExecuteBindsForNote(note, vel, state)) {
            StateMessageResponse::ExecuteBindsForNote => (),
            _ => unimplemented!("wrong response type"),
        }
    }
}

pub struct StateMessage {
    pub response_channel: oneshot::Sender<StateMessageResponse>,
    pub request: StateMessageRequest,
}

pub enum StateMessageRequest {
    LenBinds,
    NoteString(usize),
    ActionString(usize),
    SetActiveBind(Option<usize>),
    HasActiveBind,
    GetActiveBind,
    AddDefaultBind,
    DeleteActiveBind,
    UpdateActiveBind(Bind),
    MidiInputNames,
    SetMidiInputPort(usize),
    StartMidiConnection,
    StopMidiConnection,
    HasMidiConnection,
    ExecuteBindsForNote(Note, u8, BindExecuteState),
}

pub enum StateMessageResponse {
    LenBinds(usize),
    NoteString(anyhow::Result<String>),
    ActionString(anyhow::Result<String>),
    SetActiveBind,
    HasActiveBind(bool),
    GetActiveBind(Option<Bind>),
    AddDefaultBind(usize),
    DeleteActiveBind(Option<usize>),
    UpdateActiveBind(Option<usize>),
    MidiInputNames(anyhow::Result<Vec<String>>),
    SetMidiInputPort,
    StartMidiConnection,
    StopMidiConnection,
    HasMidiConnection(bool),
    ExecuteBindsForNote,
}
