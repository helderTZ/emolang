use std::io::{stdout, Write};

struct VirtualMachine {
    vars: Vec<Variable>,
    code: Vec<Instruction>,
    ip: usize,
}

struct Variable {

}

enum OpCode {
    EmoPrint,
    EmoDecl,
    EmoSum,
    EmoSub,
    EmoMul,
    EmoDiv,
    EmoMod,
    EmoCond,
}

#[derive(Debug)]
enum TokenId {
    EmojiPointRight,
    EmojiPointLeft,
    EmojiIf,
    EmojiElse,
    EmojiLoop,
    EmojiNot,
    EmojiShout,
    EmojiFistBumpRight,
    EmojiFistBumpLeft,
    EmojiConcat,
    EmojiCalc,
    EmojiDecl,
    Plus,
    Minus,
    Mult,
    Div,
    Equals,
    Assign,
    Identifier,
    UserString,
}

struct Args {
    int_args: Vec<i32>,
    str_args: Vec<String>,
}

struct Instruction {
    opcode: OpCode,
    args: Vec<Args>
}

#[derive(Debug)]
struct Token {
    token_id : TokenId,
    line : usize,
    col : usize,
    token_str : Option<String>,
}

impl Token {
    pub fn new(token_id: TokenId, line: usize, col: usize, token_str: Option<String>) -> Token {
        Token {token_id, line, col, token_str}
    }
}

fn emo_print(string: &str) -> Instruction {
    Instruction {
        opcode: OpCode::EmoPrint,
        args: vec![Args {
            int_args: vec![],
            str_args: vec![String::from(string)],
        }],
    }
}

fn emo_decl<T>(var_name: &str, var_value: &T) {

}

// fn parser(tokens: &str) -> Vec<Instruction> {
// }

fn lexer(source_code: &str) -> Vec<Token> {
    let mut tokens : Vec<Token> = vec![];
    
    for (line_nr, line) in source_code.lines().enumerate() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let mut col_nr : usize = 0;
        for i in 0..words.len() {
            let word = words[i];

            // handle user string case
            // TODO: not working for newlines in user strings yet
            if word.chars().nth(0).unwrap() == '\"' {
                let mut user_string = String::from(word);
                if user_string.ends_with('\"') {
                    tokens.push(Token::new(TokenId::UserString, line_nr, col_nr, Some(user_string)));
                    continue
                } else {
                    let mut j = 1;
                    let mut next_word = words[i+j];
                    while !next_word.ends_with('\"') {
                        user_string.push_str(" ");
                        user_string.push_str(next_word);
                        j += 1;
                        next_word = words[i+j];
                    }
                    user_string.push_str(" ");
                    user_string.push_str(next_word);
                    tokens.push(Token::new(TokenId::UserString, line_nr, col_nr, Some(user_string)));
                    continue
                }
            }

            // other cases
            match word {
                "🤬" => tokens.push(Token::new(TokenId::EmojiShout, line_nr, col_nr, None)),
                "👉" => tokens.push(Token::new(TokenId::EmojiPointRight, line_nr, col_nr, None)),
                "👈" => tokens.push(Token::new(TokenId::EmojiPointLeft, line_nr, col_nr, None)),
                "🤔" => tokens.push(Token::new(TokenId::EmojiIf, line_nr, col_nr, None)),
                "😅" => tokens.push(Token::new(TokenId::EmojiElse, line_nr, col_nr, None)),
                "🖕" => tokens.push(Token::new(TokenId::EmojiNot, line_nr, col_nr, None)),
                "🙃" => tokens.push(Token::new(TokenId::EmojiLoop, line_nr, col_nr, None)),
                "🤜" => tokens.push(Token::new(TokenId::EmojiFistBumpRight, line_nr, col_nr, None)),
                "🤛" => tokens.push(Token::new(TokenId::EmojiFistBumpLeft, line_nr, col_nr, None)),
                "🤯" => tokens.push(Token::new(TokenId::EmojiCalc, line_nr, col_nr, None)),
                "🤝" => tokens.push(Token::new(TokenId::EmojiConcat, line_nr, col_nr, None)),
                "😶" => tokens.push(Token::new(TokenId::EmojiDecl, line_nr, col_nr, None)),
                _ => tokens.push(Token::new(TokenId::Identifier, line_nr, col_nr, None)),
            }
            col_nr += word.len();
        }
    }

    tokens
}

fn main() {
    let mut line = String::new();
    print!("👉 ");
    std::io::stdout().flush().unwrap();
    let bytes = std::io::stdin().read_line(&mut line).unwrap();
    let tokens = lexer(&line);
    println!("{:?}", tokens);
}
