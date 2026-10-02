#!/usr/bin/env python3
"""Generate docs/port-inventory.md: the per-file port inventory.

This is the "comparison document" behind the plan: for every one of the 590 C
sources it prints the measured size (lines), the structural inventory taken from
the same extraction that drives the gates (functions, subrs, macros, constants,
struct fields, globals), the port batch it belongs to, its target path and its
current state.

Sources (all recomputable, nothing hand maintained)
--------------------------------------------------
* `docs/baseline/inventory.tsv` -- one row per C source, emitted by
  `tools/gen-baseline.py` from the pinned reference checkout.
* `tools/gen-port-map.py` -- the reference -> target path rule, imported (never
  copied) so the map and this inventory can never disagree.
* `docs/ports/*.md` -- the generated per-file ledgers.  A ledger with a
  remaining `- [ ]` box means the file is not finished.
* `femacs/**` -- target existence.

Status column
-------------
`todo`  the target does not exist yet
`wip`   the target exists but its ledger still has an unchecked box (or the file
        has no ledger at all)
`done`  the target exists and its ledger has no unchecked box

This is a progress ledger, not a gate: the authoritative verdict (token,
function, constant and macro-body comparison) is
`python3 tools/extract-abi.py --all`.  Because the status column depends on the
working tree, this file is regenerated at the end of every port round and is not
part of the CI freshness check (unlike `docs/port-map.md`).

Usage
-----
    python3 tools/gen-port-inventory.py [--out FILE]
"""

from __future__ import annotations

import argparse
import csv
import importlib.util
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REFERENCE = ROOT / "emacs"

BATCHES = [
    ("A", "预备（无 C 文件）", "PORTING.md / LIFETIMES.tsv / spike / 工具链"),
    ("B", "内核：解释器、内存、数据结构", "先落地，供所有后续批次 include"),
    ("C", "文本与字符", "buffer/marker/coding/charset 一族"),
    ("D", "系统面", "进程、文件、线程、网络、外部库绑定"),
    ("E", "显示与终端", "最大单件：xdisp/dispnew/window/frame/font/term"),
    ("F", "平台后端与工具程序", "X/w32/NS/Haiku/Android/pgtk/MS-DOS + lib-src 等"),
    ("G", "dump 与模块", "pdumper、emacs-module ABI"),
    ("L", "lib/ 系统层（gnulib 替代）", "随消费者批次插入"),
    ("H", "头文件（无同名 .c，随依赖插入）", "按 P-69/P-72 规则先就位"),
]

