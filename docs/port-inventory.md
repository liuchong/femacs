# femacs 逐文件复刻对照表（生成物，勿手改）

本表是 `docs/port-master-plan.md` 的数据附录：把 590 个待复刻的 C 源逐个列出，含**行数级**规模与**结构级**清单（函数 / subr / 宏 / 常量 / 结构体字段 / 全局）。

复算：`python3 tools/gen-port-inventory.py`。数据源：`docs/baseline/inventory.tsv`（由 `tools/gen-baseline.py` 从 pin 住的参照检出 `fd7afced403` 抽取）、`tools/gen-port-map.py` 的目标路径规则（import，不复制）、`docs/ports/*.md` 的勾选状态、`femacs/**` 的目标文件存在性。

状态列口径：`todo` 目标不存在；`wip` 目标存在但清单还有 `- [ ]`（或尚无清单）；`done` 目标存在且清单无 `- [ ]`。**权威判定**是 `python3 tools/extract-abi.py --all`（逐 token / 函数 / 常量 / 宏体比对）：2026-10-02 实测 27 个已有目标 = **17 done / 10 WIP / 0 failing**。

批次是**分片与排序**，不是增量交付：任何批次都不产生半成品二进制（见总计划 §6）。本表按**生成时的工作区**统计（含尚未提交的在飞文件，这是它能当进度账的原因），因此不参与 CI 的新鲜度校验（那个只校验 `docs/port-map.md`）；每轮移植收尾时重新生成本表。


## 1 总量（`docs/baseline/totals.tsv`）

| 指标 | 值 |
| --- | ---: |
| files | 590 |
| lines | 678,320 |
| functions | 10,374 |
| defuns | 1,841 |
| macros | 7,156 |
| constants | 3,898 |
| struct_fields | 5,764 |
| globals | 1,948 |
| unique_defun_c_names | 1,736 |
| unique_defun_lisp_names | 1,736 |
| 已有目标文件 | 374 |
| 其中 done / wip | 344 / 30 |
| 待开工 | 216 |

## 2 按目录

| 目录 | 文件 | 行 | 函数 | subr | 宏 | 常量 | 字段 | 全局 | todo |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `exec/` | 8 | 4,274 | 41 | 0 | 36 | 26 | 96 | 16 | 8 |
| `lib/` | 237 | 69,423 | 989 | 0 | 2,476 | 685 | 399 | 32 | 50 |
| `lib-src/` | 14 | 20,960 | 361 | 0 | 150 | 220 | 78 | 88 | 14 |
| `lwlib/` | 15 | 8,353 | 217 | 0 | 69 | 68 | 8 | 15 | 15 |
| `msdos/` | 2 | 58 | 0 | 0 | 6 | 2 | 0 | 0 | 2 |
| `nt/` | 31 | 3,292 | 25 | 0 | 351 | 124 | 63 | 8 | 31 |
| `oldXMenu/` | 33 | 5,176 | 39 | 0 | 62 | 57 | 3 | 8 | 33 |
| `src/` | 250 | 566,784 | 8,702 | 1,841 | 4,006 | 2,716 | 5,117 | 1,781 | 63 |

## 3 按批次

| 批次 | 范围 | 完成判据 | 文件 | 行 | 函数 | subr | 宏 | 字段 | 全局 | todo |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| **B** | 内核：解释器、内存、数据结构 | 先落地，供所有后续批次 include | 29 | 51,139 | 1,111 | 379 | 494 | 386 | 211 | 3 |
| **C** | 文本与字符 | buffer/marker/coding/charset 一族 | 36 | 70,340 | 975 | 330 | 717 | 420 | 148 | 1 |
| **D** | 系统面 | 进程、文件、线程、网络、外部库绑定 | 34 | 61,745 | 1,012 | 269 | 498 | 289 | 128 | 4 |
| **E** | 显示与终端 | 最大单件：xdisp/dispnew/window/frame/font/term | 50 | 172,942 | 2,240 | 490 | 1,104 | 1,976 | 432 | 13 |
| **F** | 平台后端与工具程序 | X/w32/NS/Haiku/Android/pgtk/MS-DOS + lib-src 等 | 192 | 244,303 | 3,753 | 368 | 1,834 | 2,175 | 981 | 136 |
| **G** | dump 与模块 | pdumper、emacs-module ABI | 12 | 8,428 | 294 | 5 | 33 | 119 | 16 | 9 |
| **L** | lib/ 系统层（gnulib 替代） | 随消费者批次插入 | 237 | 69,423 | 989 | 0 | 2,476 | 399 | 32 | 50 |

## 4 最大的 25 个文件（按行数）

| 参照文件 | 行 | 函数 | subr | 宏 | 字段 | 全局 | 批次 | 状态 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `src/xdisp.c` | 39,805 | 406 | 24 | 57 | 7 | 72 | E | todo |
| `src/xterm.c` | 33,125 | 438 | 0 | 35 | 33 | 103 | F | done |
| `src/sfnt.c` | 21,215 | 268 | 0 | 170 | 53 | 8 | E | done |
| `src/keyboard.c` | 14,682 | 183 | 38 | 19 | 12 | 83 | E | todo |
| `src/image.c` | 13,805 | 223 | 12 | 152 | 61 | 12 | D | done |
| `src/w32fns.c` | 12,488 | 90 | 60 | 69 | 25 | 55 | F | done |
| `src/coding.c` | 12,312 | 96 | 34 | 140 | 0 | 12 | C | todo |
| `src/w32.c` | 11,362 | 225 | 0 | 29 | 27 | 91 | F | done |
| `src/xfns.c` | 10,662 | 116 | 53 | 7 | 13 | 11 | F | done |
| `src/window.c` | 9,740 | 112 | 121 | 1 | 64 | 13 | E | todo |
| `src/macuvs.h` | 9,218 | 0 | 0 | 0 | 0 | 0 | F | todo |
| `src/process.c` | 9,122 | 125 | 63 | 24 | 9 | 20 | D | wip |
| `src/w32term.c` | 8,414 | 151 | 0 | 11 | 0 | 37 | F | done |
| `lib-src/etags.c` | 8,280 | 126 | 0 | 31 | 0 | 12 | F | todo |
| `src/androidvfs.c` | 7,950 | 181 | 1 | 5 | 81 | 25 | F | done |
| `src/frame.c` | 7,907 | 100 | 69 | 0 | 2 | 8 | E | done |
| `src/xfaces.c` | 7,843 | 101 | 33 | 34 | 3 | 17 | E | done |
| `src/pgtkterm.c` | 7,780 | 199 | 0 | 16 | 6 | 14 | F | done |
| `src/alloc.c` | 7,639 | 209 | 22 | 94 | 60 | 51 | B | done |
| `src/android.c` | 7,594 | 155 | 0 | 2 | 60 | 55 | F | done |
| `src/dispnew.c` | 7,584 | 150 | 13 | 4 | 13 | 20 | E | done |
| `src/androidterm.c` | 7,117 | 132 | 0 | 1 | 40 | 7 | F | done |
| `src/fileio.c` | 7,085 | 61 | 55 | 10 | 7 | 8 | D | done |
| `src/fns.c` | 6,973 | 100 | 104 | 4 | 6 | 6 | B | done |
| `src/gtkutil.c` | 6,753 | 146 | 0 | 19 | 10 | 9 | F | done |

## 5 逐文件对照（590 条）

