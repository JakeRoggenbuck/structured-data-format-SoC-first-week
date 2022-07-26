#[derive(PartialEq, Debug, Clone)]
enum Tokens {
    SquareRight,
    SquareLeft,
    Bar,
    ParenRight,
    ParenLeft,
    Equals,
    Greater,
    Dot,

    Identifier,
    Int,
    Float,
    String,

    IntValue(Value<i64>),
    FloatValue(Value<f64>),
    StringValue(Value<String>),
}

static VALUE_STRING: &'static [Tokens] = &[
    Tokens::ParenLeft,
    Tokens::String,
    Tokens::Equals,
    Tokens::Greater,
    Tokens::String,
    Tokens::ParenRight,
];

static VALUE_FLOAT: &'static [Tokens] = &[
    Tokens::ParenLeft,
    Tokens::String,
    Tokens::Equals,
    Tokens::Greater,
    Tokens::Float,
    Tokens::ParenRight,
];

static VALUE_INT: &'static [Tokens] = &[
    Tokens::ParenLeft,
    Tokens::String,
    Tokens::Equals,
    Tokens::Greater,
    Tokens::Int,
    Tokens::ParenRight,
];

fn test_structure(structure: &[Tokens], tokens: &Vec<Token>, index: &mut usize) -> bool {
    let mut i: usize = 0;

    if structure.len() + *index + 1 > tokens.len() {
        return false;
    }

    for s in structure {
        // print!("{:?} == {:?}", *s, tokens[*index + i].token);
        if *s == tokens[*index + i].token {
        } else {
            return false;
        }

        i += 1;
    }
    *index += structure.len();

    return true;
}

#[derive(PartialEq, Debug, Clone)]
struct Value<T> {
    name: String,
    value: T,
}

struct Dict<T> {
    key: String,
    value: T,
}

struct ListItem<T> {
    item: T,
}

struct List<T> {
    name: String,
    items: Vec<ListItem<T>>,
}

#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
}

fn tokenize(part: &str) -> Token {
    let token = match part {
        "]" => Tokens::SquareRight,
        "[" => Tokens::SquareLeft,
        "|" => Tokens::Bar,
        ")" => Tokens::ParenRight,
        "(" => Tokens::ParenLeft,
        "=" => Tokens::Equals,
        ">" => Tokens::Greater,
        "." => Tokens::Dot,
        _ => {
            if part.contains("\"") {
                Tokens::String
            } else if is_part_numeric(part) {
                if part.contains(".") {
                    Tokens::Float
                } else {
                    Tokens::Int
                }
            } else {
                Tokens::Identifier
            }
        }
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

            if !in_string {
                if is_char_numeric(next) {
                    in_number = true;
                    *index += 1;
                    continue;
                } else {
                    if in_number {
                        *index += 1;
                        return tokenize(&buffer);
                    }
                }

                if ends_token(current, next) {
                    *index += 1;
                    return tokenize(&buffer);
                }
            }
        }

        *index += 1;
    }
}

fn main() {
    let mut index = 0;
    let chrs: Vec<char> = "
( \"version\" => \"0.1.0\" )

( \"name\" => \"Jake\" )

( \"fav_num\" => 2.71828 )

( \"rounded\" => 2 )

"
    .chars()
    .collect();
    let mut lex_eof = false;

    let mut current_token: Token;
    let mut stack = Vec::<Token>::new();
    while lex_eof == false {
        current_token = next(&mut index, &chrs, &mut lex_eof);
        stack.push(current_token);
    }

    index = 0;
    loop {
        if index + 1 > stack.len() {
            break;
        }

        let mut new: Tokens = Tokens::Bar;
        if test_structure(VALUE_STRING, &stack, &mut index) {
            new = Tokens::StringValue(Value {
                name: stack[index - VALUE_STRING.len() + 1].part.clone(),
                value: stack[index - VALUE_STRING.len() + 4].part.clone(),
            });

            // for x in index..index + VALUE_STRING.len() {
            //     stack.remove(x);
            // }

            // stack.insert(
            //     index,
            //     Token {
            //         part: String::new(),
            //         token: new.clone(),
            //     },
            // );
            index -= 1;
        } else if test_structure(VALUE_INT, &stack, &mut index) {
            new = Tokens::IntValue(Value {
                name: stack[index - VALUE_INT.len() + 1].part.clone(),
                value: stack[index - VALUE_INT.len() + 4]
                    .part
                    .parse::<i64>()
                    .unwrap(),
            });
            index -= 1;
        } else if test_structure(VALUE_FLOAT, &stack, &mut index) {
            new = Tokens::FloatValue(Value {
                name: stack[index - VALUE_FLOAT.len() + 1].part.clone(),
                value: stack[index - VALUE_FLOAT.len() + 4]
                    .part
                    .parse::<f64>()
                    .unwrap(),
            });
            index -= 1;
        }
        println!("{:?}", new);

        index += 1;
    }
}
