use std::time::Instant;

use crossterm::event::KeyCode;
use tui::widgets::ListState;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

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
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct Task {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

pub enum TaskCreateFormInput {
    Title,
    Description,
}

// TODO: Add TaskState implementation methods for:
//  on_keycode, open_create_popup, close_create_popup, create_new_task
pub struct TaskState {
    // Menu items
    pub tasks: StatefulList<Task>,

    // New task creation
    pub new_task_popup_enabled: bool,
    pub new_task: Task,
    pub selected_input: TaskCreateFormInput,
}

pub enum TopMenuItem {
    Tasks,
    Timers,
}

pub fn get_menu_item_title(menu_item: TopMenuItem) -> &'static str {
    match menu_item {
        TopMenuItem::Tasks => "Tasks",
        TopMenuItem::Timers => "Timers",
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,

    pub task_state: TaskState,

    // Internals
    pub action_delay: Instant,
    pub display_debugger: bool,
    pub enhanced_graphics: bool,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec![
                get_menu_item_title(TopMenuItem::Tasks),
                get_menu_item_title(TopMenuItem::Timers),
            ]),
            enhanced_graphics,
            action_delay: Instant::now(),
            display_debugger: false,
            task_state: TaskState {
                new_task: Task {
                    title: String::from(""),
                    description: String::from(""),
                    is_completed: false,
                },
                new_task_popup_enabled: false,
                tasks: StatefulList::with_items(vec![]),
                selected_input: TaskCreateFormInput::Title,
            },
        }
    }

    // TODO: KeyCode handling should be generalized so tabs and windows are actually handling
    // based on which of them is focused and in which state the app exists at that moment
    pub fn on_keycode(&mut self, key: KeyCode) {
        if self.task_state.new_task_popup_enabled {
            return match key {
                KeyCode::Char(c) => match self.task_state.selected_input {
                    TaskCreateFormInput::Title => {
                        self.task_state.new_task.title.push(c);
                    }
                    TaskCreateFormInput::Description => {
                        self.task_state.new_task.description.push(c);
                    }
                },
                KeyCode::Backspace => match self.task_state.selected_input {
                    TaskCreateFormInput::Title => {
                        self.task_state.new_task.title.pop();
                    }
                    TaskCreateFormInput::Description => {
                        self.task_state.new_task.description.pop();
                    }
                },
                KeyCode::Tab | KeyCode::BackTab => match self.task_state.selected_input {
                    TaskCreateFormInput::Title => {
                        self.task_state.selected_input = TaskCreateFormInput::Description
                    }
                    TaskCreateFormInput::Description => {
                        self.task_state.selected_input = TaskCreateFormInput::Title
                    }
                },
                KeyCode::Esc => {
                    self.task_state.new_task_popup_enabled = false;
                }
                KeyCode::Enter => {
                    let new_task = Task {
                        title: self.task_state.new_task.title.to_owned(),
                        description: self.task_state.new_task.description.to_owned(),
                        is_completed: false,
                    };
                    self.task_state.tasks.items.push(new_task);
                    self.task_state.new_task_popup_enabled = false;
                }
                _ => {}
            };
        }
        return match key {
            // Character handling
            KeyCode::Char(c) => self.on_key(c),

            // Keyboard arrow actions
            KeyCode::Left => self.on_left(),
            KeyCode::Up => self.on_up(),
            KeyCode::Right => self.on_right(),
            KeyCode::Down => self.on_down(),

            KeyCode::Esc => {}
            _ => {}
        };
    }

    // TODO: Create Selected window based navigation controls
    pub fn on_up(&mut self) {
        self.task_state.tasks.next();
    }

    pub fn on_down(&mut self) {
        self.task_state.tasks.previous();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'd' => {
                self.display_debugger = !self.display_debugger;
            }
            'n' => {
                self.task_state.new_task_popup_enabled = true;
                self.task_state.new_task = Task {
                    title: String::from(""),
                    description: String::from(""),
                    is_completed: false,
                };
                self.task_state.selected_input = TaskCreateFormInput::Title;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Do some tick based logic
    }
}
