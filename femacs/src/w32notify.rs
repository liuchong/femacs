// femacs: port of src/w32notify.c @ fd7afced403.
/* Design overview:

   For each watch request, we launch a separate worker thread.  The
   worker thread runs the watch_worker function, which issues an
   asynchronous call to ReadDirectoryChangesW, and then calls
   WaitForSingleObjectEx to wait that an event be signaled
   to terminate the thread.
   Waiting with WaitForSingleObjectEx puts the thread in an
   "alertable" state, so it wakes up when either (a) the call to
   ReadDirectoryChangesW completes, or (b) the main thread instructs
   the worker thread to terminate by signaling an event, see below.

   When the ReadDirectoryChangesW call completes, its completion
   routine watch_completion is automatically called.  watch_completion
   stashes the received file events in a linked list used to
   communicate them to the main thread (using a critical section, so
   that several threads could alter the same linked list), posts a
   special message, WM_EMACS_FILENOTIFY, to the Emacs's message queue,
   and returns.  That causes the WaitForSingleObjectEx function call
   inside watch_worker to return, but the thread won't terminate until
   the event telling to do so will be signaled.  The completion
   routine then issues another call to ReadDirectoryChangesW as quickly
   as possible.  (Except when it does not, see below.)

   In a GUI session, the WM_EMACS_FILENOTIFY message posted to the
   message queue gets dispatched to the main Emacs window procedure,
   which queues it for processing by w32_read_socket.  When
   w32_read_socket sees this message, it accesses the linked list with file
   notifications (using a critical section), extracts the information,
   converts it to a series of FILE_NOTIFY_EVENT events, and stuffs
   them into the input event queue to be processed by keyboard.c input
   machinery (read_char via a call to kbd_buffer_get_event).

   In a non-GUI session, we send the WM_EMACS_FILENOTIFY message to
   the main (a.k.a. "Lisp") thread instead, since there are no window
   procedures in console programs.  That message wakes up
   MsgWaitForMultipleObjects inside sys_select, which then signals to
   its caller that some keyboard input is available.  This causes
   w32_console_read_socket to be called, which accesses the linked list
   with file notifications and stuffs them into the input event queue
   for keyboard.c to process.

   When the FILE_NOTIFY_EVENT event is processed by keyboard.c's
   kbd_buffer_get_event, it is converted to a Lispy event that can be
   bound to a command.  The default binding is file-notify-handle-event,
   defined on subr.el.

   Routines w32_read_socket or w32_console_read_socket process notifications
   sets as long as some are available.

   When the watch is removed by a call to w32notify-rm-watch, the main
   thread requests that the worker thread terminates by signaling the
   appropriate event and queuing an APC for the worker thread.  The
   APC specifies the watch_end function to be called.  watch_end calls
   CancelIo on the outstanding ReadDirectoryChangesW call.  When
   watch_end returns, the watch_completion function is called one last
   time with the ERROR_OPERATION_ABORTED status, which causes it to
   clean up and set a flag telling watch_worker to exit without
   issuing another ReadDirectoryChangesW call.  Since watch_worker is
   the thread procedure of the worker thread, exiting it causes the
   thread to exit.  The main thread waits for some time for the worker
   thread to exit, and if it doesn't, terminates it forcibly.  */


/* femacs: emacs/src/w32notify.c (@ 740 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/w32notify.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
