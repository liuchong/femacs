# emacs/src/nsselect.m (825 lines)

> **This file is not compiled by the frozen configuration** (`docs/baseline/frozen-build-sources.txt`,
> `docs/status.md` 3.31): the frozen target is darwin-arm64 with no window system, so `HAVE_NS`
> and `NS_IMPL_COCOA` are undefined and src/nsselect.m is one of the 77 src files `make -n`
> never reaches.  Batch F ports the Objective-C front end to Swift anyway
> (`docs/port-map.md`'s Swift table), so this ledger is the checklist of the **Cocoa/AppKit
> build** the file's `NS_IMPL_COCOA` arms describe, and the unit is
> `femacs/platform/macos/nsselect.swift`.  Line numbers are the raw reference lines.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 6
- [x] ns-own-selection-internal  (Fns_own_selection_internal, 2/2, doc ddfbb3eea97b, line 397)
- [x] ns-disown-selection-internal  (Fns_disown_selection_internal, 1/1, doc 88579a3293f6, line 449)
- [x] ns-selection-exists-p  (Fns_selection_exists_p, 0/1, doc 78f01399751f, line 469)
- [x] ns-selection-owner-p  (Fns_selection_owner_p, 0/1, doc b67785371a29, line 494)
- [x] ns-get-selection  (Fns_get_selection, 2/2, doc 9cb78c3d2999, line 514)
- [x] ns-begin-drag  (Fns_begin_drag, 3/6, doc 99cb41364f82, line 714)

## Functions defined — 20
- [x] symbol_to_nsstring  (line 52, static)
- [x] ns_symbol_to_pb  (line 63, static)
- [x] ns_string_to_symbol  (line 69, static)
- [x] clean_local_selection_data  (line 94, static)
- [x] ns_declare_pasteboard  (line 132, static)
- [x] ns_undeclare_pasteboard  (line 139, static)
- [x] ns_store_pb_change_count  (line 145, static)
- [x] ns_get_pb_change_count  (line 153, static)
- [x] ns_get_our_change_count_for  (line 160, static)
- [x] ns_string_to_pasteboard_internal  (line 169, static)
- [x] ns_get_local_selection  (line 204)
- [x] ns_get_foreign_selection  (line 222, static)
- [x] ns_string_from_pasteboard  (line 309)
- [x] ns_string_to_pasteboard  (line 383)
- [x] nxatoms_of_nsselect  (line 545)
- [x] ns_decode_data_to_pasteboard  (line 569, static)
- [x] ns_lisp_to_pasteboard  (line 656, static)
- [x] ns_dnd_action_to_operation  (line 678, static)
- [x] ns_dnd_action_from_operation  (line 693, static)
- [x] syms_of_nsselect  (line 788)

## Objective-C classes — 0
(The file has no `@implementation`; it uses `EmacsWindow`, `NSPasteboard` and the
`NSString (EmacsString)` category, each reached as a C-ABI entry - see `## ABI`.)

## Macros defined — 0

## Types and their fields — 0
(The unit declares `vectorlike_header`, `Lisp_Subr`, `Aligned_Lisp_Subr`, `specpdl_ref` and
`Lisp_Fwd` because it builds subr records and a `DEFVAR_LISP` forward; they belong to
src/lisp.h, and the layouts are asserted below, so they are not counted a second time.)

## File-scope objects — 4
- [x] static Lisp_Object Vselection_alist  (line 35, static) -> `fileprivate var Vselection_alist:
      Lisp_Object = Qnil()`; `syms_of_nsselect` sets it to `Qnil` and `staticpro`s it, as C does
- [x] static NSString NXPrimaryPboard  (line 38, static) -> `fileprivate var NXPrimaryPboard:
      NSString?`, set in `nxatoms_of_nsselect`
- [x] static NSString NXSecondaryPboard  (line 39, static) -> `fileprivate var NXSecondaryPboard:
      NSString?`
- [x] static NSMutableDictionary pasteboard_changecount  (line 42, static) ->
      `fileprivate var pasteboard_changecount: NSMutableDictionary?`

## Decisions

**The six `DEFUN`s.** C's `DEFUN` (src/lisp.h 3406-3440) is two items: a file-static
`union Aligned_Lisp_Subr` in `__DATA,subrs`, and the `Lisp_Object F…` function.  The port
keeps both, adjacently, with C's `DEFUN` text (docstring included) as the comment that
introduces them:

- the function is `@_cdecl("Fns_…")`, so the symbol keeps C's name and signature
  (`DEFUN_ARGS_ ## maxargs`, e.g. `ns-selection-exists-p`'s 0/1 takes one argument);
- the record is a `fileprivate` Swift global built by `make_aligned_subr`, which spells
  C's initialiser: header word `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS` (both are enumerators,
  so they are core ABI entries - see `## ABI`), the function slot, `min_args`/`max_args`, the
  name, a null intspec, `command_modes` = `Qnil`, `doc` = 0.  The `HAVE_NATIVE_COMP` fields
  after `doc` are C's aggregate zero-fill; the mirror gives them default values so one
  initialiser serves both configurations;
- `symbol_name` must point at *static* storage.  A Swift string literal is a temporary, so
  each name is a `StaticString` global (`lname_ns_…`), whose UTF-8 bytes live in the binary
  and are NUL-terminated - the Rust lane's `concat!($lname, "\0").as_ptr ()`;
- the function slot is C's `{.a ## maxargs = fnname}`: Swift has no unions, so
  `Lisp_Subr_function` is the single 8-byte slot and the record stores the same function
  pointer (`unsafeBitCast…to: UnsafeMutableRawPointer.self`), whichever arity C's member had.
  The core reads it back through the C type (`defsubr`/`XSUBR`);
- **the records are not statically initialised.** Swift has no static struct global with a
  function-pointer value, so the record's storage is written by Swift's lazy global
  initialisation, which the *first access* triggers - and the first access is `defsubr(&S…)`
  inside `syms_of_nsselect`, exactly where C first reads the record.  `SUBR_SECTION_ATTRIBUTE`
  (`__DATA,subrs`, src/lisp.h 3398-3402) is therefore **not reproduced**: the section exists
  so the portable dumper loads the records with fewer pages, nothing in the reference or the
  port *reads* it (`rg subrs src/pdumper.c` finds comments only), and a lazily written slot has
  no business in it.  Recorded as a deviation;
- `DEFUN`'s docstring is the fourth argument of the port's `DEFUN` text, and the record's
  `doc` field is 0 in C as well (the DOC file supplies the string).  A Swift-aware DOC scanner
  can read the `doc: /* … */` text from the comment; `tools/extract-abi.py` cannot, because it
  has no `.m -> .swift` pair and this file's digest was compared only against the frozen
  `subrs.tsv` (Evidence).

**`DEFSYM` is empty and the `V*` storage.** `#define DEFSYM(sym, name) /* empty */`
(src/lisp.h 2394) is only a marker for make-docfile, so the ten `DEFSYM` call sites in
`syms_of_nsselect` (790-799) carry no statement; they are recorded there in a comment.  The
symbols themselves come from the generated globals.  `Vns_sent_selection_hooks` is a *field of
the `globals` record* in C (`DEFVAR_LISP`'s `&globals.f_##vname`, src/lisp.h 3495-3500), and
`docs/status.md` 3.29 makes the owning unit define it.  Swift cannot emit a C-named data
symbol (`@_cdecl` refuses `var`, and `@_silgen_name` on a `var` is an *import* - both
measured), so this unit keeps the storage as a fileprivate Swift global of the C name's
spelling with C's initial `Qnil`, and no C symbol `Vns_sent_selection_hooks` exists.  Nothing
else needs it: the reference reads it only in this file (the Lisp side reaches the value
through the symbol).  `DEFVAR_LISP`'s record is a *function-local* `static struct Lisp_Fwd`;
Swift has no function-local statics, so it is a fileprivate global
(`O_FWD_Vns_sent_selection_hooks`), filled in and handed to `defvar_lisp`, which also
`staticpro`s the variable - so no separate `staticpro` call, as in C.

**Constructs another file of this lane owns.**  A Swift translation unit has no forward
declaration for another file's items, and this file is compiled on its own, so each such
construct is reached through a C-ABI entry named after the C construct (the rule the lane
adopted for header-only constructs, extended to the lane's own headers).  The ledger's `## ABI`
section names the owner of each definition:

- `ns_send_types`, `ns_return_types` (`extern NSArray *…`, src/nsterm.h 774, defined
  src/nsterm.m 300).  Swift cannot export a data symbol, so a lane variable another lane file
  reads is spelled as a C-ABI *function* of its name (`ns_send_types ()`), returning the same
  array; the definition belongs to the port of src/nsterm.m.  Both read sites in this file
  (`ns_declare_pasteboard`, `ns_string_to_pasteboard_internal`, `ns_string_from_pasteboard`)
  read the array without mutating it, so the call-per-use shape is the C's value-per-use shape.
  **This is the lane's answer to the "5 `S…` records" requirement of PORTING.md P-31**: those
  records are *data* symbols another unit takes the address of, so Swift alone cannot export
  them; see `## ABI` (3);
- `FRAME_NS_VIEW` (src/nsterm.h 1045's `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)`)
  - the macro reads `struct frame`, whose mirror belongs to the port of src/nsterm.m;
- `ns_release_object` (src/nsterm.h 1166, defined src/nsterm.m 602), which this file also
  passes as a `record_unwind_protect_ptr` callback;
- `-[EmacsWindow beginDrag:forPasteboard:withMode:returnFrameTo:prohibitSame:followTooltip:]`
  (src/nsterm.h 446-450, defined src/nsterm.m 10528) - a cross-file Objective-C method, so the
  entry takes the receiver first;
- `+[NSString stringWithLispString:]` and `-[NSString lispString]` (src/nsterm.h 367-368,
  defined src/nsfns.m 4104 and 4143).

**`calln`, the one variadic macro.** C: `#define calln(...) CALLN (Ffuncall, __VA_ARGS__)`,
`CALLN(f, ...) CALLMANY (f, ((Lisp_Object []) {__VA_ARGS__}))`, `CALLMANY(f, array)
(f) (countof (array), array)` (src/lisp.h 3455-3463).  Swift has no variadic macro and no
compound literal, and a C-ABI entry cannot be variadic (the core is Rust), so the port keeps
the macro's array half as `calln (_ args: [Lisp_Object])`, which calls the same `Ffuncall` (a
real core symbol) with C's count; the single call site (line 430) passes C's argument list in
C's order.  `error`'s variadic arguments (line 419) are the same Lisp_Object-position problem
as `add_to_log`'s in `docs/ports/nsimage.md`: `CVarArg` has no Optional-pointer conformance,
so `lispVariadicArg` passes the same bits as a `UInt`.

**`#if NS_USE_NSPasteboardTypeFileURL != 0`** (lines 71-79, 141-146, 256-260, 615-625) tests a
*macro's value*, which Swift's `#if` cannot (it tests definedness only).  src/nsterm.h 44-49
defines it as 1 for `NS_IMPL_COCOA` with macOS >= 10.13, which is this lane's target, so the
`NSPasteboardTypeFileURL`/`NSPasteboardTypeURL` arms are what the port compiles, and the
`NSFilenamesPboardType`/`NSURL` arms (with their `NSURL` variable) are recorded here, not
transcribed.  The same is true of `#if NS_USE_NSPasteboardTypeFileURL != 0` at 256.

**Dead and empty text.** The `#if 0` block at 335-390 (the disabled EOL conversion) is skipped
by the extractor's own `#if 0` rule and recorded, not transcribed; the `#endif` and `#else`
text of the arms above is likewise recorded rather than written as Swift conditionals.

**Objective-C and AppKit constructs, and their Swift counterparts.**
- ObjC nil-messaging is optional chaining (`pb?.declareTypes(…)`, `pasteboard_changecount?.…`,
  `pb?.types ?? []`) with the zero default (`?? 0`, `?? -1`, `?? []`); `[window beginDrag:…]`
  with a nil window is the explicit `else { operation = [] }` branch, because the receiver is
  an *argument* of the lane's ABI entry rather than a message.
- `[t isEqualToString: x]` with a nil `x` is `NO` in ObjC; Swift's `isEqual(to:)` takes a
  non-optional, so the two comparisons against the *mutable* globals (lines 73-77) test the
  global for nil first.
- C's manual memory: `[[NSString alloc] initWithData:…]` + `[str autorelease]` (317-323) is
  `NSString(data:encoding:)` (ARC); `temp = [[NSMutableArray alloc] init]` +
  `record_unwind_protect_ptr (ns_release_object, temp)` (600-601) is
  `NSMutableArray()` plus `UnsafeMutableRawPointer(Unmanaged.passRetained(temp).toOpaque())`,
  so the unwind-protect owns the +1 that C's `alloc` created and `unbind_to` consumes, while
  ARC's own reference keeps the array alive for the rest of the body; the
  `[[NSMutableDictionary …] retain]` of `nxatoms_of_nsselect` is the Swift global (C's own
  comment calls it a memory loss that is never released).
- `NSEnumerator` + `nextObject` loops are `for … in` (the array is a Swift sequence).
- C's `NSArray *` of `NSString *` pasteboard type names becomes `[NSPasteboard.PasteboardType]`
  at the SDK boundary through one fileprivate `pasteboardTypes` conversion; `NSPasteboardNameGeneral`
  and `NSPasteboardTypeString` are `NSPasteboard.Name`/`NSDragOperation`-family structs in
  Swift, so a place that needs C's `NSString *` (the dictionary keys, `[pb name]`,
  `symbol_to_nsstring`'s result, `ns_string_to_pasteboard_internal`'s `gtype`) keeps an
  `NSString` and converts where AppKit wants the struct (`NSPasteboard(name:)`,
  `setString(_:forType:)`, `setPropertyList(_:forType:)`).
- `NSDragOperation` is an option set in Swift: `NSDragOperationCopy` is `.copy`, and
  `NSDragOperationNone` is the empty set, so `ns_dnd_action_from_operation`'s `case
  NSDragOperationNone:` is `case []:` and a nil-receiver reply of 0 is `[]`.
- `[num longValue]` is `num.intValue` (`longValue` is unavailable in Swift and `intValue`
  is the `Int` one); `[d bytes]`/`[d length]` are `Data.withUnsafeBytes` + `count`; `NSApp` is
  `NSApp`; `[NSDictionary dictionaryWithObjectsAndKeys:…]` is a Swift dictionary literal
  bridged to `NSDictionary`, whose `valueForKey:`/`allKeysForObject:` are
  `value(forKey:)`/`allKeys(for:)` (the keys are the same pasteboard type names);
  `[NSMutableArray arrayWithCapacity:]` is `NSMutableArray(capacity:)`;
  `[types arrayByAddingObject: x]` is `types + [x]`; `intern ([t UTF8String])` is
  `intern(t.utf8String!)` (a symbol's name is a string, so the factory's result is non-nil).

**Findings while reading the reference (recorded, not changed).**
1. `Fns_begin_drag` declares `struct frame *f, *return_to;` (726) and passes `&return_to` to
   the window method.  If that message is *not* sent - `window` is nil when `FRAME_NS_VIEW`
   has no view - `return_to` stays **uninitialised** and `if (return_to)` (748) reads
   indeterminate memory (the ObjC side always writes it when it runs).  The port
   zero-initialises the local (PORTING.md 0.1-13) and its `if return_to != nil` is then false,
   which is the defined version of what the reference intends.
2. `ns_get_foreign_selection`'s type dictionary maps `NSPasteboardTypeURL` and
   `NSPasteboardTypeFileURL` to the same MIME string, so the `TARGETS` reply can contain
   duplicate values (the `containsObject:` test deduplicates by *value*, and two different
   pasteboard types map to `"text/plain"`).  Kept as it is.
3. `ns_get_pb_change_count`/`ns_get_our_change_count_for` (153-166) compare against -1 when the
   pasteboard or the count table has no entry; `defsubr`-time order (the table is built in
   `nxatoms_of_nsselect`) is what makes the ownership tests meaningful.  Kept.

## ABI

### (1) C symbols this file defines and exports (`@_cdecl`)

| C name | C prototype (src/nsterm.h unless noted) | Swift signature |
| --- | --- | --- |
| `Fns_own_selection_internal` | `Lisp_Object (Lisp_Object, Lisp_Object)` (DEFUN, lisp.h 3406) | `(Lisp_Object, Lisp_Object) -> Lisp_Object` |
| `Fns_disown_selection_internal` | `Lisp_Object (Lisp_Object)` | `(Lisp_Object) -> Lisp_Object` |
| `Fns_selection_exists_p` | `Lisp_Object (Lisp_Object)` (0/1, so `DEFUN_ARGS_1`) | `(Lisp_Object) -> Lisp_Object` |
| `Fns_selection_owner_p` | `Lisp_Object (Lisp_Object)` (0/1) | `(Lisp_Object) -> Lisp_Object` |
| `Fns_get_selection` | `Lisp_Object (Lisp_Object, Lisp_Object)` | `(Lisp_Object, Lisp_Object) -> Lisp_Object` |
| `Fns_begin_drag` | `Lisp_Object (Lisp_Object ×6)` (3/6) | `(Lisp_Object ×6) -> Lisp_Object` |
| `ns_get_local_selection` | `Lisp_Object (Lisp_Object, Lisp_Object)` (1136) | `(Lisp_Object, Lisp_Object) -> Lisp_Object` |
| `ns_string_from_pasteboard` | `Lisp_Object (id)` (1133) | `(NSPasteboard?) -> Lisp_Object` |
| `ns_string_to_pasteboard` | `void (id, Lisp_Object)` (1134) | `(NSPasteboard?, Lisp_Object) -> Void` |
| `nxatoms_of_nsselect` | `void (void)` (1138) | `() -> Void` |
| `syms_of_nsselect` | `void (void)` (1199) | `() -> Void` |

The six `S…` records C's `DEFUN` defines beside these functions are file-static in C and
`fileprivate` here, and `syms_of_nsselect` registers them with `defsubr` exactly as the
reference does.

### (2) C symbols this file calls (all `@_silgen_name`)

**a. Symbols the reference defines out of line** (the ported core defines them):

| C name | prototype | declared | defined |
| --- | --- | --- | --- |
| `defsubr` | `void defsubr (union Aligned_Lisp_Subr *)` | src/lisp.h 3444 | src/alloc.c |
| `staticpro` | `void staticpro (Lisp_Object const *)` | src/lisp.h 3919 | src/alloc.c |
| `defvar_lisp` | `void defvar_lisp (struct Lisp_Fwd const *, char const *)` | src/lisp.h 3473 | src/lread.c 5243 |
| `Fcons` / `Fcar` / `Fcdr` / `Fsetcdr` / `Fmake_vector` | subrs (`EXFUN` in globals.h) | globals.h | src/alloc.c, src/fns.c |
| `Ffuncall` | `Lisp_Object Ffuncall (ptrdiff_t, Lisp_Object *)` | globals.h 4824 | src/eval.c |
| `make_string` | `Lisp_Object make_string (const char *, ptrdiff_t)` | src/lisp.h 4541 | src/alloc.c |
| `list2` | `Lisp_Object list2 (Lisp_Object, Lisp_Object)` | src/lisp.h 4487 | src/fns.c |
| `assq_no_quit` | `Lisp_Object assq_no_quit (Lisp_Object, Lisp_Object)` | src/lisp.h 4272 | src/fns.c |
| `error` | `AVOID error (const char *, ...)` | src/lisp.h 4835 | src/eval.c |
| `signal_error` | `AVOID signal_error (const char *, Lisp_Object)` | src/lisp.h 4796 | src/eval.c |
| `check_window_system` | `void check_window_system (struct frame *)` | src/frame.h 1504 | src/frame.c |
| `decode_window_system_frame` | `struct frame *decode_window_system_frame (Lisp_Object)` | src/frame.h 1475 | src/frame.c |
| `record_unwind_protect_ptr` | `void record_unwind_protect_ptr (void (*) (void *), void *)` | src/lisp.h 4820 | src/eval.c |
| `unbind_to` | `Lisp_Object unbind_to (specpdl_ref, Lisp_Object)` | src/lisp.h 4833 | src/eval.c |

`error` and `signal_error` are declared `-> Never` (C's `AVOID` is `_Noreturn`, src/lisp.h 153).

**b. Header-only constructs** - macros, INLINE functions, enumerators and generated-globals
entries; the reference has no symbol for any of them, so the core side exports the C-ABI entry
named here:

| C name | reference form | where |
| --- | --- | --- |
| `EQ`, `NILP`, `CONSP`, `FIXNUMP`, `STRINGP`, `SYMBOLP`, `VECTORP` | `lisp_h_*` macros / INLINE predicates | src/lisp.h 390-460, 1262, 1581, 1769 |
| `XCAR`, `XCDR` | `lisp_h_XCAR` / `lisp_h_XCDR` | src/lisp.h 404-406, 455-456, 1498-1503 |
| `XFIXNUM` | `INLINE EMACS_INT XFIXNUM (Lisp_Object)` | src/lisp.h 1268 |
| `ASIZE`, `AREF`, `ASET` | `INLINE ptrdiff_t ASIZE (…)`, `INLINE Lisp_Object AREF (…)`, `INLINE void ASET (…)` | src/lisp.h 1748, 1964, 1978 |
| `make_fixnum` | `#define make_fixnum(n) lisp_h_make_fixnum (n)` | src/lisp.h 459, 1184 |
| `make_nil_vector` | `INLINE Lisp_Object make_nil_vector (ptrdiff_t)` | src/lisp.h 4621 |
| `SYMBOL_NAME` | `INLINE Lisp_Object SYMBOL_NAME (Lisp_Object)` | src/lisp.h 2350 |
| `CHECK_SYMBOL`, `CHECK_STRING`, `CHECK_LIST`, `CHECK_LIST_END` | `CHECK_TYPE` macros / INLINE | src/lisp.h 389, 444, 1588, 3326-3332 |
| `intern` | `INLINE Lisp_Object intern (const char *)` | src/lisp.h 4757 |
| `SPECPDL_INDEX` | macro over the specpdl stack, returns `specpdl_ref` | src/lisp.h 3544-3660 |
| `window_system_available` | `INLINE bool window_system_available (struct frame *)` | src/frame.h 1495 |
| `maybe_quit` | `INLINE void maybe_quit (void)` | src/lisp.h 3896 |
| `XSETFRAME` | `#define XSETFRAME(a, b) (XSETPSEUDOVECTOR (a, b, PVEC_FRAME))` | src/frame.h |
| `PVEC_SUBR`, `PSEUDOVECTOR_AREA_BITS` | `enum pvec_type` member / `enum` member | src/lisp.h 999, 1039 |
| `Lisp_Fwd_Obj` | `enum Lisp_Fwd_Type` member | src/lisp.h 532 |
| `Qnil`, `Qt`, `Qunbound`, `Qstring`, `Qfile`, `QCLIPBOARD`, `QPRIMARY`, `QSECONDARY`, `QTEXT`, `QFILE_NAME`, `QTARGETS`, `QXdndActionCopy`, `QXdndActionMove`, `QXdndActionLink`, `QXdndActionPrivate`, `Qnow` | generated globals.h `#define Qfoo builtin_lisp_symbol (iQfoo)` | `docs/baseline/globals/symbols.tsv`: `Qt` 1, `QCLIPBOARD` 5, `QFILE_NAME` 294, `QPRIMARY` 305, `QSECONDARY` 308, `QTARGETS` 311, `QTEXT` 312, `QXdndActionCopy` 317, `QXdndActionLink` 319, `QXdndActionMove` 320, `QXdndActionPrivate` 321, `Qfile` 806, `Qnow` 1409, `Qstring` 1738, `Qunbound` 1918 (`Qnil` 0) |

**c. Constructs of this lane's headers that other lane files own** (the definition owner is
named; see Decisions):

| C name | reference form | owner of the definition |
| --- | --- | --- |
| `ns_send_types`, `ns_return_types` | `extern NSArray *…` (src/nsterm.h 774) | the port of src/nsterm.m (300) |
| `FRAME_NS_VIEW` | `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (src/nsterm.h 1045) | the port of src/nsterm.m (its `struct frame` mirror) |
| `ns_release_object` | `void ns_release_object (void *)` (src/nsterm.h 1166) | the port of src/nsterm.m (602) |
| `beginDrag` (the `EmacsWindow` method) | src/nsterm.h 446-450 | the port of src/nsterm.m (10528) |
| `stringWithLispString`, `lispString` | `NSString (EmacsString)` (src/nsterm.h 367-368) | the port of src/nsfns.m (4104, 4143) |

**d. Spellings the port provides itself** (no ABI entry, because the C construct has no
function shape or the macro's text cannot be re-spelled): `eassert` (src/lisp.h 181),
`calln` (the array half of the variadic macro, 3455-3463), `lispVariadicArg` (the
`va_arg (ap, Lisp_Object)` convention), `pasteboardTypes` (C's `NSArray` of type names at the
SDK's typed-array boundary), `make_aligned_subr` (C's `DEFUN` initialiser), and the `DEFSYM`
call sites (empty in C).

### (3) What the lane still has to settle (flagged for the integrator)

1. `ns_send_types`/`ns_return_types` are reached as C-ABI *functions* of their names, because
   Swift cannot export a C data symbol.  The port of src/nsterm.m owns those two definitions
   and has to export them that way (or the lane has to add a C-ABI file, see 3).
2. `Vns_sent_selection_hooks` has no C symbol in the port (Swift cannot emit one); the unit
   that owns it keeps the storage as a Swift global.  The same situation applies to every
   `DEFVAR_*`'d variable in the `.m` files (e.g. `ns_use_srgb_colorspace`, which
   `femacs/platform/macos/nsimage.swift` currently declares as an imported C symbol - that one
   needs the same one-line change to a function entry, since its owner is the port of
   src/nsterm.m, not the core).
3. **PORTING.md P-31's five `S…` records** (`Sdraw_string`, `Sfont_drive_otf`,
   `Sfont_otf_alternates`, `Sframe_configuration`, `Srestore_frame_configuration`) are data
   symbols that *other* units take the address of (`defsubr (&Sdraw_string)` in the port of
   src/frame.c).  Swift cannot emit a data symbol at all (`@_cdecl` refuses `var`;
   `@_silgen_name` on a `var` imports), so the lane needs one of: a small C-ABI file in this
   directory that defines the C data the lane must export, or records whose address is obtained
   through a function (which would change the Rust side's `defsubr (&raw mut S…)`).  This is a
   lane-level decision, not one this file can make.
4. The `DEFUN` docstrings live in this file's comments (with `doc: /* … */`); a future DOC
   scanner for the Swift lane should read them the way `extract-abi.py` reads the Rust units'
   docstring literals.
5. `tools/extract-abi.py` has no `.m -> .swift` pair, so `--check` cannot compare this unit;
   the ledger's checklist was generated with it and the six `DEFUN` doc digests were compared
   against the frozen `subrs.tsv` (`extract-abi.py emacs/src/nsselect.m`).

## Evidence

- `swiftc --version` -> `swift-driver version: 1.168.6 Apple Swift version 6.4
  (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`, `Target: arm64-apple-macosx26.0`.
- `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/nsselect.o femacs/platform/macos/nsselect.swift` -> **exit 0**, default language mode,
  **no warnings**.
- `nm -g /tmp/nsselect.o | grep ' T _'` -> the eleven exported C names:
  `_Fns_begin_drag`, `_Fns_disown_selection_internal`, `_Fns_get_selection`,
  `_Fns_own_selection_internal`, `_Fns_selection_exists_p`, `_Fns_selection_owner_p`,
  `_ns_get_local_selection`, `_ns_string_from_pasteboard`, `_ns_string_to_pasteboard`,
  `_nxatoms_of_nsselect`, `_syms_of_nsselect` (11 of 11).
- `nm -u /tmp/nsselect.o` -> the core ABI entries this unit declares are undefined, as
  intended: `_AREF`, `_ASET`, `_ASIZE`, `_CHECK_LIST`, `_CHECK_LIST_END`, `_CHECK_STRING`,
  `_CHECK_SYMBOL`, `_CONSP`, `_EQ`, `_FIXNUMP`, `_FRAME_NS_VIEW`, `_Fcar`, `_Fcdr`, `_Fcons`,
  `_Ffuncall`, `_Fmake_vector`, `_Fsetcdr`, `_Lisp_Fwd_Obj`, `_NILP`, `_PSEUDOVECTOR_AREA_BITS`
  (and the other `Q*`, `X*`, `SPECPDL_INDEX`, `defsubr`, `staticpro`, `defvar_lisp`,
  `record_unwind_protect_ptr`, `unbind_to`, `error`, `signal_error`, `maybe_quit`,
  `list2`, `assq_no_quit`, `make_string`, `make_fixnum`, `make_nil_vector`, `intern`,
  `ns_send_types`, `ns_return_types`, `beginDrag`, `lispString`, `stringWithLispString`,
  `window_system_available`, `check_window_system`, `decode_window_system_frame`); the AppKit
  entries (`_NSApp`, `_NSPasteboardType*`, `_OBJC_CLASS_$_NSPasteboard`, …) come from the SDK.
- Layout, C side (`clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src
  -I emacs/lib` on a probe that includes `<config.h>` and `"lisp.h"`) ->
  `sizeof vectorlike_header 8 (align 8)`, `sizeof Lisp_Subr 56 (align 8)`,
  `sizeof Aligned_Lisp_Subr 56 (align 8)`, `sizeof specpdl_ref 8 (align 8)`,
  `sizeof Lisp_Fwd 16 (align 8)`; offsets `Lisp_Subr.header 0, .function 8, .min_args 16,
  .max_args 18, .symbol_name 24, .intspec 32, .command_modes 40, .doc 48`;
  `specpdl_ref.bytes 0`; `Lisp_Fwd.u 8` (`type` is a bit-field, one byte at 0);
  `Lisp_Fwd_Obj 2 PVEC_SUBR 18 PSEUDOVECTOR_AREA_BITS 24`.
  With `-DHAVE_NATIVE_COMP=1`: `sizeof Lisp_Subr 88`, `native_comp_u 56`, `native_c_name 64`.
- Layout, Swift side (`swiftc -o /tmp/run femacs/platform/macos/nsselect.swift
  /tmp/nsselect-layout-check/main.swift -Xlinker -undefined -Xlinker dynamic_lookup`, then the
  program, which prints `MemoryLayout` of the unit's own types) -> every number equals the C
  probe's: `vectorlike_header 8/8`, `Lisp_Subr 56` with the same eight offsets,
  `Aligned_Lisp_Subr 56` (align 8), `specpdl_ref 8`, `Lisp_Fwd 16` with `u` at 8; with
  `-D HAVE_NATIVE_COMP`, `Lisp_Subr 88` with `native_comp_u` 56 and `native_c_name` 64.
- `python3 tools/extract-abi.py emacs/src/nsselect.m` -> the checklist above: 6 subrs with doc
  digests `ddfbb3eea97b`/`88579a3293f6`/`78f01399751f`/`b67785371a29`/`9cb78c3d2999`/`99cb41364f82`,
  20 functions, 4 file-scope objects, 0 macros, 0 types.
- `python3 tools/check-cond-eval.py src/nsselect.m` -> `src/nsselect.m: 11 conditions, 0
  unknown, 1 disagreements [NOT VERIFIED]`, at line 576 (`#if !NS_USE_NSPasteboardTypeFileURL`,
  the `NSURL *url` declaration).  clang says that branch is dead because src/nsterm.h 44-49
  defines `NS_USE_NSPasteboardTypeFileURL` as 1 (the `NS_IMPL_COCOA` + macOS >= 10.13 arm);
  the evaluator's table is config.h's macros and has no entry for it, so it answers
  `!undefined` = live.  The other ten conditions include the `… != 0` spellings at 71, 141, 256
  and 615, which the evaluator leaves `unknown` (conservatively live) and clang also calls
  live.  The port follows clang throughout: the `NS_USE_NSPasteboardTypeFileURL` arms are what
  it compiles, the `NSFilenamesPboardType`/`NSURL` arms - including that declaration - are
  recorded, not transcribed.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (none of them this unit).
- `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`.
- `python3 tools/check-globals.py` -> `check-globals: 283 units, 0 DEFVAR_* variables without
  a definition in their own unit` (the gate reads Rust units; this unit keeps
  `Vns_sent_selection_hooks` in a Swift global, see Decisions).
- `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)` (neither is this unit).
- `python3 tools/check-symbols.py` -> `86 units compared, 2 mismatched, 0 still pending`; both
  mismatches are `lib/mempcpy` and `lib/regex` (pre-existing).
- `python3 tools/check-linkage.py` -> `3 unit object(s) compared … 0 with 0 missing symbol(s)`.
- `/tmp/nsselect.o` and the probe files were deleted after these readings.
