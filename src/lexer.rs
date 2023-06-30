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

struct Lexer {
    input: Vec<u8>,
    character: u8,
    current_position: usize,
    next_position: usize,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Self {
            input: input.into_bytes(),
            character: 0,
            current_position: 0,
            next_position: 0,
        };

        lexer.read_character();
        lexer
    }

    fn read_character(&mut self) {
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
            self.read_character();
        }

        let token = match self.character {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let starting_position = self.current_position;
                while self.character.is_ascii_alphanumeric() || self.character == b'_' {
                    self.read_character();
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
                    self.read_character();
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
        };
    }
}
