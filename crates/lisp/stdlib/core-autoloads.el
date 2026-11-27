;;; core-autoloads.el --- Generated autoloads for standalone lisp-core  -*- lexical-binding: t -*-

(autoload 'ansi-color-for-comint-mode-on "ansi-color"
"Set `ansi-color-for-comint-mode' to t." t)

(autoload 'ansi-color-process-output "ansi-color"
"Maybe translate SGR control sequences of comint output into text properties.

Depending on variable `ansi-color-for-comint-mode' the comint output is
either not processed, SGR control sequences are filtered using
`ansi-color-filter-region', or SGR control sequences are translated into
text properties using `ansi-color-apply-on-region'.

The comint output is assumed to lie between the marker
`comint-last-output-start' and the process-mark.

This is a good function to put in `comint-output-filter-functions'.

(fn IGNORED)")

(autoload 'ansi-color-compilation-filter "ansi-color"
"Maybe translate SGR control sequences into text properties.
This function depends on the `ansi-color-for-compilation-mode'
variable, and is meant to be used in `compilation-filter-hook'.")

(autoload 'backtrace "backtrace"
"Print a trace of Lisp function calls currently active.
Output stream used is value of `standard-output'.")

(autoload 'benchmark-call "benchmark"
"Measure the run time of calling FUNC a number REPETITIONS of times.
The result is a list (TIME GC GCTIME)
where TIME is the total time it took, in seconds.
GCTIME is the amount of time that was spent in the GC
and GC is the number of times the GC was called.

REPETITIONS can also be a floating point number, in which case it
specifies a minimum number of seconds that the benchmark execution
should take.  In that case the return value is prepended with the
number of repetitions actually used.

(fn FUNC &optional REPETITIONS)")

(autoload 'benchmark-run "benchmark"
"Time execution of FORMS.
If REPETITIONS is supplied as a number, run FORMS that many times,
accounting for the overhead of the resulting loop.  Otherwise run
FORMS once.
Return a list of the total elapsed time for execution, the number of
garbage collections that ran, and the time taken by garbage collection.
See also `benchmark-run-compiled'.

(fn &optional REPETITIONS &rest FORMS)" nil t)

(autoload 'benchmark-run-compiled "benchmark"
"Time execution of compiled version of FORMS.
This is like `benchmark-run', but what is timed is a funcall of the
byte code obtained by wrapping FORMS in a `lambda' and compiling the
result.  The overhead of the `lambda's is accounted for.

(fn &optional REPETITIONS &rest FORMS)" nil t)

(autoload 'benchmark "benchmark"
"Print the time taken for REPETITIONS executions of FORM.
Interactively, REPETITIONS is taken from the prefix arg, and
the command prompts for the form to benchmark.
For non-interactive use see also `benchmark-run' and
`benchmark-run-compiled'.
FORM can also be a function in which case we measure the time it takes
to call it without any argument.

(fn REPETITIONS FORM)" t)

(autoload 'benchmark-progn "benchmark"
"Evaluate BODY and message the time taken.
The return value is the value of the final form in BODY.

(fn &rest BODY)" nil t)

(autoload 'byte-compile-warning-enabled-p "bytecomp"
"Return non-nil if WARNING is enabled, according to `byte-compile-warnings'.

(fn WARNING &optional SYMBOL)")

(autoload 'byte-compile-disable-warning "bytecomp"
"Change `byte-compile-warnings' to disable WARNING.
If `byte-compile-warnings' is t, set it to `(not WARNING)'.
Otherwise, if the first element is `not', add WARNING, else remove it.
Normally you should let-bind `byte-compile-warnings' before calling this,
else the global value will be modified.

(fn WARNING)")

(autoload 'byte-compile-enable-warning "bytecomp"
"Change `byte-compile-warnings' to enable WARNING.
If `byte-compile-warnings' is t, do nothing.  Otherwise, if the
first element is `not', remove WARNING, else add it.
Normally you should let-bind `byte-compile-warnings' before calling this,
else the global value will be modified.

(fn WARNING)")

(autoload 'byte-compile-warn-obsolete "bytecomp"
"Warn that SYMBOL (a variable, function or generalized variable) is obsolete.
TYPE is a string that say which one of these three types it is.

(fn SYMBOL TYPE)")

(autoload 'byte-force-recompile "bytecomp"
"Recompile every `.el' file in DIRECTORY that already has a `.elc' file.
Files in subdirectories of DIRECTORY are processed also.

(fn DIRECTORY)" t)

(autoload 'byte-recompile-directory "bytecomp"
"Recompile every `.el' file in DIRECTORY that needs recompilation.
This happens when a `.elc' file exists but is older than the `.el' file.
Files in subdirectories of DIRECTORY are processed also.

If the `.elc' file does not exist, normally this function *does not*
compile the corresponding `.el' file.  However, if the prefix argument
ARG is 0, that means do compile all those files.  A nonzero
ARG means ask the user, for each such `.el' file, whether to
compile it.  A nonzero ARG also means ask about each subdirectory
before scanning it.

If the third argument FORCE is non-nil, recompile every `.el' file
that already has a `.elc' file.

This command will normally not follow symlinks when compiling
files.  If FOLLOW-SYMLINKS is non-nil, symlinked `.el' files will
also be compiled.

(fn DIRECTORY &optional ARG FORCE FOLLOW-SYMLINKS)" t)

(autoload 'byte-compile-file "bytecomp"
"Compile a file of Lisp code named FILENAME into a file of byte code.
The output file's name is generated by passing FILENAME to the
function `byte-compile-dest-file' (which see).
The value is non-nil if there were no errors, nil if errors.
If the file sets the file variable `no-byte-compile', it is not
compiled, any existing output file is removed, and the return
value is `no-byte-compile'.

See also `emacs-lisp-byte-compile-and-load'.

(fn FILENAME &optional LOAD)" t)

(autoload 'compile-defun "bytecomp"
"Compile and evaluate the current top-level form.
Print the result in the echo area.
With argument ARG, insert value in current buffer after the form.

(fn &optional ARG)" t)

(autoload 'byte-compile "bytecomp"
"If FORM is a symbol, byte-compile its function definition.
If FORM is a lambda or a macro, byte-compile it as a function.

(fn FORM)")

(autoload 'display-call-tree "bytecomp"
"Display a call graph of a specified file.
This lists which functions have been called, what functions called
them, and what functions they call.  The list includes all functions
whose definitions have been compiled in this Emacs session, as well as
all functions called by those functions.

The call graph does not include macros, inline functions, or
primitives that the byte-code interpreter knows about directly
(`eq', `cons', etc.).

The call tree also lists those functions which are not known to be called
(that is, to which no calls have been compiled), and which cannot be
invoked interactively.

(fn &optional FILENAME)" t)

(autoload 'batch-byte-compile-if-not-done "bytecomp"
"Like `byte-compile-file' but doesn't recompile if already up to date.
Use this from the command line, with `-batch';
it won't work in an interactive Emacs.")

(autoload 'batch-byte-compile "bytecomp"
"Run `byte-compile-file' on the files remaining on the command line.
Use this from the command line, with `-batch';
it won't work in an interactive Emacs.

Each file is processed even if an error occurred previously.  If
a file name denotes a directory, all Emacs Lisp source files in
that directory (that have previously been compiled) will be
recompiled if newer than the compiled files.  In this case,
NOFORCE is ignored.

For example, invoke \"emacs -batch -f batch-byte-compile $emacs/ ~/*.el\".

If NOFORCE is non-nil, don't recompile a file that seems to be
already up-to-date.

(fn &optional NOFORCE)")

(autoload 'batch-byte-recompile-directory "bytecomp"
"Run `byte-recompile-directory' on the dirs remaining on the command line.
Must be used only with `-batch', and kills Emacs on completion.
For example, invoke `emacs -batch -f batch-byte-recompile-directory .'.

Optional argument ARG is passed as second argument ARG to
`byte-recompile-directory'; see there for its possible values
and corresponding effects.

(fn &optional ARG)")

(autoload 'cl-old-struct-compat-mode "cl-lib"
"Enable backward compatibility with old-style structs.

This can be needed when using code byte-compiled using the old
macro-expansion of `cl-defstruct' that used vectors objects instead
of record objects.

This is a global minor mode.  If called interactively, toggle the
`Cl-Old-Struct-Compat mode' mode.  If the prefix argument is positive,
enable the mode, and if it is zero or negative, disable the mode.

If called from Lisp, toggle the mode if ARG is `toggle'.  Enable the
mode if ARG is nil, omitted, or is a positive number.  Disable the mode
if ARG is a negative number.

To check whether the minor mode is enabled in the current buffer,
evaluate `(default-value \\='cl-old-struct-compat-mode)'.

The mode's hook is called both when the mode is enabled and when it is
disabled.

(fn &optional ARG)" t)

(autoload 'cl-print-object "cl-print"
"Dispatcher to print OBJECT on STREAM according to its type.
You can add methods to it to customize the output.
But if you just want to print something, don't call this directly:
call other entry points instead, such as `cl-prin1'.

(fn OBJECT STREAM)")

(autoload 'cl-prin1 "cl-print"
"Print OBJECT on STREAM according to its type.
Output is further controlled by the variables
`cl-print-readably', `cl-print-compiled', along with output
variables for the standard printing functions.  See Info
node `(elisp)Output Variables'.

(fn OBJECT &optional STREAM)")

(autoload 'cl-prin1-to-string "cl-print"
"Return a string containing the `cl-prin1'-printed representation of OBJECT.

(fn OBJECT)")

(autoload 'cl-print-to-string-with-limit "cl-print"
"Return a string containing a printed representation of VALUE.
Attempt to get the length of the returned string under LIMIT
characters with appropriate settings of `print-level',
`print-length', and `cl-print-string-length'.  Use
PRINT-FUNCTION to print, which should take the arguments VALUE
and STREAM and which should respect `print-length',
`print-level', and `cl-print-string-length'.  LIMIT may be nil or
zero in which case PRINT-FUNCTION will be called with these
settings bound to nil, and it can also be t in which case
PRINT-FUNCTION will be called with their current values.

Use this function with `cl-prin1' to print an object,
abbreviating it with ellipses to fit within a size limit.

(fn PRINT-FUNCTION VALUE LIMIT)")

(autoload 'comp-function-type-spec "comp-common"
"Return the type specifier of FUNCTION.

This function returns a cons cell whose car is the function specifier,
and cdr is a symbol, either `inferred' or `declared'.  If the symbol is
`inferred', the type specifier is automatically inferred from the code
itself by the native compiler; if it is `declared', the type specifier
comes from `comp-primitive-type-specifiers' or the function type declaration
itself.

(fn FUNCTION)")

(autoload 'debug "debug"
"Enter debugger.  \\<debugger-mode-map>`\\[debugger-continue]' returns from the debugger.

In interactive sessions, this switches to a backtrace buffer and shows
the Lisp backtrace of function calls there.  In batch mode (more accurately,
when `noninteractive' is non-nil), it shows the Lisp backtrace on the
standard error stream (unless `backtrace-on-error-noninteractive' is nil),
and then kills Emacs, causing it to exit with a negative exit code.

Arguments are mainly for use when this is called from the internals
of the evaluator.

You may call with no args, or you may pass nil as the first arg and
any other args you like.  In that case, the list of args after the
first will be printed into the backtrace buffer.

If `inhibit-redisplay' is non-nil when this function is called,
the debugger will not be entered.

(fn &rest ARGS)" t)

(autoload 'debug-on-entry "debug"
"Request FUNCTION to invoke debugger each time it is called.

When called interactively, prompt for FUNCTION in the minibuffer.

This works by modifying the definition of FUNCTION.  If you tell the
debugger to continue, FUNCTION's execution proceeds.  If FUNCTION is a
normal function or a macro written in Lisp, you can also step through
its execution.  FUNCTION can also be a primitive that is not a special
form, in which case stepping is not possible.  Break-on-entry for
primitive functions only works when that function is called from Lisp.

Use \\[cancel-debug-on-entry] to cancel the effect of this command.
Redefining FUNCTION also cancels it.

(fn FUNCTION)" t)

(autoload 'cancel-debug-on-entry "debug"
"Undo effect of \\[debug-on-entry] on FUNCTION.
If FUNCTION is nil, cancel `debug-on-entry' for all functions.
When called interactively, prompt for FUNCTION in the minibuffer.
To specify a nil argument interactively, exit with an empty minibuffer.

(fn &optional FUNCTION)" t)

(autoload 'debug-on-variable-change "debug"
"Trigger a debugger invocation when VARIABLE is changed.

When called interactively, prompt for VARIABLE in the minibuffer.

This works by calling `add-variable-watcher' on VARIABLE.  If you
quit from the debugger, this will abort the change (unless the
change is caused by the termination of a let-binding).

The watchpoint may be circumvented by C code that changes the
variable directly (i.e., not via `set').  Changing the value of
the variable (e.g., `setcar' on a list variable) will not trigger
watchpoint.

Use \\[cancel-debug-on-variable-change] to cancel the effect of
this command.  Uninterning VARIABLE or making it an alias of
another symbol also cancels it.

(fn VARIABLE)" t)

(autoload 'cancel-debug-on-variable-change "debug"
"Undo effect of \\[debug-on-variable-change] on VARIABLE.
If VARIABLE is nil, cancel `debug-on-variable-change' for all variables.
When called interactively, prompt for VARIABLE in the minibuffer.
To specify a nil argument interactively, exit with an empty minibuffer.

(fn &optional VARIABLE)" t)

(autoload 'define-derived-mode "derived"
"Create a new mode CHILD which is a variant of an existing mode PARENT.

The arguments are as follows:

CHILD:     the name of the command for the derived mode.
PARENT:    the name of the command for the parent mode (e.g. `text-mode')
           or nil if there is no parent.
NAME:      a string that will appear in the mode line (e.g. \"HTML\")
DOCSTRING: an optional documentation string--if you do not supply one,
           the function will attempt to invent something useful.
KEYWORD-ARGS:
           optional arguments in the form of pairs of keyword and value.
           The following keyword arguments are currently supported:

           :group GROUP
                   Declare the customization group that corresponds
                   to this mode.  The command `customize-mode' uses this.
           :syntax-table TABLE
                   Use TABLE instead of the default (CHILD-syntax-table).
                   A nil value means to simply use the same syntax-table
                   as the parent.
           :abbrev-table TABLE
                   Use TABLE instead of the default (CHILD-abbrev-table).
                   A nil value means to simply use the same abbrev-table
                   as the parent.
           :after-hook FORM
                   A single Lisp form which is evaluated after the mode
                   hooks have been run.  It should not be quoted.
           :interactive BOOLEAN
                   Whether the derived mode should be `interactive' or not.
                   The default is t.

BODY:      forms to execute just before running the
           hooks for the new mode.  Do not use `interactive' here.

Here is how you could define LaTeX-Thesis mode as a variant of LaTeX mode:

  (define-derived-mode LaTeX-thesis-mode LaTeX-mode \"LaTeX-Thesis\")

You could then make new key bindings for `LaTeX-thesis-mode-map'
without changing regular LaTeX mode.  In this example, BODY is empty,
and DOCSTRING is generated by default.

As a more complex example, the following command uses `sgml-mode' as
the parent, and then sets the variable `case-fold-search' to nil:

  (define-derived-mode article-mode sgml-mode \"Article\"
    \"Major mode for editing technical articles.\"
    (setq case-fold-search nil))

Note that if the documentation string had been left out, it would have
been generated automatically, with a reference to the keymap.

The new mode runs the hook named MODE-hook.  For `foo-mode',
the hook will be named `foo-mode-hook'.

See Info node `(elisp)Derived Modes' for more details.

(fn CHILD PARENT NAME [DOCSTRING] [KEYWORD-ARGS...] &rest BODY)" nil t)

(autoload 'disassemble "disass"
"Print disassembled code for OBJECT in (optional) BUFFER.
OBJECT can be a symbol defined as a function, or a function itself
(a lambda expression or a byte-code-function object).
If OBJECT is not already compiled, we compile it, but do not
redefine OBJECT if it is a symbol.

(fn OBJECT &optional BUFFER INDENT INTERACTIVE-P)" t)

(autoload 'define-minor-mode "easy-mmode"
"Define a new minor mode MODE.
This defines the toggle command MODE and (by default) a control variable
MODE (you can override this with the :variable keyword, see below).
DOC is the documentation for the mode toggle command.

The defined mode command takes one optional (prefix) argument.
Interactively with no prefix argument, it toggles the mode.
A prefix argument enables the mode if the argument is positive,
and disables it otherwise.

When called from Lisp, the mode command toggles the mode if the
argument is `toggle', disables the mode if the argument is a
non-positive integer, and enables the mode otherwise (including
if the argument is omitted or nil or a positive integer).

If DOC is nil, give the mode command a basic doc-string
documenting what its argument does.  If the word \"ARG\" does not
appear in DOC, a paragraph is added to DOC explaining
usage of the mode argument.

BODY contains code to execute each time the mode is enabled or disabled.
  It is executed after toggling the mode, and before running MODE-hook.
  Before the actual body code, you can write keyword arguments, i.e.
  alternating keywords and values.  If you provide BODY, then you must
  provide at least one keyword argument (e.g. `:lighter nil`).
  The following special keywords are supported (other keywords are passed
  to `defcustom' if the minor mode is global):

:global GLOBAL	If non-nil specifies that the minor mode is not meant to be
		buffer-local, so don't make the variable MODE buffer-local.
		By default, the mode is buffer-local.
:init-value VAL	the initial value of the mode's variable.
		Note that the minor mode function won't be called by setting
		this option, so the value *reflects* the minor mode's natural
		initial state, rather than *setting* it.
		In the vast majority of cases it should be nil.
		Not used if you also specify :variable.
:lighter SPEC	Text displayed in the mode line when the mode is on.
:keymap MAP	Keymap bound to the mode keymap.  Defaults to `MODE-map'.
                If non-nil, it should be an unquoted variable name (whose value
                is a keymap), or an expression that returns either a keymap or
		a list of (KEY . BINDING) pairs where KEY and BINDING are
		suitable for `define-key'.  If you supply a KEYMAP argument
		that is not a symbol, this macro defines the variable MODE-map
		and gives it the value that KEYMAP specifies.
:interactive VAL  Whether this mode should be a command or not.  The default
                is to make it one; use nil to avoid that.  If VAL is a list,
                it's interpreted as a list of major modes this minor mode
                is useful in.
:variable PLACE	The location to use instead of the variable MODE to store
		the state of the mode.	This can be simply a different
		named variable, or a generalized variable.
		PLACE can also be of the form (GET . SET), where GET is
		an expression that returns the current state, and SET is
		a function that takes one argument, the new state, which should
                be assigned to PLACE.  If you specify a :variable, this function
                does not define a MODE variable (nor any of the terms used
		in :variable).
:after-hook     A single Lisp form which is evaluated after the mode hooks
                have been run.  It should not be quoted.

For example, you could write
  (define-minor-mode foo-mode \"If enabled, foo on you!\"
    :lighter \" Foo\" :require \\='foo :global t :group \\='hassle :version \"27.5\"
    ...BODY CODE...)

For backward compatibility with the Emacs<21 calling convention,
the keywords can also be preceded by the obsolete triplet
INIT-VALUE LIGHTER KEYMAP.

(fn MODE DOC [KEYWORD VAL ... &rest BODY])" nil t)

(autoload 'define-globalized-minor-mode "easy-mmode"
"Make a global mode GLOBAL-MODE corresponding to buffer-local minor MODE.
TURN-ON is a function that will be called with no args in every buffer
and that should try to turn MODE on if applicable for that buffer.

Each of KEY VALUE is a pair of CL-style keyword arguments.
The :predicate key specifies in which major modes should the
globalized minor mode be switched on.  The value should be t (meaning
switch on the minor mode in all major modes), nil (meaning don't
switch on in any major mode), a list of modes (meaning switch on only
in those modes and their descendants), or a list (not MODES...),
meaning switch on in any major mode except MODES.  The value can also
mix all of these forms, see the Info node `(elisp)Defining Minor Modes'
for details.  The :predicate key causes the macro to create a user option
named the same as MODE, but ending with \"-modes\" instead of \"-mode\".
That user option can then be used to customize in which modes this
globalized minor mode will be switched on.
As the minor mode defined by this function is always global, any
:global keyword is ignored.
Other keywords have the same meaning as in `define-minor-mode',
which see.  In particular, :group specifies the custom group.
The most useful keywords are those that are passed on to the `defcustom'.
It normally makes no sense to pass the :lighter or :keymap keywords
to `define-globalized-minor-mode', since these are usually passed to
the buffer-local version of the minor mode.

BODY contains code to execute each time the mode is enabled or disabled.
It is executed after toggling the mode, and before running
GLOBAL-MODE-hook.

If MODE's set-up depends on the major mode in effect when it was
enabled, then disabling and reenabling MODE should make MODE work
correctly with the current major mode.  This is important to
prevent problems with derived modes, that is, major modes that
call another major mode in their body.

When a major mode is initialized, MODE is actually turned on just
after running the major mode's hook.  However, MODE is not turned
on if the hook has explicitly disabled it.

(fn GLOBAL-MODE MODE TURN-ON [KEY VALUE]... BODY...)" nil t)

(autoload 'easy-mmode-define-keymap "easy-mmode"
"Return a keymap built from bindings BS.
BS must be a list of (KEY . BINDING) where
KEY and BINDINGS are suitable for `define-key'.
Optional NAME is passed to `make-sparse-keymap'.
Optional map M can be used to modify an existing map.
ARGS is a list of additional keyword arguments.

Valid keywords and arguments are:

  :name      Name of the keymap; overrides NAME argument.
  :dense     Non-nil for a dense keymap.
  :inherit   Parent keymap.
  :group     Ignored.
  :suppress  Non-nil to call `suppress-keymap' on keymap,
             `nodigits' to suppress digits as prefix arguments.

(fn BS &optional NAME M ARGS)")

(autoload 'easy-mmode-defmap "easy-mmode"
"Define a constant M whose value is the result of `easy-mmode-define-keymap'.
The M, BS, and ARGS arguments are as per that function.  DOC is
the constant's documentation.

This macro is deprecated; use `defvar-keymap' instead.

(fn M BS DOC &rest ARGS)" nil t)

(autoload 'easy-mmode-defsyntax "easy-mmode"
"Define variable ST as a syntax-table.
CSS contains a list of syntax specifications of the form (CHAR . SYNTAX).

(fn ST CSS DOC &rest ARGS)" nil t)

(autoload 'defclass "eieio"
"Define NAME as a new class derived from SUPERCLASS with SLOTS.
OPTIONS-AND-DOC is used as the class' options and base documentation.
SUPERCLASSES is a list of superclasses to inherit from, with SLOTS
being the slots residing in that class definition.  Supported tags are:

  :initform   - Initializing form.
  :initarg    - Tag used during initialization.
  :accessor   - Tag used to create a function to access this slot.
  :allocation - Specify where the value is stored.
                Defaults to `:instance', but could also be `:class'.
  :writer     - A function symbol which will `write' an object's slot.
  :reader     - A function symbol which will `read' an object.
  :type       - The type of data allowed in this slot (see `typep').
  :documentation
              - A string documenting use of this slot.

The following are extensions on CLOS:
  :custom     - When customizing an object, the custom :type.  Public only.
  :label      - A text string label used for a slot when customizing.
  :group      - Name of a customization group this slot belongs in.
  :printer    - A function to call to print the value of a slot.
                See `eieio-override-prin1' as an example.

A class can also have optional options.  These options happen in place
of documentation (including a :documentation tag), in addition to
documentation, or not at all.  Supported options are:

  :documentation - The doc-string used for this class.

Options added to EIEIO:

  :allow-nil-initform - Non-nil to skip typechecking of null initforms.
  :custom-groups      - List of custom group names.  Organizes slots into
                        reasonable groups for customizations.
  :abstract           - Non-nil to prevent instances of this class.
                        If a string, use as an error string if someone does
                        try to make an instance.
  :method-invocation-order
                      - Control the method invocation order if there is
                        multiple inheritance.  Valid values are:
                         :breadth-first - The default.
                         :depth-first

Options in CLOS not supported in EIEIO:

  :metaclass - Class to use in place of `standard-class'
  :default-initargs - Initargs to use when initializing new objects of
                      this class.

Due to the way class options are set up, you can add any tags you wish,
and reference them using the function `class-option'.

(fn NAME SUPERCLASSES SLOTS &rest OPTIONS-AND-DOC)" nil t)

(autoload 'make-instance "eieio"
"Make a new instance of CLASS based on INITARGS.
For example:

  (make-instance \\='foo)

INITARGS is a property list with keywords based on the `:initarg'
for each slot.  For example:

  (make-instance \\='foo :slot1 value1 :slotN valueN)

(fn CLASS &rest INITARGS)")

(autoload 'eieio-defclass-autoload "eieio-core"
"Create autoload symbols for the EIEIO class CNAME.
SUPERCLASSES are the superclasses that CNAME inherits from.
DOC is the docstring for CNAME.
This function creates a mock-class for CNAME and adds it into
SUPERCLASSES as children.
It creates an autoload function for CNAME's constructor.

(fn CNAME SUPERCLASSES FILENAME DOC)")

(autoload 'ert-deftest "ert"
"Define NAME (a symbol) as a test.

BODY is evaluated as a `progn' when the test is run.  It should
signal a condition on failure or just return if the test passes.

`should', `should-not', `should-error', `skip-when', and
`skip-unless' are useful for assertions in BODY.

Use `ert' to run tests interactively.

Tests that are expected to fail can be marked as such
using :expected-result.  See `ert-test-result-type-p' for a
description of valid values for RESULT-TYPE.

Macros in BODY are expanded when the test is defined, not when it
is run.  If a macro (possibly with side effects) is to be tested,
it has to be wrapped in `(eval (quote ...))'.

If NAME is already defined as a test and Emacs is running
in batch mode, an error is signaled.

(fn NAME () [DOCSTRING] [:expected-result RESULT-TYPE] [:tags \\='(TAG...)] BODY...)" nil t)

(autoload 'ert-run-tests-batch "ert"
"Run the tests specified by SELECTOR, printing results to the terminal.

SELECTOR selects which tests to run as described in `ert-select-tests' when
called with its second argument t, except if SELECTOR is nil, in which case
all tests rather than none will be run; this makes the command line
 \"emacs -batch -l my-tests.el -f ert-run-tests-batch-and-exit\" useful.

Returns the stats object.

(fn &optional SELECTOR)")

(autoload 'ert-run-tests-batch-and-exit "ert"
"Like `ert-run-tests-batch', but exits Emacs when done.

The exit status will be 0 if all test results were as expected, 1
on unexpected results, or 2 if the tool detected an error outside
of the tests (e.g. invalid SELECTOR or bug in the code that runs
the tests).

(fn &optional SELECTOR)")

(autoload 'ert-run-tests-interactively "ert"
"Run the tests specified by SELECTOR and display the results in a buffer.

SELECTOR selects which tests to run as described in `ert-select-tests'
when called with its second argument t.  Interactively, prompt for
SELECTOR; the default t means run all the defined tests.

(fn SELECTOR)" t)

(autoload 'ert-describe-test "ert"
"Display the documentation for TEST-OR-TEST-NAME (a symbol or ert-test).

(fn TEST-OR-TEST-NAME)" t)

(autoload 'ewoc-create "ewoc"
"Create an empty ewoc.

The ewoc will be inserted in the current buffer at the current position.

PRETTY-PRINTER should be a function that takes one argument, an
element, and inserts a string representing it in the buffer (at
point).  The string PRETTY-PRINTER inserts may be empty or span
several lines.  The PRETTY-PRINTER should use `insert', and not
`insert-before-markers'.

Optional second and third arguments HEADER and FOOTER are strings,
possibly empty, that will always be present at the top and bottom,
respectively, of the ewoc.

Normally, a newline is automatically inserted after the header,
the footer and every node's printed representation.  Optional
fourth arg NOSEP non-nil inhibits this.

(fn PRETTY-PRINTER &optional HEADER FOOTER NOSEP)")

(autoload 'find-library "find-func"
"Find the Emacs Lisp source of LIBRARY.

Interactively, prompt for LIBRARY using the one at or near point.

This function searches `find-library-source-path' if non-nil, and
`load-path' otherwise.

See the `find-library-include-other-files' user option for
customizing the candidate completions.

(fn LIBRARY)" t)

(autoload 'read-library-name "find-func"
"Read and return a library name, defaulting to the one near point.

A library name is the filename of an Emacs Lisp library located
in a directory under `load-path' (or `find-library-source-path',
if non-nil).")

(autoload 'find-library-other-window "find-func"
"Find the Emacs Lisp source of LIBRARY in another window.

See `find-library' for more details.

(fn LIBRARY)" t)

(autoload 'find-library-other-frame "find-func"
"Find the Emacs Lisp source of LIBRARY in another frame.

See `find-library' for more details.

(fn LIBRARY)" t)

(autoload 'find-function-search-for-symbol "find-func"
"Search for SYMBOL's definition of type TYPE in LIBRARY.
Visit the library in a buffer, and return a cons cell (BUFFER . POSITION),
or just (BUFFER . nil) if the definition can't be found in the file.

If TYPE is nil, look for a function definition,
otherwise, TYPE specifies the kind of definition.
TYPE is looked up in SYMBOL's property `find-function-type-alist'
(which can be maintained with `find-function-update-type-alist')
or the variable `find-function-regexp-alist'.

The search is done in the source for library LIBRARY.

(fn SYMBOL TYPE LIBRARY)")

(autoload 'find-function-update-type-alist "find-func"
"Update SYMBOL property `find-function-type-alist' with (TYPE . VARIABLE).
Property `find-function-type-alist' is a symbol-specific version
of variable `find-function-regexp-alist' and has the same format.

(fn SYMBOL TYPE VARIABLE)")

(autoload 'find-function-noselect "find-func"
"Return a pair (BUFFER . POINT) pointing to the definition of FUNCTION.

Finds the source file containing the definition of FUNCTION
in a buffer and the point of the definition.  The buffer is
not selected.  If the function definition can't be found in
the buffer, returns (BUFFER).

If FUNCTION is a built-in function, this function normally
attempts to find it in the Emacs C sources; however, if LISP-ONLY
is non-nil, signal an error instead.

(fn FUNCTION &optional LISP-ONLY)")

(autoload 'find-function "find-func"
"Find the definition of the Emacs Lisp FUNCTION near point.

Finds the source file containing the definition of the function
near point (selected by `function-called-at-point') in a buffer and
places point before the definition.
Set mark before moving, if the buffer already existed.

See also `find-function-recenter-line' and `find-function-after-hook'.

Use \\[xref-find-definitions] to find definitions of functions and variables
that are not part of Emacs.

(fn FUNCTION)" t)

(autoload 'find-function-other-window "find-func"
"Find, in another window, the definition of FUNCTION near point.

See `find-function' for more details.

(fn FUNCTION)" t)

(autoload 'find-function-other-frame "find-func"
"Find, in another frame, the definition of FUNCTION near point.

See `find-function' for more details.

(fn FUNCTION)" t)

(autoload 'find-variable-noselect "find-func"
"Return a pair `(BUFFER . POINT)' pointing to the definition of VARIABLE.

Finds the library containing the definition of VARIABLE in a buffer and
the point of the definition.  The buffer is not selected.
If the variable's definition can't be found in the buffer, return (BUFFER).

(fn VARIABLE &optional FILE)")

(autoload 'find-variable "find-func"
"Find the definition of the VARIABLE at or before point.

Finds the library containing the definition of the variable
near point (selected by `variable-at-point') in a buffer and
places point before the definition.

Set mark before moving, if the buffer already existed.

See also `find-function-recenter-line' and `find-function-after-hook'.

(fn VARIABLE)" t)

(autoload 'find-variable-other-window "find-func"
"Find, in another window, the definition of VARIABLE near point.

See `find-variable' for more details.

(fn VARIABLE)" t)

(autoload 'find-variable-other-frame "find-func"
"Find, in another frame, the definition of VARIABLE near point.

See `find-variable' for more details.

(fn VARIABLE)" t)

(autoload 'find-definition-noselect "find-func"
"Return a pair `(BUFFER . POINT)' pointing to the definition of SYMBOL.
If the definition can't be found in the buffer, return (BUFFER).
TYPE says what type of definition: nil for a function, `defvar' for a
variable, `defface' for a face.  This function does not switch to the
buffer nor display it.

(fn SYMBOL TYPE &optional FILE)")

(autoload 'find-face-definition "find-func"
"Find the definition of FACE.  FACE defaults to the name near point.

Finds the Emacs Lisp library containing the definition of the face
near point (selected by `variable-at-point') in a buffer and
places point before the definition.

Set mark before moving, if the buffer already existed.

See also `find-function-recenter-line' and `find-function-after-hook'.

(fn FACE)" t)

(autoload 'find-function-on-key "find-func"
"Find the function that KEY invokes.  KEY is a string.
Set mark before moving, if the buffer already existed.

(fn KEY)" t)

(autoload 'find-function-on-key-other-window "find-func"
"Find, in the other window, the function that KEY invokes.
See `find-function-on-key'.

(fn KEY)" t)

(autoload 'find-function-on-key-other-frame "find-func"
"Find, in the other frame, the function that KEY invokes.
See `find-function-on-key'.

(fn KEY)" t)

(autoload 'find-function-at-point "find-func"
"Find directly the function at point in the other window." t)

(autoload 'find-variable-at-point "find-func"
"Find directly the variable at point in the other window." t)

(autoload 'find-function-mode "find-func"
"Enable some key bindings for the `find-function' family of functions.

This is a global minor mode.  If called interactively, toggle the
`Find-Function mode' mode.  If the prefix argument is positive, enable
the mode, and if it is zero or negative, disable the mode.

If called from Lisp, toggle the mode if ARG is `toggle'.  Enable the
mode if ARG is nil, omitted, or is a positive number.  Disable the mode
if ARG is a negative number.

To check whether the minor mode is enabled in the current buffer,
evaluate `(default-value \\='find-function-mode)'.

The mode's hook is called both when the mode is enabled and when it is
disabled.

\\{find-function-mode-map}

(fn &optional ARG)" t)

(autoload 'find-function-setup-keys "find-func"
"Turn on `find-function-mode', which see.")

(autoload 'gv-get "gv"
"Build the code that applies DO to PLACE.
PLACE must be a valid generalized variable.
DO must be a function; it will be called with 2 arguments: GETTER and SETTER,
where GETTER is a (copyable) Elisp expression that returns the value of PLACE,
and SETTER is a function which returns the code to set PLACE when called
with a (not necessarily copyable) Elisp expression that returns the value to
set it to.
DO must return an Elisp expression.

(fn PLACE DO)")

(autoload 'gv-letplace "gv"
"Build the code manipulating the generalized variable PLACE.
GETTER will be bound to a copyable expression that returns the value
of PLACE.
SETTER will be bound to a function that takes an expression V and returns
a new expression that sets PLACE to V.
BODY should return some Elisp expression E manipulating PLACE via GETTER
and SETTER.
The returned value will then be an Elisp expression that first evaluates
all the parts of PLACE that can be evaluated and then runs E.

(fn (GETTER SETTER) PLACE &rest BODY)" nil t)

(autoload 'gv-define-expander "gv"
"Use HANDLER to handle NAME as a generalized var.
NAME is a symbol: the name of a function, macro, or special form.
HANDLER is a function which takes an argument DO followed by the same
arguments as NAME.  DO is a function as defined in `gv-get'.

(fn NAME HANDLER)" nil t)

(autoload 'gv--defun-declaration "gv"
"

(fn SYMBOL NAME ARGS HANDLER &optional FIX)")

(autoload 'gv-define-setter "gv"
"Define a setter method for generalized variable NAME.
This macro is an easy-to-use substitute for `gv-define-expander' that works
well for simple place forms.
Assignments of VAL to (NAME ARGS...) are expanded by binding the argument
forms (VAL ARGS...) according to ARGLIST, then executing BODY, which must
return a Lisp form that does the assignment.
The first arg in ARGLIST (the one that receives VAL) receives an expression
which can do arbitrary things, whereas the other arguments are all guaranteed
to be pure and copyable.  Example use:
  (gv-define-setter aref (v a i) \\=`(aset ,a ,i ,v))

(fn NAME ARGLIST &rest BODY)" nil t)

(autoload 'gv-define-simple-setter "gv"
"Define a simple setter method for generalized variable NAME.
This macro is an easy-to-use substitute for `gv-define-expander' that works
well for simple place forms.  Assignments of VAL to (NAME ARGS...) are
turned into calls of the form (SETTER ARGS... VAL).

If FIX-RETURN is non-nil, then SETTER is not assumed to return VAL and
instead the assignment is turned into something equivalent to
  (let ((temp VAL))
    (SETTER ARGS... temp)
    temp)
so as to preserve the semantics of `setf'.

(fn NAME SETTER &optional FIX-RETURN)" nil t)

(autoload 'setf "gv"
"Set each PLACE to the value of its VAL.
This is a generalized version of `setq'; the PLACEs may be symbolic
references such as (car x) or (aref x i), as well as plain symbols.
For example, (setf (cadr x) y) is equivalent to (setcar (cdr x) y).
The return value is the last VAL in the list.

(fn PLACE VAL PLACE VAL ...)" nil t)

(autoload 'incf "gv"
"Increment generalized variable PLACE by DELTA (default to 1).

The DELTA is first added to PLACE, and then stored in PLACE.
Return the incremented value of PLACE.

For more information about generalized variables, see Info node
`(elisp) Generalized Variables'.

See also `decf'.

(fn PLACE &optional DELTA)" nil t)

(autoload 'decf "gv"
"Decrement generalized variable PLACE by DELTA (default to 1).

The DELTA is first subtracted from PLACE, and then stored in PLACE.
Return the decremented value of PLACE.

For more information about generalized variables, see Info node
`(elisp) Generalized Variables'.

See also `incf'.

(fn PLACE &optional DELTA)" nil t)

(autoload 'gv-ref "gv"
"Return a reference to PLACE.
This is like the `&' operator of the C language.
Note: this only works reliably with lexical binding mode, except for very
simple PLACEs such as (symbol-function \\='foo) which will also work in dynamic
binding mode.

(fn PLACE)" nil t)

(autoload 'describe-function "help-fns"
"Display the full documentation of FUNCTION (a symbol).
When called from Lisp, FUNCTION may also be a function object.

See the `help-enable-symbol-autoload' variable for special
handling of autoloaded functions.

(fn FUNCTION)" t)

(autoload 'help-find-source "help-fns"
"Switch to a buffer visiting the source of what is being described in *Help*." t)

(autoload 'describe-command "help-fns"
"Display the full documentation of COMMAND (a symbol).
When called from Lisp, COMMAND may also be a function object.

(fn COMMAND)" t)

(autoload 'help-C-file-name "help-fns"
"Return the name of the C file where SUBR-OR-VAR is defined.
KIND should be `var' for a variable or `subr' for a subroutine.
If we can't find the file name, nil is returned.

(fn SUBR-OR-VAR KIND)")

(autoload 'find-lisp-object-file-name "help-fns"
"Guess the file that defined the Lisp object OBJECT, of type TYPE.
OBJECT should be a symbol associated with a function, variable, or face;
  alternatively, it can be a function definition.
If TYPE is `defvar', search for a variable definition.
If TYPE is `defface', search for a face definition.
If TYPE is not a symbol, search for a function definition.

The return value is the absolute name of a readable file where OBJECT is
defined.  If several such files exist, preference is given to a file
found via `load-path'.  The return value can also be `C-source', which
means that OBJECT is a function or variable defined in C, but
it's currently unknown where.  If no suitable file is found,
return nil.

If ALSO-C-SOURCE is non-nil, instead of returning `C-source',
this function will attempt to locate the definition of OBJECT in
the C sources, too.

(fn OBJECT TYPE &optional ALSO-C-SOURCE)")

(autoload 'describe-function-1 "help-fns"
"

(fn FUNCTION)")

(autoload 'variable-at-point "help-fns"
"Return the bound variable symbol found at or before point.
Return 0 if there is no such symbol.
If ANY-SYMBOL is non-nil, don't insist the symbol be bound.

(fn &optional ANY-SYMBOL)")

(autoload 'describe-variable "help-fns"
"Display the full documentation of VARIABLE (a symbol).
Returns the documentation as a string, also.
If VARIABLE has a buffer-local value in BUFFER or FRAME
(default to the current buffer and current frame),
it is displayed along with the global value.

(fn VARIABLE &optional BUFFER FRAME)" t)

(autoload 'describe-face "help-fns"
"Display the properties of face FACE on FRAME.
Interactively, FACE defaults to the faces of the character after point
and FRAME defaults to the selected frame.

If the optional argument FRAME is given, report on face FACE in that frame.
If FRAME is t, report on the defaults for face FACE (for new frames).
If FRAME is omitted or nil, use the selected frame.

(fn FACE &optional FRAME)" t)

(autoload 'describe-symbol "help-fns"
"Display the full documentation of SYMBOL.
Will show the info of SYMBOL as a function, variable, and/or face.
Optional arguments BUFFER and FRAME specify for which buffer and
frame to show the information about SYMBOL; they default to the
current buffer and the selected frame, respectively.

(fn SYMBOL &optional BUFFER FRAME)" t)

(autoload 'describe-syntax "help-fns"
"Describe the syntax specifications in the syntax table of BUFFER.
The descriptions are inserted in a help buffer, which is then displayed.
BUFFER defaults to the current buffer.

(fn &optional BUFFER)" t)

(autoload 'describe-categories "help-fns"
"Describe the category specifications in the current category table.
The descriptions are inserted in a buffer, which is then displayed.
If BUFFER is non-nil, then describe BUFFER's category table instead.
BUFFER should be a buffer or a buffer name.

(fn &optional BUFFER)" t)

(autoload 'describe-keymap "help-fns"
"Describe key bindings in KEYMAP.
When called interactively, prompt for a variable that has a
keymap value.

(fn KEYMAP)" t)

(autoload 'describe-mode "help-fns"
"Display documentation of current major mode and minor modes.
A brief summary of the minor modes comes first, followed by the
major mode description.  This is followed by detailed
descriptions of the minor modes, each on a separate page.

For this to work correctly for a minor mode, the mode's indicator
variable (listed in `minor-mode-alist') must also be a function
whose documentation describes the minor mode.

If called from Lisp with a non-nil BUFFER argument, display
documentation for the major and minor modes of that buffer.

When `describe-mode-outline' is non-nil, Outline minor mode
is enabled in the Help buffer.

(fn &optional BUFFER)" t)

(autoload 'describe-widget "help-fns"
"Display a buffer with information about a widget.
You can use this command to describe buttons (e.g., the links in a *Help*
buffer), editable fields of the customization buffers, etc.

Interactively, click on a widget to describe it, or hit RET to describe the
widget at point.

When called from Lisp, POS may be a buffer position or a mouse position list.

Calls each function of the list `describe-widget-functions' in turn, until
one of them returns non-nil.

(fn &optional POS)" t)

(autoload 'doc-file-to-man "help-fns"
"Produce an nroff buffer containing the doc-strings from the DOC file.

(fn FILE)" t)

(autoload 'doc-file-to-info "help-fns"
"Produce a texinfo buffer with sorted doc-strings from the DOC file.

(fn FILE)" t)

(autoload 'help-fns-function-name "help-fns"
"Return a short buttonized string representing FUNCTION.
The string is propertized with a button; clicking on that
provides further details about FUNCTION.
FUNCTION can be a function, a built-in, a keyboard macro,
or a compile function.
This function is intended to be used to display various
callable symbols in buffers in a way that allows the user
to find out more details about the symbols.

(fn FUNCTION)")

(autoload 'help--help-screen "help-macro"
"

(fn HELP-LINE HELP-TEXT HELPED-MAP BUFFER-NAME)")

(autoload 'help-mode--add-function-link "help-mode"
"

(fn STR FUN)")

(autoload 'help-mode "help-mode"
"Major mode for viewing help text and navigating references in it.
Also see the `help-enable-variable-value-editing' variable.

Commands:
\\{help-mode-map}

In addition to any hooks its parent mode `special-mode' might have
run, this mode runs the hook `help-mode-hook', as the final or
penultimate step during initialization." t)

(autoload 'help-mode-setup "help-mode"
"Enter Help mode in the current buffer.")

(autoload 'help-mode-finish "help-mode"
"Finalize Help mode setup in current buffer.")

(autoload 'help-setup-xref "help-mode"
"Invoked from commands using the \"*Help*\" buffer to install some xref info.

ITEM is a (FUNCTION . ARGS) pair appropriate for recreating the help
buffer after following a reference.  INTERACTIVE-P is non-nil if the
calling command was invoked interactively.  In this case the stack of
items for help buffer \"back\" buttons is cleared.

This function also re-enables the major mode of the buffer, thus
resetting local variables to the values set by the mode and running the
mode hooks.

So this should be called very early, before the output buffer is
cleared, also because we want to record the \"previous\" position of
point so we can restore it properly when going back.

(fn ITEM INTERACTIVE-P)")

(autoload 'help-buffer "help-mode"
"Return the name of a buffer for inserting help.
If `help-xref-following' is non-nil and the current buffer is
derived from `help-mode', this is the name of the current buffer.

Otherwise, return \"*Help*\", creating a buffer with that name if
it does not already exist.")

(autoload 'help-make-xrefs "help-mode"
"Parse and hyperlink documentation cross-references in the given BUFFER.

Find cross-reference information in a buffer and activate such cross
references for selection with `help-follow-symbol'.  Cross-references have
the canonical form `...'  and the type of reference may be
disambiguated by the preceding word(s) used in
`help-xref-symbol-regexp'.  Faces only get cross-referenced if
preceded or followed by the word `face'.  Variables without
variable documentation do not get cross-referenced, unless
preceded by the word `variable' or `option'.

If the variable `help-xref-mule-regexp' is non-nil, find also
cross-reference information related to multilingual environment
(e.g., coding-systems).  This variable is also used to disambiguate
the type of reference as the same way as `help-xref-symbol-regexp'.

A special reference `back' is made to return back through a stack of
help buffers.  Variable `help-back-label' specifies the text for
that.

(fn &optional BUFFER)" t)

(autoload 'help-xref-button "help-mode"
"Make a hyperlink for cross-reference text previously matched.
MATCH-NUMBER is the subexpression of interest in the last matched
regexp.  TYPE is the type of button to use.  Any remaining arguments are
passed to the button's help-function when it is invoked.
See `help-make-xrefs'.

This function removes quotes surrounding the match if the
variable `help-clean-buttons' is non-nil.

(fn MATCH-NUMBER TYPE &rest ARGS)")

(autoload 'help-insert-xref-button "help-mode"
"Insert STRING and make a hyperlink from cross-reference text on it.
TYPE is the type of button to use.  Any remaining arguments are passed
to the button's help-function when it is invoked.
See `help-make-xrefs'.

(fn STRING TYPE &rest ARGS)")

(autoload 'help-xref-on-pp "help-mode"
"Add xrefs for symbols in `pp's output between FROM and TO.

(fn FROM TO)")

(autoload 'help-bookmark-jump "help-mode"
"Jump to `help-mode' bookmark BOOKMARK.
Handler function for record returned by `help-bookmark-make-record'.
BOOKMARK is a bookmark name or a bookmark record.

(fn BOOKMARK)")

(autoload 'describe-icon "icons"
"Pop to a buffer to describe ICON.

(fn ICON)" t)

(autoload 'define-inline "inline"
"Define an inline function NAME with arguments ARGS and body in BODY.
This is halfway between `defmacro' and `defun'.  BODY is used as a blueprint
both for the body of the function and for the body of the compiler-macro
used to generate the code inlined at each call site.
See Info node `(elisp)Inline Functions' for more details.

A (noinline t) in the `declare' form prevents the definition of the
compiler macro.  This is for the rare case in which you want to use this
macro to define a function that should not be inlined.

(fn NAME ARGS &rest BODY)" nil t)

(autoload 'let-alist "let-alist"
"Let-bind dotted symbols to their cdrs in ALIST and execute BODY.
Dotted symbol is any symbol starting with a `.'.  This macro creates
let-bindings for dotted symbols that appear literally in BODY (whether
or not they are actually used).  It does not create bindings for dotted
symbols that are introduced by macro-expansion in BODY.

A symbol of the form `.foo.N' where N is a natural number refers to the
Nth element of the value that ALIST associates to key `foo'.

For instance, the following code

  (let-alist alist
    (if (and .title.0 .body)
        .body
      .site
      .site.contents))

essentially expands to

  (let ((.title.0 (elt (cdr (assq \\='title alist)) 0))
        (.body  (cdr (assq \\='body alist)))
        (.site  (cdr (assq \\='site alist)))
        (.site.contents (cdr (assq \\='contents (cdr (assq \\='site alist))))))
    (if (and .title.0 .body)
        .body
      .site
      .site.contents))

If you nest `let-alist' invocations, the inner one can't access
the variables of the outer one.  You can, however, access alists
inside the original alist by using dots inside the symbol, as
displayed in the example above.

To refer to a non-`let-alist' variable starting with a dot in BODY, use
two dots instead of one.  For example, in the following form `..foo'
refers to the variable `.foo' bound outside of the `let-alist':

    (let ((.foo 42)) (let-alist \\='((foo . nil)) ..foo))

Note that there is no way to differentiate the case where a key
is missing from when it is present, but its value is nil.  Thus,
the following form evaluates to nil:

    (let-alist \\='((some-key . nil))
      .some-key)

(fn ALIST &rest BODY)" nil t)

(autoload 'store-substring "mule-util"
"Embed OBJ (string or character) at index IDX of STRING.

(fn STRING IDX OBJ)")

(autoload 'truncate-string-to-width "mule-util"
"Truncate string STR to end at column END-COLUMN.
The optional 3rd arg START-COLUMN, if non-nil, specifies the starting
column (default: zero); that means to return the characters occupying
columns START-COLUMN ... END-COLUMN of STR.  Both END-COLUMN and
START-COLUMN are specified in terms of character display width in the
current buffer; see `char-width'.

Since character composition on display can produce glyphs whose
width is smaller than the sum of `char-width' values of the
composed characters, this function can produce inaccurate results
when used in such cases.

The optional 4th arg PADDING, if non-nil, specifies a padding
character (which should have a display width of 1) to add at the end
of the result if STR doesn't reach column END-COLUMN, or if END-COLUMN
comes in the middle of a character in STR.  PADDING is also added at
the beginning of the result if column START-COLUMN appears in the
middle of a character in STR.

If PADDING is nil, no padding is added in these cases, so
the resulting string may be narrower than END-COLUMN.

If ELLIPSIS is non-nil, it should be a string which will replace the
end of STR (including any padding) if it extends beyond END-COLUMN,
unless the display width of STR is equal to or less than the display
width of ELLIPSIS.  If it is non-nil and not a string, then ELLIPSIS
defaults to `truncate-string-ellipsis', or to three dots when it's nil.

If ELLIPSIS-TEXT-PROPERTY is non-nil, a too-long string will not
be truncated, but instead the elided parts will be covered by a
`display' text property showing the ellipsis.

(fn STR END-COLUMN &optional START-COLUMN PADDING ELLIPSIS ELLIPSIS-TEXT-PROPERTY)")

(autoload 'set-nested-alist "mule-util"
"Set ENTRY for KEYSEQ in a nested alist ALIST.
Optional 4th arg LEN non-nil means the first LEN elements in KEYSEQ
 are considered.
Optional 5th argument BRANCHES if non-nil is branches for a keyseq
longer than KEYSEQ.
See the documentation of `nested-alist-p' for more detail.

(fn KEYSEQ ENTRY ALIST &optional LEN BRANCHES)")

(autoload 'lookup-nested-alist "mule-util"
"Look up key sequence KEYSEQ in nested alist ALIST.  Return the definition.
Optional 3rd argument LEN specifies the length of KEYSEQ.
Optional 4th argument START specifies index of the starting key.
The returned value is normally a nested alist of which
car part is the entry for KEYSEQ.
If ALIST is not deep enough for KEYSEQ, return number which is
 how many key elements at the front of KEYSEQ it takes
 to reach a leaf in ALIST.
Optional 5th argument NIL-FOR-TOO-LONG non-nil means return nil
 even if ALIST is not deep enough.

(fn KEYSEQ ALIST &optional LEN START NIL-FOR-TOO-LONG)")

(autoload 'coding-system-post-read-conversion "mule-util"
"Return the value of CODING-SYSTEM's `post-read-conversion' property.

(fn CODING-SYSTEM)")

(autoload 'coding-system-pre-write-conversion "mule-util"
"Return the value of CODING-SYSTEM's `pre-write-conversion' property.

(fn CODING-SYSTEM)")

(autoload 'coding-system-translation-table-for-decode "mule-util"
"Return the value of CODING-SYSTEM's `decode-translation-table' property.

(fn CODING-SYSTEM)")

(autoload 'coding-system-translation-table-for-encode "mule-util"
"Return the value of CODING-SYSTEM's `encode-translation-table' property.

(fn CODING-SYSTEM)")

(autoload 'with-coding-priority "mule-util"
"Execute BODY like `progn' with CODING-SYSTEMS at the front of priority list.
CODING-SYSTEMS is a list of coding systems.  See `set-coding-system-priority'.
This affects the implicit sorting of lists of coding systems returned by
operations such as `find-coding-systems-region'.

(fn CODING-SYSTEMS &rest BODY)" nil t)

(autoload 'detect-coding-with-language-environment "mule-util"
"Detect a coding system for the text between FROM and TO with LANG-ENV.
The detection takes into account the coding system priorities for the
language environment LANG-ENV.

(fn FROM TO LANG-ENV)")

(autoload 'filepos-to-bufferpos "mule-util"
"Try to return the buffer position corresponding to a particular file position.
The file position is given as a (0-based) BYTE count.
The function presumes the file is encoded with CODING-SYSTEM, which defaults
to `buffer-file-coding-system'.
QUALITY can be:
  `approximate', in which case we may cut some corners to avoid
    excessive work.
  `exact', in which case we may end up re-(en/de)coding a large
    part of the file/buffer, this can be expensive and slow.  (It
    is an error to request the `exact' method when the buffer's
    EOL format is not yet decided.)
  nil, in which case we may return nil rather than an approximation.

(fn BYTE &optional QUALITY CODING-SYSTEM)")

(autoload 'bufferpos-to-filepos "mule-util"
"Try to return the file byte corresponding to a particular buffer POSITION.
Value is the file position given as a (0-based) byte count.
The function presumes the file is encoded with CODING-SYSTEM, which defaults
to `buffer-file-coding-system'.
QUALITY can be:
  `approximate', in which case we may cut some corners to avoid
    excessive work.
  `exact', in which case we may end up re-(en/de)coding a large
    part of the file/buffer, this can be expensive and slow.  (It
    is an error to request the `exact' method when the buffer's
    EOL format is not yet decided.)
  nil, in which case we may return nil rather than an approximation.

(fn POSITION &optional QUALITY CODING-SYSTEM)")

(autoload 'pcase "pcase"
"Evaluate EXP to get EXPVAL; try passing control to one of CASES.
CASES is a list of elements of the form (PATTERN CODE...).
For the first CASE whose PATTERN \"matches\" EXPVAL,
evaluate its CODE..., and return the value of the last form.
If no CASE has a PATTERN that matches, return nil.

Each PATTERN expands, in essence, to a predicate to call
on EXPVAL.  When the return value of that call is non-nil,
PATTERN matches.  PATTERN can take one of the forms:

  _                matches anything.
  \\='VAL             matches if EXPVAL is `equal' to VAL.
  KEYWORD          shorthand for \\='KEYWORD
  INTEGER          shorthand for \\='INTEGER
  STRING           shorthand for \\='STRING
  SYMBOL           matches anything and binds it to SYMBOL.
                   If a SYMBOL is used twice in the same pattern
                   the second occurrence becomes an `eq'uality test.
  (pred FUN)       matches if FUN called on EXPVAL returns non-nil.
  (pred (not FUN)) matches if FUN called on EXPVAL returns nil.
  (app FUN PAT)    matches if FUN called on EXPVAL matches PAT.
  (guard BOOLEXP)  matches if BOOLEXP evaluates to non-nil.
  (and PAT...)     matches if all the patterns match.
  (or PAT...)      matches if any of the patterns matches.

FUN in `pred' and `app' can take one of the forms:
  SYMBOL  or  (lambda ARGS BODY)
     call it with one argument
  (F ARG1 .. ARGn)
     call F with ARG1..ARGn and EXPVAL as n+1'th argument
  (F ARG1 .. _ .. ARGn)
     call F, passing EXPVAL at the _ position.

FUN, BOOLEXP, and subsequent PAT can refer to variables
bound earlier in the pattern by a SYMBOL pattern.

Additional patterns can be defined using `pcase-defmacro'.

See Info node `(elisp) Pattern-Matching Conditional' in the
Emacs Lisp manual for more information and examples.

(fn EXP &rest CASES)" nil t)

(autoload 'pcase--make-docstring "pcase")

(autoload 'pcase-exhaustive "pcase"
"The exhaustive version of `pcase' (which see).
If EXP fails to match any of the patterns in CASES, signal an error.

In contrast, `pcase' will return nil if there is no match, but will
not signal an error.

(fn EXP &rest CASES)" nil t)

(autoload 'pcase-lambda "pcase"
"Like `lambda' but allow each argument to be a pattern.
I.e. accepts the usual &optional and &rest keywords, but every formal
argument can be any pattern destructed by `pcase-let' (a mere variable
name being but a special case of it).

Each argument should match its respective pattern in the parameter
list (i.e. be of a compatible structure); a mismatch may signal an error
or may go undetected, binding arguments to arbitrary values, such as
nil.

(fn LAMBDA-LIST &rest BODY)" nil t)

(autoload 'pcase-let* "pcase"
"Like `let*', but supports destructuring BINDINGS using `pcase' patterns.
As with `pcase-let', BINDINGS are of the form (PATTERN EXP), but the
EXP in each binding in BINDINGS can use the results of the destructuring
bindings that precede it in BINDINGS' order.

Each EXP should match its respective PATTERN (i.e. be of structure
compatible to PATTERN); a mismatch may signal an error or may go
undetected, binding variables to arbitrary values, such as nil.

(fn BINDINGS &rest BODY)" nil t)

(autoload 'pcase-let*-strict "pcase"
"Like `pcase-let*', but always signal an error when a pattern does not match.
As with `pcase-let*', BINDINGS are of the form (PATTERN EXP), and the
EXP in each binding in BINDINGS can use the results of the destructuring
bindings that precede it in BINDINGS' order.

(fn BINDINGS &rest BODY)" nil t)

(autoload 'pcase-let "pcase"
"Like `let', but supports destructuring BINDINGS using `pcase' patterns.
BODY should be a list of expressions, and BINDINGS should be a list of
bindings of the form (PATTERN EXP).
All EXPs are evaluated first, and then used to perform destructuring
bindings by matching each EXP against its respective PATTERN.  Then
BODY is evaluated with those bindings in effect.

Each EXP should match its respective PATTERN (i.e. be of structure
compatible to PATTERN); a mismatch may signal an error or may go
undetected, binding variables to arbitrary values, such as nil.

(fn BINDINGS &rest BODY)" nil t)

(autoload 'pcase-dolist "pcase"
"Eval BODY once for each set of bindings defined by PATTERN and LIST elements.
PATTERN should be a `pcase' pattern describing the structure of
LIST elements, and LIST is a list of objects that match PATTERN,
i.e. have a structure that is compatible with PATTERN.
For each element of LIST, this macro binds the variables in
PATTERN to the corresponding subfields of the LIST element, and
then evaluates BODY with these bindings in effect.  The
destructuring bindings of variables in PATTERN to the subfields
of the elements of LIST is performed as if by `pcase-let'.

(fn (PATTERN LIST) BODY...)" nil t)

(autoload 'pcase-setq "pcase"
"Assign values to variables by destructuring with `pcase'.
Each PATTERN is a normal `pcase' pattern, and each VALUE an expression.

Evaluation happens sequentially as in `setq' (not in parallel).

An example: (pcase-setq \\=`((,a) [(,b)]) \\='((1) [(2)]))

Each VALUE is presumed to match its PATTERN.  Failure to match may
signal an error or go undetected, binding variables to arbitrary values,
such as nil.

(fn PATTERN VALUE PATTERN VALUE ...)" nil t)

(autoload 'pcase-defmacro "pcase"
"Define a new kind of pcase PATTERN, by macro expansion.
Patterns of the form (NAME ...) will be expanded according
to this macro.

By convention, DOC should use \"EXPVAL\" to stand
for the result of evaluating EXP (first arg to `pcase').

(fn NAME ARGS [DOC] &rest BODY...)" nil t)

(autoload 'pp-to-string "pp"
"Return a string containing the pretty-printed representation of OBJECT.
OBJECT can be any Lisp object.  Quoting characters are used as needed
to make output that `read' can handle, whenever this is possible.
Optional argument PP-FUNCTION overrides `pp-default-function'.

(fn OBJECT &optional PP-FUNCTION)")

(autoload 'pp-buffer "pp"
"Prettify the current buffer with printed representation of a Lisp object." t)

(autoload 'pp "pp"
"Output the pretty-printed representation of OBJECT, any Lisp object.
Quoting characters are printed as needed to make output that `read'
can handle, whenever this is possible.

Uses the pretty-printing code specified in `pp-default-function'.

Output stream is STREAM, or value of `standard-output' (which see).

(fn OBJECT &optional STREAM)")

(autoload 'pp-display-expression "pp"
"Prettify and display EXPRESSION in an appropriate way, depending on length.
If LISP, format with `pp-emacs-lisp-code'; use `pp' otherwise.

If a temporary buffer is needed for representation, it will be named
after OUT-BUFFER-NAME.

(fn EXPRESSION OUT-BUFFER-NAME &optional LISP)")

(autoload 'pp-eval-expression "pp"
"Evaluate EXPRESSION and pretty-print its value.
Also add the value to the front of the list in the variable `values'.
When called interactively, read an Emacs Lisp expression.
With a prefix argument (when called from Lisp, with optional argument
INSERT-VALUE non-nil), insert the value into the current buffer instead
of displaying it in the echo area or a temporary buffer.

(fn EXPRESSION &optional INSERT-VALUE)" t)

(autoload 'pp-macroexpand-expression "pp"
"Macroexpand EXPRESSION and pretty-print its value.

(fn EXPRESSION)" t)

(autoload 'pp-eval-last-sexp "pp"
"Run `pp-eval-expression' on sexp before point.
With ARG, pretty-print output into current buffer.
Ignores leading comment characters.

(fn ARG)" t)

(autoload 'pp-macroexpand-last-sexp "pp"
"Run `pp-macroexpand-expression' on sexp before point.
With ARG, pretty-print output into current buffer.
Ignores leading comment characters.

(fn ARG)" t)

(autoload 'pp-emacs-lisp-code "pp"
"Insert SEXP into the current buffer, formatted as Emacs Lisp code.
Use the `pp-max-width' variable to control the desired line length.
Note that this could be slow for large SEXPs.
Can also be called with two arguments, in which case they're taken to be
the bounds of a region containing Lisp code to pretty-print.

(fn SEXP &optional END)")

(autoload 'regexp-opt "regexp-opt"
"Return a regexp to match a string in the list STRINGS.
Each member of STRINGS is treated as a fixed string, not as a regexp.
Optional PAREN specifies how the returned regexp is surrounded by
grouping constructs.

If STRINGS is the empty list, the return value is a regexp that
never matches anything.

The optional argument PAREN can be any of the following:

a string
    the resulting regexp is preceded by PAREN and followed by
    \\), e.g.  use \"\\\\(?1:\" to produce an explicitly numbered
    group.

`words'
    the resulting regexp is surrounded by \\=\\<\\( and \\)\\>.

`symbols'
    the resulting regexp is surrounded by \\_<\\( and \\)\\_>.

non-nil
    the resulting regexp is surrounded by \\( and \\).

nil
    the resulting regexp is surrounded by \\(?: and \\), if it is
    necessary to ensure that a postfix operator appended to it will
    apply to the whole expression.

The returned regexp is ordered in such a way that it will always
match the longest string possible.

Up to reordering, the resulting regexp is equivalent to but
usually more efficient than that of a simplified version:

 (defun simplified-regexp-opt (strings &optional paren)
   (let ((parens
          (cond ((stringp paren)       (cons paren \"\\\\)\"))
                ((eq paren \\='words)    \\='(\"\\\\\\=<\\\\(\" . \"\\\\)\\\\>\"))
                ((eq paren \\='symbols) \\='(\"\\\\_<\\\\(\" . \"\\\\)\\\\_>\"))
                ((null paren)          \\='(\"\\\\(?:\" . \"\\\\)\"))
                (t                       \\='(\"\\\\(\" . \"\\\\)\")))))
     (concat (car parens)
             (mapconcat \\='regexp-quote strings \"\\\\|\")
             (cdr parens))))

(fn STRINGS &optional PAREN)")

(autoload 'regexp-opt-depth "regexp-opt"
"Return the depth of REGEXP.
This means the number of non-shy regexp grouping constructs
(parenthesized expressions) in REGEXP.

(fn REGEXP)")

(autoload 'ring-p "ring"
"Return t if X is a ring; nil otherwise.

(fn X)")

(autoload 'make-ring "ring"
"Make a ring that can contain SIZE elements.

(fn SIZE)")

(autoload 'rx-to-string "rx"
"Translate FORM from `rx' sexp syntax into a string regexp.
The arguments to `literal' and `regexp' forms inside FORM must be
constant strings.
If NO-GROUP is non-nil, don't bracket the result in a non-capturing
group.

For extending the `rx' notation in FORM, use `rx-define' or `rx-let-eval'.

(fn FORM &optional NO-GROUP)")

(autoload 'rx "rx"
"Translate regular expressions REGEXPS in sexp form to a regexp string.
Each argument is one of the forms below; RX is a subform, and RX... stands
for zero or more RXs.  For details, see Info node `(elisp) Rx Notation'.
See `rx-to-string' for the corresponding function.

STRING         Match a literal string.
CHAR           Match a literal character.

(seq RX...)    Match the RXs in sequence.  Alias: :, sequence, and.
(or RX...)     Match one of the RXs.  Alias: |.

(zero-or-more RX...) Match RXs zero or more times.  Alias: 0+.
(one-or-more RX...)  Match RXs one or more times.  Alias: 1+.
(zero-or-one RX...)  Match RXs or the empty string.  Alias: opt, optional.
(* RX...)       Match RXs zero or more times; greedy.
(+ RX...)       Match RXs one or more times; greedy.
(? RX...)       Match RXs or the empty string; greedy.
(*? RX...)      Match RXs zero or more times; non-greedy.
(+? RX...)      Match RXs one or more times; non-greedy.
(?? RX...)      Match RXs or the empty string; non-greedy.
(= N RX...)     Match RXs exactly N times.
(>= N RX...)    Match RXs N or more times.
(** N M RX...)  Match RXs N to M times.  Alias: repeat.
(minimal-match RX)  Match RX, with zero-or-more, one-or-more, zero-or-one
                and aliases using non-greedy matching.
(maximal-match RX)  Match RX, with zero-or-more, one-or-more, zero-or-one
                and aliases using greedy matching, which is the default.

(any SET...)    Match a character from one of the SETs.  Each SET is a
                character, a string, a range as string \"A-Z\" or cons
                (?A . ?Z), or a character class (see below).  Alias: in, char.
(not CHARSPEC)  Match one character not matched by CHARSPEC.  CHARSPEC
                can be a character, single-char string, (any ...), (or ...),
                (intersection ...), (syntax ...), (category ...),
                or a character class.
(intersection CHARSET...) Match all CHARSETs.
                CHARSET is (any...), (not...), (or...) or (intersection...),
                a character or a single-char string.
not-newline     Match any character except a newline.  Alias: nonl.
anychar         Match any character.  Alias: anything.
unmatchable     Never match anything at all.

CHARCLASS       Match a character from a character class.  One of:
 alpha, alphabetic, letter   Alphabetic characters (defined by Unicode).
 alnum, alphanumeric         Alphabetic or decimal digit chars (Unicode).
 digit, numeric, num         0-9.
 xdigit, hex-digit, hex      0-9, A-F, a-f.
 cntrl, control              ASCII codes 0-31.
 blank                       Horizontal whitespace (Unicode).
 space, whitespace, white    Chars with whitespace syntax.
 lower, lower-case           Lower-case chars, from current case table.
 upper, upper-case           Upper-case chars, from current case table.
 graph, graphic              Graphic characters (Unicode).
 print, printing             Whitespace or graphic (Unicode).
 punct, punctuation          Not control, space, letter or digit (ASCII);
                              not word syntax (non-ASCII).
 word, wordchar              Characters with word syntax.
 ascii                       ASCII characters (codes 0-127).
 nonascii                    Non-ASCII characters (but not raw bytes).

(syntax SYNTAX)  Match a character with syntax SYNTAX, being one of:
  whitespace, punctuation, word, symbol, open-parenthesis,
  close-parenthesis, expression-prefix, string-quote,
  paired-delimiter, escape, character-quote, comment-start,
  comment-end, string-delimiter, comment-delimiter

(category CAT)   Match a character in category CAT, being one of:
  space-for-indent, base, consonant, base-vowel,
  upper-diacritical-mark, lower-diacritical-mark, tone-mark, symbol,
  digit, vowel-modifying-diacritical-mark, vowel-sign,
  semivowel-lower, not-at-end-of-line, not-at-beginning-of-line,
  alpha-numeric-two-byte, chinese-two-byte, greek-two-byte,
  japanese-hiragana-two-byte, indian-two-byte,
  japanese-katakana-two-byte, strong-left-to-right,
  korean-hangul-two-byte, strong-right-to-left, cyrillic-two-byte,
  combining-diacritic, ascii, arabic, chinese, ethiopic, greek,
  korean, indian, japanese, japanese-katakana, latin, lao,
  tibetan, japanese-roman, thai, vietnamese, hebrew, cyrillic,
  can-break

Zero-width assertions: these all match the empty string in specific places.
 line-start         At the beginning of a line.  Alias: bol.
 line-end           At the end of a line.  Alias: eol.
 string-start       At the start of the string or buffer.
                     Alias: buffer-start, bos, bot.
 string-end         At the end of the string or buffer.
                     Alias: buffer-end, eos, eot.
 point              At point.
 word-start         At the beginning of a word.  Alias: bow.
 word-end           At the end of a word.  Alias: eow.
 word-boundary      At the beginning or end of a word.
 not-word-boundary  Not at the beginning or end of a word.
 symbol-start       At the beginning of a symbol.
 symbol-end         At the end of a symbol.

(group RX...)  Match RXs and define a capture group.  Alias: submatch.
(group-n N RX...) Match RXs and define capture group N.  Alias: submatch-n.
(backref N)    Match the text that capture group N matched.

(literal EXPR) Match the literal string from evaluating EXPR at run time.
(regexp EXPR)  Match the string regexp from evaluating EXPR at run time.
(eval EXPR)    Match the rx sexp from evaluating EXPR at macro-expansion
                (compile) time.

Additional constructs can be defined using `rx-define' and `rx-let',
which see.

(fn REGEXPS...)" nil t)

(autoload 'rx-let-eval "rx"
"Evaluate BODY with local BINDINGS for `rx-to-string'.
BINDINGS, after evaluation, is a list of definitions each on the form
(NAME [(ARGS...)] RX), in effect for calls to `rx-to-string'
in BODY.

For bindings without an ARGS list, NAME is defined as an alias
for the `rx' expression RX.  Where ARGS is supplied, NAME is
defined as an `rx' form with ARGS as argument list.  The
parameters are bound from the values in the (NAME ...) form and
are substituted in RX.  ARGS can contain `&rest' parameters,
whose values are spliced into RX where the parameter name occurs.

Any previous definitions with the same names are shadowed during
the expansion of BODY only.
For extensions when using the `rx' macro, use `rx-let'.
To make global rx extensions, use `rx-define'.
For more details, see Info node `(elisp) Extending Rx'.

(fn BINDINGS BODY...)" nil t)

(autoload 'rx-let "rx"
"Evaluate BODY with local BINDINGS for `rx'.
BINDINGS is an unevaluated list of bindings each on the form
(NAME [(ARGS...)] RX).
They are bound lexically and are available in `rx' expressions in
BODY only.

For bindings without an ARGS list, NAME is defined as an alias
for the `rx' expression RX.  Where ARGS is supplied, NAME is
defined as an `rx' form with ARGS as argument list.  The
parameters are bound from the values in the (NAME ...) form and
are substituted in RX.  ARGS can contain `&rest' parameters,
whose values are spliced into RX where the parameter name occurs.

Any previous definitions with the same names are shadowed during
the expansion of BODY only.
For local extensions to `rx-to-string', use `rx-let-eval'.
To make global rx extensions, use `rx-define'.
For more details, see Info node `(elisp) Extending Rx'.

(fn BINDINGS BODY...)" nil t)

(autoload 'rx-define "rx"
"Define NAME as a global `rx' definition.
If the ARGS list is omitted, define NAME as an alias for the `rx'
expression RX.

If the ARGS list is supplied, define NAME as an `rx' form with
ARGS as argument list.  The parameters are bound from the values
in the (NAME ...) form and are substituted in RX.
ARGS can contain `&rest' parameters, whose values are spliced
into RX where the parameter name occurs.

Any previous global definition of NAME is overwritten with the new one.
To make local rx extensions, use `rx-let' for `rx',
`rx-let-eval' for `rx-to-string'.
For more details, see Info node `(elisp) Extending Rx'.

(fn NAME [(ARGS...)] RX)" nil t)

(autoload 'rx--pcase-macroexpander "rx"
"A pattern that matches strings against `rx' REGEXPS in sexp form.
REGEXPS are interpreted as in `rx'.  The pattern matches any
string that is a match for REGEXPS, as if by `string-match'.

In addition to the usual `rx' syntax, REGEXPS can contain the
following constructs:

  (let REF RX...)  binds the symbol REF to a submatch that matches
                   the regular expressions RX.  REF is bound in
                   CODE to the string of the submatch or nil, but
                   can also be used in `backref'.
  (backref REF)    matches whatever the submatch REF matched.
                   REF can be a number, as usual, or a name
                   introduced by a previous (let REF ...)
                   construct.

(fn &rest REGEXPS)")

(autoload 'define-short-documentation-group "shortdoc"
"Add GROUP to the list of defined documentation groups.
FUNCTIONS is a list of elements on the form:

  (FUNC
   :no-manual BOOL
   :args ARGS
   :eval EVAL
   :no-eval EXAMPLE-FORM
   :no-value EXAMPLE-FORM
   :no-eval* EXAMPLE-FORM
   :result RESULT-FORM
   :result-string RESULT-STRING
   :eg-result RESULT-FORM
   :eg-result-string RESULT-STRING)

FUNC is the function being documented.

NO-MANUAL should be non-nil if FUNC isn't documented in the
manual.

ARGS is optional list of function FUNC's arguments.  FUNC's
signature is displayed automatically if ARGS is not present.
Specifying ARGS might be useful where you don't want to document
some of the uncommon arguments a function might have.

While the `:no-manual' and `:args' property can be used for
any (FUNC ..) form, all of the other properties shown above
cannot be used simultaneously in such a form.

Here are some common forms with examples of properties that go
together:

1. Document a form or string, and its evaluated return value.
   (FUNC
    :eval EVAL)

If EVAL is a string, it will be inserted as is, and then that
string will be `read' and evaluated.

2. Document a form or string, but manually document its evaluation
   result.  The provided form will not be evaluated.

  (FUNC
   :no-eval EXAMPLE-FORM
   :result RESULT-FORM)   ;Use `:result-string' if value is in string form

Using `:no-value' is the same as using `:no-eval'.

Use `:no-eval*' instead of `:no-eval' where the successful
execution of the documented form depends on some conditions.

3. Document a form or string EXAMPLE-FORM.  Also manually
   document an example result.  This result could be unrelated to
   the documented form.

  (FUNC
   :no-eval EXAMPLE-FORM
   :eg-result RESULT-FORM) ;Use `:eg-result-string' if value is in string form

A FUNC form can have any number of `:no-eval' (or `:no-value'),
`:no-eval*', `:result', `:result-string', `:eg-result' and
`:eg-result-string' properties.

(fn GROUP &rest FUNCTIONS)" nil t)

(autoload 'shortdoc-display-group "shortdoc"
"Pop to a buffer with short documentation summary for functions in GROUP.
Interactively, prompt for GROUP.
If FUNCTION is non-nil, place point on the entry for FUNCTION (if any).
If SAME-WINDOW, don't pop to a new window.

(fn GROUP &optional FUNCTION SAME-WINDOW)" t)

(autoload 'string-truncate-left "subr-x"
"If STRING is longer than LENGTH, return a truncated version.
When truncating, \"...\" is always prepended to the string, so
the resulting string may be longer than the original if LENGTH is
3 or smaller.

(fn STRING LENGTH)")

(autoload 'string-clean-whitespace "subr-x"
"Clean up whitespace in STRING.
All sequences of whitespaces in STRING are collapsed into a
single space character, and leading/trailing whitespace is
removed.

(fn STRING)")

(autoload 'string-fill "subr-x"
"Try to word-wrap STRING so that it displays with lines no wider than WIDTH.
STRING is wrapped where there is whitespace in it.  If there are
individual words in STRING that are wider than WIDTH, the result
will have lines that are wider than WIDTH.

(fn STRING WIDTH)")

(autoload 'string-limit "subr-x"
"Return a substring of STRING that is (up to) LENGTH characters long.
If STRING is shorter than or equal to LENGTH characters, return the
entire string unchanged.

If STRING is longer than LENGTH characters, return a substring
consisting of the first LENGTH characters of STRING.  If END is
non-nil, return the last LENGTH characters instead.

If CODING-SYSTEM is non-nil, STRING will be encoded before
limiting, and LENGTH is interpreted as the number of bytes to
limit the string to.  The result will be a unibyte string that is
shorter than LENGTH, but will not contain \"partial\"
characters (or glyphs), even if CODING-SYSTEM encodes characters
with several bytes per character.  If the coding system specifies
prefix like the byte order mark (aka \"BOM\") or a shift-in sequence,
their bytes will be normally counted as part of LENGTH.  This is
the case, for instance, with `utf-16'.  If this isn't desired, use a
coding system that doesn't specify a BOM, like `utf-16le' or `utf-16be'.

When shortening strings for display purposes,
`truncate-string-to-width' is almost always a better alternative
than this function.

(fn STRING LENGTH &optional END CODING-SYSTEM)")

(autoload 'string-pad "subr-x"
"Pad STRING to LENGTH using PADDING.
If PADDING is nil, the space character is used.  If not nil, it
should be a character.

If STRING is longer than the absolute value of LENGTH, no padding
is done.

If START is nil (or not present), the padding is done to the end
of the string, and if non-nil, padding is done to the start of
the string.

(fn STRING LENGTH &optional PADDING START)")

(autoload 'string-chop-newline "subr-x"
"Remove the final newline (if any) from STRING.

(fn STRING)")

(autoload 'named-let "subr-x"
"Looping construct taken from Scheme.
Like `let', bind variables in BINDINGS and then evaluate BODY,
but with the twist that BODY can evaluate itself recursively by
calling NAME, where the arguments passed to NAME are used
as the new values of the bound variables in the recursive invocation.

This construct can only be used with lexical binding.

(fn NAME BINDINGS &rest BODY)" nil t)

(autoload 'work-buffer--release "subr-x"
"Release work BUFFER.

(fn BUFFER)")

(autoload 'with-work-buffer "subr-x"
"Create a work buffer, and evaluate BODY there like `progn'.
Like `with-temp-buffer', but reuse an already created temporary
buffer when possible, instead of creating a new one on each call.
Avoid retaining state referring to a work buffer, and kill
any indirect buffers you create that use a work buffer as a base.

(fn &rest BODY)" nil t)

(autoload 'string-pixel-width "subr-x"
"Return the width of STRING in pixels.
If BUFFER is non-nil, use the face remappings, alternative and default
properties from that buffer when determining the width.
If you call this function to measure pixel width of a string
with embedded newlines, it returns the width of the widest
substring that does not include newlines.

(fn STRING &optional BUFFER)")

(autoload 'truncate-string-pixelwise "subr-x"
"Return STRING truncated to fit within MAX-PIXELS.
If BUFFER is non-nil, use the face remappings, alternative and default
properties from that buffer when determining the width.
If you call this function to measure pixel width of a string
with embedded newlines, it returns the width of the widest
substring that does not include newlines.

If ELLIPSIS is non-nil, it should be a string which will replace the end
of STRING if it extends beyond MAX-PIXELS, unless the pixel width of
STRING is equal to or less than the pixel width of ELLIPSIS.  If it is
non-nil and not a string, then ELLIPSIS defaults to
`truncate-string-ellipsis', or to three dots when it's nil.

If ELLIPSIS-PIXELS is non-nil, it is the pixel width of ELLIPSIS, and
can be used to avoid the cost of recomputing this for multiple calls to
this function using the same ELLIPSIS.

(fn STRING MAX-PIXELS &optional BUFFER ELLIPSIS ELLIPSIS-PIXELS)")

(autoload 'string-glyph-split "subr-x"
"Split STRING into a list of strings representing separate glyphs.
This takes into account combining characters and grapheme clusters:
if compositions are enabled, each sequence of characters composed
on display into a single grapheme cluster is treated as a single
indivisible unit.
Caveat: for this function to recognize characters compositions, the
automatic compositions should be enabled (see `auto-composition-mode')
and the current buffer must be displayed in some window.

(fn STRING)")

(autoload 'add-display-text-property "subr-x"
"Add the display specification (SPEC VALUE) to the text from START to END.
If any text in the region has a non-nil `display' property, the existing
display specifications are retained.

OBJECT is either a string or a buffer to add the specification to.
If omitted, OBJECT defaults to the current buffer.

(fn START END SPEC VALUE &optional OBJECT)")

(autoload 'remove-display-text-property "subr-x"
"Remove the display specification SPEC from the text from START to END.
SPEC is the car of the display specification to remove, e.g. `height'.
If any text in the region has other display specifications, those specs
are retained.

OBJECT is either a string or a buffer to remove the specification from.
If omitted, OBJECT defaults to the current buffer.

(fn START END SPEC &optional OBJECT)")

(autoload 'read-process-name "subr-x"
"Query the user for a process and return the process object.

(fn PROMPT)")

(autoload 'display-warning "warnings"
"Display a warning message, MESSAGE.
TYPE is the warning type: either a custom group name (a symbol),
or a list of symbols whose first element is a custom group name.
(The rest of the symbols represent subcategories, for warning purposes
only, and you can use whatever symbols you like.)

LEVEL should be either :debug, :warning, :error, or :emergency
(but see `warning-minimum-level' and `warning-minimum-log-level').
Default is :warning.

:emergency -- a problem that will seriously impair Emacs operation soon
	      if you do not attend to it promptly.
:error     -- data or circumstances that are inherently wrong.
:warning   -- data or circumstances that are not inherently wrong,
	      but raise suspicion of a possible problem.
:debug     -- info for debugging only.

BUFFER-NAME, if specified, is the name of the buffer for logging
the warning.  By default, it is `*Warnings*'.  If this function
has to create the buffer, it disables undo in the buffer.

See the `warnings' custom group for user customization features.

See also `warning-series', `warning-prefix-function',
`warning-fill-prefix', and `warning-fill-column' for additional
programming features.

This will also display buttons allowing the user to permanently
disable automatic display of the warning or disable the warning
entirely by setting `warning-suppress-types' or
`warning-suppress-log-types' on their behalf.

(fn TYPE MESSAGE &optional LEVEL BUFFER-NAME)")

(autoload 'lwarn "warnings"
"Display a warning message made from (format-message MESSAGE ARGS...).
\\<special-mode-map>
Aside from generating the message with `format-message',
this is equivalent to `display-warning'.

TYPE is the warning type: either a custom group name (a symbol),
or a list of symbols whose first element is a custom group name.
(The rest of the symbols represent subcategories and
can be whatever you like.)

LEVEL should be either :debug, :warning, :error, or :emergency
(but see `warning-minimum-level' and `warning-minimum-log-level').

:emergency -- a problem that will seriously impair Emacs operation soon
	      if you do not attend to it promptly.
:error     -- invalid data or circumstances.
:warning   -- suspicious data or circumstances.
:debug     -- info for debugging only.

(fn TYPE LEVEL MESSAGE &rest ARGS)")

(autoload 'warn "warnings"
"Display a warning message made from (format-message MESSAGE ARGS...).
Aside from generating the message with `format-message',
this is equivalent to `display-warning', using
`emacs' as the type and `:warning' as the level.

(fn MESSAGE &rest ARGS)")

(autoload 'xml-parse-file "xml"
"Parse the well-formed XML file FILE.
Return the top node with all its children.
If PARSE-DTD is non-nil, the DTD is parsed rather than skipped.

If PARSE-NS is non-nil, then QNAMES are expanded.  By default,
the variable `xml-default-ns' is the mapping from namespaces to
URIs, and expanded names will be returned as a cons

  (\"namespace:\" . \"foo\").

If PARSE-NS is an alist, it will be used as the mapping from
namespace to URIs instead.

If it is the symbol `symbol-qnames', expanded names will be
returned as a plain symbol `namespace:foo' instead of a cons.

Both features can be combined by providing a cons cell

  (symbol-qnames . ALIST).

(fn FILE &optional PARSE-DTD PARSE-NS)")

(autoload 'xml-parse-region "xml"
"Parse the region from BEG to END in BUFFER.
Return the XML parse tree, or raise an error if the region does
not contain well-formed XML.

If BEG is nil, it defaults to `point-min'.
If END is nil, it defaults to `point-max'.
If BUFFER is nil, it defaults to the current buffer.
If PARSE-DTD is non-nil, parse the DTD and return it as the first
element of the list.
If PARSE-NS is non-nil, then QNAMES are expanded.  By default,
the variable `xml-default-ns' is the mapping from namespaces to
URIs, and expanded names will be returned as a cons

  (\"namespace:\" . \"foo\").

If PARSE-NS is an alist, it will be used as the mapping from
namespace to URIs instead.

If it is the symbol `symbol-qnames', expanded names will be
returned as a plain symbol `namespace:foo' instead of a cons.

Both features can be combined by providing a cons cell

  (symbol-qnames . ALIST).

(fn &optional BEG END BUFFER PARSE-DTD PARSE-NS)")

(autoload 'xml-remove-comments "xml"
"Remove XML/HTML comments in the region between BEG and END.
All text between the <!-- ... --> markers will be removed.

(fn BEG END)")

(provide 'core-autoloads)
