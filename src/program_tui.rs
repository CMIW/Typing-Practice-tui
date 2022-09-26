use std::{error::Error, io};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers, DisableMouseCapture, EnableMouseCapture},
};
use tui::{
    Frame,
    Terminal,
    text::{Text, Span, Spans},
    style::{Color, Modifier, Style},
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Layout, Direction, Constraint},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use typing_state::TypingState;

pub fn run_tui(typing_state: &mut TypingState) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, typing_state);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// Loop that displays the UI and waits for event triggers
fn run_app<B: Backend>(terminal: &mut Terminal<B>, typing_state: &mut TypingState)
-> io::Result<()>{
    loop{
        // display ui
        terminal.draw(|f| ui(f, typing_state))?;
        // match key press events
        if let Event::Key(key) = event::read()? {
            match key {
                // match Ctrl + c to exit the program
                KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL} => {
                    return Ok(());
                }
                // do nothing on everything else
                _ => {
                    match key.code {
                        // match every character
                        KeyCode::Char(c) => { typing_state.update_state(c);}
                        KeyCode::Enter => { typing_state.update_state('\n');}
                        // do nothing on everything else
                        _ => {/*println!{"{:?}", key.code}*/}
                    }
                }
            }
        }

        // TODO: do something when the practice is done rather than closing the program
        if typing_state.is_complete(){
            return Ok(());
        }
    }
}

// We need to separate the "\n" escape character into different tui::text::Spans to mimic the
// new line when displaying with tui::widgets::Paragraph.
fn into_text<'a>(t_s: &TypingState) -> Text<'static>{
    // The different styles we need to display
    let typed_style = Style::default().fg(Color::Gray);
    let mistyped_style = Style::default().fg(Color::Red).bg(Color::White)
    .add_modifier(Modifier::BOLD);
    let current_style = Style::default().fg(Color::Black).bg(Color::White)
    .add_modifier(Modifier::BOLD);

    // Split the new lines and collect them into vectors
    // We use a map functio to convert the &str from the split into Strings
    let mut vec0: Vec<String> = t_s.typed.split("\n").into_iter().map(|x| x.to_owned()).collect();
    let mut vec1: Vec<String> = t_s.untyped.split("\n").into_iter().map(|x| x.to_owned()).collect();

    // We need the last element of vector0 into a span
    let typed_span = Span::styled(vec0.pop().unwrap(), typed_style);
    let mistyped_span = Span::styled(t_s.mistyped.clone(), mistyped_style);
    let current_span = Span::styled(t_s.current.clone(), current_style);
    // We need the first element of vector1 into a span
    let untyped_span = Span::styled(vec1.remove(0), Style::default());

    // Create the tui::text::Text struct
    let mut text = Text::styled(vec0.join("\n"), typed_style);
    // When there is a new line on mistyped or current we need to separate the untyped_span into
    // it's own tui::text::Spans to mimic the new line
    if t_s.mistyped == "\n" || t_s.current == "\n" {
        let text_vec0: Spans = Spans([typed_span, mistyped_span, current_span].to_vec());
        let text_vec1: Spans = Spans([untyped_span].to_vec());
        text.lines.extend_from_slice(&[text_vec0]);
        text.lines.extend_from_slice(&[text_vec1]);
    }
    else{
        let text_vec: Spans = Spans(
            [typed_span, mistyped_span, current_span, untyped_span].to_vec()
        );
        text.lines.extend_from_slice(&[text_vec]);
    }
    text.extend(Text::from(vec1.join("\n")));

    text
}

// Layout for the ui
fn ui<B: Backend>(f: &mut Frame<B>, typing_state: &TypingState) {
    let size = f.size();

    // Vertical layout
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
    .split(size);

    // Title block
    let title = Paragraph::new("Typing Practice")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::LightCyan))
    .block(
        Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White))
    );
    f.render_widget(title, chunks[0]);

    let text = into_text(typing_state);

    // Display the text on screen
    let p0 = Paragraph::new(text)
    .wrap(Wrap { trim: true });
    f.render_widget(p0, chunks[1]);
}
