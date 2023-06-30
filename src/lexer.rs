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

    // Misc
    Identifier(String),
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
}
