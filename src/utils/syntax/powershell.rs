//! PowerShell syntax highlighter — keywords, comparison operators,
//! cmdlet-style identifiers, attribute blocks, variables, and string forms.

use super::tok;
use crate::utils::text::esc;

const PS_KW: &[&str] = &[
    "if",
    "else",
    "elseif",
    "foreach",
    "for",
    "while",
    "do",
    "switch",
    "break",
    "continue",
    "return",
    "function",
    "param",
    "begin",
    "process",
    "end",
    "try",
    "catch",
    "finally",
    "throw",
    "in",
    "until",
    "trap",
    "exit",
    "true",
    "false",
    "null",
    "-eq",
    "-ne",
    "-gt",
    "-lt",
    "-le",
    "-ge",
    "-like",
    "-notlike",
    "-match",
    "-notmatch",
    "-contains",
    "-notcontains",
    "-in",
    "-notin",
    "-is",
    "-isnot",
    "-not",
    "-and",
    "-or",
    "-xor",
    "-band",
    "-bor",
    "-bnot",
    "-shl",
    "-shr",
    "-f",
];

fn ps_class(w: &str, next: Option<char>) -> &'static str {
    let lo = w.to_lowercase();
    if PS_KW.contains(&lo.as_str()) {
        return "tok-kw";
    }
    if !w.starts_with('-')
        && w.contains('-')
        && w.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false)
    {
        return "tok-fn";
    }
    let all_up = w.len() >= 2
        && w.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        && w.chars().any(|c| c.is_ascii_uppercase());
    if all_up {
        return "tok-const";
    }
    if next == Some('(') {
        return "tok-fn";
    }
    let pascal = w.starts_with(|c: char| c.is_uppercase())
        && w.chars().any(|c| c.is_lowercase())
        && !w.contains('_');
    if pascal {
        return "tok-ty";
    }
    ""
}

pub(super) fn hl_ps(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        out.push_str(&hl_ps_line(line));
        out.push('\n');
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn hl_ps_line(line: &str) -> String {
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
        if ch[i] == '#' {
            out.push_str(&tok("tok-cmt", &ch[i..].iter().collect::<String>()));
            break;
        }
        if ch[i] == '"' {
            let mut j = i + 1;
            while j < n {
                if ch[j] == '`' {
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
        if ch[i] == '\'' {
            let mut j = i + 1;
            while j < n {
                if ch[j] == '\'' {
                    j += 1;
                    if j < n && ch[j] == '\'' {
                        j += 1;
                        continue;
                    }
                    break;
                }
                j += 1;
            }
            out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        if ch[i] == '$' {
            let mut j = i + 1;
            if j < n && ch[j] == '{' {
                while j < n && ch[j] != '}' {
                    j += 1;
                }
                if j < n {
                    j += 1;
                }
            } else if j < n && ch[j] == '(' {
                j += 1;
            } else {
                while j < n && (ch[j].is_alphanumeric() || ch[j] == '_') {
                    j += 1;
                }
            }
            out.push_str(&tok("tok-var", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        if ch[i] == '[' {
            let mut j = i + 1;
            let mut d = 1;
            while j < n && d > 0 {
                if ch[j] == '[' {
                    d += 1;
                } else if ch[j] == ']' {
                    d -= 1;
                }
                j += 1;
            }
            let s: String = ch[i..j].iter().collect();
            let cls = if s.contains('(') {
                "tok-attr"
            } else {
                "tok-ty"
            };
            out.push_str(&tok(cls, &s));
            i = j;
            continue;
        }
        if ch[i] == '-' && i + 1 < n && ch[i + 1].is_alphabetic() {
            let mut j = i + 1;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == '_') {
                j += 1;
            }
            let s: String = ch[i..j].iter().collect();
            let cls = if PS_KW.contains(&s.to_lowercase().as_str()) {
                "tok-kw"
            } else {
                "tok-param"
            };
            out.push_str(&tok(cls, &s));
            i = j;
            continue;
        }
        if ch[i].is_ascii_digit() {
            let mut j = i;
            while j < n && (ch[j].is_ascii_alphanumeric() || ch[j] == '.' || ch[j] == '_') {
                j += 1;
            }
            out.push_str(&tok("tok-num", &ch[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        if ch[i].is_alphabetic() || ch[i] == '_' {
            let mut j = i;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == '_' || ch[j] == '-') {
                j += 1;
            }
            while j > i && ch[j - 1] == '-' {
                j -= 1;
            }
            let s: String = ch[i..j].iter().collect();
            let next = if j < n { Some(ch[j]) } else { None };
            let cls = ps_class(&s, next);
            if cls.is_empty() {
                out.push_str(&esc(&s));
            } else {
                out.push_str(&tok(cls, &s));
            }
            i = j;
            continue;
        }
        if i + 1 < n {
            let two: String = ch[i..i + 2].iter().collect();
            if matches!(
                two.as_str(),
                "==" | "!=" | "->" | "<=" | ">=" | "+=" | "-=" | "*=" | "::" | ".." | "&&" | "||"
            ) {
                out.push_str(&tok("tok-op", &two));
                i += 2;
                continue;
            }
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
