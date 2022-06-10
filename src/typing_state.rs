#[derive(Debug)]
pub struct TypingState {
    pub typed:      String,
    pub mistyped:   String,
    pub current:    String,
    pub untyped:    String,
}

impl TypingState {
    pub fn new(text: &str) -> Self {
        TypingState {
            typed: String::from(""),
            mistyped: String::from(""),
            current: String::from(text.chars().nth(0).unwrap()),
            untyped: text[1..].to_string(),
        }
    }

    // This function updates the state of the typing practice
    pub fn update_state(&mut self, input: char){
        // When the input is the mistyped char (the correct value) move it to the current char
        if self.mistyped == input.to_string() {
            self.current = self.mistyped.clone();
            self.mistyped = String::from("");
        }
        // When the input is the current char shift to the left
        if self.current == input.to_string() {
            self.typed = format!("{}{}", &self.typed, &self.current);
            if !self.untyped.is_empty(){
                self.current = self.untyped.remove(0).to_string();
            }
            else {
                self.current = String::from("");
            }
        }
        // When the input is mistyped move the current char to mistyped
        else if self.mistyped == ""{
            self.mistyped = self.current.clone();
            self.current = String::from("");
        }
    }

    // When untyped and current are empty the practice is completed
    pub fn is_complete(&self) -> bool{
        self.untyped.is_empty() && self.current.is_empty()
    }
}

// Implement the "==" so the struct can be compared
impl PartialEq for TypingState {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed &&
        self.mistyped == other.mistyped &&
        self.current == other.current &&
        self.untyped == other.untyped
    }
}

impl Eq for TypingState {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Tests that the state doesn't panic when is completed
    fn update_state_completed(){
        let mut sentence = TypingState::new("T.");
        sentence.update_state('T');
        sentence.update_state('.');

        let solution = TypingState {
            typed: String::from("T."),
            mistyped: String::from(""),
            current: String::from(""),
            untyped: String::from(""),
        };

        assert_eq!(solution, sentence);
    }

    #[test]
    // Tests that the state retains the correct values after a char was mistyped several times
    fn update_state_double_mistyped(){
        let mut sentence = TypingState::new("This is a simple sentence.");
        sentence.update_state('t');
        sentence.update_state('d');

        let solution = TypingState {
            typed: String::from(""),
            mistyped: String::from("T"),
            current: String::from(""),
            untyped: String::from("his is a simple sentence."),
        };

        assert_eq!(solution, sentence);
    }

    #[test]
    // Tests that the state was updated correctly after a char was mistyped
    fn update_state_mistyped(){
        let mut sentence = TypingState::new("This is a simple sentence.");
        sentence.update_state('t');

        let solution = TypingState {
            typed: String::from(""),
            mistyped: String::from("T"),
            current: String::from(""),
            untyped: String::from("his is a simple sentence."),
        };

        assert_eq!(solution, sentence);
    }

    #[test]
    // Tests that the state was updated correctly
    fn update_state(){
        let mut sentence = TypingState::new("This is a simple sentence.");
        sentence.update_state('T');

        let solution = TypingState {
            typed: String::from("T"),
            mistyped: String::from(""),
            current: String::from("h"),
            untyped: String::from("is is a simple sentence."),
        };

        assert_eq!(solution, sentence);
    }

    #[test]
    // Tests that the state was created correctly from an &str
    fn typing_state(){
        let sentence = TypingState::new("This is a simple sentence.");
        let solution = TypingState {
            typed: String::from(""),
            mistyped: String::from(""),
            current: String::from("T"),
            untyped: String::from("his is a simple sentence."),
        };

        assert_eq!(solution, sentence);
    }

}
