
pub enum Token {
    Identifior(String),
    Integer(i64),
    Floater(f64),
    StrLiteral(String),
    CharLiteral(char),
    Other(String),
}

pub struct LexerTokens {
    tokens: Vec<Token>
}

impl LexerTokens {
    pub fn parse(src: &'static str, strict: bool) -> LexerTokens {
        let mut tokens = Vec::new();
        let mut str = String::new();
        /* 
            0 => Undecided
            1 => Number
            2 => Identifier
            3 => Decimal Number
            4 => Undecided Dot
            5 => Character Literal
            6 => String Literal
        */
        let mut state = 0 as u8;
        for c in src.chars() {
            match c {
                '0'..='9' => { push(c, proc_digit(state), &mut state, &mut tokens, &mut str); }
                'a'..='z' | 'A'..='Z' => { push(c, proc_letter(state), &mut state, &mut tokens, &mut str); }
                '.' => { push(c, proc_dot(state), &mut state, &mut tokens, &mut str); }
                '\'' => {
                    let change = {
                        let s = strict as u8;
                        let l = 6 * (!strict) as u8;
                        l + proc_quote(true) * s
                    };
                    push(c, change, &mut state, &mut tokens, &mut str)
                }
                '\"' => { push(c, 6, &mut state, &mut tokens, &mut str) }
                _ => { push(c, 255, &mut state, &mut tokens, &mut str); }
            }
        }
        LexerTokens { tokens }
    }
    pub fn tokens(&self) -> &Vec<Token> { &self.tokens }
}

#[inline]
fn proc_digit(state: u8) -> u8 {
    match state {
        1|2 => { 0 }
        _ => { 1 }
    }
}

#[inline]
fn proc_quote(single: bool) -> u8 {
    match single {
        false => { 6 }
        true => { 5 }
    }
}

#[inline]
fn proc_letter(state: u8) -> u8 {
    match state {
        0 => { 2 }
        2 => { 0 }
        _ => { 2 }
    }
}

#[inline]
fn proc_dot(state: u8) -> u8 {
    match state {
        0 => { 4 }
        1 => { 3 }
        2 => { 4 }
        _ => { 4 }
    }
}

#[inline]
fn pop(state: u8, str: &String) -> Option<Token> {
    match state {
        0 => { None }
        1 => { Some(Token::Integer(str.parse().unwrap())) }
        2 => { Some(Token::Identifior(str.clone())) }
        _ => { Some(Token::Other(str.clone())) }
    }
}

#[inline]
fn push(c: char, change: u8, state: &mut u8, tokens: &mut Vec<Token>, str: &mut String) {
    if change > 0 {
        let res = pop(*state, &str);
        if res.is_some() { tokens.push(res.unwrap()); }
        *state = change;
        str.clear();
    }
    if change >= 6 && change <=5 { str.push(c); }
}
