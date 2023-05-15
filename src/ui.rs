use crate::app::{App, TaskCreateFormInput};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Gray))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title(app.title),
        )
        .highlight_style(Style::default().fg(Color::Green))
        .select(app.tabs.index);

    f.render_widget(tabs, chunks[0]);

    match app.tabs.index {
        0 => draw_task_tab(f, app, chunks[1]),
        1 => draw_timers_tab(f, app, chunks[1]),
        _ => {}
    };

    if app.task_state.new_task_popup_enabled {
        draw_new_task_popup(f, app, chunks[1]);
    }

    // Debugging window
    if app.display_debugger {
        draw_debugger(f, chunks[1], app.tabs.index.to_string());
    }
}

fn draw_new_task_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let popup_chunk = centered_rect(50, 20, area);

    let block = Block::default()
        .title("New Task")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);

    let inner_content = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(popup_chunk);

    let title_input = Paragraph::new(String::from(app.task_state.new_task.title.to_owned()))
        .style(match app.task_state.selected_input {
            TaskCreateFormInput::Title => Style::default().fg(Color::Yellow),
            TaskCreateFormInput::Description => Style::default(),
        })
        .block(Block::default().borders(Borders::BOTTOM).title("Title"));

    let subtitle_input =
        Paragraph::new(String::from(app.task_state.new_task.description.to_owned()))
            .style(match app.task_state.selected_input {
                TaskCreateFormInput::Title => Style::default(),
                TaskCreateFormInput::Description => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .title("Description"),
            );

    f.render_widget(Clear, popup_chunk);
    f.render_widget(block, popup_chunk);

    f.render_widget(title_input, inner_content[0]);
    f.render_widget(subtitle_input, inner_content[1]);

    match app.task_state.selected_input {
        TaskCreateFormInput::Title => f.set_cursor(
            inner_content[0].x + app.task_state.new_task.title.width() as u16,
            inner_content[0].y + 1,
        ),
        TaskCreateFormInput::Description => f.set_cursor(
            inner_content[1].x + app.task_state.new_task.description.width() as u16,
            inner_content[1].y + 1,
        ),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn draw_debugger<B>(f: &mut Frame<B>, area: Rect, debug_info: String)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Max(4)])
        .split(area);

    let text = vec![
        Spans::from(vec![Span::raw("Printed info:")]),
        Spans::from(vec![Span::raw(debug_info)]),
    ];
    let para = Paragraph::new(text)
        .block(Block::default().title("Debug info").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true });

    f.render_widget(para, chunks[1]);
}

fn draw_task_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);

    let tasks: Vec<ListItem> = app
        .task_state
        .tasks
        .items
        .iter()
        .map(|task| ListItem::new(vec![Spans::from(Span::raw(&task.title))]))
        .collect();

    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Task List"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("-> ");

    let current_selection = app.task_state.tasks.state.selected().unwrap_or(0);
    let current_selected_task = if app.task_state.tasks.items.len() > 0 {
        Some(&app.task_state.tasks.items[current_selection])
    } else {
        None
    };

    let info_paragraph: Paragraph;
    match current_selected_task {
        Some(task) => {
            let information_block = vec![
                Spans::from(Span::styled(
                    "Title:",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::raw("")),
                Spans::from(Span::raw(&task.title)),
                Spans::from(Span::raw("")),
                Spans::from(Span::styled(
                    "Description:",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::raw("")),
                Spans::from(Span::raw(&task.description)),
                Spans::from(Span::raw("")),
                Spans::from(Span::styled(
                    "Is finished?",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::raw("")),
                Spans::from(Span::raw(if task.is_completed { "Yes." } else { "No." })),
                Spans::from(Span::raw("")),
            ];
            info_paragraph = Paragraph::new(information_block).wrap(Wrap { trim: false });
        }
        None => {
            let information_block = vec![Spans::from(Span::styled(
                "Select task to display information...",
                Style::default().fg(Color::Yellow),
            ))];
            info_paragraph = Paragraph::new(information_block).wrap(Wrap { trim: false });
        }
    }

    let wrapper = Block::default()
        .borders(Borders::ALL)
        .title("Task Information");

    let parent_chunk = Layout::default()
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    f.render_widget(wrapper, chunks[1]);
    f.render_widget(info_paragraph, parent_chunk[0]);

    f.render_stateful_widget(tasks, chunks[0], &mut app.task_state.tasks.state);
}

fn draw_timers_tab<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Active Timers",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    let text = vec![Spans::from("Implement timers...")];

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
