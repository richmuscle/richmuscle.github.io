//! YAML highlighter — key/value detection with quoted-string awareness,
//! booleans/null/number recognition, comment handling, and list markers.

use super::tok;
use crate::utils::text::esc;

const YAML_KW: &[&str] = &["true", "false", "yes", "no", "on", "off", "null", "~"];

pub(super) fn hl_yaml(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        out.push_str(&hl_yaml_line(line));
        out.push('\n');
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn hl_yaml_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    let pad: String = " ".repeat(indent);
    if trimmed.is_empty() {
        return esc(line);
    }
    if trimmed.starts_with("---") || trimmed.starts_with("...") {
        return pad + &tok("tok-cmt", trimmed);
    }
    if trimmed.starts_with('#') {
        return pad + &tok("tok-cmt", trimmed);
    }
    let (list_tok, body) = if let Some(rest) = trimmed.strip_prefix("- ") {
        (tok("tok-op", "- "), rest)
    } else if trimmed == "-" {
        return pad + &tok("tok-op", "-");
    } else {
        (String::new(), trimmed)
    };
    pad + &list_tok + &yaml_kv(body)
}

fn yaml_kv(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let ch: Vec<char> = s.chars().collect();
    let n = ch.len();
    let mut colon = None;
    let mut in_q = false;
    let mut qc = ' ';
    for idx in 0..n {
        if in_q {
            if ch[idx] == qc {
                in_q = false;
            }
            continue;
        }
        if ch[idx] == '"' || ch[idx] == '\'' {
            in_q = true;
            qc = ch[idx];
            continue;
        }
        if ch[idx] == ':' && (idx + 1 >= n || ch[idx + 1] == ' ') {
            colon = Some(idx);
            break;
        }
    }
    if let Some(ci) = colon {
        let key: String = ch[..ci].iter().collect();
        let rest: String = ch[ci + 1..].iter().collect();
        tok("tok-yk", &key) + ":" + &yaml_scalar(&rest)
    } else {
        yaml_scalar(s)
    }
}

fn yaml_scalar(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let sp = s.len() - s.trim_start_matches(' ').len();
    let lead: String = " ".repeat(sp);
    let v = s.trim_start_matches(' ');
    if v.is_empty() {
        return lead;
    }
    let (val, cmt) = if let Some(pos) = v.find(" #") {
        (&v[..pos], Some(&v[pos..]))
    } else {
        (v, None)
    };
    let mut out = lead;
    out += &if val.starts_with('"') || val.starts_with('\'') {
        tok("tok-str", val)
    } else if YAML_KW.iter().any(|k| k.eq_ignore_ascii_case(val)) {
        tok("tok-kw", val)
    } else if val.parse::<f64>().is_ok() || val.starts_with("0x") {
        tok("tok-num", val)
    } else if val.contains("{{") {
        tok("tok-str", val)
    } else {
        tok("tok-yv", val)
    };
    if let Some(c) = cmt {
        out += &tok("tok-cmt", c);
    }
    out
}
