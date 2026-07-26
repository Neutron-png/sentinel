use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Back,
    NavigateHome,
    NavigateCreate,
    NavigateOpen,
    NavigateReport,
    NavigateExportImport,
    NavigateSettings,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Space,
    Tab,
    ShiftTab,
    Backspace,
    Delete,
    Home,
    End,
    Char(char),
    Resize(u16, u16),
    Tick,
    Noop,
}

impl From<KeyEvent> for Action {
    fn from(key: KeyEvent) -> Self {
        match key.code {
            KeyCode::Esc => Action::Back,
            KeyCode::Up => Action::Up,
            KeyCode::Down => Action::Down,
            KeyCode::Left => Action::Left,
            KeyCode::Right => Action::Right,
            KeyCode::Enter => Action::Enter,
            KeyCode::Char(' ') => Action::Space,
            KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => Action::ShiftTab,
            KeyCode::Tab => Action::Tab,
            KeyCode::BackTab => Action::ShiftTab,
            KeyCode::Backspace => Action::Backspace,
            KeyCode::Delete => Action::Delete,
            KeyCode::Home => Action::Home,
            KeyCode::End => Action::End,
            KeyCode::Char('1') => Action::NavigateHome,
            KeyCode::Char('2') => Action::NavigateCreate,
            KeyCode::Char('3') => Action::NavigateOpen,
            KeyCode::Char('4') => Action::NavigateSettings,
            KeyCode::Char('5') => Action::NavigateReport,
            KeyCode::Char('6') => Action::NavigateExportImport,
            KeyCode::Char(c) => Action::Char(c),
            _ => Action::Noop,
        }
    }
}
