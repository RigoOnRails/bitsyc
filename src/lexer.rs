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