# Explicit stem -> batch for `src/`.  Headers share the batch of their stem, so
# a header is scheduled with the unit it belongs to.  Anything unmatched is
# reported by the generator and must be added here.
SRC_BATCH = {
    # B 内核
    "alloc": "B", "gmalloc": "B", "ralloc": "B", "vm-limit": "B", "eval": "B",
    "lread": "B", "print": "B", "bytecode": "B", "fns": "B", "data": "B",
    "doc": "B", "doprnt": "B", "floatfns": "B", "sort": "B", "bignum": "B",
    "cmds": "B", "macros": "B", "callint": "B", "atimer": "B",
    "lisp": "B", "blockinput": "B", "commands": "B", "disptab": "B",
    "getpagesize": "B", "sysstdio": "B", "conf_post": "B",
    # C 文本与字符
    "buffer": "C", "insdel": "C", "marker": "C", "intervals": "C",
    "textprop": "C", "casefiddle": "C", "casetab": "C", "indent": "C",
    "syntax": "C", "search": "C", "regex-emacs": "C", "region-cache": "C",
    "itree": "C", "undo": "C", "editfns": "C", "character": "C",
    "charset": "C", "chartab": "C", "category": "C", "ccl": "C",
    "composite": "C", "coding": "C", "bidi": "C",
    # D 系统面
    "emacs": "D", "sysdep": "D", "fileio": "D", "filelock": "D", "dired": "D",
    "process": "D", "callproc": "D", "thread": "D", "systhread": "D",
    "timefns": "D", "inotify": "D", "kqueue": "D", "gfilenotify": "D",
    "dynlib": "D", "sound": "D", "json": "D", "sqlite": "D", "gnutls": "D",
    "xml": "D", "dbusbind": "D", "decompress": "D", "lcms": "D",
    "image": "D", "profiler": "D",
    "systime": "D", "sysselect": "D", "syssignal": "D", "syswait": "D",
    "systty": "D",
    # E 显示与终端
    "xdisp": "E", "dispnew": "E", "window": "E", "frame": "E", "term": "E",
    "terminal": "E", "termcap": "E", "terminfo": "E", "tparam": "E",
    "font": "E", "fontset": "E", "ftfont": "E", "ftcrfont": "E",
    "hbfont": "E", "sfnt": "E", "sfntfont": "E", "xfaces": "E",
    "fringe": "E", "cm": "E", "scroll": "E", "keyboard": "E", "keymap": "E",
    "minibuf": "E", "menu": "E", "widget": "E", "textconv": "E",
    "treesit": "E", "comp": "E",
    "dispextern": "E", "termchar": "E", "termhooks": "E", "termopts": "E",
    "widgetprv": "E", "emacs-icon": "E",
    # F 平台后端与工具程序
    "android-emacs": "F", "android": "F", "androidfns": "F", "androidfont": "F",
    "androidmenu": "F", "androidselect": "F", "androidterm": "F",
    "androidvfs": "F", "cygw32": "F", "dosfns": "F", "emacsgtkfixed": "F",
    "gtkutil": "F", "haiku": "F", "haiku_io": "F", "haikufns": "F",
    "haikufont": "F", "haikuimage": "F", "haikumenu": "F",
    "haikuselect": "F", "haikuterm": "F", "msdos": "F", "pgtkfns": "F",
    "pgtkim": "F", "pgtkmenu": "F", "pgtkselect": "F", "pgtkterm": "F",
    "sfntfont-android": "F", "w16select": "F", "w32": "F", "w32console": "F",
    "w32cygwinx": "F", "w32dwrite": "F", "w32fns": "F", "w32font": "F",
    "w32heap": "F", "w32image": "F", "w32inevt": "F", "w32menu": "F",
    "w32notify": "F", "w32proc": "F", "w32reg": "F", "w32select": "F",
    "w32term": "F", "w32uniscribe": "F", "w32xfns": "F", "xfns": "F",
    "xfont": "F", "xftfont": "F", "xgselect": "F", "xmenu": "F", "xrdb": "F",
    "xselect": "F", "xsettings": "F", "xsmfns": "F", "xterm": "F",
    "xwidget": "F",
    "android-asset": "F", "androidgui": "F", "haiku_support": "F",
    "haikugui": "F", "macfont": "F", "macuvs": "F", "nsgui": "F",
    "nsterm": "F", "nsxwidget": "F", "pgtkgui": "F", "w32common": "F",
    "w32gdiplus": "F", "w32gui": "F", "w32term": "F",
    # G dump 与模块
    "pdumper": "G", "emacs-module": "G", "emacs-module.in": "G",
    "module-env-25": "G", "module-env-26": "G", "module-env-27": "G",
    "module-env-28": "G", "module-env-29": "G", "module-env-30": "G",
    "module-env-31": "G", "module-env-32": "G",
}

# Stems that the tables above do not cover, kept explicit so the generator can
# assert there are none left (`--strict`).
HEADER_BATCH = {"lisp": "B"}


