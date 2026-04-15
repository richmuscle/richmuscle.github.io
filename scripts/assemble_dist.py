#!/usr/bin/env python3
"""Merge Trunk `dist/index.html` (hydrate shell + hashed assets) with SSG body fragments.

Reads the post-Trunk index as a template: inserts each route's Leptos SSR fragment
immediately before the first ``<noscript>`` inside ``<body>``, preserving Trunk-injected
scripts and styles.

Usage:
  python3 scripts/assemble_dist.py --ssg ssg-out --dist dist --template dist/index.html
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


def split_trunk_template(html: str) -> tuple[str, str, str]:
    """Return (preamble_through_head, body_attrs, body_tail_from_noscript)."""
    head_close = re.search(r"</head>", html, flags=re.IGNORECASE | re.DOTALL)
    if not head_close:
        raise ValueError("template: missing </head>")
    preamble = html[: head_close.end()].strip()

    body_m = re.search(r"<body([^>]*)>", html, flags=re.IGNORECASE)
    if not body_m:
        raise ValueError("template: missing <body>")
    body_start = body_m.end()
    lower = html.lower()
    ns = lower.find("<noscript", body_start)
    if ns < 0:
        raise ValueError("template: missing <noscript> in body (needed for merge point)")

    body_close = lower.rfind("</body>")
    if body_close < 0:
        raise ValueError("template: missing </body>")
    tail = html[ns:body_close].strip()
    body_attrs = body_m.group(1).strip()
    return preamble, body_attrs, tail


def assemble_page(
    preamble: str,
    body_attrs: str,
    body_tail: str,
    fragment: str,
) -> str:
    attr_part = f" {body_attrs}" if body_attrs else ""
    return (
        f"{preamble}\n<body{attr_part}>\n{fragment.strip()}\n{body_tail}\n</body>\n</html>\n"
    )


def fragment_to_dist_path(ssg_root: Path, frag_path: Path, dist: Path) -> Path | None:
    rel = frag_path.relative_to(ssg_root)
    parts = rel.parts
    if len(parts) < 1 or parts[-1] != "index.body.html":
        return None
    parent_parts = parts[:-1]
    if parent_parts == ("this-route-should-404",):
        return dist / "404.html"
    if not parent_parts:
        return dist / "index.html"
    return dist.joinpath(*parent_parts) / "index.html"


def main() -> int:
    ap = argparse.ArgumentParser(description="Merge SSG fragments into Trunk dist/")
    ap.add_argument("--ssg", required=True, type=Path, help="SSG_OUT directory with index.body.html trees")
    ap.add_argument("--dist", required=True, type=Path, help="Trunk output directory (template = dist/index.html)")
    ap.add_argument("--template", type=Path, default=None, help="Trunk index path (default: --dist/index.html)")
    args = ap.parse_args()

    ssg_root: Path = args.ssg.resolve()
    dist: Path = args.dist.resolve()
    template_path = (args.template or (dist / "index.html")).resolve()

    if not template_path.is_file():
        print(f"assemble_dist: missing template {template_path}", file=sys.stderr)
        return 1
    if not ssg_root.is_dir():
        print(f"assemble_dist: missing SSG directory {ssg_root}", file=sys.stderr)
        return 1

    html = template_path.read_text(encoding="utf-8")
    preamble, body_attrs, body_tail = split_trunk_template(html)

    frags = sorted(ssg_root.rglob("index.body.html"))
    if not frags:
        print(f"assemble_dist: no index.body.html under {ssg_root}", file=sys.stderr)
        return 1

    for frag_file in frags:
        out_path = fragment_to_dist_path(ssg_root, frag_file, dist)
        if out_path is None:
            continue
        fragment = frag_file.read_text(encoding="utf-8")
        assembled = assemble_page(preamble, body_attrs, body_tail, fragment)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(assembled, encoding="utf-8")
        print(f"assembled {frag_file.relative_to(ssg_root)} -> {out_path.relative_to(dist)}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
