#[derive(PartialEq, Debug, Clone)]
enum Tokens {
    EOF,
    DoubleQuote,
    SquareRight,
    SquareLeft,
    Bar,
    ParenRight,
    ParenLeft,
    Equals,
    Greater,
    Dot,
    None,
}

#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
}

fn tokenize(part: &str) -> Token {
    let token = match part {
        "\"" => Tokens::DoubleQuote,
        "]" => Tokens::SquareRight,
        "[" => Tokens::SquareLeft,
        "|" => Tokens::Bar,
        ")" => Tokens::ParenRight,
        "(" => Tokens::ParenLeft,
        "=" => Tokens::Equals,
        ">" => Tokens::Greater,
        "." => Tokens::Dot,
        _ => Tokens::None,
    };

    return Token {
        part: part.to_string(),
        token,
    };
}

fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

fn is_char_symbol(ch: char) -> bool {
    match ch {
        ']' | '[' | '|' | '(' | ')' | '=' | '>' | '.' => true,
        _ => false,
    }
}

fn is_char_numeric(ch: char) -> bool {
    return ch.is_digit(10) || ch == '.';
}

fn is_part_numeric(part: &str) -> bool {
    for c in part.chars() {
        if is_char_numeric(c) {
            return true;
        }
    }
    return false;
}

fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }
    if is_char_symbol(cur) {
        return true;
    }
    if is_char_symbol(next) {
        return true;
    }
    if is_char_whitespace(cur) {
        return false;
    }
    return false;
}

fn next(index: &mut usize, chars: &Vec<char>, lex_eof: &mut bool) -> Token {
    let mut buffer = String::new();
    let mut in_string: bool = false;
    let mut in_number: bool = false;

    loop {
        if *index + 1 == chars.len() {
            *lex_eof = true;
            buffer.push(chars[*index]);
            return tokenize(&buffer);
        }

        let current: char = chars[*index];
        let next: char = chars[*index + 1];

        if !is_char_whitespace(current) {
            buffer.push(current);

            if is_char_numeric(current) {
                in_number = true;
                *index += 1;
                continue;
            } else {
                if in_number {
                    *index += 1;
                    return tokenize(&buffer);
                }
            }

            if current == '"' {
                if in_string {
                    *index += 1;
                    return tokenize(&buffer);
                } else {
                    in_string = true;
                }

                *index += 1;
                continue;
            }

            if ends_token(current, next) {
                *index += 1;
                return tokenize(&buffer);
            }
        }

        *index += 1;
    }
}

fn main() {
    let mut index = 0;
    let chrs: Vec<char> = "project() ( \"version\" => 0.1.0 )".chars().collect();
    let mut lex_eof = false;

    let mut current_token: Token;
    while lex_eof == false {
        current_token = next(&mut index, &chrs, &mut lex_eof);

        println!("{:?}", current_token);
    }
}
