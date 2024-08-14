#[derive(Clone, Debug)]
pub enum Token {
    And,
    Or,
    Not,
    IfOnlyIf,
    IfThen,
    LeftParen,
    RightParen,
    Sentence(char)
}

impl Token {
    pub fn to_char(&self) -> char {
        match self {
            Token::And => '&',
            Token::Or => '|',
            Token::Not => '!',
            Token::IfOnlyIf => '~',
            Token::IfThen => '>',
            Token::LeftParen => '(',
            Token::RightParen => ')',
            Token::Sentence(c) => *c
        }
    }
}