use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

// TODO: Code up input field component which would handle offscreen and other issues?

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tab_titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Gray))))
        .collect();

    let tabs = Tabs::new(tab_titles)
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
    if app.timer_state.new_timer_popup_enabled {
        draw_new_timer_popup(f, app, chunks[1]);
    }
}

fn draw_new_task_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let popup_chunk = centered_rect(60, 3, area);

    let block = Block::default()
        .title("New Task")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let title_input = Paragraph::new(app.task_state.new_task.title.to_owned()).block(block);

    f.render_widget(Clear, popup_chunk);
    f.render_widget(title_input, popup_chunk);

    f.set_cursor(
        popup_chunk.x + app.task_state.new_task.title.width() as u16 + 1,
        popup_chunk.y + 1,
    );
}

fn draw_new_timer_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let popup_chunk = centered_rect(60, 3, area);

    let block = Block::default()
        .title("New Timer")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let title_input = Paragraph::new(app.timer_state.new_timer.title.to_owned()).block(block);

    f.render_widget(Clear, popup_chunk);
    f.render_widget(title_input, popup_chunk);

    f.set_cursor(
        popup_chunk.x + app.timer_state.new_timer.title.width() as u16 + 1,
        popup_chunk.y + 1,
    );
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    // TODO: Might have some edge cases with specific viewports
    let empty_space = ((r.height - height) * 100) / r.height / 2;

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(empty_space),
                Constraint::Percentage(100 - empty_space * 2),
                Constraint::Percentage(empty_space),
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

fn draw_task_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let task_list_block = Block::default().borders(Borders::ALL).title("Task List");

    let tasks: Vec<ListItem> = app
        .task_state
        .tasks
        .items
        .iter()
        .map(|task| {
            ListItem::new(vec![Spans::from(vec![
                Span::raw(if task.is_completed { "[*]" } else { "[ ]" }),
                Span::raw(" - "),
                Span::raw(format!("\"{}\"", &task.title)),
            ])])
            .style(Style::default().fg(if task.is_completed {
                Color::Green
            } else {
                Color::Red
            }))
        })
        .collect();

    if tasks.len() == 0 {
        let empty_information = Paragraph::new(Span::styled(
            "You don't have any tasks! Create one using ('n' key).",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        ))
        .block(task_list_block);

        f.render_widget(empty_information, chunks[0]);
    } else {
        let tasks = List::new(tasks).block(task_list_block).highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .add_modifier(Modifier::BOLD),
        );

        f.render_stateful_widget(tasks, chunks[0], &mut app.task_state.tasks.state);
    }
}

fn draw_timers_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let timers: Vec<ListItem> = app
        .timer_state
        .timers
        .items
        .iter()
        .map(|timer| {
            let timer_seconds = timer.time_active.as_secs();
            let formatted_active_time_information = format!(
                "{:02}:{:02}:{:02}",
                timer_seconds / 3600,
                timer_seconds / 60,
                timer_seconds % 60
            );

            let lines = vec![
                Spans::from(vec![
                    Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("\"{}\"", &timer.title)),
                ]),
                Spans::from(vec![
                    Span::styled(" - Status: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(if timer.is_active {
                        "[Active]"
                    } else {
                        "[Inactive]"
                    }),
                ]),
                Spans::from(vec![
                    Span::styled(
                        " - Active Duration: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(formatted_active_time_information),
                ]),
                Spans::from(vec![
                    Span::styled(
                        " - Creation Date: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(timer.time_created.to_string()),
                ]),
            ];

            ListItem::new(lines).style(Style::default().fg(if timer.is_active {
                Color::Green
            } else {
                Color::Red
            }))
        })
        .collect();

    let timer_list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Timer List");

    if timers.len() == 0 {
        let empty_information = Paragraph::new(Span::styled(
            "You don't have any timers! Create one using ('n' key).",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        ))
        .block(timer_list_block);

        f.render_widget(empty_information, chunks[0]);
    } else {
        let timers = List::new(timers)
            .block(timer_list_block)
            .highlight_style(Style::default().bg(Color::Rgb(50, 50, 50)));

        f.render_stateful_widget(timers, chunks[0], &mut app.timer_state.timers.state);
    }
}
