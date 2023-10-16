use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(idx) => {
                if idx + 1 >= self.items.len() {
                    0
                } else {
                    idx + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.is_empty() {
                    0
                } else if i == 0 && !self.items.is_empty() {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn delete_current(&mut self) {
        if let Some(idx) = self.state.selected() {
            if idx < self.items.len() {
                self.items.remove(idx);
                if idx == 0 {
                    self.state.select(Some(0));
                } else {
                    self.previous();
                }
            }
        };
    }
}
