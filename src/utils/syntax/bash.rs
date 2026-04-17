//! Bash / POSIX-shell syntax highlighter — tracks command position across
//! pipes and control-flow keywords to promote the first identifier of each
//! sub-statement into a `tok-fn` span.

use super::tok;
use crate::utils::text::esc;

const SH_KW: &[&str] = &[
    "if", "then", "else", "elif", "fi", "for", "while", "do", "done", "case", "esac", "in",
    "function", "return", "break", "continue", "exit", "local", "export", "readonly", "declare",
    "unset", "shift", "until", "select", "trap", "set", "source",
];
const SH_CMD: &[&str] = &[
    "echo",
    "printf",
    "cat",
    "ls",
    "cd",
    "mkdir",
    "rmdir",
    "rm",
    "cp",
    "mv",
    "ln",
    "chmod",
    "chown",
    "chgrp",
    "chattr",
    "lsattr",
    "touch",
    "find",
    "xargs",
    "grep",
    "sed",
    "awk",
    "sort",
    "uniq",
    "wc",
    "head",
    "tail",
    "cut",
    "tr",
    "tee",
    "apt",
    "apt-get",
    "yum",
    "dnf",
    "pacman",
    "systemctl",
    "service",
    "journalctl",
    "mount",
    "umount",
    "df",
    "du",
    "lsblk",
    "fdisk",
    "parted",
    "mkfs",
    "useradd",
    "usermod",
    "userdel",
    "groupadd",
    "passwd",
    "visudo",
    "sudo",
    "su",
    "id",
    "whoami",
    "curl",
    "wget",
    "ssh",
    "scp",
    "rsync",
    "git",
    "make",
    "gcc",
    "python",
    "python3",
    "pip",
    "clang",
    "cargo",
    "bpftool",
    "bpffs",
    "kubectl",
    "kubeadm",
    "helm",
    "ansible",
    "ansible-playbook",
    "sbsign",
    "mokutil",
    "openssl",
    "ping",
    "nmap",
    "traceroute",
    "netstat",
    "ss",
    "ip",
    "ifconfig",
    "tar",
    "gzip",
    "unzip",
    "date",
    "sleep",
    "read",
    "which",
    "test",
    "kill",
    "ps",
    "top",
    "free",
    "uptime",
    "reboot",
    "shutdown",
];

pub(super) fn hl_sh(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        out.push_str(&hl_sh_line(line));
        out.push('\n');
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn hl_sh_line(line: &str) -> String {
    let ch: Vec<char> = line.chars().collect();
    let n = ch.len();
    let mut out = String::new();
    let mut i = 0;
    let mut cmd_pos = true;
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
            cmd_pos = false;
            i = j;
            continue;
        }
        if ch[i] == '\'' {
            let mut j = i + 1;
            while j < n {
                if ch[j] == '\'' {
                    j += 1;
                    break;
                }
                j += 1;
            }
            out.push_str(&tok("tok-str", &ch[i..j].iter().collect::<String>()));
            cmd_pos = false;
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
                if j < n && ch[j] == '(' {
                    j += 1;
                }
            } else {
                while j < n && (ch[j].is_alphanumeric() || ch[j] == '_') {
                    j += 1;
                }
            }
            out.push_str(&tok("tok-var", &ch[i..j].iter().collect::<String>()));
            cmd_pos = false;
            i = j;
            continue;
        }
        if ch[i] == '-' && i + 1 < n && ch[i + 1] == '-' && i + 2 < n && ch[i + 2].is_alphabetic() {
            let mut j = i + 2;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == '-' || ch[j] == '_') {
                j += 1;
            }
            out.push_str(&tok("tok-param", &ch[i..j].iter().collect::<String>()));
            cmd_pos = false;
            i = j;
            continue;
        }
        if ch[i] == '-' && i + 1 < n && ch[i + 1].is_alphanumeric() {
            let mut j = i + 1;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == ':') {
                j += 1;
            }
            out.push_str(&tok("tok-param", &ch[i..j].iter().collect::<String>()));
            cmd_pos = false;
            i = j;
            continue;
        }
        if ch[i].is_ascii_digit() {
            let mut j = i;
            while j < n && (ch[j].is_ascii_alphanumeric() || ch[j] == '.') {
                j += 1;
            }
            out.push_str(&tok("tok-num", &ch[i..j].iter().collect::<String>()));
            cmd_pos = false;
            i = j;
            continue;
        }
        if ch[i].is_alphabetic()
            || ch[i] == '_'
            || (ch[i] == '.' && i + 1 < n && ch[i + 1].is_alphabetic())
        {
            let mut j = i;
            while j < n && (ch[j].is_alphanumeric() || ch[j] == '_' || ch[j] == '-' || ch[j] == '.')
            {
                j += 1;
            }
            while j > i && matches!(ch[j - 1], '-' | '.') {
                j -= 1;
            }
            let s: String = ch[i..j].iter().collect();
            let all_up = s.len() >= 2
                && s.chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                && s.chars().any(|c| c.is_ascii_uppercase());
            let cls = if SH_KW.contains(&s.as_str()) {
                cmd_pos = matches!(s.as_str(), "then" | "do" | "else" | "elif");
                "tok-kw"
            } else if all_up {
                cmd_pos = false;
                "tok-const"
            } else if cmd_pos || SH_CMD.contains(&s.as_str()) {
                cmd_pos = false;
                "tok-fn"
            } else {
                cmd_pos = false;
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
        if matches!(ch[i], '|' | ';') {
            cmd_pos = true;
            if i + 1 < n && (ch[i + 1] == '|' || ch[i + 1] == '&') {
                out.push_str(&tok("tok-op", &ch[i..i + 2].iter().collect::<String>()));
                i += 2;
                continue;
            }
            out.push_str(&tok("tok-op", &ch[i].to_string()));
            i += 1;
            continue;
        }
        if ch[i] == '&' {
            cmd_pos = true;
            if i + 1 < n && ch[i + 1] == '&' {
                out.push_str(&tok("tok-op", "&&"));
                i += 2;
                continue;
            }
            out.push_str(&tok("tok-op", "&"));
            i += 1;
            continue;
        }
        if matches!(
            ch[i],
            '=' | '+'
                | '-'
                | '*'
                | '/'
                | '%'
                | '!'
                | '<'
                | '>'
                | ','
                | ':'
                | '.'
                | '('
                | ')'
                | '{'
                | '}'
                | '['
                | ']'
                | '\\'
                | '`'
                | '~'
                | '@'
        ) {
            out.push_str(&tok("tok-op", &ch[i].to_string()));
        } else {
            out.push_str(&esc(&ch[i].to_string()));
        }
        i += 1;
    }
    out
}
