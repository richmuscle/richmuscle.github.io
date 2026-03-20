//! Sanitization, tracking, and code highlighting utilities.

#[cfg(debug_assertions)]
use leptos::logging::log;
use leptos::wasm_bindgen::{JsCast, JsValue};
#[allow(unused_imports)] // used by track() in release builds only
use js_sys;
use std::sync::OnceLock;

static WASM_START_TIME_MS: OnceLock<f64> = OnceLock::new();

fn perf_now_ms() -> Option<f64> {
    let window = web_sys::window()?;
    let perf = js_sys::Reflect::get(&window, &JsValue::from_str("performance")).ok()?;
    let now_fn = js_sys::Reflect::get(&perf, &JsValue::from_str("now")).ok()?;
    let now_fn = now_fn.dyn_into::<js_sys::Function>().ok()?;
    now_fn.call0(&perf).ok()?.as_f64()
}

pub fn capture_wasm_start_time() {
    if let Some(now) = perf_now_ms() {
        let _ = WASM_START_TIME_MS.set(now);
    }
}

pub fn wasm_start_time_ms() -> Option<f64> {
    WASM_START_TIME_MS.get().copied()
}

pub fn sanitize_slug(slug: &str) -> String {
    let out: String = slug.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .take(100)
        .collect();
    #[cfg(debug_assertions)]
    if out != slug {
        log!("sanitize_slug: {:?} -> {:?}", slug, out);
    }
    out
}

#[allow(unused_variables)]
pub fn track(event: &str, props: &str) {
    #[cfg(not(debug_assertions))]
    let _ = js_sys::eval(&format!(
        "window.dispatchEvent(new CustomEvent('portfolio:{}', {{ detail: JSON.parse({:?}) }}))",
        event, props
    ));
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '&'  => o.push_str("&amp;"),
            '<'  => o.push_str("&lt;"),
            '>'  => o.push_str("&gt;"),
            '"'  => o.push_str("&quot;"),
            '\'' => o.push_str("&#39;"),
            _    => o.push(c),
        }
    }
    o
}

#[inline]
fn tok(cls: &str, text: &str) -> String {
    if text.is_empty() { return String::new(); }
    format!("<span class=\"{}\">{}</span>", cls, esc(text))
}

pub fn highlight_code(lang: &str, code: &str) -> String {
    match lang.trim().to_uppercase().as_str() {
        "POWERSHELL" | "PWSH" | "PS1" | "PS" => hl_ps(code),
        "BASH" | "SHELL" | "SH"              => hl_sh(code),
        "YAML" | "YML"                        => hl_yaml(code),
        "RUST" | "RS"                         => hl_rust(code),
        _                                     => esc(code),
    }
}

// ── Rust highlighter (minimal) ────────────────────────────────
const RUST_KW: &[&str] = &[
    "fn","let","mut","pub","use","mod","impl","struct","enum","trait",
    "where","for","in","if","else","match","return","loop","while",
    "break","continue","type","const","static","self","Self","super",
    "crate","extern","unsafe","async","await","dyn","ref","move",
    "box","true","false","as","impl","Some","None","Ok","Err",
];

fn hl_rust(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() { out.push_str(&hl_rust_line(line)); out.push('\n'); }
    if out.ends_with('\n') { out.pop(); }
    out
}

