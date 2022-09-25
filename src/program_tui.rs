use std::{error::Error, io};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers, DisableMouseCapture, EnableMouseCapture},
};
use tui::{
    Frame,
    Terminal,
    text::{Span, Spans},
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
fn run_app<B: Backend>(terminal: &mut Terminal<B>, typing_state: &mut TypingState) -> io::Result<()>{
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

    let text_vec = vec![
        Span::styled(&typing_state.typed, Style::default().fg(Color::Gray)),
        Span::styled(
            &typing_state.mistyped,
            Style::default().fg(Color::Red).bg(Color::White).add_modifier(Modifier::BOLD)
        ),
        Span::styled(
            &typing_state.current,
            Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(&typing_state.untyped, Style::default()),
    ];

    // Style the text to be displayed
    let text = Spans::from(text_vec);

    // Display the text on screen
    let p0 = Paragraph::new(text)
    .wrap(Wrap { trim: true });
    f.render_widget(p0, chunks[1]);
}
