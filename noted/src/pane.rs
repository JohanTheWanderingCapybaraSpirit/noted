pub struct Panes {
    pub files: bool
}

impl Default for Panes {
    fn default() -> Self {
        Self {
            files: false
        }
    }
}

pub enum PaneToggle {
    Files(bool)
}
