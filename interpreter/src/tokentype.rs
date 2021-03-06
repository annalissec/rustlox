#![allow(non_camel_case_types)]


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType {
      // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, TRUE, VAR, WHILE, BREAK, CONTINUE,

  EOF
}

