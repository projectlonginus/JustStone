pub(crate) enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub(crate) struct App {
    /// Current value of the input box
    pub(crate) input: String,
    /// Position of cursor in the editor area.
    pub(crate) character_index: usize,
    /// Current input mode
    pub(crate) input_mode: InputMode,
    /// History of recorded messages
    pub(crate) messages: Vec<String>,
}