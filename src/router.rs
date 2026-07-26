#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    CreateAssessment,
    OpenAssessment,
    AssessmentView,
    Report,
    ExportImport,
    Settings,
}

#[derive(Debug, Clone)]
pub struct Router {
    pub current_screen: Screen,
    history: Vec<Screen>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Home,
            history: Vec::new(),
        }
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        if screen != self.current_screen {
            self.history.push(self.current_screen);
            self.current_screen = screen;
        }
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current_screen = prev;
            true
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }
}
