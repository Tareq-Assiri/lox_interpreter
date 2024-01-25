use phf::phf_map;
use substring::Substring;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER(u32),
    FLOAT(f32),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
    ERR,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    // literal ?
    // I dont know how to define the literal ???
}

#[derive(Debug, Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub fn scan(source: String) -> Vec<Token> {
    let mut scanner = Scanner {
        source,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
    };
    scan_tokens(&mut scanner)
}
static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::AND,
    "class" => TokenType::CLASS,
    "else" => TokenType::ELSE,
    "false" => TokenType::FALSE,
    "for" => TokenType::FOR,
    "fun" => TokenType::FUN,
    "if" => TokenType::IF,
    "nil" => TokenType::NIL,
    "or" => TokenType::OR,
    "print" => TokenType::PRINT,
    "return" => TokenType::RETURN,
    "super" => TokenType::SUPER,
    "this" => TokenType::THIS,
    "true" => TokenType::TRUE,
    "var" => TokenType::VAR,
    "while" => TokenType::WHILE,
};
pub fn scan_tokens(scanner: &mut Scanner) -> Vec<Token> {
    println!("{}", scanner.source);
    while !at_end(scanner) {
        scanner.start = scanner.current;
        scan_token(scanner)
    }
    scanner.tokens.push(Token {
        token_type: TokenType::EOF,
        lexeme: "".to_string(),
        line: scanner.line,
    });
    scanner.tokens.clone()
}

fn scan_token(scanner: &mut Scanner) {
    match advance(scanner) {
        '(' => add_token(
            Token {
                token_type: TokenType::LEFT_PAREN,
                lexeme: "(".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        ')' => add_token(
            Token {
                token_type: TokenType::RIGHT_PAREN,
                lexeme: ")".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '{' => add_token(
            Token {
                token_type: TokenType::LEFT_BRACE,
                lexeme: "{".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '}' => add_token(
            Token {
                token_type: TokenType::RIGHT_BRACE,
                lexeme: "}".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        ',' => add_token(
            Token {
                token_type: TokenType::COMMA,
                lexeme: ",".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '.' => add_token(
            Token {
                token_type: TokenType::DOT,
                lexeme: ".".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '-' => add_token(
            Token {
                token_type: TokenType::MINUS,
                lexeme: "-".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '+' => add_token(
            Token {
                token_type: TokenType::PLUS,
                lexeme: "+".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        ';' => add_token(
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: ";".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '*' => add_token(
            Token {
                token_type: TokenType::STAR,
                lexeme: "*".to_string(),
                line: scanner.line,
            },
            scanner,
        ),
        '!' => {
            if next('=', scanner) {
                add_token(
                    Token {
                        token_type: TokenType::BANG_EQUAL,
                        lexeme: "!=".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            } else {
                add_token(
                    Token {
                        token_type: TokenType::BANG,
                        lexeme: "!".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            }
        }

        '=' => {
            if next('=', scanner) {
                add_token(
                    Token {
                        token_type: TokenType::EQUAL_EQUAL,
                        lexeme: "==".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            } else {
                add_token(
                    Token {
                        token_type: TokenType::EQUAL,
                        lexeme: "=".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            }
        }
        '<' => {
            if next('=', scanner) {
                add_token(
                    Token {
                        token_type: TokenType::LESS_EQUAL,
                        lexeme: "<=".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            } else {
                add_token(
                    Token {
                        token_type: TokenType::LESS,
                        lexeme: "<".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            }
        }
        '>' => {
            if next('=', scanner) {
                add_token(
                    Token {
                        token_type: TokenType::GREATER_EQUAL,
                        lexeme: ">=".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            } else {
                add_token(
                    Token {
                        token_type: TokenType::GREATER,
                        lexeme: ">".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            }
        }
        '/' => {
            if next('/', scanner) {
                while peek(&scanner).ne(&'\n') && !at_end(&scanner) {
                    advance(scanner);
                }
            } else {
                add_token(
                    Token {
                        token_type: TokenType::SLASH,
                        lexeme: "/".to_string(),
                        line: scanner.line,
                    },
                    scanner,
                )
            }
        }
        '"' => string(scanner),
        '\n' => scanner.line += 1,
        ' ' | '\t' | '\r' => (),
        c => {
            if c.is_digit(10) {
                number(scanner);
            } else if is_alphanumeric(c) {
                identifier(scanner);
            } else {
                eprintln!("invalid : {}", c);
            }
        }
    }
}

fn add_token(token: Token, scanner: &mut Scanner) {
    scanner.tokens.push(token)
}
fn at_end(scanner: &Scanner) -> bool {
    scanner.current >= scanner.source.len()
}

fn advance(scanner: &mut Scanner) -> char {
    let char = scanner.source.chars().nth(scanner.current);
    scanner.current += 1;
    char.unwrap()
}
fn next(next_char: char, scanner: &mut Scanner) -> bool {
    let next = scanner.source.chars().nth(scanner.current).unwrap();

    if at_end(scanner) || next.ne(&next_char) {
        false
    } else {
        scanner.current += 1;
        true
    }
}
fn peek(scanner: &Scanner) -> char {
    scanner.source.chars().nth(scanner.current).unwrap_or('\0')
}
fn peek_next(scanner: &Scanner) -> char {
    scanner
        .source
        .chars()
        .nth(scanner.current + 1)
        .unwrap_or('\0')
}
fn number(scanner: &mut Scanner) {
    let mut token_type = 'u';
    while peek(scanner).is_digit(10) {
        advance(scanner);
    }
    if peek(scanner).eq(&'.') && peek_next(scanner).is_digit(10) {
        advance(scanner);
        while peek(scanner).is_digit(10) {
            println!("found float here nig");
            token_type = 'f';
            advance(scanner);
        }
    }

    let number = scanner.source.substring(scanner.start, scanner.current);
    let token = match token_type {
        'u' => TokenType::NUMBER(number.parse::<u32>().unwrap()),
        'f' => TokenType::FLOAT(number.parse::<f32>().unwrap()),
        _ => {
            eprintln!("you missed up.");
            TokenType::ERR
        }
    };

    add_token(
        Token {
            token_type: token,
            lexeme: number.to_string(),
            line: scanner.line,
        },
        scanner,
    )
}

fn string(scanner: &mut Scanner) {
    while peek(scanner).ne(&'"') && !at_end(scanner) {
        if peek(scanner).eq(&'\n') {
            scanner.line += 1;
        }
        advance(scanner);
    }
    if at_end(scanner) {
        eprintln!("unterminated String at line : {}", scanner.line);
        return;
    }
    advance(scanner);

    add_token(
        Token {
            token_type: TokenType::STRING,
            lexeme: scanner
                .source
                .substring(scanner.start + 1, scanner.current - 1)
                .to_string(),
            line: scanner.line,
        },
        scanner,
    )
}
fn identifier(scanner: &mut Scanner) {
    while is_alphanumeric(peek(scanner)) {
        advance(scanner);
    }
    let identifier = scanner.source.substring(scanner.start, scanner.current);

    let token_type = KEYWORDS.get(identifier).unwrap_or(&TokenType::IDENTIFIER);

    add_token(
        Token {
            token_type: token_type.clone(),
            lexeme: identifier.to_string(),
            line: scanner.line,
        },
        scanner,
    )
}
fn is_alphanumeric(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c.eq(&'_')
}
