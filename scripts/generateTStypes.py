#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Rust → TypeScript (global ambient) generator

- Structs -> interfaces (inside `declare global { ... }`)
- Enums -> string-literal unions
- Always includes:
    type WithNameAndId = { id: string | number; name: string };
    interface HeaderItem { text: string; href: string; }

Notes:
- chrono::DateTime<...> -> string (ISO)
- Option<T> -> T | null
- Vec<T> -> T[]
- [T; N] -> T[]
- HashMap<K,V> -> Record<string|number, V> (best-effort)
"""

import re
from typing import List, Tuple, Optional

RE_BLOCK_COMMENT = re.compile(r"/\*.*?\*/", re.DOTALL)
RE_LINE_COMMENT = re.compile(r"//.*?$", re.MULTILINE)

RE_ENUM = re.compile(r"\bpub\s+enum\s+([A-Za-z_]\w*)\s*{\s*(.*?)\s*}", re.DOTALL)
RE_STRUCT = re.compile(r"\bpub\s+struct\s+([A-Za-z_]\w*)\s*{\s*(.*?)\s*}", re.DOTALL)

RE_FIELD = re.compile(r"\bpub\s+([A-Za-z_]\w*)\s*:\s*([^,}]+)\s*,?")
RE_ATTR_POSTGRES_NAME = re.compile(r'name\s*=\s*"([^"]+)"')

PRIMS = {
    "i8": "number", "i16": "number", "i32": "number", "i64": "number", "isize": "number",
    "u8": "number", "u16": "number", "u32": "number", "u64": "number", "usize": "number",
    "f32": "number", "f64": "number",
    "bool": "boolean",
    "String": "string",
    "&str": "string",
    "char": "string",
}


def strip_comments(src: str) -> str:
    src = RE_BLOCK_COMMENT.sub("", src)
    src = RE_LINE_COMMENT.sub("", src)
    return src


def compact_ws(s: str) -> str:
    return " ".join(s.strip().split())


def split_generics(inner: str) -> List[str]:
    parts, depth, cur = [], 0, []
    for ch in inner:
        if ch == '<':
            depth += 1
            cur.append(ch)
        elif ch == '>':
            depth -= 1
            cur.append(ch)
        elif ch == ',' and depth == 0:
            parts.append("".join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    if cur:
        parts.append("".join(cur).strip())
    return parts


def map_rust_type_to_ts(ty: str) -> str:
    t = compact_ws(ty)
    if t.startswith('(') and t.endswith(')'):
        t = t[1:-1].strip()

    # chrono DateTime
    if re.search(r"\b(?:chrono::)?DateTime\s*<\s*[^>]+>", t):
        return "string"

    # Option<T>
    m = re.match(r"Option\s*<\s*(.+)\s*>$", t)
    if m:
        inner = map_rust_type_to_ts(m.group(1))
        return f"{inner} | null"

    # Vec<T>
    m = re.match(r"Vec\s*<\s*(.+)\s*>$", t)
    if m:
        inner = map_rust_type_to_ts(m.group(1))
        return f"{inner}[]"

    # HashMap<K,V>
    m = re.match(r"(?:std::collections::)?HashMap\s*<\s*(.+)\s*>$", t)
    if m:
        kv = split_generics(m.group(1))
        if len(kv) == 2:
            kt = map_rust_type_to_ts(kv[0])
            vt = map_rust_type_to_ts(kv[1])
            key_ts = "number" if kt == "number" else "string"
            return f"Record<{key_ts}, {vt}>"

    # Arrays [T; N]
    m = re.match(r"\[\s*(.+?)\s*;\s*\d+\s*\]$", t)
    if m:
        inner = map_rust_type_to_ts(m.group(1))
        return f"{inner}[]"

    # Tuple
    if t.startswith("(") and t.endswith(")"):
        inner = t[1:-1].strip()
        if inner:
            parts = [p.strip() for p in split_generics(inner)] if ("<" in inner) else [p.strip() for p in
                                                                                       inner.split(",")]
            ts_parts = [map_rust_type_to_ts(p) for p in parts if p]
            return f"[{', '.join(ts_parts)}]"
        return "[]"

    if t in PRIMS:
        return PRIMS[t]

    # strip module path for custom types
    if "::" in t:
        t = t.split("::")[-1]

    # generic wrappers we don't map -> take inner or unknown
    if "<" in t and ">" in t:
        head = t.split("<", 1)[0].strip()
        if head in ("Box", "Arc", "Rc"):
            inner = t[t.find("<") + 1:t.rfind(">")]
            return map_rust_type_to_ts(inner)
        return "unknown"

    return t


def parse_enum_body(body: str) -> List[str]:
    variants: List[str] = []
    pending_name: Optional[str] = None
    for raw in body.splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("#["):
            m = RE_ATTR_POSTGRES_NAME.search(line)
            if m:
                pending_name = m.group(1)
            continue
        line = line.split("//", 1)[0].strip().rstrip(",")
        if not line:
            continue
        m = re.match(r"([A-Za-z_]\w*)", line)
        if not m:
            continue
        ident = m.group(1)
        lit = pending_name if pending_name else ident
        pending_name = None
        variants.append(lit)
    # dedupe keep order
    seen, uniq = set(), []
    for v in variants:
        if v not in seen:
            seen.add(v)
            uniq.append(v)
    return uniq


def parse_struct_fields(body: str) -> List[Tuple[str, str]]:
    fields: List[Tuple[str, str]] = []
    for raw in body.splitlines():
        line = raw.strip()
        if not line or line.startswith("#["):
            continue
        m = RE_FIELD.search(line)
        if m:
            fields.append((m.group(1), m.group(2).strip()))
    return fields


def rust_to_ts_global(src: str) -> str:
    clean = strip_comments(src)
    enums = list(RE_ENUM.finditer(clean))
    structs = list(RE_STRUCT.finditer(clean))

    # Order by appearance: first enums then structs (still stable by start index)
    enums_sorted = sorted(enums, key=lambda m: m.start())
    structs_sorted = sorted(structs, key=lambda m: m.start())

    out: List[str] = []
    out.append("// AUTO-GENERATED FROM RUST. Edit Rust models instead.\n")
    out.append("declare global {")
    # Enums
    for m in enums_sorted:
        name = m.group(1)
        values = parse_enum_body(m.group(2))
        union = " | ".join(f'"{v}"' for v in values)
        out.append(f"  type {name} = {union};\n")

    # Helper types requested by format
    out.append("  type WithNameAndId = { id: string | number; name: string };\n")
    out.append("  interface HeaderItem {")
    out.append("    text: string;")
    out.append("    href: string;")
    out.append("  }\n")

    # Structs
    for m in structs_sorted:
        name = m.group(1)
        fields = parse_struct_fields(m.group(2))
        out.append(f"  interface {name} {{")
        for fname, fty in fields:
            ts_ty = map_rust_type_to_ts(fty)
            out.append(f"    {fname}: {ts_ty};")
        out.append("  }\n")

    out.append("}")
    # If you place this in a .ts module and want global augmentation, uncomment:
    # out.append("\nexport {};")
    return "\n".join(out) + "\n"


def main():
    with open(r"../src/utils/types.rs", "r", encoding="utf-8") as f:
        src = f.read()

    ts = rust_to_ts_global(src)
    with open("global-rust-types.d.ts", "w+", encoding="utf-8") as f:
        f.write(ts)
    with open("global-rust-types.d.ts", "a", encoding="utf-8") as f:
        f.write("// This file is auto-generated from Rust types. Do not edit.\n"
                "export {};")


if __name__ == "__main__":
    main()