fn hl_rust_line(line: &str) -> String {
    let ch: Vec<char> = line.chars().collect();
    let n = ch.len();
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        if ch[i] == ' ' || ch[i] == '\t' { out.push(ch[i]); i += 1; continue; }
        // line comment
        if i + 1 < n && ch[i] == '/' && ch[i+1] == '/' {
            out.push_str(&tok("tok-cmt", &ch[i..].iter().collect::<String>())); break;
        }
        // block comment start
        if i + 1 < n && ch[i] == '/' && ch[i+1] == '*' {
            let mut j = i + 2;
            while j + 1 < n && !(ch[j] == '*' && ch[j+1] == '/') { j += 1; }
            if j + 1 < n { j += 2; }
            out.push_str(&tok("tok-cmt", &ch[i..j].iter().collect::<String>())); i = j; continue;
        }
        // string
        if ch[i] == '"' || (ch[i] == 'r' && i+1 < n && ch[i+1] == '#') {
            // raw string r#"..."#
            if ch[i] == 'r' {
                let mut hashes = 0;
                let mut j = i + 1;
                while j < n && ch[j] == '#' { hashes += 1; j += 1; }
                if j < n && ch[j] == '"' {
                    j += 1;
                    let close: String = std::iter::once('"').chain(std::iter::repeat('#').take(hashes)).collect();
                    let close_chars: Vec<char> = close.chars().collect();
                    while j < n {
                        if ch[j..].starts_with(&close_chars) { j += close_chars.len(); break; }
                        j += 1;
                    }
                    out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>())); i = j; continue;
                }
            }
            let mut j = i + 1;
            while j < n { if ch[j] == '\\' { j += 2; } else if ch[j] == '"' { j += 1; break; } else { j += 1; } }
            out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>())); i = j; continue;
        }
        // char literal
        if ch[i] == '\'' && i + 2 < n {
            let mut j = i + 1;
            if j < n && ch[j] == '\\' { j += 2; } else { j += 1; }
            if j < n && ch[j] == '\'' { j += 1; out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>())); i = j; continue; }
        }
        // attribute / lifetime
        if ch[i] == '#' && i+1 < n && ch[i+1] == '[' {
            let mut j = i; let mut d = 0;
            while j < n { if ch[j] == '[' { d += 1; } else if ch[j] == ']' { d -= 1; if d == 0 { j += 1; break; } } j += 1; }
            out.push_str(&tok("tok-attr", &ch[i..j].iter().collect::<String>())); i = j; continue;
        }
        // number
        if ch[i].is_ascii_digit() || (ch[i] == '-' && i+1 < n && ch[i+1].is_ascii_digit()) {
            let mut j = i + 1;
            while j < n && (ch[j].is_ascii_alphanumeric() || ch[j] == '.' || ch[j] == '_') { j += 1; }
            out.push_str(&tok("tok-num", &ch[i..j].iter().collect::<String>())); i = j; continue;
        }
        // identifier
        if ch[i].is_alphabetic() || ch[i] == '_' {
            let mut j = i;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == '_') { j += 1; }
            let s: String = ch[i..j].iter().collect();
            let next = if j < n { Some(ch[j]) } else { None };
            let cls = if RUST_KW.contains(&s.as_str()) { "tok-kw" }
                else if s.starts_with(|c: char| c.is_uppercase()) && next != Some('(') { "tok-ty" }
                else if next == Some('(') || next == Some('!') { "tok-fn" }
                else { "" };
            if cls.is_empty() { out.push_str(&esc(&s)); } else { out.push_str(&tok(cls, &s)); }
            i = j; continue;
        }
        if matches!(ch[i], '='|'+'|'-'|'*'|'/'|'%'|'|'|'&'|'!'|'<'|'>'|','|';'|':'|'.'|'('|')'|'{'|'}'|'['|']'|'@'|'`'|'?'|'\\') {
            out.push_str(&tok("tok-op", &ch[i].to_string()));
        } else { out.push_str(&esc(&ch[i].to_string())); }
        i += 1;
    }
    out
}

// ── PowerShell ────────────────────────────────────────────────
const PS_KW: &[&str] = &[
    "if","else","elseif","foreach","for","while","do","switch",
    "break","continue","return","function","param","begin","process",
    "end","try","catch","finally","throw","in","until","trap","exit",
    "true","false","null",
    "-eq","-ne","-gt","-lt","-le","-ge","-like","-notlike",
    "-match","-notmatch","-contains","-notcontains","-in","-notin",
    "-is","-isnot","-not","-and","-or","-xor",
    "-band","-bor","-bnot","-shl","-shr","-f",
];

fn ps_class(w: &str, next: Option<char>) -> &'static str {
    let lo = w.to_lowercase();
    if PS_KW.contains(&lo.as_str()) { return "tok-kw"; }
    if !w.starts_with('-') && w.contains('-') && w.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) { return "tok-fn"; }
    let all_up = w.len() >= 2 && w.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') && w.chars().any(|c| c.is_ascii_uppercase());
    if all_up { return "tok-const"; }
    if next == Some('(') { return "tok-fn"; }
    let pascal = w.starts_with(|c: char| c.is_uppercase()) && w.chars().any(|c| c.is_lowercase()) && !w.contains('_');
    if pascal { return "tok-ty"; }
    ""
}