def load_port_map():
    spec = importlib.util.spec_from_file_location(
        "gen_port_map", ROOT / "tools" / "gen-port-map.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def batch_of(rel: Path) -> tuple[str, str]:
    top = rel.parts[0]
    if top == "lib":
        return "L", "随消费者批次插入"
    if top in ("lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos"):
        return "F", "平台后端与工具程序"
    if top == "src":
        stem = rel.stem
        if stem in SRC_BATCH:
            return SRC_BATCH[stem], ""
        return "H", "无同名 .c，随依赖插入"
    return "?", "未分类"


def ledger_state(stem: str, is_header: bool, target: Path) -> str:
    if not target.exists():
        return "todo"
    names = ([f"include-{stem}.md", f"{stem}.md"] if is_header else [f"{stem}.md"])
    for name in names:
        cand = ROOT / "docs" / "ports" / name
        if cand.is_file():
            text = cand.read_text(encoding="utf-8", errors="replace")
            return "wip" if "- [ ]" in text else "done"
    return "wip"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="docs/port-inventory.md")
    ap.add_argument("--strict", action="store_true",
                    help="fail when a file has no batch assignment")
    args = ap.parse_args()

    port_map = load_port_map()
    rows = list(csv.DictReader(
        (ROOT / "docs" / "baseline" / "inventory.tsv").open(encoding="utf-8"),
        delimiter="\t"))
    extras: dict[str, int] = {}
    for line in (ROOT / "docs" / "baseline" / "totals.tsv").read_text(
            encoding="utf-8").splitlines()[1:]:
        key, value = line.split("\t")
        extras[key] = int(value)

    entries = []
    unclassified = []
    for r in rows:
        rel = Path(r["file"])
        mode, target = port_map.target_for(rel)
        assert mode == "1:1 Rust", f"{rel} is not a 1:1 file ({mode})"
        batch, note = batch_of(rel)
        if batch == "?":
            unclassified.append(str(rel))
        entries.append({
            "file": str(rel), "lines": int(r["lines"]),
            "functions": int(r["functions"]), "subrs": int(r["subrs"]),
            "macros": int(r["macros"]), "constants": int(r["constants"]),
            "fields": int(r["struct_fields"]), "globals": int(r["globals"]),
            "batch": batch, "note": note, "target": target,
            "state": ledger_state(rel.stem, rel.suffix == ".h", ROOT / target),
        })

    KEYS = ("lines", "functions", "subrs", "macros", "constants", "fields",
            "globals")

    def agg(items: list[dict]) -> list[int]:
        """Sum the seven counted columns over a set of entries."""
        return [sum(e[k] for e in items) for k in KEYS]

    out = []
    out.append("# femacs 逐文件复刻对照表（生成物，勿手改）\n")
    out.append(
        "本表是 `docs/port-master-plan.md` 的数据附录：把 590 个待复刻的 C 源逐个列出，"
        "含**行数级**规模与**结构级**清单（函数 / subr / 宏 / 常量 / 结构体字段 / 全局）。\n\n"
        "复算：`python3 tools/gen-port-inventory.py`。数据源：`docs/baseline/inventory.tsv`"
        "（由 `tools/gen-baseline.py` 从 pin 住的参照检出 `fd7afced403` 抽取）、"
        "`tools/gen-port-map.py` 的目标路径规则（import，不复制）、`docs/ports/*.md` 的勾选状态、"
        "`femacs/**` 的目标文件存在性。\n\n"
        "状态列口径：`todo` 目标不存在；`wip` 目标存在但清单还有 `- [ ]`（或尚无清单）；"
        "`done` 目标存在且清单无 `- [ ]`。**权威判定**是 "
        "`python3 tools/extract-abi.py --all`（逐 token / 函数 / 常量 / 宏体比对）："
        "2026-10-02 实测 27 个已有目标 = **17 done / 10 WIP / 0 failing**。\n\n"
        "批次是**分片与排序**，不是增量交付：任何批次都不产生半成品二进制（见总计划 §6）。"
        "本表按**生成时的工作区**统计（含尚未提交的在飞文件，这是它能当进度账的原因），"
        "因此不参与 CI 的新鲜度校验（那个只校验 `docs/port-map.md`）；"
        "每轮移植收尾时重新生成本表。\n")

    out.append("\n## 1 总量（`docs/baseline/totals.tsv`）\n")
    out.append("| 指标 | 值 |\n| --- | ---: |")
    for k, v in extras.items():
        out.append(f"| {k} | {int(v):,} |")
    counts = {"done": 0, "wip": 0, "todo": 0}
    for e in entries:
        counts[e["state"]] += 1
    out.append(f"| 已有目标文件 | {counts['done'] + counts['wip']:,} |")
    out.append(f"| 其中 done / wip | {counts['done']:,} / {counts['wip']:,} |")
    out.append(f"| 待开工 | {counts['todo']:,} |")

    out.append("\n## 2 按目录\n")
    out.append("| 目录 | 文件 | 行 | 函数 | subr | 宏 | 常量 | 字段 | 全局 | todo |")
    out.append("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    dirs: dict[str, list] = {}
    for e in entries:
        dirs.setdefault(e["file"].split("/")[0], []).append(e)
    for d in sorted(dirs):
        items = dirs[d]
        t = agg(items)
        out.append(f"| `{d}/` | {len(items)} | {t[0]:,} | {t[1]:,} | {t[2]:,} | "
                   f"{t[3]:,} | {t[4]:,} | {t[5]:,} | {t[6]:,} | "
                   f"{sum(1 for e in items if e['state'] == 'todo')} |")

    out.append("\n## 3 按批次\n")
    out.append("| 批次 | 范围 | 完成判据 | 文件 | 行 | 函数 | subr | 宏 | 字段 | 全局 | todo |")
    out.append("| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    for bid, title, note in BATCHES:
        items = [e for e in entries if e["batch"] == bid]
        if not items:
            continue
        t = agg(items)
        out.append(f"| **{bid}** | {title} | {note} | {len(items)} | {t[0]:,} | "
                   f"{t[1]:,} | {t[2]:,} | {t[3]:,} | {t[5]:,} | {t[6]:,} | "
                   f"{sum(1 for e in items if e['state'] == 'todo')} |")

    out.append("\n## 4 最大的 25 个文件（按行数）\n")
    out.append("| 参照文件 | 行 | 函数 | subr | 宏 | 字段 | 全局 | 批次 | 状态 |")
    out.append("| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |")
    for e in sorted(entries, key=lambda x: -x["lines"])[:25]:
        out.append(f"| `{e['file']}` | {e['lines']:,} | {e['functions']:,} | "
                   f"{e['subrs']:,} | {e['macros']:,} | {e['fields']:,} | "
                   f"{e['globals']:,} | {e['batch']} | {e['state']} |")

    out.append("\n## 5 逐文件对照（590 条）\n")
    out.append("| 参照源 | 行 | 函数 | subr | 宏 | 常量 | 字段 | 全局 | 批次 | 目标 | 状态 |")
    out.append("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |")
    for e in sorted(entries, key=lambda x: x["file"]):
        out.append(f"| `{e['file']}` | {e['lines']:,} | {e['functions']:,} | "
                   f"{e['subrs']:,} | {e['macros']:,} | {e['constants']:,} | "
                   f"{e['fields']:,} | {e['globals']:,} | {e['batch']} | "
                   f"`{e['target']}` | {e['state']} |")

    total = agg(entries)
    out.append(f"\n---\n\n合计 **{len(entries)} 文件 / {total[0]:,} 行 / "
               f"{total[1]:,} 函数 / {total[2]:,} subr / {total[3]:,} 宏 / "
               f"{total[4]:,} 常量 / {total[5]:,} 结构体字段 / {total[6]:,} 全局**。\n")

    dest = ROOT / args.out
    dest.write_text("\n".join(out), encoding="utf-8")
    print(f"wrote {args.out}: {len(entries)} files, "
          f"done={counts['done']} wip={counts['wip']} todo={counts['todo']}")
    if unclassified:
        print(f"unclassified ({len(unclassified)}): {' '.join(unclassified)}",
              file=sys.stderr)
        if args.strict:
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
