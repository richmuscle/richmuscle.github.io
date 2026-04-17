//! Minimal Rust syntax highlighter — tokenizes keywords, types, fn calls,
//! numbers, strings/char literals, comments, and attributes into token spans.

use super::tok;
use crate::utils::text::esc;

const RUST_KW: &[&str] = &[
    "fn", "let", "mut", "pub", "use", "mod", "impl", "struct", "enum", "trait", "where", "for",
    "in", "if", "else", "match", "return", "loop", "while", "break", "continue", "type", "const",
    "static", "self", "Self", "super", "crate", "extern", "unsafe", "async", "await", "dyn", "ref",
    "move", "box", "true", "false", "as", "impl", "Some", "None", "Ok", "Err",
];

pub(super) fn hl_rust(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        out.push_str(&hl_rust_line(line));
        out.push('\n');
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn hl_rust_line(line: &str) -> String {
    let ch: Vec<char> = line.chars().collect();
    let n = ch.len();
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        if ch[i] == ' ' || ch[i] == '\t' {
            out.push(ch[i]);
            i += 1;
            continue;
        }
        // line comment
        if i + 1 < n && ch[i] == '/' && ch[i + 1] == '/' {
            out.push_str(&tok("tok-cmt", &ch[i..].iter().collect::<String>()));
            break;
        }
        // block comment start
        if i + 1 < n && ch[i] == '/' && ch[i + 1] == '*' {
            let mut j = i + 2;
            while j + 1 < n && !(ch[j] == '*' && ch[j + 1] == '/') {
                j += 1;
            }
            if j + 1 < n {
                j += 2;
            }
            out.push_str(&tok("tok-cmt", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        // string
        if ch[i] == '"' || (ch[i] == 'r' && i + 1 < n && ch[i + 1] == '#') {
            // raw string r#"..."#
            if ch[i] == 'r' {
                let mut hashes = 0;
                let mut j = i + 1;
                while j < n && ch[j] == '#' {
                    hashes += 1;
                    j += 1;
                }
                if j < n && ch[j] == '"' {
                    j += 1;
                    let close: String = std::iter::once('"')
                        .chain(std::iter::repeat_n('#', hashes))
                        .collect();
                    let close_chars: Vec<char> = close.chars().collect();
                    while j < n {
                        if ch[j..].starts_with(&close_chars) {
                            j += close_chars.len();
                            break;
                        }
                        j += 1;
                    }
                    out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>()));
                    i = j;
                    continue;
                }
            }
            let mut j = i + 1;
            while j < n {
                if ch[j] == '\\' {
                    j += 2;
                } else if ch[j] == '"' {
                    j += 1;
                    break;
                } else {
                    j += 1;
                }
            }
            out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        // char literal
        if ch[i] == '\'' && i + 2 < n {
            let mut j = i + 1;
            if j < n && ch[j] == '\\' {
                j += 2;
            } else {
                j += 1;
            }
            if j < n && ch[j] == '\'' {
                j += 1;
                out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>()));
                i = j;
                continue;
            }
        }
        // attribute / lifetime
        if ch[i] == '#' && i + 1 < n && ch[i + 1] == '[' {
            let mut j = i;
            let mut d = 0;
            while j < n {
                if ch[j] == '[' {
                    d += 1;
                } else if ch[j] == ']' {
                    d -= 1;
                    if d == 0 {
                        j += 1;
                        break;
                    }
                }
                j += 1;
            }
            out.push_str(&tok("tok-attr", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        // number
        if ch[i].is_ascii_digit() || (ch[i] == '-' && i + 1 < n && ch[i + 1].is_ascii_digit()) {
            let mut j = i + 1;
            while j < n && (ch[j].is_ascii_alphanumeric() || ch[j] == '.' || ch[j] == '_') {
                j += 1;
            }
            out.push_str(&tok("tok-num", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        // identifier
        if ch[i].is_alphabetic() || ch[i] == '_' {
            let mut j = i;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == '_') {
                j += 1;
            }
            let s: String = ch[i..j].iter().collect();
            let next = if j < n { Some(ch[j]) } else { None };
            let cls = if RUST_KW.contains(&s.as_str()) {
                "tok-kw"
            } else if s.starts_with(|c: char| c.is_uppercase()) && next != Some('(') {
                "tok-ty"
            } else if next == Some('(') || next == Some('!') {
                "tok-fn"
            } else {
                ""
            };
            if cls.is_empty() {
                out.push_str(&esc(&s));
            } else {
                out.push_str(&tok(cls, &s));
            }
            i = j;
            continue;
        }
        if matches!(
            ch[i],
            '=' | '+'
                | '-'
                | '*'
                | '/'
                | '%'
                | '|'
                | '&'
                | '!'
                | '<'
                | '>'
                | ','
                | ';'
                | ':'
                | '.'
                | '('
                | ')'
                | '{'
                | '}'
                | '['
                | ']'
                | '@'
                | '`'
                | '?'
                | '\\'
        ) {
            out.push_str(&tok("tok-op", &ch[i].to_string()));
        } else {
            out.push_str(&esc(&ch[i].to_string()));
        }
        i += 1;
    }
    out
}
