#[derive(Debug)]
pub struct DeTuiModel {
    pub(crate) state: DeTuiState,
    counter: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum DeTuiState {
    Running,
    Exiting,
}

pub enum DeTuiMessage {
    Exit,
    Increment,
    Decrement,
}

impl DeTuiModel {
    pub fn new() -> Self {
        Self {
            state: DeTuiState::Running,
            counter: 0,
        }
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    fn exit(&self) -> Option<Self> {
        Some(Self {
            state: DeTuiState::Exiting,
            counter: self.counter,
        })
    }

    fn incr(&self) -> Option<Self> {
        if self.counter == 100 {
            None
        } else {
            Some(Self {
                state: self.state,
                counter: self.counter + 1,
            })
        }
    }

    fn decr(&self) -> Option<Self> {
        if self.counter == 0 {
            None
        } else {
            Some(Self {
                state: self.state,
                counter: self.counter - 1,
            })
        }
    }

    pub fn handle_message(&self, message: DeTuiMessage) -> Option<Self> {
        match message {
            DeTuiMessage::Exit => self.exit(),
            DeTuiMessage::Increment => self.incr(),
            DeTuiMessage::Decrement => self.decr(),
        }
    }
}
