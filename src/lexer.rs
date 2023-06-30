// TODO: Good error messages.

#[derive(Debug, PartialEq)]
enum Token {
    // Keywords
    Begin,
    End,
    IfPositive,
    IfZero,
    IfNegative,
    Else,
    Loop,
    Break,
    Print,
    Read,

    // Literals
    Number(i32),

    // Operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Assign,

    // Separators
    LeftParenthesis,
    RightParenthesis,

    // Misc
    Identifier(String),
}

/// Bitsy's lexer. Implements the [Iterator] trait, returning a stream of tokens.
struct Lexer {
    /// The loaded program as a vector of bytes.
    input: Vec<u8>,
    /// The current character.
    character: u8,
    /// The current position in the input.
    current_position: usize,
    /// The next position in the input.
    next_position: usize,
}

impl Lexer {
    /// Creates a new lexer from the given input.
    fn new(input: String) -> Lexer {
        let mut lexer = Self {
            input: input.into_bytes(),
            character: 0,
            current_position: 0,
            next_position: 0,
        };

        // Set the current character to the first character.
        lexer.set_next_character();
        lexer
    }

    /// Sets the current character to the next character.
    fn set_next_character(&mut self) {
        // If no next character, set to 0 (EOF).
        if self.next_position >= self.input.len() {
            self.character = 0;
        } else {
            self.character = self.input[self.next_position];
        }

        self.current_position = self.next_position;
        self.next_position += 1;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace.
        while self.character.is_ascii_whitespace() {
            self.set_next_character();
        }

        let token = match self.character {
            b'{' => {
                while self.character != b'}' {
                    self.set_next_character();
                }

                self.set_next_character();
                return self.next();
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let starting_position = self.current_position;
                while self.character.is_ascii_alphanumeric() || self.character == b'_' {
                    self.set_next_character();
                }

                let keyword = String::from_utf8(self.input[starting_position..self.current_position].to_vec()).unwrap();
                return Some(match keyword.as_str() {
                    "BEGIN" => Token::Begin,
                    "END" => Token::End,
                    "IFP" => Token::IfPositive,
                    "IFZ" => Token::IfZero,
                    "IFN" => Token::IfNegative,
                    "ELSE" => Token::Else,
                    "LOOP" => Token::Loop,
                    "BREAK" => Token::Break,
                    "PRINT" => Token::Print,
                    "READ" => Token::Read,
                    _ => Token::Identifier(keyword),
                })
            },
            b'0'..=b'9' => {
                let starting_position = self.current_position;
                while self.character.is_ascii_digit() {
                    self.set_next_character();
                }

                let number = String::from_utf8(self.input[starting_position..self.current_position].to_vec()).unwrap();
                return Some(Token::Number(number.parse::<i32>().unwrap()));
            },
            b'+' => Token::Add,
            b'-' => Token::Subtract,
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'%' => Token::Modulo,
            b'=' => Token::Assign,
            b'(' => Token::LeftParenthesis,
            b')' => Token::RightParenthesis,
            0 => return None,
            _ => unreachable!("Invalid character: {}", self.character as char),
        };

        self.set_next_character();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_correctly() {
        let sample_program = String::from("
            {This is a sample program. 🤠}
            BEGIN
                my_number = -5
                IFP my_number
                    PRINT my_number {Won't happen.}
                ELSE
                    PRINT -999
                END

                IFZ my_number
                    PRINT my_number {Also won't happen.}
                END

                IFN my_number
                    PRINT my_number {This will actually happen.}
                END

                READ cool
                PRINT cool + 1
                PRINT cool - 1
                PRINT cool * 2
                PRINT cool / 2
                PRINT cool % 2
                PRINT (cool + (5 * 2))

                current = 10
                LOOP
                    PRINT current
                    current = current - 1
                    IFZ cool
                        BREAK
                    END
                END
            END
        ");

        let tokens: Vec<Token> = Lexer::new(sample_program).collect();
        assert_eq!(tokens, vec![
            Token::Begin,

            Token::Identifier(String::from("my_number")),
            Token::Assign,
            Token::Subtract,
            Token::Number(5),

            Token::IfPositive,
            Token::Identifier(String::from("my_number")),

            Token::Print,
            Token::Identifier(String::from("my_number")),

            Token::Else,

            Token::Print,
            Token::Subtract,
            Token::Number(999),

            Token::End,

            Token::IfZero,
            Token::Identifier(String::from("my_number")),

            Token::Print,
            Token::Identifier(String::from("my_number")),

            Token::End,

            Token::IfNegative,
            Token::Identifier(String::from("my_number")),

            Token::Print,
            Token::Identifier(String::from("my_number")),

            Token::End,

            Token::Read,
            Token::Identifier(String::from("cool")),

            Token::Print,
            Token::Identifier(String::from("cool")),
            Token::Add,
            Token::Number(1),

            Token::Print,
            Token::Identifier(String::from("cool")),
            Token::Subtract,
            Token::Number(1),

            Token::Print,
            Token::Identifier(String::from("cool")),
            Token::Multiply,
            Token::Number(2),

            Token::Print,
            Token::Identifier(String::from("cool")),
            Token::Divide,
            Token::Number(2),

            Token::Print,
            Token::Identifier(String::from("cool")),
            Token::Modulo,
            Token::Number(2),

            Token::Print,
            Token::LeftParenthesis,
            Token::Identifier(String::from("cool")),
            Token::Add,
            Token::LeftParenthesis,
            Token::Number(5),
            Token::Multiply,
            Token::Number(2),
            Token::RightParenthesis,
            Token::RightParenthesis,

            Token::Identifier(String::from("current")),
            Token::Assign,
            Token::Number(10),

            Token::Loop,

            Token::Print,
            Token::Identifier(String::from("current")),

            Token::Identifier(String::from("current")),
            Token::Assign,
            Token::Identifier(String::from("current")),
            Token::Subtract,
            Token::Number(1),

            Token::IfZero,
            Token::Identifier(String::from("cool")),

            Token::Break,

            Token::End,

            Token::End,

            Token::End,
        ]);
    }

    #[test]
    fn allows_single_line_programs() {
        let tokens: Vec<Token> = Lexer::new(String::from("BEGIN PRINT lol END")).collect();
        assert_eq!(tokens, vec![
            Token::Begin,
            Token::Print,
            Token::Identifier(String::from("lol")),
            Token::End,
        ]);
    }

    #[test]
    fn handles_empty_input() {
        let tokens: Vec<Token> = Lexer::new(String::from("")).collect();
        assert_eq!(tokens, vec![]);
    }
}
