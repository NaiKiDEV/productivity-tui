use crossterm::event::KeyCode;

use crate::structures::stateful_list::StatefulList;

#[derive(Default)]
pub struct Task {
    pub title: String,
    pub is_completed: bool,
}

impl Task {
    pub fn default() -> Self {
        Self {
            title: String::from(""),
            is_completed: false,
        }
    }

    pub fn new(title: String) -> Self {
        Self {
            title,
            is_completed: false,
        }
    }
}

pub struct TaskState {
    pub tasks: StatefulList<Task>,

    pub new_task: Task,

    pub new_task_popup_enabled: bool,
}

// TODO: add editing for TASKS
impl TaskState {
    pub fn new() -> Self {
        Self {
            tasks: StatefulList::with_items(vec![]),

            new_task: Task::default(),

            new_task_popup_enabled: false,
        }
    }

    fn delete_selected_task(&mut self) {
        self.tasks.delete_current();
    }

    fn open_create_popup(&mut self) {
        self.new_task_popup_enabled = true;
    }

    fn close_create_popup(&mut self) {
        self.new_task_popup_enabled = false;
    }

    fn create_new_task(&mut self) {
        let new_task = Task::new(self.new_task.title.to_owned());
        self.tasks.items.push(new_task);
        self.new_task = Task::default();
    }

    pub fn on_keycode(&mut self, key: KeyCode) -> bool {
        if self.new_task_popup_enabled {
            self.on_popup_keycode(key);

            return true;
        }

        match key {
            KeyCode::Up => self.tasks.previous(),
            KeyCode::Down => self.tasks.next(),

            KeyCode::Enter => {
                let current_selection = self.tasks.state.selected();
                if let Some(selection) = current_selection {
                    self.tasks.items[selection].is_completed =
                        !self.tasks.items[selection].is_completed;
                }
            }

            KeyCode::Char(c) => match c {
                'd' => {
                    self.delete_selected_task();
                }
                'n' => {
                    self.open_create_popup();
                    self.new_task = Task::default();
                }
                _ => {}
            },
            _ => {}
        };

        false
    }

    fn on_popup_keycode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.new_task.title.push(c);
            }
            KeyCode::Backspace => {
                self.new_task.title.pop();
            }
            KeyCode::Esc => {
                self.close_create_popup();
            }
            KeyCode::Enter => {
                self.create_new_task();
                self.close_create_popup();
            }
            _ => {}
        };
    }
}
