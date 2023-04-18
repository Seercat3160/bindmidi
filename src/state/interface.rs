use std::sync::{mpsc, Arc};

use oneshot;

use crate::config::Bind;

pub struct StateInterface {
    channel: mpsc::Sender<StateMessage>,
}

impl StateInterface {
    pub fn new() -> (Arc<Self>, mpsc::Receiver<StateMessage>) {
        let (send_channel, recv_channel) = mpsc::channel();

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
}