fn hl_ps(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() { out.push_str(&hl_ps_line(line)); out.push('\n'); }
    if out.ends_with('\n') { out.pop(); }
    out
}

fn hl_ps_line(line: &str) -> String {
    let ch: Vec<char> = line.chars().collect();
    let n = ch.len();
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        if ch[i] == ' ' || ch[i] == '\t' { out.push(ch[i]); i += 1; continue; }
        if ch[i] == '#' { out.push_str(&tok("tok-cmt", &ch[i..].iter().collect::<String>())); break; }
        if ch[i] == '"' { let mut j = i+1; while j < n { if ch[j]=='`' { j+=2; } else if ch[j]=='"' { j+=1; break; } else { j+=1; } } out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>())); i=j; continue; }
        if ch[i] == '\'' { let mut j = i+1; while j < n { if ch[j]=='\'' { j+=1; if j<n && ch[j]=='\'' { j+=1; continue; } break; } j+=1; } out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>())); i=j; continue; }
        if ch[i] == '$' { let mut j=i+1; if j<n && ch[j]=='{' { while j<n && ch[j]!='}' { j+=1; } if j<n { j+=1; } } else if j<n && ch[j]=='(' { j+=1; } else { while j<n && (ch[j].is_alphanumeric()||ch[j]=='_') { j+=1; } } out.push_str(&tok("tok-var", &ch[i..j].iter().collect::<String>())); i=j; continue; }
        if ch[i]=='[' { let mut j=i+1; let mut d=1; while j<n && d>0 { if ch[j]=='[' { d+=1; } else if ch[j]==']' { d-=1; } j+=1; } let s: String=ch[i..j].iter().collect(); let cls=if s.contains('(') { "tok-attr" } else { "tok-ty" }; out.push_str(&tok(cls,&s)); i=j; continue; }
        if ch[i]=='-' && i+1<n && ch[i+1].is_alphabetic() { let mut j=i+1; while j<n && (ch[j].is_alphanumeric()||ch[j]=='_') { j+=1; } let s: String=ch[i..j].iter().collect(); let cls=if PS_KW.contains(&s.to_lowercase().as_str()) { "tok-kw" } else { "tok-param" }; out.push_str(&tok(cls,&s)); i=j; continue; }
        if ch[i].is_ascii_digit() { let mut j=i; while j<n && (ch[j].is_ascii_alphanumeric()||ch[j]=='.'||ch[j]=='_') { j+=1; } out.push_str(&tok("tok-num",&ch[i..j].iter().collect::<String>())); i=j; continue; }
        if ch[i].is_alphabetic()||ch[i]=='_' { let mut j=i; while j<n && (ch[j].is_alphanumeric()||ch[j]=='_'||ch[j]=='-') { j+=1; } while j>i && ch[j-1]=='-' { j-=1; } let s: String=ch[i..j].iter().collect(); let next=if j<n { Some(ch[j]) } else { None }; let cls=ps_class(&s,next); if cls.is_empty() { out.push_str(&esc(&s)); } else { out.push_str(&tok(cls,&s)); } i=j; continue; }
        if i+1<n { let two: String=ch[i..i+2].iter().collect(); if matches!(two.as_str(),"=="| "!="| "->"| "<="| ">="| "+="| "-="| "*="| "::"| ".."| "&&"| "||") { out.push_str(&tok("tok-op",&two)); i+=2; continue; } }
        if matches!(ch[i],'='|'+'|'-'|'*'|'/'|'%'|'|'|'&'|'!'|'<'|'>'|','|';'|':'|'.'|'('|')'|'{'|'}'|'@'|'`'|'?'|'\\') { out.push_str(&tok("tok-op",&ch[i].to_string())); } else { out.push_str(&esc(&ch[i].to_string())); }
        i+=1;
    }
    out
}

