use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    f.render_widget(tabs, chunks[0]);

    match app.tabs.index {
        0 => draw_task_tab(f, app, chunks[1]),
        1 => draw_timers_tab(f, app, chunks[1]),
        _ => {}
    };

    // Debugging window
    if app.display_debugger {
        draw_debugger(f, chunks[1], app.tabs.index.to_string());
    }
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
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);

    let text = vec![Spans::from("Test box")];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
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