| 参照源 | 行 | 函数 | subr | 宏 | 常量 | 字段 | 全局 | 批次 | 目标 | 状态 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |
| `exec/exec.c` | 1,185 | 10 | 0 | 3 | 0 | 16 | 8 | F | `femacs/exec/exec.rs` | todo |
| `exec/exec.h` | 215 | 0 | 0 | 1 | 0 | 57 | 0 | F | `femacs/exec/include/exec.rs` | todo |
| `exec/exec1.c` | 94 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/exec/exec1.rs` | todo |
| `exec/mipsel-user.h` | 46 | 0 | 0 | 1 | 0 | 7 | 0 | F | `femacs/exec/include/mipsel-user.rs` | todo |
| `exec/mipsfpu.c` | 289 | 4 | 0 | 18 | 18 | 5 | 0 | F | `femacs/exec/mipsfpu.rs` | todo |
| `exec/mipsfpu.h` | 82 | 0 | 0 | 4 | 3 | 11 | 0 | F | `femacs/exec/include/mipsfpu.rs` | todo |
| `exec/test.c` | 105 | 2 | 0 | 0 | 0 | 0 | 1 | F | `femacs/exec/test.rs` | todo |
| `exec/trace.c` | 2,258 | 24 | 0 | 9 | 5 | 0 | 7 | F | `femacs/exec/trace.rs` | todo |
| `lib-src/asset-directory-tool.c` | 338 | 5 | 0 | 0 | 0 | 5 | 1 | F | `femacs/lib-src/asset-directory-tool.rs` | todo |
| `lib-src/ebrowse.c` | 3,926 | 66 | 0 | 37 | 125 | 41 | 41 | F | `femacs/lib-src/ebrowse.rs` | todo |
| `lib-src/emacsclient.c` | 2,524 | 52 | 0 | 11 | 4 | 4 | 15 | F | `femacs/lib-src/emacsclient.rs` | todo |
| `lib-src/etags.c` | 8,280 | 126 | 0 | 31 | 62 | 0 | 12 | F | `femacs/lib-src/etags.rs` | todo |
| `lib-src/hexl.c` | 221 | 3 | 0 | 0 | 0 | 0 | 1 | F | `femacs/lib-src/hexl.rs` | todo |
| `lib-src/make-docfile.c` | 1,232 | 21 | 0 | 0 | 10 | 15 | 6 | F | `femacs/lib-src/make-docfile.rs` | todo |
| `lib-src/make-fingerprint.c` | 178 | 1 | 0 | 2 | 0 | 0 | 1 | F | `femacs/lib-src/make-fingerprint.rs` | todo |
| `lib-src/movemail.c` | 871 | 13 | 0 | 7 | 1 | 0 | 1 | F | `femacs/lib-src/movemail.rs` | todo |
| `lib-src/ntlib.c` | 601 | 24 | 0 | 2 | 0 | 2 | 4 | F | `femacs/lib-src/ntlib.rs` | todo |
| `lib-src/ntlib.h` | 96 | 0 | 0 | 29 | 0 | 0 | 0 | F | `femacs/lib-src/include/ntlib.rs` | todo |
| `lib-src/pop.c` | 1,703 | 29 | 0 | 17 | 12 | 2 | 3 | F | `femacs/lib-src/pop.rs` | todo |
| `lib-src/pop.h` | 75 | 0 | 0 | 5 | 2 | 7 | 2 | F | `femacs/lib-src/include/pop.rs` | todo |
| `lib-src/seccomp-filter.c` | 409 | 5 | 0 | 5 | 2 | 0 | 1 | F | `femacs/lib-src/seccomp-filter.rs` | todo |
| `lib-src/update-game-score.c` | 506 | 16 | 0 | 4 | 2 | 2 | 0 | F | `femacs/lib-src/update-game-score.rs` | todo |
| `lib/_Noreturn.h` | 43 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/_Noreturn.rs` | todo |
| `lib/acl-errno-valid.c` | 52 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/acl-errno-valid.rs` | done |
| `lib/acl-internal.c` | 523 | 11 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/acl-internal.rs` | done |
| `lib/acl-internal.h` | 284 | 0 | 0 | 40 | 27 | 13 | 0 | L | `femacs/lib/include/acl-internal.rs` | done |
| `lib/acl.h` | 118 | 0 | 0 | 4 | 1 | 7 | 0 | L | `femacs/lib/include/acl.rs` | wip |
| `lib/acl_entries.c` | 68 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/acl_entries.rs` | done |
| `lib/alloca.in.h` | 72 | 0 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/alloca.in.rs` | todo |
| `lib/allocator.c` | 22 | 0 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/allocator.rs` | done |
| `lib/allocator.h` | 68 | 0 | 0 | 1 | 0 | 4 | 0 | L | `femacs/lib/include/allocator.rs` | wip |
| `lib/arg-nonnull.h` | 26 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/arg-nonnull.rs` | todo |
| `lib/assert.in.h` | 27 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/include/assert.in.rs` | todo |
| `lib/at-func.c` | 142 | 1 | 0 | 5 | 1 | 0 | 0 | L | `femacs/lib/at-func.rs` | done |
| `lib/attribute.h` | 364 | 0 | 0 | 32 | 0 | 0 | 0 | L | `femacs/lib/include/attribute.rs` | wip |
| `lib/binary-io.c` | 38 | 1 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/binary-io.rs` | done |
| `lib/binary-io.h` | 89 | 2 | 0 | 5 | 0 | 0 | 0 | L | `femacs/lib/include/binary-io.rs` | done |
| `lib/boot-time-aux.h` | 426 | 11 | 0 | 3 | 0 | 0 | 2 | L | `femacs/lib/include/boot-time-aux.rs` | todo |
| `lib/boot-time.c` | 306 | 3 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/boot-time.rs` | done |
| `lib/boot-time.h` | 49 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/boot-time.rs` | done |
| `lib/byteswap.c` | 20 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/byteswap.rs` | done |
| `lib/byteswap.in.h` | 73 | 3 | 0 | 2 | 1 | 0 | 0 | L | `femacs/lib/include/byteswap.in.rs` | done |
| `lib/c++defs.h` | 367 | 0 | 0 | 23 | 0 | 0 | 0 | L | `femacs/lib/include/c++defs.rs` | todo |
| `lib/c-ctype.c` | 20 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/c-ctype.rs` | done |
| `lib/c-ctype.h` | 366 | 15 | 0 | 12 | 1 | 0 | 0 | L | `femacs/lib/include/c-ctype.rs` | wip |
| `lib/c-strcase.h` | 61 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/c-strcase.rs` | todo |
| `lib/c-strcasecmp.c` | 51 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/c-strcasecmp.rs` | done |
| `lib/c-strncasecmp.c` | 51 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/c-strncasecmp.rs` | done |
| `lib/canonicalize-lgpl.c` | 474 | 8 | 0 | 13 | 1 | 3 | 0 | L | `femacs/lib/canonicalize-lgpl.rs` | done |
| `lib/careadlinkat.c` | 173 | 2 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/careadlinkat.rs` | done |
| `lib/careadlinkat.h` | 82 | 0 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/careadlinkat.rs` | wip |
| `lib/cdefs.h` | 900 | 0 | 0 | 125 | 5 | 0 | 0 | L | `femacs/lib/include/cdefs.rs` | todo |
| `lib/cloexec.c` | 83 | 2 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/cloexec.rs` | done |
| `lib/cloexec.h` | 44 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/include/cloexec.rs` | done |
| `lib/close-stream.c` | 77 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/close-stream.rs` | done |
| `lib/close-stream.h` | 30 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/include/close-stream.rs` | done |
| `lib/copy-file-range.c` | 99 | 1 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/copy-file-range.rs` | done |
| `lib/diffseq.h` | 578 | 2 | 0 | 5 | 1 | 10 | 0 | L | `femacs/lib/include/diffseq.rs` | todo |
| `lib/dirent-private.h` | 67 | 0 | 0 | 4 | 1 | 7 | 0 | L | `femacs/lib/include/dirent-private.rs` | todo |
| `lib/dirent.in.h` | 427 | 0 | 0 | 29 | 14 | 2 | 0 | L | `femacs/lib/include/dirent.in.rs` | todo |
| `lib/dirfd.c` | 44 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/dirfd.rs` | done |
| `lib/dtoastr.c` | 19 | 0 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/dtoastr.rs` | done |
| `lib/dtotimespec.c` | 53 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/dtotimespec.rs` | done |
| `lib/dup2.c` | 205 | 5 | 0 | 3 | 0 | 0 | 0 | L | `femacs/lib/dup2.rs` | done |
| `lib/dynarray.h` | 284 | 0 | 0 | 8 | 0 | 0 | 0 | L | `femacs/lib/include/dynarray.rs` | todo |
| `lib/endian.c` | 22 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/endian.rs` | done |
| `lib/endian.in.h` | 301 | 12 | 0 | 30 | 15 | 0 | 0 | L | `femacs/lib/include/endian.in.rs` | done |
| `lib/errno.in.h` | 270 | 0 | 0 | 74 | 73 | 0 | 0 | L | `femacs/lib/include/errno.in.rs` | todo |
| `lib/euidaccess.c` | 217 | 2 | 0 | 14 | 4 | 0 | 0 | L | `femacs/lib/euidaccess.rs` | done |
| `lib/execinfo.c` | 20 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/execinfo.rs` | done |
| `lib/execinfo.in.h` | 69 | 3 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/execinfo.in.rs` | todo |
| `lib/faccessat.c` | 98 | 2 | 0 | 8 | 0 | 0 | 0 | L | `femacs/lib/faccessat.rs` | done |
| `lib/fchmodat.c` | 159 | 2 | 0 | 8 | 1 | 0 | 0 | L | `femacs/lib/fchmodat.rs` | done |
| `lib/fcntl.c` | 663 | 6 | 0 | 4 | 2 | 0 | 0 | L | `femacs/lib/fcntl.rs` | done |
| `lib/fcntl.in.h` | 504 | 0 | 0 | 48 | 38 | 6 | 0 | L | `femacs/lib/include/fcntl.in.rs` | wip |
| `lib/fdopendir.c` | 238 | 5 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/fdopendir.rs` | done |
| `lib/file-has-acl.c` | 986 | 16 | 0 | 8 | 8 | 0 | 0 | L | `femacs/lib/file-has-acl.rs` | done |
| `lib/filemode.c` | 166 | 3 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/filemode.rs` | done |
| `lib/filemode.h` | 50 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/filemode.rs` | done |
| `lib/filename.h` | 115 | 0 | 0 | 11 | 1 | 0 | 0 | L | `femacs/lib/include/filename.rs` | done |
| `lib/filevercmp.c` | 188 | 5 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/filevercmp.rs` | done |
| `lib/filevercmp.h` | 93 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/filevercmp.rs` | done |
| `lib/fingerprint.c` | 66 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/fingerprint.rs` | done |
| `lib/fingerprint.h` | 28 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/include/fingerprint.rs` | done |
| `lib/flexmember.h` | 77 | 0 | 0 | 3 | 0 | 0 | 0 | L | `femacs/lib/include/flexmember.rs` | done |
| `lib/fpending.c` | 63 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/fpending.rs` | done |
| `lib/fpending.h` | 45 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/include/fpending.rs` | done |
| `lib/free.c` | 53 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/free.rs` | done |
| `lib/fseterr.c` | 80 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/fseterr.rs` | done |
| `lib/fseterr.h` | 55 | 0 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/fseterr.rs` | wip |
| `lib/fstatat.c` | 150 | 4 | 0 | 10 | 2 | 0 | 0 | L | `femacs/lib/fstatat.rs` | done |
| `lib/fsusage.c` | 226 | 3 | 0 | 4 | 1 | 0 | 0 | L | `femacs/lib/fsusage.rs` | done |
| `lib/fsusage.h` | 49 | 0 | 0 | 1 | 0 | 7 | 0 | L | `femacs/lib/include/fsusage.rs` | done |
| `lib/fsync.c` | 86 | 1 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/fsync.rs` | done |
| `lib/ftoastr.c` | 152 | 2 | 0 | 10 | 1 | 0 | 0 | L | `femacs/lib/ftoastr.rs` | done |
| `lib/ftoastr.h` | 162 | 0 | 0 | 17 | 0 | 0 | 0 | L | `femacs/lib/include/ftoastr.rs` | todo |
| `lib/futimens.c` | 37 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/futimens.rs` | done |
| `lib/get-permissions.c` | 291 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/get-permissions.rs` | done |
| `lib/getdelim.c` | 150 | 2 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/getdelim.rs` | done |
| `lib/getdtablesize.c` | 121 | 3 | 0 | 4 | 0 | 0 | 1 | L | `femacs/lib/getdtablesize.rs` | done |
| `lib/getgroups.c` | 121 | 2 | 0 | 2 | 1 | 0 | 0 | L | `femacs/lib/getgroups.rs` | done |
| `lib/getline.c` | 27 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/getline.rs` | done |
| `lib/getloadavg.c` | 897 | 1 | 0 | 19 | 4 | 2 | 7 | L | `femacs/lib/getloadavg.rs` | done |
| `lib/getopt-cdefs.in.h` | 74 | 0 | 0 | 5 | 1 | 0 | 0 | L | `femacs/lib/include/getopt-cdefs.in.rs` | todo |
| `lib/getopt-core.h` | 96 | 0 | 0 | 1 | 1 | 0 | 4 | L | `femacs/lib/include/getopt-core.rs` | done |
| `lib/getopt-ext.h` | 77 | 0 | 0 | 4 | 4 | 4 | 0 | L | `femacs/lib/include/getopt-ext.rs` | done |
| `lib/getopt-pfx-core.h` | 76 | 0 | 0 | 10 | 1 | 0 | 0 | L | `femacs/lib/include/getopt-pfx-core.rs` | done |
| `lib/getopt-pfx-ext.h` | 69 | 0 | 0 | 9 | 2 | 0 | 0 | L | `femacs/lib/include/getopt-pfx-ext.rs` | wip |
| `lib/getopt.c` | 811 | 6 | 0 | 8 | 0 | 0 | 5 | L | `femacs/lib/getopt.rs` | done |
| `lib/getopt.in.h` | 66 | 0 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/include/getopt.in.rs` | wip |
| `lib/getopt1.c` | 166 | 5 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/getopt1.rs` | done |
| `lib/getopt_int.h` | 118 | 0 | 0 | 2 | 4 | 9 | 0 | L | `femacs/lib/include/getopt_int.rs` | done |
| `lib/getrandom.c` | 193 | 2 | 0 | 10 | 4 | 0 | 2 | L | `femacs/lib/getrandom.rs` | done |
| `lib/gettext.h` | 542 | 18 | 0 | 27 | 4 | 0 | 0 | L | `femacs/lib/include/gettext.rs` | wip |
| `lib/gettime.c` | 51 | 2 | 0 | 0 | 0 | 1 | 0 | L | `femacs/lib/gettime.rs` | done |
| `lib/gettimeofday.c` | 154 | 2 | 0 | 4 | 0 | 0 | 2 | L | `femacs/lib/gettimeofday.rs` | done |
| `lib/group-member.c` | 108 | 4 | 0 | 0 | 1 | 2 | 0 | L | `femacs/lib/group-member.rs` | done |
| `lib/idx.h` | 152 | 0 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/idx.rs` | done |
| `lib/ieee754.in.h` | 215 | 0 | 0 | 8 | 4 | 86 | 0 | L | `femacs/lib/include/ieee754.in.rs` | todo |
| `lib/ignore-value.h` | 51 | 0 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/ignore-value.rs` | todo |
| `lib/intprops-internal.h` | 413 | 0 | 0 | 29 | 3 | 0 | 0 | L | `femacs/lib/include/intprops-internal.rs` | todo |
| `lib/intprops.h` | 377 | 0 | 0 | 38 | 0 | 0 | 0 | L | `femacs/lib/include/intprops.rs` | todo |
| `lib/inttypes.in.h` | 1,029 | 0 | 0 | 167 | 113 | 0 | 0 | L | `femacs/lib/include/inttypes.in.rs` | todo |
| `lib/issymlink.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/issymlink.rs` | done |
| `lib/issymlink.h` | 65 | 1 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/issymlink.rs` | wip |
| `lib/issymlinkat.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/issymlinkat.rs` | done |
| `lib/issymlinkat.h` | 68 | 1 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/issymlinkat.rs` | wip |
| `lib/lchmod.c` | 97 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/lchmod.rs` | done |
| `lib/libc-config.h` | 209 | 0 | 0 | 16 | 1 | 0 | 0 | L | `femacs/lib/include/libc-config.rs` | todo |
| `lib/limits.in.h` | 151 | 0 | 0 | 28 | 5 | 0 | 0 | L | `femacs/lib/include/limits.in.rs` | wip |
| `lib/lstat.c` | 97 | 2 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/lstat.rs` | done |
| `lib/malloc.c` | 56 | 1 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/malloc.rs` | done |
| `lib/malloc/dynarray-skeleton.c` | 528 | 1 | 0 | 7 | 1 | 7 | 0 | L | `femacs/lib/malloc/dynarray-skeleton.rs` | todo |
| `lib/malloc/dynarray.h` | 177 | 2 | 0 | 1 | 0 | 5 | 0 | L | `femacs/lib/malloc/include/dynarray.rs` | todo |
| `lib/malloc/dynarray_at_failure.c` | 40 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/dynarray_at_failure.rs` | todo |
| `lib/malloc/dynarray_emplace_enlarge.c` | 77 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/dynarray_emplace_enlarge.rs` | todo |
| `lib/malloc/dynarray_finalize.c` | 66 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/dynarray_finalize.rs` | todo |
| `lib/malloc/dynarray_resize.c` | 68 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/dynarray_resize.rs` | todo |
| `lib/malloc/dynarray_resize_clear.c` | 39 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/dynarray_resize_clear.rs` | todo |
| `lib/malloc/scratch_buffer.h` | 135 | 5 | 0 | 1 | 0 | 2 | 0 | L | `femacs/lib/malloc/include/scratch_buffer.rs` | todo |
| `lib/malloc/scratch_buffer_grow.c` | 56 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/scratch_buffer_grow.rs` | todo |
| `lib/malloc/scratch_buffer_grow_preserve.c` | 67 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/scratch_buffer_grow_preserve.rs` | todo |
| `lib/malloc/scratch_buffer_set_array_size.c` | 64 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/malloc/scratch_buffer_set_array_size.rs` | todo |
| `lib/md5-stream.c` | 134 | 1 | 0 | 7 | 2 | 0 | 0 | L | `femacs/lib/md5-stream.rs` | done |
| `lib/md5.c` | 394 | 7 | 0 | 16 | 1 | 0 | 0 | L | `femacs/lib/md5.rs` | done |
| `lib/md5.h` | 181 | 0 | 0 | 14 | 5 | 7 | 0 | L | `femacs/lib/include/md5.rs` | done |
| `lib/memeq.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/memeq.rs` | done |
| `lib/memmem.c` | 71 | 1 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/memmem.rs` | done |
| `lib/mempcpy.c` | 33 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/mempcpy.rs` | done |
| `lib/memrchr.c` | 158 | 1 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/memrchr.rs` | done |
| `lib/memset_explicit.c` | 54 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/memset_explicit.rs` | done |
| `lib/min-eloop-threshold.h` | 42 | 0 | 0 | 2 | 2 | 0 | 0 | L | `femacs/lib/include/min-eloop-threshold.rs` | done |
| `lib/min-max.h` | 6 | 0 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/min-max.rs` | todo |
| `lib/mini-gmp-gnulib.c` | 52 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/mini-gmp-gnulib.rs` | done |
| `lib/mini-gmp.c` | 4,699 | 205 | 0 | 39 | 5 | 6 | 1 | L | `femacs/lib/mini-gmp.rs` | done |
| `lib/mini-gmp.h` | 314 | 0 | 0 | 6 | 0 | 0 | 1 | L | `femacs/lib/include/mini-gmp.rs` | done |
| `lib/minmax.h` | 65 | 0 | 0 | 3 | 0 | 0 | 0 | L | `femacs/lib/include/minmax.rs` | wip |
| `lib/mkostemp.c` | 46 | 1 | 0 | 2 | 1 | 0 | 0 | L | `femacs/lib/mkostemp.rs` | done |
| `lib/mktime-internal.h` | 83 | 0 | 0 | 11 | 0 | 0 | 0 | L | `femacs/lib/include/mktime-internal.rs` | done |
| `lib/mktime.c` | 582 | 12 | 0 | 8 | 6 | 0 | 0 | L | `femacs/lib/mktime.rs` | done |
| `lib/nanosleep.c` | 191 | 3 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/nanosleep.rs` | done |
| `lib/nproc.c` | 582 | 7 | 0 | 2 | 1 | 0 | 0 | L | `femacs/lib/nproc.rs` | done |
| `lib/nproc.h` | 46 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/include/nproc.rs` | done |
| `lib/nstrftime.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/nstrftime.rs` | done |
| `lib/open.c` | 240 | 3 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/open.rs` | done |
| `lib/openat-die.c` | 7 | 2 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/openat-die.rs` | done |
| `lib/openat-priv.h` | 74 | 0 | 0 | 5 | 0 | 0 | 0 | L | `femacs/lib/include/openat-priv.rs` | wip |
| `lib/openat-proc.c` | 154 | 1 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/openat-proc.rs` | done |
| `lib/openat.h` | 137 | 6 | 0 | 6 | 0 | 0 | 0 | L | `femacs/lib/include/openat.rs` | todo |
| `lib/pathmax.h` | 88 | 0 | 0 | 3 | 2 | 0 | 0 | L | `femacs/lib/include/pathmax.rs` | todo |
| `lib/pipe2.c` | 166 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/pipe2.rs` | done |
| `lib/pselect.c` | 115 | 2 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/pselect.rs` | done |
| `lib/pthread_sigmask.c` | 84 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/pthread_sigmask.rs` | done |
| `lib/qcopy-acl.c` | 125 | 2 | 0 | 3 | 3 | 0 | 0 | L | `femacs/lib/qcopy-acl.rs` | done |
| `lib/rawmemchr.c` | 158 | 1 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/rawmemchr.rs` | done |
| `lib/readlink.c` | 114 | 2 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/readlink.rs` | done |
| `lib/readlinkat.c` | 123 | 1 | 0 | 5 | 0 | 0 | 0 | L | `femacs/lib/readlinkat.rs` | done |
| `lib/readutmp.h` | 352 | 0 | 0 | 32 | 8 | 21 | 0 | L | `femacs/lib/include/readutmp.rs` | todo |
| `lib/realloc.c` | 101 | 1 | 0 | 2 | 1 | 0 | 0 | L | `femacs/lib/realloc.rs` | done |
| `lib/regcomp.c` | 3,814 | 63 | 0 | 19 | 2 | 0 | 2 | L | `femacs/lib/regcomp.rs` | done |
| `lib/regex.c` | 82 | 0 | 0 | 14 | 1 | 0 | 1 | L | `femacs/lib/regex.rs` | done |
| `lib/regex.h` | 684 | 0 | 0 | 78 | 9 | 4 | 1 | L | `femacs/lib/include/regex.rs` | done |
| `lib/regex_internal.c` | 1,718 | 34 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/regex_internal.rs` | done |
| `lib/regex_internal.h` | 850 | 12 | 0 | 90 | 66 | 102 | 0 | L | `femacs/lib/include/regex_internal.rs` | done |
| `lib/regexec.c` | 4,306 | 64 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/regexec.rs` | done |
| `lib/root-uid.h` | 30 | 0 | 0 | 2 | 1 | 0 | 0 | L | `femacs/lib/include/root-uid.rs` | todo |
| `lib/save-cwd.c` | 90 | 3 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/save-cwd.rs` | done |
| `lib/save-cwd.h` | 44 | 0 | 0 | 1 | 1 | 2 | 0 | L | `femacs/lib/include/save-cwd.rs` | done |
| `lib/scratch_buffer.h` | 117 | 0 | 0 | 6 | 0 | 0 | 0 | L | `femacs/lib/include/scratch_buffer.rs` | done |
| `lib/set-permissions.c` | 836 | 10 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/set-permissions.rs` | done |
| `lib/sha1.c` | 362 | 7 | 0 | 14 | 4 | 0 | 0 | L | `femacs/lib/sha1.rs` | done |
| `lib/sha1.h` | 136 | 0 | 0 | 4 | 4 | 8 | 0 | L | `femacs/lib/include/sha1.rs` | done |
| `lib/sha256.c` | 431 | 12 | 0 | 13 | 0 | 0 | 0 | L | `femacs/lib/sha256.rs` | done |
| `lib/sha256.h` | 142 | 0 | 0 | 3 | 3 | 4 | 0 | L | `femacs/lib/include/sha256.rs` | done |
| `lib/sha3.c` | 442 | 12 | 0 | 3 | 0 | 0 | 0 | L | `femacs/lib/sha3.rs` | done |
| `lib/sha3.h` | 132 | 0 | 0 | 1 | 1 | 6 | 0 | L | `femacs/lib/include/sha3.rs` | done |
| `lib/sha512.c` | 476 | 12 | 0 | 12 | 0 | 0 | 0 | L | `femacs/lib/sha512.rs` | done |
| `lib/sha512.h` | 145 | 0 | 0 | 3 | 3 | 4 | 0 | L | `femacs/lib/include/sha512.rs` | done |
| `lib/sig2str.c` | 364 | 3 | 0 | 4 | 1 | 0 | 0 | L | `femacs/lib/sig2str.rs` | done |
| `lib/sig2str.h` | 32 | 0 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/include/sig2str.rs` | done |
| `lib/sigdescr_np.c` | 351 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/sigdescr_np.rs` | done |
| `lib/signal.in.h` | 536 | 0 | 0 | 26 | 17 | 16 | 0 | L | `femacs/lib/include/signal.in.rs` | todo |
| `lib/stat-time.c` | 20 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stat-time.rs` | done |
| `lib/stat-time.h` | 266 | 10 | 0 | 6 | 1 | 0 | 0 | L | `femacs/lib/include/stat-time.rs` | wip |
| `lib/stdbit.c` | 23 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdbit.rs` | done |
| `lib/stdbit.in.h` | 2,229 | 173 | 0 | 90 | 33 | 0 | 0 | L | `femacs/lib/include/stdbit.in.rs` | todo |
| `lib/stdc_bit_width.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdc_bit_width.rs` | done |
| `lib/stdc_count_ones.c` | 23 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdc_count_ones.rs` | done |
| `lib/stdc_leading_zeros.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdc_leading_zeros.rs` | done |
| `lib/stdc_memreverse8u.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdc_memreverse8u.rs` | done |
| `lib/stdc_trailing_zeros.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdc_trailing_zeros.rs` | done |
| `lib/stdckdint.in.h` | 58 | 0 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/include/stdckdint.in.rs` | todo |
| `lib/stdcountof.in.h` | 124 | 0 | 0 | 4 | 1 | 1 | 0 | L | `femacs/lib/include/stdcountof.in.rs` | todo |
| `lib/stddef.in.h` | 248 | 0 | 0 | 12 | 2 | 0 | 0 | L | `femacs/lib/include/stddef.in.rs` | todo |
| `lib/stdint.in.h` | 740 | 0 | 0 | 136 | 9 | 0 | 0 | L | `femacs/lib/include/stdint.in.rs` | wip |
| `lib/stdio-consolesafe.c` | 202 | 7 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdio-consolesafe.rs` | done |
| `lib/stdio-impl.h` | 308 | 0 | 0 | 31 | 17 | 8 | 0 | L | `femacs/lib/include/stdio-impl.rs` | wip |
| `lib/stdio.in.h` | 2,224 | 1 | 0 | 84 | 11 | 0 | 0 | L | `femacs/lib/include/stdio.in.rs` | todo |
| `lib/stdlib.c` | 21 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stdlib.rs` | done |
| `lib/stdlib.in.h` | 1,991 | 2 | 0 | 75 | 15 | 7 | 0 | L | `femacs/lib/include/stdlib.in.rs` | todo |
| `lib/stpcpy.c` | 49 | 1 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/stpcpy.rs` | done |
| `lib/str-two-way.h` | 455 | 3 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/include/str-two-way.rs` | done |
| `lib/streq.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/streq.rs` | done |
| `lib/strftime.c` | 2,446 | 11 | 0 | 81 | 21 | 0 | 0 | L | `femacs/lib/strftime.rs` | done |
| `lib/strftime.h` | 104 | 0 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/include/strftime.rs` | done |
| `lib/string.in.h` | 1,852 | 2 | 0 | 45 | 4 | 0 | 0 | L | `femacs/lib/include/string.in.rs` | todo |
| `lib/strnlen.c` | 34 | 1 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/strnlen.rs` | done |
| `lib/strnul.c` | 19 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/strnul.rs` | done |
| `lib/strtoimax.c` | 70 | 1 | 0 | 5 | 0 | 0 | 0 | L | `femacs/lib/strtoimax.rs` | done |
| `lib/strtol.c` | 410 | 1 | 0 | 22 | 1 | 0 | 0 | L | `femacs/lib/strtol.rs` | done |
| `lib/strtoll.c` | 33 | 0 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/strtoll.rs` | done |
| `lib/symlink.c` | 57 | 2 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/symlink.rs` | done |
| `lib/sys-limits.h` | 38 | 0 | 0 | 1 | 1 | 0 | 0 | L | `femacs/lib/include/sys-limits.rs` | done |
| `lib/sys_random.in.h` | 105 | 0 | 0 | 4 | 2 | 0 | 0 | L | `femacs/lib/include/sys_random.in.rs` | done |
| `lib/sys_select.in.h` | 342 | 1 | 0 | 25 | 2 | 0 | 0 | L | `femacs/lib/include/sys_select.in.rs` | todo |
| `lib/sys_stat.in.h` | 995 | 2 | 0 | 71 | 10 | 14 | 0 | L | `femacs/lib/include/sys_stat.in.rs` | wip |
| `lib/sys_time.in.h` | 235 | 0 | 0 | 22 | 1 | 2 | 0 | L | `femacs/lib/include/sys_time.in.rs` | todo |
| `lib/sys_types.in.h` | 138 | 0 | 0 | 12 | 6 | 0 | 0 | L | `femacs/lib/include/sys_types.in.rs` | wip |
| `lib/tempname.c` | 284 | 9 | 0 | 16 | 5 | 0 | 0 | L | `femacs/lib/tempname.rs` | done |
| `lib/tempname.h` | 72 | 0 | 0 | 4 | 3 | 0 | 0 | L | `femacs/lib/include/tempname.rs` | done |
| `lib/time-internal.h` | 54 | 0 | 0 | 1 | 0 | 3 | 0 | L | `femacs/lib/include/time-internal.rs` | done |
| `lib/time.in.h` | 646 | 0 | 0 | 26 | 6 | 2 | 0 | L | `femacs/lib/include/time.in.rs` | wip |
| `lib/time_r.c` | 49 | 3 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/time_r.rs` | done |
| `lib/time_rz.c` | 322 | 14 | 0 | 0 | 2 | 0 | 0 | L | `femacs/lib/time_rz.rs` | done |
| `lib/timegm.c` | 57 | 2 | 0 | 0 | 0 | 0 | 0 | L | `femacs/lib/timegm.rs` | done |
| `lib/timespec-add.c` | 54 | 1 | 0 | 0 | 0 | 2 | 0 | L | `femacs/lib/timespec-add.rs` | done |
| `lib/timespec-sub.c` | 55 | 1 | 0 | 0 | 0 | 2 | 0 | L | `femacs/lib/timespec-sub.rs` | done |
| `lib/timespec.c` | 20 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/timespec.rs` | done |
| `lib/timespec.h` | 102 | 4 | 0 | 2 | 0 | 0 | 0 | L | `femacs/lib/include/timespec.rs` | done |
| `lib/u64.c` | 22 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/u64.rs` | done |
| `lib/u64.h` | 236 | 14 | 0 | 17 | 2 | 0 | 0 | L | `femacs/lib/include/u64.rs` | done |
| `lib/unistd.c` | 22 | 0 | 0 | 1 | 0 | 0 | 0 | L | `femacs/lib/unistd.rs` | done |
| `lib/unistd.in.h` | 2,447 | 2 | 0 | 91 | 10 | 0 | 1 | L | `femacs/lib/include/unistd.in.rs` | wip |
| `lib/unlocked-io.h` | 148 | 0 | 0 | 32 | 1 | 0 | 0 | L | `femacs/lib/include/unlocked-io.rs` | todo |
| `lib/utimens.c` | 690 | 7 | 0 | 5 | 1 | 0 | 2 | L | `femacs/lib/utimens.rs` | done |
| `lib/utimens.h` | 82 | 1 | 0 | 3 | 0 | 0 | 0 | L | `femacs/lib/include/utimens.rs` | done |
| `lib/utimensat.c` | 222 | 2 | 0 | 6 | 0 | 0 | 0 | L | `femacs/lib/utimensat.rs` | done |
| `lib/verify.h` | 407 | 0 | 0 | 24 | 6 | 1 | 0 | L | `femacs/lib/include/verify.rs` | wip |
| `lib/warn-on-use.h` | 153 | 0 | 0 | 4 | 0 | 0 | 0 | L | `femacs/lib/include/warn-on-use.rs` | todo |
| `lwlib/lwlib-Xaw.c` | 844 | 20 | 0 | 0 | 0 | 8 | 1 | F | `femacs/lwlib/lwlib-Xaw.rs` | todo |
| `lwlib/lwlib-Xaw.h` | 29 | 0 | 0 | 1 | 0 | 0 | 1 | F | `femacs/lwlib/include/lwlib-Xaw.rs` | todo |
| `lwlib/lwlib-Xlw.c` | 271 | 13 | 0 | 0 | 0 | 0 | 0 | F | `femacs/lwlib/lwlib-Xlw.rs` | todo |
| `lwlib/lwlib-Xlw.h` | 29 | 0 | 0 | 1 | 0 | 0 | 2 | F | `femacs/lwlib/include/lwlib-Xlw.rs` | todo |
| `lwlib/lwlib-Xm.c` | 1,964 | 57 | 0 | 0 | 4 | 0 | 1 | F | `femacs/lwlib/lwlib-Xm.rs` | todo |
| `lwlib/lwlib-Xm.h` | 58 | 0 | 0 | 1 | 0 | 0 | 1 | F | `femacs/lwlib/include/lwlib-Xm.rs` | todo |
| `lwlib/lwlib-int.h` | 75 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/lwlib/include/lwlib-int.rs` | todo |
| `lwlib/lwlib-utils.c` | 291 | 12 | 0 | 0 | 0 | 0 | 0 | F | `femacs/lwlib/lwlib-utils.rs` | todo |
| `lwlib/lwlib-utils.h` | 63 | 0 | 0 | 8 | 0 | 0 | 0 | F | `femacs/lwlib/include/lwlib-utils.rs` | todo |
| `lwlib/lwlib-widget.h` | 105 | 0 | 0 | 1 | 7 | 0 | 0 | F | `femacs/lwlib/include/lwlib-widget.rs` | todo |
| `lwlib/lwlib.c` | 1,330 | 51 | 0 | 3 | 0 | 0 | 1 | F | `femacs/lwlib/lwlib.rs` | todo |
| `lwlib/lwlib.h` | 129 | 0 | 0 | 1 | 13 | 0 | 0 | F | `femacs/lwlib/include/lwlib.rs` | todo |
| `lwlib/xlwmenu.c` | 2,920 | 64 | 0 | 7 | 1 | 0 | 6 | F | `femacs/lwlib/xlwmenu.rs` | todo |
| `lwlib/xlwmenu.h` | 87 | 0 | 0 | 44 | 43 | 0 | 1 | F | `femacs/lwlib/include/xlwmenu.rs` | todo |
| `lwlib/xlwmenuP.h` | 158 | 0 | 0 | 1 | 0 | 0 | 1 | F | `femacs/lwlib/include/xlwmenuP.rs` | todo |
| `msdos/inttypes.h` | 38 | 0 | 0 | 5 | 2 | 0 | 0 | F | `femacs/msdos/include/inttypes.rs` | todo |
| `msdos/langinfo.h` | 20 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/msdos/include/langinfo.rs` | todo |
| `nt/addpm.c` | 347 | 3 | 0 | 6 | 4 | 0 | 0 | F | `femacs/nt/addpm.rs` | todo |
| `nt/cmdproxy.c` | 965 | 17 | 0 | 5 | 1 | 0 | 3 | F | `femacs/nt/cmdproxy.rs` | todo |
| `nt/ddeclient.c` | 81 | 2 | 0 | 3 | 1 | 0 | 0 | F | `femacs/nt/ddeclient.rs` | todo |
| `nt/inc/arpa/inet.h` | 2 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/nt/inc/arpa/include/inet.rs` | todo |
| `nt/inc/dirent.h` | 59 | 0 | 0 | 3 | 2 | 9 | 0 | F | `femacs/nt/inc/include/dirent.rs` | todo |
| `nt/inc/grp.h` | 36 | 0 | 0 | 1 | 0 | 2 | 0 | F | `femacs/nt/inc/include/grp.rs` | todo |
| `nt/inc/inttypes.h` | 36 | 0 | 0 | 3 | 0 | 0 | 0 | F | `femacs/nt/inc/include/inttypes.rs` | todo |
| `nt/inc/langinfo.h` | 64 | 0 | 0 | 23 | 23 | 0 | 0 | F | `femacs/nt/inc/include/langinfo.rs` | todo |
| `nt/inc/mingw_time.h` | 32 | 0 | 0 | 6 | 3 | 0 | 0 | F | `femacs/nt/inc/include/mingw_time.rs` | todo |
| `nt/inc/ms-w32.h` | 501 | 0 | 0 | 107 | 35 | 2 | 3 | F | `femacs/nt/inc/include/ms-w32.rs` | todo |
| `nt/inc/netdb.h` | 2 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/nt/inc/include/netdb.rs` | todo |
| `nt/inc/netinet/in.h` | 2 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/nt/inc/netinet/include/in.rs` | todo |
| `nt/inc/netinet/tcp.h` | 1 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/nt/inc/netinet/include/tcp.rs` | todo |
| `nt/inc/nl_types.h` | 25 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/nt/inc/include/nl_types.rs` | todo |
| `nt/inc/pwd.h` | 29 | 0 | 0 | 1 | 0 | 8 | 0 | F | `femacs/nt/inc/include/pwd.rs` | todo |
| `nt/inc/signal.h` | 39 | 0 | 0 | 2 | 0 | 0 | 0 | F | `femacs/nt/inc/include/signal.rs` | todo |
| `nt/inc/stdalign.h` | 20 | 0 | 0 | 5 | 0 | 0 | 0 | F | `femacs/nt/inc/include/stdalign.rs` | todo |
| `nt/inc/stdbool.h` | 16 | 0 | 0 | 5 | 2 | 0 | 0 | F | `femacs/nt/inc/include/stdbool.rs` | todo |
| `nt/inc/stdint.h` | 72 | 0 | 0 | 21 | 4 | 0 | 0 | F | `femacs/nt/inc/include/stdint.rs` | todo |
| `nt/inc/sys/acl.h` | 25 | 0 | 0 | 4 | 3 | 0 | 0 | F | `femacs/nt/inc/sys/include/acl.rs` | todo |
| `nt/inc/sys/file.h` | 12 | 0 | 0 | 5 | 5 | 0 | 0 | F | `femacs/nt/inc/sys/include/file.rs` | todo |
| `nt/inc/sys/ioctl.h` | 6 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/nt/inc/sys/include/ioctl.rs` | todo |
| `nt/inc/sys/mman.h` | 31 | 0 | 0 | 9 | 6 | 0 | 0 | F | `femacs/nt/inc/sys/include/mman.rs` | todo |
| `nt/inc/sys/param.h` | 11 | 0 | 0 | 2 | 1 | 0 | 0 | F | `femacs/nt/inc/sys/include/param.rs` | todo |
| `nt/inc/sys/resource.h` | 51 | 0 | 0 | 5 | 3 | 2 | 0 | F | `femacs/nt/inc/sys/include/resource.rs` | todo |
| `nt/inc/sys/socket.h` | 287 | 0 | 0 | 66 | 5 | 0 | 1 | F | `femacs/nt/inc/sys/include/socket.rs` | todo |
| `nt/inc/sys/stat.h` | 175 | 0 | 0 | 50 | 13 | 38 | 1 | F | `femacs/nt/inc/sys/include/stat.rs` | todo |
| `nt/inc/sys/time.h` | 21 | 0 | 0 | 3 | 2 | 2 | 0 | F | `femacs/nt/inc/sys/include/time.rs` | todo |
| `nt/inc/sys/wait.h` | 33 | 0 | 0 | 6 | 5 | 0 | 0 | F | `femacs/nt/inc/sys/include/wait.rs` | todo |
| `nt/inc/unistd.h` | 41 | 0 | 0 | 8 | 6 | 0 | 0 | F | `femacs/nt/inc/include/unistd.rs` | todo |
| `nt/runemacs.c` | 270 | 3 | 0 | 1 | 0 | 0 | 0 | F | `femacs/nt/runemacs.rs` | todo |
| `oldXMenu/Activate.c` | 642 | 3 | 0 | 0 | 0 | 0 | 4 | F | `femacs/oldXMenu/Activate.rs` | todo |
| `oldXMenu/AddPane.c` | 121 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/AddPane.rs` | todo |
| `oldXMenu/AddSel.c` | 123 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/AddSel.rs` | todo |
| `oldXMenu/ChgPane.c` | 85 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/ChgPane.rs` | todo |
| `oldXMenu/ChgSel.c` | 107 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/ChgSel.rs` | todo |
| `oldXMenu/Create.c` | 752 | 2 | 0 | 16 | 8 | 0 | 0 | F | `femacs/oldXMenu/Create.rs` | todo |
| `oldXMenu/DelPane.c` | 107 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/DelPane.rs` | todo |
| `oldXMenu/DelSel.c` | 91 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/DelSel.rs` | todo |
| `oldXMenu/Destroy.c` | 135 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/Destroy.rs` | todo |
| `oldXMenu/Error.c` | 47 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/Error.rs` | todo |
| `oldXMenu/EvHand.c` | 43 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/EvHand.rs` | todo |
| `oldXMenu/FindPane.c` | 81 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/FindPane.rs` | todo |
| `oldXMenu/FindSel.c` | 104 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/FindSel.rs` | todo |
| `oldXMenu/InsPane.c` | 130 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/InsPane.rs` | todo |
| `oldXMenu/InsSel.c` | 134 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/InsSel.rs` | todo |
| `oldXMenu/Internal.c` | 1,023 | 11 | 0 | 2 | 2 | 0 | 3 | F | `femacs/oldXMenu/Internal.rs` | todo |
| `oldXMenu/Locate.c` | 97 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/Locate.rs` | todo |
| `oldXMenu/Post.c` | 104 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/Post.rs` | todo |
| `oldXMenu/Recomp.c` | 122 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/Recomp.rs` | todo |
| `oldXMenu/SetAEQ.c` | 48 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/SetAEQ.rs` | todo |
| `oldXMenu/SetFrz.c` | 47 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/SetFrz.rs` | todo |
| `oldXMenu/SetPane.c` | 62 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/SetPane.rs` | todo |
| `oldXMenu/SetSel.c` | 69 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/SetSel.rs` | todo |
| `oldXMenu/X10.h` | 77 | 0 | 0 | 6 | 5 | 0 | 0 | F | `femacs/oldXMenu/include/X10.rs` | todo |
| `oldXMenu/XCrAssoc.c` | 74 | 0 | 0 | 1 | 1 | 0 | 0 | F | `femacs/oldXMenu/XCrAssoc.rs` | todo |
| `oldXMenu/XDelAssoc.c` | 68 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/XDelAssoc.rs` | todo |
| `oldXMenu/XDestAssoc.c` | 54 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/XDestAssoc.rs` | todo |
| `oldXMenu/XLookAssoc.c` | 74 | 0 | 0 | 1 | 1 | 0 | 0 | F | `femacs/oldXMenu/XLookAssoc.rs` | todo |
| `oldXMenu/XMakeAssoc.c` | 98 | 1 | 0 | 1 | 1 | 0 | 0 | F | `femacs/oldXMenu/XMakeAssoc.rs` | todo |
| `oldXMenu/XMenu.h` | 292 | 0 | 0 | 27 | 36 | 0 | 1 | F | `femacs/oldXMenu/include/XMenu.rs` | todo |
| `oldXMenu/XMenuInt.h` | 90 | 0 | 0 | 8 | 3 | 0 | 0 | F | `femacs/oldXMenu/include/XMenuInt.rs` | todo |
| `oldXMenu/copyright.h` | 18 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/oldXMenu/include/copyright.rs` | todo |
| `oldXMenu/insque.c` | 57 | 2 | 0 | 0 | 0 | 3 | 0 | F | `femacs/oldXMenu/insque.rs` | todo |
| `src/alloc.c` | 7,639 | 209 | 22 | 94 | 40 | 60 | 51 | B | `femacs/src/alloc.rs` | done |
| `src/android-asset.h` | 528 | 9 | 0 | 0 | 2 | 19 | 1 | F | `femacs/src/include/android-asset.rs` | todo |
| `src/android-emacs.c` | 114 | 1 | 0 | 0 | 0 | 0 | 0 | F | `femacs/src/android-emacs.rs` | done |
| `src/android.c` | 7,594 | 155 | 0 | 2 | 0 | 60 | 55 | F | `femacs/src/android.rs` | done |
| `src/android.h` | 363 | 0 | 0 | 7 | 0 | 56 | 14 | F | `femacs/src/include/android.rs` | todo |
| `src/androidfns.c` | 3,797 | 50 | 35 | 1 | 16 | 5 | 8 | F | `femacs/src/androidfns.rs` | done |
| `src/androidfont.c` | 1,102 | 24 | 0 | 4 | 0 | 52 | 7 | F | `femacs/src/androidfont.rs` | done |
| `src/androidgui.h` | 930 | 0 | 0 | 37 | 125 | 194 | 2 | F | `femacs/src/include/androidgui.rs` | todo |
| `src/androidmenu.c` | 860 | 14 | 1 | 2 | 0 | 16 | 3 | F | `femacs/src/androidmenu.rs` | done |
| `src/androidselect.c` | 1,220 | 12 | 8 | 1 | 0 | 18 | 5 | F | `femacs/src/androidselect.rs` | done |
| `src/androidterm.c` | 7,117 | 132 | 0 | 1 | 4 | 40 | 7 | F | `femacs/src/androidterm.rs` | done |
| `src/androidterm.h` | 496 | 0 | 0 | 19 | 3 | 103 | 4 | F | `femacs/src/include/androidterm.rs` | todo |
| `src/androidvfs.c` | 7,950 | 181 | 1 | 5 | 10 | 81 | 25 | F | `femacs/src/androidvfs.rs` | done |
| `src/atimer.c` | 625 | 17 | 1 | 1 | 2 | 2 | 6 | B | `femacs/src/atimer.rs` | done |
| `src/atimer.h` | 84 | 0 | 0 | 1 | 3 | 6 | 0 | B | `femacs/src/include/atimer.rs` | done |
| `src/bidi.c` | 3,727 | 53 | 0 | 12 | 14 | 0 | 13 | C | `femacs/src/bidi.rs` | done |
| `src/bignum.c` | 603 | 33 | 0 | 0 | 2 | 0 | 1 | B | `femacs/src/bignum.rs` | done |
| `src/bignum.h` | 135 | 7 | 0 | 2 | 2 | 2 | 1 | B | `femacs/src/include/bignum.rs` | done |
| `src/blockinput.h` | 72 | 2 | 0 | 1 | 0 | 0 | 0 | B | `femacs/src/include/blockinput.rs` | done |
| `src/buffer.c` | 6,163 | 116 | 53 | 13 | 1 | 13 | 15 | C | `femacs/src/buffer.rs` | done |
| `src/buffer.h` | 1,824 | 112 | 0 | 53 | 9 | 134 | 6 | C | `femacs/src/include/buffer.rs` | done |
| `src/bytecode.c` | 1,852 | 8 | 2 | 21 | 1 | 5 | 0 | B | `femacs/src/bytecode.rs` | done |
| `src/callint.c` | 933 | 6 | 4 | 0 | 0 | 0 | 3 | B | `femacs/src/callint.rs` | done |
| `src/callproc.c` | 2,262 | 21 | 3 | 4 | 6 | 0 | 4 | D | `femacs/src/callproc.rs` | done |
| `src/casefiddle.c` | 764 | 17 | 11 | 0 | 6 | 9 | 0 | C | `femacs/src/casefiddle.rs` | done |
| `src/casetab.c` | 303 | 7 | 5 | 0 | 0 | 0 | 4 | C | `femacs/src/casetab.rs` | done |
| `src/category.c` | 506 | 10 | 13 | 0 | 0 | 0 | 0 | C | `femacs/src/category.rs` | done |
| `src/category.h` | 124 | 2 | 0 | 12 | 0 | 0 | 0 | C | `femacs/src/include/category.rs` | done |
| `src/ccl.c` | 2,394 | 7 | 5 | 91 | 68 | 3 | 7 | C | `femacs/src/ccl.rs` | done |
| `src/ccl.h` | 88 | 0 | 0 | 8 | 5 | 14 | 0 | C | `femacs/src/include/ccl.rs` | done |
| `src/character.c` | 1,167 | 23 | 10 | 0 | 0 | 0 | 1 | C | `femacs/src/character.rs` | done |
| `src/character.h` | 615 | 34 | 0 | 3 | 59 | 0 | 1 | C | `femacs/src/include/character.rs` | done |
| `src/charset.c` | 2,464 | 20 | 23 | 7 | 1 | 10 | 21 | C | `femacs/src/charset.rs` | done |
| `src/charset.h` | 567 | 2 | 0 | 63 | 40 | 26 | 19 | C | `femacs/src/include/charset.rs` | done |
| `src/chartab.c` | 1,391 | 31 | 13 | 4 | 0 | 0 | 2 | C | `femacs/src/chartab.rs` | done |
| `src/cm.c` | 467 | 8 | 0 | 7 | 5 | 0 | 1 | E | `femacs/src/cm.rs` | done |
| `src/cm.h` | 173 | 0 | 0 | 45 | 0 | 40 | 2 | E | `femacs/src/include/cm.rs` | done |
| `src/cmds.c` | 538 | 3 | 7 | 0 | 0 | 0 | 0 | B | `femacs/src/cmds.rs` | done |
| `src/coding.c` | 12,312 | 96 | 34 | 140 | 72 | 0 | 12 | C | `femacs/src/coding.rs` | todo |
| `src/coding.h` | 790 | 2 | 0 | 63 | 80 | 72 | 3 | C | `femacs/src/include/coding.rs` | done |
| `src/commands.h` | 30 | 0 | 0 | 3 | 0 | 0 | 0 | B | `femacs/src/include/commands.rs` | done |
| `src/comp.c` | 5,857 | 140 | 15 | 91 | 17 | 3 | 4 | E | `femacs/src/comp.rs` | done |
| `src/comp.h` | 106 | 3 | 0 | 1 | 0 | 11 | 0 | E | `femacs/src/include/comp.rs` | wip |
| `src/composite.c` | 2,322 | 26 | 6 | 3 | 1 | 3 | 7 | C | `femacs/src/composite.rs` | done |
| `src/composite.h` | 362 | 5 | 0 | 53 | 20 | 11 | 2 | C | `femacs/src/include/composite.rs` | done |
| `src/conf_post.h` | 445 | 0 | 0 | 66 | 5 | 0 | 0 | B | `femacs/src/include/conf_post.rs` | todo |
| `src/cygw32.c` | 139 | 5 | 2 | 0 | 0 | 0 | 0 | F | `femacs/src/cygw32.rs` | done |
| `src/cygw32.h` | 40 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/src/include/cygw32.rs` | todo |
| `src/data.c` | 4,400 | 60 | 120 | 1 | 12 | 2 | 0 | B | `femacs/src/data.rs` | done |
| `src/dbusbind.c` | 2,190 | 24 | 6 | 19 | 1 | 0 | 3 | D | `femacs/src/dbusbind.rs` | done |
| `src/decompress.c` | 349 | 6 | 2 | 4 | 1 | 5 | 3 | D | `femacs/src/decompress.rs` | done |
| `src/dired.c` | 1,213 | 14 | 8 | 3 | 3 | 0 | 0 | D | `femacs/src/dired.rs` | done |
| `src/dispextern.h` | 4,025 | 11 | 0 | 130 | 219 | 551 | 16 | E | `femacs/src/include/dispextern.rs` | wip |
| `src/dispnew.c` | 7,584 | 150 | 13 | 4 | 1 | 13 | 20 | E | `femacs/src/dispnew.rs` | done |
| `src/disptab.h` | 117 | 0 | 0 | 20 | 14 | 0 | 0 | B | `femacs/src/include/disptab.rs` | wip |
| `src/doc.c` | 746 | 6 | 6 | 6 | 0 | 0 | 2 | B | `femacs/src/doc.rs` | done |
| `src/doprnt.c` | 607 | 6 | 0 | 0 | 1 | 0 | 0 | B | `femacs/src/doprnt.rs` | done |
| `src/dosfns.c` | 798 | 12 | 10 | 2 | 0 | 0 | 1 | F | `femacs/src/dosfns.rs` | done |
| `src/dosfns.h` | 35 | 0 | 0 | 1 | 1 | 0 | 0 | F | `femacs/src/include/dosfns.rs` | todo |
| `src/dynlib.c` | 340 | 14 | 0 | 3 | 2 | 0 | 2 | D | `femacs/src/dynlib.rs` | done |
| `src/dynlib.h` | 41 | 0 | 0 | 1 | 0 | 0 | 0 | D | `femacs/src/include/dynlib.rs` | done |
| `src/editfns.c` | 4,975 | 41 | 80 | 7 | 0 | 4 | 2 | C | `femacs/src/editfns.rs` | done |
| `src/emacs-icon.h` | 353 | 0 | 0 | 2 | 2 | 0 | 0 | E | `femacs/src/include/emacs-icon.rs` | todo |
| `src/emacs-module.c` | 1,798 | 80 | 1 | 6 | 1 | 22 | 4 | G | `femacs/src/emacs-module.rs` | done |
| `src/emacs-module.in.h` | 246 | 0 | 0 | 6 | 0 | 3 | 0 | G | `femacs/src/include/emacs-module.in.rs` | todo |
| `src/emacs.c` | 3,653 | 24 | 5 | 4 | 1 | 4 | 28 | D | `femacs/src/emacs.rs` | done |
| `src/emacsgtkfixed.c` | 210 | 8 | 0 | 0 | 0 | 1 | 0 | F | `femacs/src/emacsgtkfixed.rs` | done |
| `src/emacsgtkfixed.h` | 60 | 0 | 0 | 3 | 0 | 3 | 0 | F | `femacs/src/include/emacsgtkfixed.rs` | todo |
| `src/eval.c` | 4,680 | 109 | 54 | 2 | 0 | 2 | 4 | B | `femacs/src/eval.rs` | done |
| `src/fileio.c` | 7,085 | 61 | 55 | 10 | 3 | 7 | 8 | D | `femacs/src/fileio.rs` | done |
| `src/filelock.c` | 840 | 16 | 5 | 2 | 5 | 0 | 0 | D | `femacs/src/filelock.rs` | done |
| `src/floatfns.c` | 656 | 15 | 25 | 2 | 0 | 0 | 0 | B | `femacs/src/floatfns.rs` | done |
| `src/fns.c` | 6,973 | 100 | 104 | 4 | 8 | 6 | 6 | B | `femacs/src/fns.rs` | done |
| `src/font.c` | 6,091 | 82 | 22 | 10 | 29 | 8 | 8 | E | `femacs/src/font.rs` | done |
| `src/font.h` | 1,034 | 20 | 0 | 28 | 23 | 71 | 5 | E | `femacs/src/include/font.rs` | wip |
| `src/fontset.c` | 2,257 | 35 | 7 | 30 | 0 | 0 | 6 | E | `femacs/src/fontset.rs` | done |
| `src/fontset.h` | 48 | 0 | 0 | 1 | 0 | 0 | 0 | E | `femacs/src/include/fontset.rs` | todo |
| `src/frame.c` | 7,907 | 100 | 69 | 0 | 5 | 2 | 8 | E | `femacs/src/frame.rs` | done |
| `src/frame.h` | 1,956 | 48 | 0 | 143 | 37 | 182 | 7 | E | `femacs/src/include/frame.rs` | wip |
| `src/fringe.c` | 1,880 | 20 | 4 | 9 | 5 | 6 | 4 | E | `femacs/src/fringe.rs` | done |
| `src/ftcrfont.c` | 826 | 22 | 0 | 6 | 1 | 0 | 1 | E | `femacs/src/ftcrfont.rs` | done |
| `src/ftfont.c` | 3,243 | 48 | 0 | 17 | 5 | 13 | 9 | E | `femacs/src/ftfont.rs` | done |
| `src/ftfont.h` | 94 | 0 | 0 | 1 | 0 | 14 | 0 | E | `femacs/src/include/ftfont.rs` | todo |
| `src/getpagesize.h` | 48 | 0 | 0 | 2 | 1 | 0 | 0 | B | `femacs/src/include/getpagesize.rs` | todo |
| `src/gfilenotify.c` | 336 | 3 | 4 | 0 | 0 | 0 | 1 | D | `femacs/src/gfilenotify.rs` | done |
| `src/gmalloc.c` | 1,908 | 42 | 0 | 26 | 1 | 21 | 35 | B | `femacs/src/gmalloc.rs` | done |
| `src/gnutls.c` | 3,196 | 28 | 19 | 115 | 1 | 0 | 3 | D | `femacs/src/gnutls.rs` | done |
| `src/gnutls.h` | 100 | 0 | 0 | 10 | 9 | 0 | 0 | D | `femacs/src/include/gnutls.rs` | todo |
| `src/gtkutil.c` | 6,753 | 146 | 0 | 19 | 7 | 10 | 9 | F | `femacs/src/gtkutil.rs` | done |
| `src/gtkutil.h` | 245 | 0 | 0 | 10 | 6 | 0 | 2 | F | `femacs/src/include/gtkutil.rs` | todo |
| `src/haiku.c` | 286 | 7 | 0 | 0 | 0 | 4 | 4 | F | `femacs/src/haiku.rs` | done |
| `src/haiku_io.c` | 217 | 8 | 0 | 1 | 1 | 0 | 3 | F | `femacs/src/haiku_io.rs` | done |
| `src/haiku_support.h` | 784 | 0 | 0 | 3 | 125 | 100 | 3 | F | `femacs/src/include/haiku_support.rs` | todo |
| `src/haikufns.c` | 3,324 | 56 | 34 | 5 | 0 | 12 | 7 | F | `femacs/src/haikufns.rs` | done |
| `src/haikufont.c` | 1,480 | 31 | 3 | 3 | 1 | 0 | 1 | F | `femacs/src/haikufont.rs` | done |
| `src/haikugui.h` | 205 | 0 | 0 | 35 | 19 | 4 | 0 | F | `femacs/src/include/haikugui.rs` | todo |
| `src/haikuimage.c` | 116 | 3 | 0 | 0 | 0 | 0 | 0 | F | `femacs/src/haikuimage.rs` | done |
| `src/haikumenu.c` | 845 | 14 | 2 | 0 | 0 | 0 | 2 | F | `femacs/src/haikumenu.rs` | done |
| `src/haikuselect.c` | 1,450 | 19 | 9 | 0 | 0 | 0 | 3 | F | `femacs/src/haikuselect.rs` | done |
| `src/haikuselect.h` | 88 | 0 | 0 | 1 | 3 | 0 | 0 | F | `femacs/src/include/haikuselect.rs` | todo |
| `src/haikuterm.c` | 4,898 | 108 | 0 | 3 | 1 | 0 | 6 | F | `femacs/src/haikuterm.rs` | done |
| `src/haikuterm.h` | 375 | 0 | 0 | 18 | 1 | 119 | 8 | F | `femacs/src/include/haikuterm.rs` | todo |
| `src/hbfont.c` | 612 | 9 | 0 | 28 | 0 | 0 | 2 | E | `femacs/src/hbfont.rs` | done |
| `src/image.c` | 13,805 | 223 | 12 | 152 | 209 | 61 | 12 | D | `femacs/src/image.rs` | done |
| `src/indent.c` | 2,592 | 19 | 7 | 2 | 1 | 0 | 5 | C | `femacs/src/indent.rs` | done |
| `src/indent.h` | 60 | 0 | 0 | 1 | 0 | 6 | 1 | C | `femacs/src/include/indent.rs` | done |
| `src/inotify.c` | 590 | 11 | 5 | 3 | 3 | 0 | 2 | D | `femacs/src/inotify.rs` | done |
| `src/insdel.c` | 2,592 | 55 | 1 | 6 | 0 | 4 | 2 | C | `femacs/src/insdel.rs` | done |
| `src/intervals.c` | 2,416 | 53 | 0 | 2 | 0 | 0 | 0 | C | `femacs/src/intervals.rs` | done |
| `src/intervals.h` | 314 | 3 | 0 | 29 | 0 | 14 | 0 | C | `femacs/src/include/intervals.rs` | done |
| `src/itree.c` | 1,435 | 45 | 0 | 0 | 0 | 6 | 0 | C | `femacs/src/itree.rs` | done |
| `src/itree.h` | 190 | 1 | 0 | 3 | 4 | 20 | 0 | C | `femacs/src/include/itree.rs` | done |
| `src/json.c` | 1,851 | 63 | 4 | 3 | 7 | 25 | 0 | D | `femacs/src/json.rs` | done |
| `src/keyboard.c` | 14,682 | 183 | 38 | 19 | 9 | 12 | 83 | E | `femacs/src/keyboard.rs` | todo |
| `src/keyboard.h` | 535 | 9 | 0 | 18 | 23 | 29 | 26 | E | `femacs/src/include/keyboard.rs` | done |
| `src/keymap.c` | 3,487 | 37 | 29 | 0 | 0 | 14 | 10 | E | `femacs/src/keymap.rs` | done |
| `src/keymap.h` | 51 | 0 | 0 | 3 | 0 | 0 | 1 | E | `femacs/src/include/keymap.rs` | done |
| `src/kqueue.c` | 550 | 6 | 3 | 0 | 0 | 0 | 2 | D | `femacs/src/kqueue.rs` | done |
| `src/lcms.c` | 604 | 14 | 8 | 13 | 0 | 0 | 2 | D | `femacs/src/lcms.rs` | done |
| `src/lisp.h` | 5,924 | 275 | 0 | 201 | 170 | 205 | 56 | B | `femacs/src/include/lisp.rs` | done |
| `src/lread.c` | 5,901 | 105 | 17 | 20 | 18 | 24 | 12 | B | `femacs/src/lread.rs` | done |
| `src/macfont.h` | 88 | 0 | 0 | 3 | 8 | 6 | 0 | F | `femacs/src/include/macfont.rs` | todo |
| `src/macros.c` | 444 | 7 | 6 | 0 | 0 | 0 | 2 | B | `femacs/src/macros.rs` | done |
| `src/macros.h` | 51 | 0 | 0 | 1 | 0 | 0 | 2 | B | `femacs/src/include/macros.rs` | done |
| `src/macuvs.h` | 9,218 | 0 | 0 | 0 | 0 | 0 | 0 | F | `femacs/src/include/macuvs.rs` | todo |
| `src/marker.c` | 874 | 18 | 8 | 4 | 2 | 0 | 4 | C | `femacs/src/marker.rs` | done |
| `src/menu.c` | 1,638 | 30 | 3 | 0 | 0 | 3 | 7 | E | `femacs/src/menu.rs` | done |
| `src/menu.h` | 72 | 0 | 0 | 1 | 3 | 0 | 0 | E | `femacs/src/include/menu.rs` | done |
| `src/minibuf.c` | 2,759 | 24 | 24 | 4 | 2 | 0 | 9 | E | `femacs/src/minibuf.rs` | done |
| `src/module-env-25.h` | 131 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-25.rs` | todo |
| `src/module-env-26.h` | 3 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-26.rs` | todo |
| `src/module-env-27.h` | 18 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-27.rs` | todo |
| `src/module-env-28.h` | 19 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-28.rs` | todo |
| `src/module-env-29.h` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-29.rs` | todo |
| `src/module-env-30.h` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-30.rs` | todo |
| `src/module-env-31.h` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-31.rs` | todo |
| `src/module-env-32.h` | 12 | 0 | 0 | 0 | 0 | 0 | 0 | G | `femacs/src/include/module-env-32.rs` | todo |
| `src/msdos.c` | 4,350 | 96 | 5 | 33 | 22 | 12 | 38 | F | `femacs/src/msdos.rs` | done |
| `src/msdos.h` | 156 | 0 | 0 | 21 | 7 | 0 | 3 | F | `femacs/src/include/msdos.rs` | todo |
| `src/nsgui.h` | 137 | 0 | 0 | 35 | 19 | 0 | 0 | F | `femacs/src/include/nsgui.rs` | todo |
| `src/nsterm.h` | 1,395 | 0 | 0 | 140 | 21 | 102 | 8 | F | `femacs/src/include/nsterm.rs` | todo |
| `src/nsxwidget.h` | 82 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/src/include/nsxwidget.rs` | todo |
| `src/pdumper.c` | 5,920 | 202 | 4 | 16 | 56 | 92 | 11 | G | `femacs/src/pdumper.rs` | done |
| `src/pdumper.h` | 281 | 12 | 0 | 5 | 9 | 2 | 1 | G | `femacs/src/include/pdumper.rs` | done |
| `src/pgtkfns.c` | 3,906 | 63 | 39 | 6 | 4 | 0 | 9 | F | `femacs/src/pgtkfns.rs` | done |
| `src/pgtkgui.h` | 121 | 0 | 0 | 38 | 19 | 0 | 0 | F | `femacs/src/include/pgtkgui.rs` | todo |
| `src/pgtkim.c` | 311 | 15 | 1 | 0 | 0 | 0 | 0 | F | `femacs/src/pgtkim.rs` | done |
| `src/pgtkmenu.c` | 1,123 | 22 | 2 | 0 | 0 | 0 | 2 | F | `femacs/src/pgtkmenu.rs` | done |
| `src/pgtkselect.c` | 1,975 | 34 | 8 | 1 | 0 | 21 | 7 | F | `femacs/src/pgtkselect.rs` | done |
| `src/pgtkterm.c` | 7,780 | 199 | 0 | 16 | 9 | 6 | 14 | F | `femacs/src/pgtkterm.rs` | done |
| `src/pgtkterm.h` | 681 | 1 | 0 | 46 | 3 | 169 | 1 | F | `femacs/src/include/pgtkterm.rs` | todo |
| `src/print.c` | 3,069 | 47 | 11 | 3 | 7 | 44 | 12 | B | `femacs/src/print.rs` | done |
| `src/process.c` | 9,122 | 125 | 63 | 24 | 17 | 9 | 20 | D | `femacs/src/process.rs` | wip |
| `src/process.h` | 320 | 6 | 0 | 2 | 4 | 59 | 2 | D | `femacs/src/include/process.rs` | done |
| `src/profiler.c` | 733 | 24 | 9 | 0 | 3 | 3 | 7 | D | `femacs/src/profiler.rs` | done |
| `src/ralloc.c` | 1,214 | 18 | 0 | 7 | 0 | 0 | 12 | B | `femacs/src/ralloc.rs` | done |
| `src/regex-emacs.c` | 5,367 | 41 | 0 | 120 | 75 | 12 | 3 | C | `femacs/src/regex-emacs.rs` | done |
| `src/regex-emacs.h` | 202 | 0 | 0 | 1 | 20 | 16 | 3 | C | `femacs/src/include/regex-emacs.rs` | done |
| `src/region-cache.c` | 781 | 13 | 0 | 5 | 0 | 10 | 0 | C | `femacs/src/region-cache.rs` | done |
| `src/region-cache.h` | 112 | 0 | 0 | 1 | 0 | 0 | 0 | C | `femacs/src/include/region-cache.rs` | done |
| `src/scroll.c` | 993 | 9 | 0 | 1 | 0 | 6 | 0 | E | `femacs/src/scroll.rs` | done |
| `src/search.c` | 3,514 | 36 | 19 | 2 | 1 | 8 | 4 | C | `femacs/src/search.rs` | done |
| `src/sfnt.c` | 21,215 | 268 | 0 | 170 | 8 | 53 | 8 | E | `femacs/src/sfnt.rs` | done |
| `src/sfnt.h` | 2,151 | 0 | 0 | 8 | 108 | 436 | 0 | E | `femacs/src/include/sfnt.rs` | todo |
| `src/sfntfont-android.c` | 821 | 14 | 1 | 2 | 0 | 2 | 3 | F | `femacs/src/sfntfont-android.rs` | done |
| `src/sfntfont.c` | 4,222 | 60 | 0 | 0 | 2 | 92 | 13 | E | `femacs/src/sfntfont.rs` | done |
| `src/sfntfont.h` | 78 | 0 | 0 | 1 | 0 | 0 | 0 | E | `femacs/src/include/sfntfont.rs` | todo |
| `src/sort.c` | 1,178 | 30 | 0 | 3 | 2 | 7 | 0 | B | `femacs/src/sort.rs` | done |
| `src/sound.c` | 1,554 | 30 | 2 | 4 | 18 | 44 | 2 | D | `femacs/src/sound.rs` | done |
| `src/sqlite.c` | 933 | 12 | 17 | 29 | 1 | 0 | 1 | D | `femacs/src/sqlite.rs` | done |
| `src/syntax.c` | 3,851 | 40 | 21 | 5 | 3 | 13 | 7 | C | `femacs/src/syntax.rs` | done |
| `src/syntax.h` | 215 | 11 | 0 | 2 | 17 | 12 | 1 | C | `femacs/src/include/syntax.rs` | done |
| `src/sysdep.c` | 4,766 | 135 | 1 | 23 | 8 | 4 | 11 | D | `femacs/src/sysdep.rs` | done |
| `src/sysselect.h` | 98 | 3 | 0 | 8 | 2 | 0 | 0 | D | `femacs/src/include/sysselect.rs` | todo |
| `src/syssignal.h` | 88 | 0 | 0 | 10 | 1 | 0 | 1 | D | `femacs/src/include/syssignal.rs` | done |
| `src/sysstdio.h` | 47 | 0 | 0 | 4 | 2 | 0 | 0 | B | `femacs/src/include/sysstdio.rs` | todo |
| `src/systhread.c` | 553 | 41 | 0 | 2 | 2 | 0 | 3 | D | `femacs/src/systhread.rs` | done |
| `src/systhread.h` | 160 | 0 | 0 | 3 | 3 | 0 | 0 | D | `femacs/src/include/systhread.rs` | done |
| `src/systime.h` | 94 | 2 | 0 | 2 | 1 | 0 | 0 | D | `femacs/src/include/systime.rs` | done |
| `src/systty.h` | 93 | 0 | 0 | 2 | 1 | 2 | 0 | D | `femacs/src/include/systty.rs` | todo |
| `src/syswait.h` | 62 | 0 | 0 | 9 | 1 | 0 | 0 | D | `femacs/src/include/syswait.rs` | todo |
| `src/term.c` | 5,479 | 94 | 20 | 15 | 22 | 9 | 21 | E | `femacs/src/term.rs` | done |
| `src/termcap.c` | 640 | 12 | 0 | 2 | 2 | 5 | 3 | E | `femacs/src/termcap.rs` | done |
| `src/termchar.h` | 257 | 0 | 0 | 3 | 1 | 93 | 1 | E | `femacs/src/include/termchar.rs` | done |
| `src/termhooks.h` | 993 | 4 | 0 | 18 | 88 | 86 | 3 | E | `femacs/src/include/termhooks.rs` | done |
| `src/terminal.c` | 725 | 29 | 9 | 0 | 0 | 0 | 3 | E | `femacs/src/terminal.rs` | done |
| `src/terminfo.c` | 55 | 1 | 0 | 0 | 0 | 0 | 3 | E | `femacs/src/terminfo.rs` | done |
| `src/termopts.h` | 29 | 0 | 0 | 1 | 0 | 0 | 2 | E | `femacs/src/include/termopts.rs` | todo |
| `src/textconv.c` | 2,640 | 46 | 1 | 1 | 2 | 2 | 3 | E | `femacs/src/textconv.rs` | done |
| `src/textconv.h` | 163 | 0 | 0 | 1 | 13 | 12 | 0 | E | `femacs/src/include/textconv.rs` | todo |
| `src/textprop.c` | 2,477 | 26 | 20 | 2 | 5 | 0 | 2 | C | `femacs/src/textprop.rs` | done |
| `src/thread.c` | 1,314 | 41 | 21 | 2 | 0 | 14 | 4 | D | `femacs/src/thread.rs` | done |
| `src/thread.h` | 348 | 9 | 0 | 18 | 0 | 43 | 2 | D | `femacs/src/include/thread.rs` | done |
| `src/timefns.c` | 2,144 | 48 | 14 | 8 | 10 | 9 | 5 | D | `femacs/src/timefns.rs` | done |
| `src/tparam.c` | 284 | 4 | 0 | 0 | 0 | 0 | 3 | E | `femacs/src/tparam.rs` | done |
| `src/tparam.h` | 49 | 0 | 0 | 1 | 0 | 0 | 3 | E | `femacs/src/include/tparam.rs` | done |
| `src/treesit.c` | 5,634 | 70 | 58 | 58 | 1 | 6 | 24 | E | `femacs/src/treesit.rs` | done |
| `src/treesit.h` | 257 | 9 | 0 | 1 | 0 | 28 | 1 | E | `femacs/src/include/treesit.rs` | todo |
| `src/undo.c` | 490 | 10 | 1 | 0 | 0 | 0 | 1 | C | `femacs/src/undo.rs` | done |
| `src/vm-limit.c` | 220 | 6 | 0 | 3 | 4 | 0 | 6 | B | `femacs/src/vm-limit.rs` | done |
| `src/w16select.c` | 692 | 11 | 3 | 11 | 11 | 0 | 1 | F | `femacs/src/w16select.rs` | done |
| `src/w32.c` | 11,362 | 225 | 0 | 29 | 16 | 27 | 91 | F | `femacs/src/w32.rs` | done |
| `src/w32.h` | 273 | 0 | 0 | 23 | 21 | 0 | 4 | F | `femacs/src/include/w32.rs` | todo |
| `src/w32common.h` | 103 | 1 | 0 | 12 | 2 | 0 | 7 | F | `femacs/src/include/w32common.rs` | todo |
| `src/w32console.c` | 1,262 | 38 | 5 | 5 | 3 | 0 | 23 | F | `femacs/src/w32console.rs` | done |
| `src/w32cygwinx.c` | 134 | 2 | 1 | 0 | 0 | 0 | 0 | F | `femacs/src/w32cygwinx.rs` | done |
| `src/w32dwrite.c` | 1,399 | 23 | 2 | 7 | 30 | 4 | 10 | F | `femacs/src/w32dwrite.rs` | done |
| `src/w32fns.c` | 12,488 | 90 | 60 | 69 | 45 | 25 | 55 | F | `femacs/src/w32fns.rs` | done |
| `src/w32font.c` | 3,079 | 47 | 1 | 18 | 7 | 6 | 4 | F | `femacs/src/w32font.rs` | done |
| `src/w32font.h` | 120 | 0 | 0 | 11 | 7 | 18 | 0 | F | `femacs/src/include/w32font.rs` | todo |
| `src/w32gdiplus.h` | 145 | 0 | 0 | 27 | 0 | 0 | 28 | F | `femacs/src/include/w32gdiplus.rs` | todo |
| `src/w32gui.h` | 115 | 0 | 0 | 39 | 19 | 0 | 1 | F | `femacs/src/include/w32gui.rs` | todo |
| `src/w32heap.c` | 554 | 17 | 0 | 2 | 1 | 0 | 4 | F | `femacs/src/w32heap.rs` | done |
| `src/w32heap.h` | 69 | 0 | 0 | 1 | 0 | 0 | 1 | F | `femacs/src/include/w32heap.rs` | todo |
| `src/w32image.c` | 717 | 13 | 1 | 1 | 8 | 5 | 34 | F | `femacs/src/w32image.rs` | done |
| `src/w32inevt.c` | 862 | 12 | 0 | 4 | 4 | 0 | 8 | F | `femacs/src/w32inevt.rs` | done |
| `src/w32inevt.h` | 32 | 0 | 0 | 1 | 0 | 0 | 1 | F | `femacs/src/include/w32inevt.rs` | todo |
| `src/w32menu.c` | 1,850 | 19 | 1 | 7 | 5 | 4 | 12 | F | `femacs/src/w32menu.rs` | done |
| `src/w32notify.c` | 740 | 11 | 3 | 3 | 2 | 9 | 2 | F | `femacs/src/w32notify.rs` | done |
| `src/w32proc.c` | 4,903 | 65 | 20 | 16 | 9 | 6 | 28 | F | `femacs/src/w32proc.rs` | done |
| `src/w32reg.c` | 165 | 3 | 0 | 2 | 1 | 0 | 0 | F | `femacs/src/w32reg.rs` | done |
| `src/w32select.c` | 1,370 | 20 | 5 | 4 | 2 | 0 | 17 | F | `femacs/src/w32select.rs` | done |
| `src/w32select.h` | 28 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/src/include/w32select.rs` | todo |
| `src/w32term.c` | 8,414 | 151 | 0 | 11 | 9 | 0 | 37 | F | `femacs/src/w32term.rs` | done |
| `src/w32term.h` | 929 | 0 | 0 | 119 | 19 | 129 | 25 | F | `femacs/src/include/w32term.rs` | todo |
| `src/w32uniscribe.c` | 1,628 | 24 | 0 | 16 | 1 | 0 | 9 | F | `femacs/src/w32uniscribe.rs` | done |
| `src/w32xfns.c` | 447 | 12 | 0 | 3 | 3 | 0 | 8 | F | `femacs/src/w32xfns.rs` | done |
| `src/widget.c` | 514 | 20 | 0 | 1 | 0 | 0 | 0 | E | `femacs/src/widget.rs` | done |
| `src/widget.h` | 102 | 0 | 0 | 36 | 35 | 0 | 1 | E | `femacs/src/include/widget.rs` | todo |
| `src/widgetprv.h` | 70 | 0 | 0 | 1 | 0 | 0 | 0 | E | `femacs/src/include/widgetprv.rs` | todo |
| `src/window.c` | 9,740 | 112 | 121 | 1 | 5 | 64 | 13 | E | `femacs/src/window.rs` | todo |
| `src/window.h` | 1,267 | 16 | 0 | 95 | 5 | 102 | 9 | E | `femacs/src/include/window.rs` | wip |
| `src/xdisp.c` | 39,805 | 406 | 24 | 57 | 36 | 7 | 72 | E | `femacs/src/xdisp.rs` | todo |
| `src/xfaces.c` | 7,843 | 101 | 33 | 34 | 22 | 3 | 17 | E | `femacs/src/xfaces.rs` | done |
| `src/xfns.c` | 10,662 | 116 | 53 | 7 | 17 | 13 | 11 | F | `femacs/src/xfns.rs` | done |
| `src/xfont.c` | 1,166 | 21 | 0 | 0 | 0 | 4 | 2 | F | `femacs/src/xfont.rs` | done |
| `src/xftfont.c` | 832 | 22 | 0 | 1 | 1 | 4 | 2 | F | `femacs/src/xftfont.rs` | done |
| `src/xgselect.c` | 296 | 5 | 0 | 0 | 0 | 0 | 3 | F | `femacs/src/xgselect.rs` | done |
| `src/xgselect.h` | 35 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/src/include/xgselect.rs` | todo |
| `src/xmenu.c` | 2,932 | 45 | 3 | 0 | 0 | 5 | 6 | F | `femacs/src/xmenu.rs` | done |
| `src/xml.c` | 358 | 8 | 3 | 6 | 0 | 0 | 0 | D | `femacs/src/xml.rs` | done |
| `src/xrdb.c` | 513 | 11 | 0 | 1 | 1 | 0 | 1 | F | `femacs/src/xrdb.rs` | done |
| `src/xselect.c` | 3,541 | 61 | 9 | 8 | 2 | 30 | 9 | F | `femacs/src/xselect.rs` | done |
| `src/xsettings.c` | 1,421 | 28 | 3 | 19 | 26 | 9 | 7 | F | `femacs/src/xsettings.rs` | done |
| `src/xsettings.h` | 51 | 0 | 0 | 1 | 0 | 0 | 0 | F | `femacs/src/include/xsettings.rs` | todo |
| `src/xsmfns.c` | 581 | 16 | 1 | 5 | 4 | 0 | 7 | F | `femacs/src/xsmfns.rs` | done |
| `src/xterm.c` | 33,125 | 438 | 0 | 35 | 62 | 33 | 103 | F | `femacs/src/xterm.rs` | done |
| `src/xterm.h` | 2,045 | 3 | 0 | 99 | 12 | 323 | 14 | F | `femacs/src/include/xterm.rs` | todo |
| `src/xwidget.c` | 4,304 | 75 | 36 | 4 | 3 | 5 | 6 | F | `femacs/src/xwidget.rs` | done |
| `src/xwidget.h` | 243 | 6 | 0 | 11 | 2 | 53 | 0 | F | `femacs/src/include/xwidget.rs` | todo |

---

合计 **590 文件 / 678,320 行 / 10,374 函数 / 1,841 subr / 7,156 宏 / 3,898 常量 / 5,764 结构体字段 / 1,948 全局**。