// ── Bash ──────────────────────────────────────────────────────
const SH_KW: &[&str] = &["if","then","else","elif","fi","for","while","do","done","case","esac","in","function","return","break","continue","exit","local","export","readonly","declare","unset","shift","until","select","trap","set","source"];
const SH_CMD: &[&str] = &["echo","printf","cat","ls","cd","mkdir","rmdir","rm","cp","mv","ln","chmod","chown","chgrp","chattr","lsattr","touch","find","xargs","grep","sed","awk","sort","uniq","wc","head","tail","cut","tr","tee","apt","apt-get","yum","dnf","pacman","systemctl","service","journalctl","mount","umount","df","du","lsblk","fdisk","parted","mkfs","useradd","usermod","userdel","groupadd","passwd","visudo","sudo","su","id","whoami","curl","wget","ssh","scp","rsync","git","make","gcc","python","python3","pip","clang","cargo","bpftool","bpffs","kubectl","kubeadm","helm","ansible","ansible-playbook","sbsign","mokutil","openssl","ping","nmap","traceroute","netstat","ss","ip","ifconfig","tar","gzip","unzip","date","sleep","read","which","test","kill","ps","top","free","uptime","reboot","shutdown"];

fn hl_sh(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() { out.push_str(&hl_sh_line(line)); out.push('\n'); }
    if out.ends_with('\n') { out.pop(); }
    out
}

fn hl_sh_line(line: &str) -> String {
    let ch: Vec<char> = line.chars().collect();
    let n = ch.len();
    let mut out = String::new();
    let mut i = 0;
    let mut cmd_pos = true;
    while i < n {
        if ch[i]==' '||ch[i]=='\t' { out.push(ch[i]); i+=1; continue; }
        if ch[i]=='#' { out.push_str(&tok("tok-cmt",&ch[i..].iter().collect::<String>())); break; }
        if ch[i]=='"' { let mut j=i+1; while j<n { if ch[j]=='\\' { j+=2; } else if ch[j]=='"' { j+=1; break; } else { j+=1; } } out.push_str(&tok("tok-str",&ch[i..j].iter().collect::<String>())); cmd_pos=false; i=j; continue; }
        if ch[i]=='\'' { let mut j=i+1; while j<n { if ch[j]=='\'' { j+=1; break; } j+=1; } out.push_str(&tok("tok-str",&ch[i..j].iter().collect::<String>())); cmd_pos=false; i=j; continue; }
        if ch[i]=='$' { let mut j=i+1; if j<n&&ch[j]=='{' { while j<n&&ch[j]!='}' { j+=1; } if j<n { j+=1; } } else if j<n&&ch[j]=='(' { j+=1; if j<n&&ch[j]=='(' { j+=1; } } else { while j<n&&(ch[j].is_alphanumeric()||ch[j]=='_') { j+=1; } } out.push_str(&tok("tok-var",&ch[i..j].iter().collect::<String>())); cmd_pos=false; i=j; continue; }
        if ch[i]=='-'&&i+1<n&&ch[i+1]=='-'&&i+2<n&&ch[i+2].is_alphabetic() { let mut j=i+2; while j<n&&(ch[j].is_alphanumeric()||ch[j]=='-'||ch[j]=='_') { j+=1; } out.push_str(&tok("tok-param",&ch[i..j].iter().collect::<String>())); cmd_pos=false; i=j; continue; }
        if ch[i]=='-'&&i+1<n&&ch[i+1].is_alphanumeric() { let mut j=i+1; while j<n&&(ch[j].is_alphanumeric()||ch[j]==':') { j+=1; } out.push_str(&tok("tok-param",&ch[i..j].iter().collect::<String>())); cmd_pos=false; i=j; continue; }
        if ch[i].is_ascii_digit() { let mut j=i; while j<n&&(ch[j].is_ascii_alphanumeric()||ch[j]=='.') { j+=1; } out.push_str(&tok("tok-num",&ch[i..j].iter().collect::<String>())); cmd_pos=false; i=j; continue; }
        if ch[i].is_alphabetic()||ch[i]=='_'||(ch[i]=='.'&&i+1<n&&ch[i+1].is_alphabetic()) { let mut j=i; while j<n&&(ch[j].is_alphanumeric()||ch[j]=='_'||ch[j]=='-'||ch[j]=='.') { j+=1; } while j>i&&matches!(ch[j-1],'-'|'.') { j-=1; } let s: String=ch[i..j].iter().collect(); let all_up=s.len()>=2&&s.chars().all(|c|c.is_ascii_uppercase()||c.is_ascii_digit()||c=='_')&&s.chars().any(|c|c.is_ascii_uppercase()); let cls=if SH_KW.contains(&s.as_str()) { if matches!(s.as_str(),"then"|"do"|"else"|"elif") { cmd_pos=true; } else { cmd_pos=false; } "tok-kw" } else if all_up { cmd_pos=false; "tok-const" } else if cmd_pos||SH_CMD.contains(&s.as_str()) { cmd_pos=false; "tok-fn" } else { cmd_pos=false; "" }; if cls.is_empty() { out.push_str(&esc(&s)); } else { out.push_str(&tok(cls,&s)); } i=j; continue; }
        if matches!(ch[i],'|'|';') { cmd_pos=true; if i+1<n&&(ch[i+1]=='|'||ch[i+1]=='&') { out.push_str(&tok("tok-op",&ch[i..i+2].iter().collect::<String>())); i+=2; continue; } out.push_str(&tok("tok-op",&ch[i].to_string())); i+=1; continue; }
        if ch[i]=='&' { cmd_pos=true; if i+1<n&&ch[i+1]=='&' { out.push_str(&tok("tok-op","&&")); i+=2; continue; } out.push_str(&tok("tok-op","&")); i+=1; continue; }
        if matches!(ch[i],'='|'+'|'-'|'*'|'/'|'%'|'!'|'<'|'>'|','|':'|'.'|'('|')'|'{'|'}'|'['|']'|'\\'|'`'|'~'|'@') { out.push_str(&tok("tok-op",&ch[i].to_string())); } else { out.push_str(&esc(&ch[i].to_string())); }
        i+=1;
    }
    out
}

