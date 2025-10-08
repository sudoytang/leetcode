use std::fmt;

#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn decode_string(s: String) -> String {
        format!("{}", EncodedString::parse(&s).1)
    }
}

enum Token {
    Str(String),
    Rep(usize),
    SubPat(EncodedString),
}

struct EncodedString {
    elems: Vec<Token>,
}


impl EncodedString {
    fn new() -> Self {
        Self { elems: Vec::new() }
    }

    fn parse(s: &str) -> (usize, Self) {
        let mut es = Self::new();
        let mut i = 0;
        while i < s.len() {
            let ch = s.as_bytes()[i];
            match (es.elems.last_mut(), ch) {
                (None, b'a'..=b'z') => es.elems.push(Token::Str(String::from(ch as char))),
                (None, b'0'..=b'9') => es.elems.push(Token::Rep((ch - b'0') as usize)),
                (None, b'[') => panic!("At char #{i} of string \"{s}\": Expecting number as repetion before sub-pattern, got nothing."),
                (None, b']') => panic!("At char #{i} of string \"{s}\": Got nothing as a sub-pattern."),
                (None, _) => panic!("At char #{i} of string \"{s}\": Invalid character: {}", ch as char),

                (Some(Token::Str(s)), b'a'..=b'z') => s.push(ch as char),
                (Some(Token::Str(_)), b'0'..=b'9') => es.elems.push(Token::Rep((ch - b'0') as usize)),
                (Some(Token::Str(_)), b'[') => panic!("At char #{i} of string \"{s}\": Expecting number before '['"),
                (Some(Token::Str(_)), b']') => return (i, es),
                (Some(Token::Str(_)), _) => panic!("At char #{i} of string \"{s}\": Invalid character: {}", ch as char),

                (Some(Token::Rep(_)), b'a'..=b'z') => panic!("At char #{i} of string \"{s}\": Expecting '[', got {}", ch),
                (Some(Token::Rep(i)), b'0'..=b'9') => *i = (*i * 10) + (ch - b'0') as usize,
                (Some(Token::Rep(_)), b'[') => {
                    let (end, subpat) = EncodedString::parse(&s[i+1..]);
                    es.elems.push(Token::SubPat(subpat));
                    i = end + (i+1);
                }
                (Some(Token::Rep(_)), b']') => panic!("At char #{i} of string \"{s}\": Expecting '[', got ']'."),
                (Some(Token::Rep(_)), _) => panic!("At char #{i} of string \"{s}\": Invalid character: {}", ch as char),

                (Some(Token::SubPat(_)), b'a'..=b'z') => es.elems.push(Token::Str(String::from(ch as char))),
                (Some(Token::SubPat(_)), b'0'..=b'9') => es.elems.push(Token::Rep((ch - b'0') as usize)),
                (Some(Token::SubPat(sp)), b'[') => panic!("At char #{i} of string \"{s}\": Expecting number as repetion before sub-pattern, got sub-pattern [{sp}]."),
                (Some(Token::SubPat(_)), b']') => return (i, es),
                (Some(Token::SubPat(_)), _) => panic!("At char #{i} of string \"{s}\": Invalid character: {}", ch as char),
            }
            i += 1;
        }

        (i, es)
    }
}

impl fmt::Display for EncodedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.elems.len() {
            match &self.elems[i] {
                Token::Str(s) => write!(f, "{s}")?,
                Token::Rep(_) => continue,
                Token::SubPat(sub) => {
                    match self.elems.get(i - 1) {
                        Some(Token::Rep(rep)) => {
                            for _ in 0..*rep {
                                write!(f, "{sub}")?;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        Ok(())
    }
}
