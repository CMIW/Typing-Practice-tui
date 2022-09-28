use tui::{
    text::{Text, Span, Spans},
    style::{Color, Modifier, Style},
};
use typing_state::TypingState;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TuiTypingState(TypingState);

impl TuiTypingState {
    pub fn new(text: &str) -> Self {
        TuiTypingState {
            0: TypingState::new(text)
        }
    }

    pub fn update_state(&mut self, input: char) {
        self.0.update_state(input);
    }

    pub fn is_complete(&self)-> bool{
        self.0.is_complete()
    }
}

// We need to separate the "\n" escape character into different tui::text::Spans to mimic the
// new line when displaying with tui::widgets::Paragraph.
impl<'a> Into<tui::text::Text<'a>> for &TuiTypingState {
    fn into(self) -> tui::text::Text<'a> {
        // The different styles we need to display
        let typed_style = Style::default().fg(Color::Gray);
        let mistyped_style = Style::default().fg(Color::Red).bg(Color::White)
        .add_modifier(Modifier::BOLD);
        let current_style = Style::default().fg(Color::Black).bg(Color::White)
        .add_modifier(Modifier::BOLD);

        // Split the new lines and collect them into vectors
        // We use a map functio to convert the &str from the split into Strings
        let mut vec0: Vec<String> = self.0.typed.split("\n").into_iter().map(|x| x.to_owned()).collect();
        let mut vec1: Vec<String> = self.0.untyped.split("\n").into_iter().map(|x| x.to_owned()).collect();

        // We need the last element of vector0 into a span
        let typed_span = Span::styled(vec0.pop().unwrap(), typed_style);
        let mistyped_span = Span::styled(self.0.mistyped.clone(), mistyped_style);
        let current_span = Span::styled(self.0.current.clone(), current_style);
        // We need the first element of vector1 into a span
        let untyped_span = Span::styled(vec1.remove(0), Style::default());

        // Create the tui::text::Text struct
        let mut text = Text::styled(vec0.join("\n"), typed_style);
        // When there is a new line on mistyped or current we need to separate the untyped_span into
        // it's own tui::text::Spans to mimic the new line
        if self.0.mistyped == "\n" || self.0.current == "\n" {
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
}
