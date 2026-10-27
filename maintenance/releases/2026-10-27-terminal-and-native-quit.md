# Current terminal workflows and native menu Quit

## Terminal restoration: a real persistent-shell check

`tools/test-terminal-modes.py` keeps an owning PTY supervisor alive while the
editor runs as its child. The editor is not the session leader. All input/output
flags, control flags, local flags, baud rates and control characters are retained
before startup, during editing, immediately after normal exit and after further
input. HOME, XDG and temporary directories belong to this invocation. The
program, applicable adjacent/bundled dump, fixture, snapshot helper and driver
are hashed before and after.

The installed reference31.1 and current P393 terminal candidate both produce
the same result: native exit0, only local flag PENDIN differs immediately after
exit, and every raw attribute equals the original after the next input.
[PENDIN in the platform header](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/sys/termios.h)
is pending-input state, matching the host SDK declaration. The test preserves
that raw difference, compares all settings without just that state bit, then
requires actual canonical half-line buffering, the exact completed line, echo
and exact original raw attributes after input. It does not ignore an arbitrary
configuration difference or change the editor to clear a diagnostic flag.

The original first observer lost the PTY when its session-leading editor exited.
The persistent-supervisor observer exposed the state-bit difference. A later
observer read its buffered screen log before flushing and falsely reported no
echo; the closed original screen log contains the echo. Those failed reports
and source revisions remain. The final driver flushes before reading and binds
all known fixture/helper/dump inputs. Intermediate4/5 reports are preserved;
the final source and both product observations are frozen under
`build/acceptance/g1-terminal-mode-restoration-6`.

The unchanged P393 program SHA-256 is
`463a41e0ef992ca6131755ffae5264553ebc75d52b36f3e3622a3e3078384a54`,
with dump
`f08f08063c1cf83e5803ed3a2b395341205f28b1f8e660c55d8e16ee2e96d19b`.
All27 earlier passing key/file cases retain identical product/fixture/helper
identities and are reused. With this restored-terminal case, the bounded
normal-workflow collection is28/28. The6 earlier fatal/recoverable TTY cases
remain separate evidence; this is not allocator-free proof.

Reproduce after building a terminal product, using the runtime resource
controller around the command:

```sh
python3 tools/test-terminal-modes.py --binary build/terminal/port/src/emacs --out build/acceptance/terminal-mode-local
```

The output directory must be new. This is a macOS case, not an unsupported
platform compatibility claim.

## Native menu Quit: actual event chain

The exact signed program
`2cdeba9bd4b273380f7da729ff5074a77aea85228f61e74b8ebd557b009f5eda`
was observed through the native menu: specialized Swift terminate body,
NS_NONKEY_EVENT kind30/code0x10000001, Lisp save-buffers-kill-emacs, Fkill_emacs,
native exit0 and debugger exit0. The shared Objective-C thunk also fires at
startup and is excluded as standalone delivery proof. The source body matches
the immutable reference, so no product patch is justified. This is recorded in
`build/acceptance/g1-menu-quit-diagnosis-1`; an initial tool delivery interruption
was followed by a fresh UI observation. No paired prelaunch hash snapshot or
missing controller metrics is invented from this diagnostic.

## Boundary

The existing independent partner has adopted the bounded six TTY error cases
and actual menu event chain. It has now read the final restoration driver and
reference/candidate raw receipts and adopted the scoped28-case result.
Pending-composition cross-frame/cancel/save, later source changes and full
stable-stage compatibility remain open. This delivery does not complete G1,
G2/G3 or the whole seven-item Goal. Artwork selection remains a separate thread
of this work; no preview constitutes compatibility or performance acceptance.