// ── YAML ──────────────────────────────────────────────────────
const YAML_KW: &[&str] = &["true","false","yes","no","on","off","null","~"];

fn hl_yaml(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() { out.push_str(&hl_yaml_line(line)); out.push('\n'); }
    if out.ends_with('\n') { out.pop(); }
    out
}

fn hl_yaml_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent  = line.len() - trimmed.len();
    let pad: String = " ".repeat(indent);
    if trimmed.is_empty() { return esc(line); }
    if trimmed.starts_with("---") || trimmed.starts_with("...") { return pad + &tok("tok-cmt", trimmed); }
    if trimmed.starts_with('#') { return pad + &tok("tok-cmt", trimmed); }
    let (list_tok, body) = if trimmed.starts_with("- ") { (tok("tok-op","- "), &trimmed[2..]) } else if trimmed == "-" { return pad + &tok("tok-op","-"); } else { (String::new(), trimmed) };
    pad + &list_tok + &yaml_kv(body)
}

fn yaml_kv(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    let ch: Vec<char> = s.chars().collect();
    let n = ch.len();
    let mut colon = None;
    let mut in_q = false; let mut qc = ' ';
    for idx in 0..n {
        if in_q { if ch[idx]==qc { in_q=false; } continue; }
        if ch[idx]=='"'||ch[idx]=='\'' { in_q=true; qc=ch[idx]; continue; }
        if ch[idx]==':'&&(idx+1>=n||ch[idx+1]==' ') { colon=Some(idx); break; }
    }
    if let Some(ci) = colon {
        let key: String=ch[..ci].iter().collect();
        let rest: String=ch[ci+1..].iter().collect();
        tok("tok-yk",&key)+":"+&yaml_scalar(&rest)
    } else { yaml_scalar(s) }
}

fn yaml_scalar(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    let sp=s.len()-s.trim_start_matches(' ').len();
    let lead: String=" ".repeat(sp);
    let v=s.trim_start_matches(' ');
    if v.is_empty() { return lead; }
    let (val,cmt)=if let Some(pos)=v.find(" #") { (&v[..pos],Some(&v[pos..])) } else { (v,None) };
    let mut out=lead;
    out+=&if val.starts_with('"')||val.starts_with('\'') { tok("tok-str",val) } else if YAML_KW.iter().any(|k|k.eq_ignore_ascii_case(val)) { tok("tok-kw",val) } else if val.parse::<f64>().is_ok()||val.starts_with("0x") { tok("tok-num",val) } else if val.contains("{{") { tok("tok-str",val) } else { tok("tok-yv",val) };
    if let Some(c)=cmt { out+=&tok("tok-cmt",c); }
    out
}
