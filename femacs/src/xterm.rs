// femacs: port of src/xterm.c @ fd7afced403.
/* New display code by Gerd Moellmann <gerd@gnu.org>.  */
/* Xt features made by Fred Pierresteguy.  */

/* X window system support

   This file is part of the X window system support.  It
   contains subroutines comprising the redisplay interface, setting up
   scroll bars and widgets, and handling input.

   X WINDOW SYSTEM

   The X Window System is a windowing system for bitmap graphics
   displays which originated at MIT in 1984.  Version 11, which is
   currently supported by Emacs, first appeared in September 1987.

   X has a long history and has been developed by many different
   organizations over the years; at present, it is being primarily
   developed by the X.Org Foundation.  It is the main window system
   that Emacs is developed and tested against, and X version 10 was
   the first window system that Emacs was ported to.  As a consequence
   of its age and wide availability, X contains many idiosyncrasies,
   but that has not prevented it from becoming the dominant free
   window system, and the platform of reference for all GUI code in
   Emacs.

   Some of what is explained below also applies to the other window
   systems that Emacs supports, to varying degrees.  YMMV.

   INPUT

   Emacs handles input by running pselect in a loop, which returns
   whenever there is input available on the connection to the X
   server.  On some systems, Emacs also arranges for any new input on
   that connection to send an asynchronous signal.  Whenever pselect
   returns, or such a signal is received and input is not blocked,
   XTread_socket is called and translates X11 events read by Xlib into
   struct input_events, which are then stored in the keyboard buffer,
   to be processed and acted upon at some later time.  The function
   handle_one_xevent is responsible for handling core events after
   they are filtered, and filtering X Input Extension events.  It also
   performs actions on some special events, such as updating the
   dimensions of a frame after a ConfigureNotify is sent by the X
   server to inform us that it changed.

   Before such events are translated, an Emacs build with
   internationalization enabled (the default since X11R6) will filter
   events through an X Input Method (XIM) or GTK, which might decide
   to intercept the event and send a different one in its place, for
   reasons such as enabling the user to insert international
   characters that aren't on his keyboard by typing a sequence of
   characters which are.  See the function x_filter_event and its
   callers for more details.

   Events that cause Emacs to quit are treated specially by the code
   that stores them in the keyboard buffer and generally cause an
   immediate interrupt.  Such an interrupt can lead to a longjmp from
   the code that stored the keyboard event, which isn't safe inside
   XTread_socket.  To avoid this problem, XTread_socket is provided a
   special event buffer named hold_quit.  When a quit event is
   encountered, it is stored inside this special buffer, which will
   cause the keyboard code that called XTread_socket to store it at a
   later time when it is safe to do so.

   handle_one_xevent will generally have to determine which frame an
   event should be attributed to.  This is not easy, because events
   can come from multiple X windows, and a frame can also have
   multiple windows.  handle_one_xevent usually calls the function
   x_any_window_to_frame, which searches for a frame by toplevel
   window and widget windows.  There are also some other functions for
   searching by specific types of window, such as
   x_top_window_to_frame (which only searches for frames by toplevel
   window), and x_menubar_window_to_frame (which will only search
   through frame menu bars).

   INPUT FOCUS

   Under X, the window where keyboard input is sent is not always
   explicitly defined.  When there is a focus window, it receives what
   is referred to as "explicit focus", but when there is none, it
   receives "implicit focus" whenever the pointer enters it, and loses
   that focus when the pointer leaves.  When the toplevel window of a
   frame receives an explicit focus event (FocusIn or FocusOut), we
   treat that frame as having the current input focus, but when there
   is no focus window, we treat each frame as having the input focus
   whenever the pointer enters it, and undo that treatment when the
   pointer leaves it.  See the callers of x_detect_focus_change for
   more details.

   REDISPLAY

   The redisplay engine communicates with X through the "redisplay
   interface", which is a structure containing pointers to functions
   which output graphics to a frame.

   Some of the functions included in the redisplay interface include
   `x_clear_frame_area', which is called by the display engine when it
   determines that a part of the display has to be cleared,
   x_draw_window_cursor, which is called to perform the calculations
   necessary to display the cursor glyph with a special "highlight"
   (more on that later) and to set the input method spot location.

   Most of the actual display is performed by the function
   `x_draw_glyph_string', also included in the redisplay interface.
   It takes a list of glyphs of the same type and face, computes the
   correct graphics context for the string through the function
   `x_set_glyph_string_gc', and draws whichever glyphs it might
   contain, along with decorations such as the box face, underline and
   overline.  That list is referred to as a "glyph string".

   GRAPHICS CONTEXTS

   A graphics context ("GC") is an X server-side object which contains
   drawing attributes such as fill style, stipple, and foreground and
   background pixel values.

   Usually, one graphics context is computed for each face when it is
   about to be displayed for the first time, and this graphics context
   is the one which is used for future X drawing operations in a glyph
   string with that face.  (See `prepare_face_for_display' in
   xfaces.c).

   However, when drawing glyph strings for special display elements
   such as the cursor, or mouse sensitive text, different GCs may be
   used.  When displaying the cursor, for example, the frame's cursor
   graphics context is used for the common case where the cursor is
   drawn with the default font, and the colors of the string's face
   are the same as the default face.  In all other cases, a temporary
   graphics context is created with the foreground and background
   colors of the cursor face adjusted to ensure that the cursor can be
   distinguished from its surroundings and that the text inside the
   cursor stays visible.

   Various graphics contexts are also calculated when the frame is
   created by the function `x_make_gcs' in xfns.c, and are adjusted
   whenever the foreground or background colors change.  The "normal"
   graphics context is used for operations performed without a face,
   and always corresponds to the foreground and background colors of
   the frame's default face, the "reverse" graphics context is used to
   draw text in inverse video, and the cursor graphics context is used
   to display the cursor in the most common case.

   N.B. that some of the other window systems supported by use an
   emulation of graphics contexts to hold the foreground and
   background colors used in a glyph string, while the some others
   ports compute those colors directly based on the colors of the
   string's face and its highlight, but only on X are graphics
   contexts a data structure inherent to the window system.

   COLOR ALLOCATION

   In (and only in) X, pixel values for colors are not guaranteed to
   correspond to their individual components.  The rules for
   converting colors into pixel values are defined by the visual class
   of each display opened by Emacs.  When a display is opened, a
   suitable visual is obtained from the X server, and a colormap is
   created based on that visual, which is then used for each frame
   created.

   The colormap is then used by the X server to convert pixel values
   from a frame created by Emacs into actual colors which are output
   onto the physical display.

   When the visual class is TrueColor, the colormap will be indexed
   based on the red, green, and blue (RGB) components of the pixel
   values, and the colormap will be statically allocated so as to
   contain linear ramps for each component.  As such, most of the
   color allocation described below is bypassed, and the pixel values
   are computed directly from the color.

   Otherwise, each time Emacs wants a pixel value that corresponds to
   a color, Emacs has to ask the X server to obtain the pixel value
   that corresponds to a "color cell" containing the color (or a close
   approximation) from the colormap.  Exactly how this is accomplished
   further depends on the visual class, since some visuals have
   immutable colormaps which contain color cells with pre-defined
   values, while others have colormaps where the color cells are
   dynamically allocated by individual X clients.

   With visuals that have a visual class of StaticColor and StaticGray
   (where the former is the case), the X server is asked to procure
   the pixel value of a color cell that contains the closest
   approximation of the color which Emacs wants.  On the other hand,
   when the visual class is DirectColor, PseudoColor, or GrayScale,
   where color cells are dynamically allocated by clients, Emacs asks
   the X server to allocate a color cell containing the desired color,
   and uses its pixel value.

   (If the color already exists, the X server returns an existing color
   cell, but increases its reference count, so it still has to be
   freed afterwards.)

   Otherwise, if no color could be allocated (due to the colormap
   being full), Emacs looks for a color cell inside the colormap
   closest to the desired color, and uses its pixel value instead.

   Since the capacity of a colormap is finite, X clients have to take
   special precautions in order to not allocate too many color cells
   that are never used.  Emacs allocates its color cells when a face
   is being realized or when a frame changes its foreground and
   background colors, and releases them alongside the face or frame.
   See calls to `unload_color' and `load_color' in xterm.c, xfaces.c
   and xfns.c for more details.

   The driving logic behind color allocation is in
   `x_alloc_nearest_color_1', while the optimization for TrueColor
   visuals is in `x_make_truecolor_pixel'.  Also see `x_query_colors`,
   which is used to determine the color values for given pixel
   values.

   In other window systems supported by Emacs, color allocation is
   handled by the window system itself, to whom Emacs simply passes 24
   (or 32-bit) RGB values.

   OPTIONAL FEATURES

   While X servers and client libraries tend to come with many
   extensions to the core X11R6 protocol, dependencies on anything
   other than the core X11R6 protocol and Xlib should be optional at
   both compile-time and runtime.  Emacs should also not crash
   regardless of what combination of X server and client-side features
   are present.  For example, if you are developing a feature that
   will need Xfixes, then add a test in configure.ac for the library
   at compile-time which defines `HAVE_XFIXES', like this:

     ### Use Xfixes (-lXfixes) if available
     HAVE_XFIXES=no
     if test "${HAVE_X11}" = "yes"; then
       XFIXES_REQUIRED=4.0.0
       XFIXES_MODULES="xfixes >= $XFIXES_REQUIRED"
       EMACS_CHECK_MODULES([XFIXES], [$XFIXES_MODULES])
       if test $HAVE_XFIXES = no; then
	 # Test old way in case pkg-config doesn't have it (older machines).
	 AC_CHECK_HEADER([X11/extensions/Xfixes.h],
	   [AC_CHECK_LIB([Xfixes], [XFixesHideCursor], [HAVE_XFIXES=yes])])
	 if test $HAVE_XFIXES = yes; then
	   XFIXES_LIBS=-lXfixes
	 fi
       fi
       if test $HAVE_XFIXES = yes; then
	 AC_DEFINE([HAVE_XFIXES], [1],
	   [Define to 1 if you have the Xfixes extension.])
       fi
     fi
     AC_SUBST([XFIXES_CFLAGS])
     AC_SUBST([XFIXES_LIBS])

  Then, make sure to adjust CFLAGS and LIBES in src/Makefile.in and
  add the new XFIXES_CFLAGS and XFIXES_LIBS variables to
  msdos/sed1v2.inp.  (The latter has to be adjusted for any new
  variables that are included in CFLAGS and LIBES even if the
  libraries are not used by the MS-DOS port.)

  Finally, add some fields in `struct x_display_info' which specify
  the major and minor versions of the extension, and whether or not to
  support them.  They (and their accessors) should be protected by the
  `HAVE_XFIXES' preprocessor conditional.  Then, these fields should
  be set in `x_term_init', and all Xfixes calls must be protected by
  not only the preprocessor conditional, but also by checks against
  those variables.

  X TOOLKIT SUPPORT

  Emacs supports being built with many different toolkits (and also no
  toolkit at all), which provide decorations such as menu bars and
  scroll bars, along with handy features like file panels, dialog
  boxes, font panels, and popup menus.  Those configurations can
  roughly be classified as belonging to one of three categories:

    - Using no toolkit at all.
    - Using the X Toolkit Intrinsics (Xt).
    - Using GTK.

  The no toolkit configuration is the simplest: no toolkit widgets are
  used, Emacs uses its own implementation of scroll bars, and the
  XMenu library that came with X11R2 and earlier versions of X is used
  for popup menus.  There is also no complicated window structure to
  speak of.

  The Xt configurations come in either the Lucid or Motif flavors.
  The former utilizes Emacs's own Xt-based Lucid widget library for
  menus, and Xaw (or derivatives such as neXTaw and Xaw3d) for dialog
  boxes and, optionally, scroll bars.  It does not support file
  panels.  The latter uses either Motif or LessTif for menu bars,
  popup menus, dialogs and file panels.

  The GTK configurations come in the GTK+ 2 or GTK 3 configurations,
  where the toolkit provides all the aforementioned decorations and
  features.  They work mostly the same, though GTK 3 has various small
  annoyances that complicate maintenance.

  All of those configurations have various special technicalities
  about event handling and the layout of windows inside a frame that
  must be kept in mind when writing X code which is run on all of
  them.

  The no toolkit configuration has no noteworthy aspects about the
  layout of windows inside a frame, since each frame has only one
  associated window aside from scroll bars.  However, in the Xt
  configurations, every widget is a separate window, and there are
  quite a few widgets.  The "outer widget", a widget of class
  ApplicationShell, is the top-level window of a frame.  Its window is
  accessed via the macro `FRAME_OUTER_WINDOW'.  The "edit widget", a
  widget class of EmacsFrame, is a child of the outer widget that
  controls the size of a frame as known to Emacs, and is the widget
  that Emacs draws to during display operations.  The "menu bar
  widget" is the widget holding the menu bar.

  Special care must be taken when performing operations on a frame.
  Properties that are used by the window manager, for example, must be
  set on the outer widget.  Drawing, on the other hand, must be done
  to the edit widget, and button press events on the menu bar widget
  must be redirected and not sent to Xt until the Lisp code is run to
  update the menu bar.

  The EmacsFrame widget is specific to Emacs and is implemented in
  widget.c.  See that file for more details.

  In the GTK configurations, GTK widgets do not necessarily correspond
  to X windows, since the toolkit might decide to keep only a
  client-side record of the widgets for performance reasons.

  Because the GtkFixed widget that holds the "edit area" might not
  correspond to an X window, drawing operations may be directly
  performed on the outer window, with special care taken to not
  overwrite the surrounding GTK widgets.  This also means that the
  only important window for most purposes is the outer window, which
  on GTK builds can usually be accessed using the macro
  `FRAME_X_WINDOW'.

  How `handle_one_xevent' is called also depends on the configuration.
  Without a toolkit, Emacs performs all event processing by itself,
  running XPending and XNextEvent in a loop whenever there is input,
  passing the event to `handle_one_xevent'.

  When using Xt, the same is performed, but `handle_one_xevent' may
  also decide to call XtDispatchEvent on an event after Emacs finishes
  processing it.

  When using GTK, however, `handle_one_xevent' is called from an event
  filter installed on the GTK event loop.  Unless the event filter
  elects to drop the event, it will be passed to GTK right after
  leaving the event filter.

  Fortunately, `handle_one_xevent' is provided a `*finish' parameter
  that abstracts away all these details.  If it is `X_EVENT_DROP',
  then the event will not be dispatched to Xt or utilized by GTK.
  Code inside `handle_one_xevent' should thus avoid making assumptions
  about the event dispatch mechanism and use that parameter
  instead.

  FRAME RESIZING

  In the following explanations "frame size" refers to the "native
  size" of a frame as reported by the (frame.h) macros
  FRAME_PIXEL_WIDTH and FRAME_PIXEL_HEIGHT.  These specify the size of
  a frame as the values passed to/received from a toolkit and the
  window manager.  The "text size" Emacs Lisp code uses in functions
  like 'set-frame-size' or sees in the ‘width’ and 'height' frame
  parameters is only loosely related to the native size.  The
  necessary translations are provided by the macros
  FRAME_TEXT_TO_PIXEL_WIDTH and FRAME_TEXT_TO_PIXEL_HEIGHT as well as
  FRAME_PIXEL_TO_TEXT_WIDTH and FRAME_PIXEL_TO_TEXT_HEIGHT (in
  frame.h).

  Lisp functions may ask for resizing a frame either explicitly, using
  one of the interfaces provided for that purpose like, for example,
  'set-frame-size' or changing the 'height' or 'width' parameter of
  that frame, or implicitly, for example, by turning off/on or
  changing the width of fringes or scroll bars for that frame.  Any
  such request passes through the routine 'adjust_frame_size' (in
  frame.c) which decides, among others, whether the native frame size
  would really change and whether it is allowed to change it at that
  moment.  Only if 'adjust_frame_size' decides that the corresponding
  terminal's 'set_window_size_hook' may be run, it will dispatch
  execution to the appropriate function which, for X builds, is
  'x_set_window_size' in this file.

  For GTK builds, 'x_set_window_size' calls 'xg_frame_set_char_size'
  in gtkutil.c if the frame has an edit widget and
  'x_set_window_size_1' in this file otherwise.  For non-GTK builds,
  'x_set_window_size' always calls 'x_set_window_size_1' directly.

  'xg_frame_set_char_size' calls the GTK function 'gtk_window_resize'
  for the frame's outer widget; x_set_window_size_1 calls the Xlib
  function 'XResizeWindow' instead.  In either case, if Emacs thinks
  that the frame is visible, it will wait for a ConfigureNotify event
  (see below) to occur within a timeout of 'x-wait-for-event-timeout'
  (the default is 0.1 seconds).  If Emacs thinks that the frame is not
  visible, it calls 'adjust_frame_size' to run 'resize_frame_windows'
  (see below) and hopes for the best.

  Note that if Emacs receives a ConfigureEvent in response to an
  earlier resize request, the sizes specified by that event are not
  necessarily the sizes Emacs requested.  Window manager and toolkit
  may override any of the requested sizes for their own reasons.

  On X, size notifications are received as ConfigureNotify events.
  The expected reaction to such an event on the Emacs side is to
  resize all Emacs windows that are on the frame referred to by the
  event.  Since resizing Emacs windows and redisplaying their buffers
  is a costly operation, Emacs may collapse several subsequent
  ConfigureNotify events into one to avoid that Emacs falls behind in
  user interactions like resizing a frame by dragging one of its
  borders with the mouse.

  Each ConfigureEvent event specifies a window, a width and a height.
  The event loop uses 'x_top_window_to_frame' to associate the window
  with its frame.  Once the frame has been identified, on GTK the
  event is dispatched to 'xg_frame_resized'.  On Motif/Lucid
  'x_window' has installed 'EmacsFrameResize' as the routine that
  handles resize events.  In either case, these routines end up
  calling the function 'change_frame_size' in dispnew.c.  On
  non-toolkit builds the effect is to call 'change_frame_size'
  directly from the event loop.  In either case, the value true is
  passed as the DELAY argument.

  'change_frame_size' is the central function to decide whether it is
  safe to process a resize request immediately or it has to be delayed
  (usually because its DELAY argument is true).  Since resizing a
  frame's windows may run arbitrary Lisp code, Emacs cannot generally
  process resize requests during redisplay and therefore has to queue
  them.  If processing the event must be delayed, the new sizes (that
  is, the ones requested by the ConfigureEvent) are stored in the
  new_width and new_height slots of the respective frame structure,
  possibly replacing ones that have been stored there upon the receipt
  of a preceding ConfigureEvent.

  Delayed size changes are applied eventually upon calls of the
  function 'do_pending_window_change' (in dispnew.c) which is called
  by the redisplay code at suitable spots where it's safe to change
  sizes.  'do_pending_window_change' calls 'change_frame_size' with
  its DELAY argument false in the hope that it is now safe to call the
  function 'resize_frame_windows' (in window.c) which is in charge of
  adjusting the sizes of all Emacs windows on the frame accordingly.
  Note that if 'resize_frame_windows' decides that the windows of a
  frame do not fit into the constraints set up by the new frame sizes,
  it will resize the windows to some minimum sizes with the effect
  that parts of the frame at the right and bottom will appear clipped
  off.

  In addition to explicitly passing width and height values in
  functions like 'gtk_window_resize' or 'XResizeWindow', Emacs also
  sets window manager size hints - a more implicit form of asking for
  the size Emacs would like its frames to assume.  Some of these hints
  only restate the size and the position explicitly requested for a
  frame.  Another hint specifies the increments in which the window
  manager should resize a frame to - either set to the default
  character size of a frame or to one pixel for a non-nil value of
  'frame-resize-pixelwise'.  See the function 'x_wm_set_size_hint' -
  in gtkutil.c for GTK and in this file for other builds - for the
  details.

  We have not discussed here a number of special issues like, for
  example, how to handle size requests and notifications for maximized
  and fullscreen frames or how to resize child frames.  Some of these
  require special treatment depending on the desktop or window manager
  used.

  One thing that might come handy when investigating problems wrt
  resizing frames is the variable 'frame-size-history'.  Setting this
  to a non-nil value, will cause Emacs to start recording frame size
  adjustments, usually specified by the function that asked for an
  adjustment, a sizes part that records the old and new values of the
  frame's width and height and maybe some additional information.  The
  internal function `frame--size-history' can then be used to display
  the value of this variable in a more readable form.

  FRAME RESIZE SYNCHRONIZATION

  The X window system operates asynchronously.  That is to say, the
  window manager and X server might think a window has been resized
  before Emacs has a chance to process the ConfigureNotify event that
  was sent.

  When a compositing manager is present, and the X server and Emacs
  both support the X synchronization extension, the semi-standard
  frame synchronization protocol can be used to notify the compositing
  manager of when Emacs has actually finished redisplaying the
  contents of a frame after a resize.  The compositing manager will
  customarily then postpone displaying the contents of the frame until
  the redisplay is complete.

  Emacs announces support for this protocol by creating an X
  server-side counter object, and setting it as the
  `_NET_WM_SYNC_REQUEST_COUNTER' property of the frame's top-level
  window.  The window manager then initiates the synchronized resize
  process by sending Emacs a ClientMessage event before the
  ConfigureNotify event where:

    type = ClientMessage
    window = the respective client window
    message_type = WM_PROTOCOLS
    format = 32
    data.l[0] = _NET_WM_SYNC_REQUEST
    data.l[1] = timestamp
    data.l[2] = low 32 bits of a provided frame counter value
    data.l[3] = high 32 bits of a provided frame counter value
    data.l[4] = 1 if the extended frame counter should be updated,
    otherwise 0

  Upon receiving such an event, Emacs constructs and saves a counter
  value from the provided low and high 32 bits.  Then, when the
  display engine tells us that a frame has been completely updated
  (presumably because of a redisplay caused by a ConfigureNotify
  event), we set the counter to the saved value, telling the
  compositing manager that the contents of the window now accurately
  reflect the new size.  The compositing manager will then display the
  contents of the window, and the window manager might also postpone
  updating the window decorations until this moment.

  DRAG AND DROP

  Drag and drop in Emacs is implemented in two ways, depending on
  which side initiated the drag-and-drop operation.  When another X
  client initiates a drag, and the user drops something on Emacs, a
  `drag-n-drop-event' is sent with the contents of the ClientMessage,
  and further processing (i.e. retrieving selection contents and
  replying to the initiating client) is performed from Lisp inside
  `x-dnd.el'.

  However, dragging contents from Emacs is implemented almost entirely
  in C.  X Windows has several competing drag-and-drop protocols, of
  which Emacs supports two on the C level: the XDND protocol (see
  https://freedesktop.org/wiki/Specifications/XDND) and the Motif drag
  and drop protocols.  These protocols are based on the initiator
  owning a special selection, specifying an action the recipient
  should perform, grabbing the mouse, and sending various different
  client messages to the toplevel window underneath the mouse as it
  moves, or when buttons are released.

  The Lisp interface to drag-and-drop is synchronous, and involves
  running a nested event loop with some global state until the drag
  finishes.  When the mouse moves, Emacs looks up the toplevel window
  underneath the pointer (the target window) either using a cache
  provided by window managers that support the
  _NET_WM_CLIENT_LIST_STACKING root window property, or by calling
  XTranslateCoordinates in a loop until a toplevel window is found,
  and sends various entry, exit, or motion events to the window
  containing a list of targets the special selection can be converted
  to, and the chosen action that the recipient should perform.  The
  recipient can then send messages in reply detailing the action it
  has actually chosen to perform.  Finally, when the mouse buttons are
  released over the recipient window, Emacs sends a "drop" message to
  the target window, waits for a reply, and returns the action
  selected by the recipient to the Lisp code that initiated the
  drag-and-drop operation.

  When a drop happens on a window not supporting any protocol
  implemented on the C level, the function inside
  `x-dnd-unsupported-drop-function' is called with some parameters of
  the drop.  If it returns non-nil, then Emacs tries to simulate a
  drop happening with the primary selection and synthetic button
  events (see `x_dnd_do_unsupported_drop').  That function implements
  the OffiX drag-and-drop protocol by default.  See
  `x-dnd-handle-unsupported-drop' in `x-dnd.el' for more details.

  DISPLAY ERROR HANDLING

  While error handling under X was originally designed solely as a
  mechanism for the X server to report fatal errors to clients, most
  clients (including Emacs) have adopted a system of "error traps" to
  handle or discard these errors as they arrive.  Discarding errors is
  usually necessary when Emacs performs an X request that might fail:
  for example, sending a message to a window that may no longer exist,
  or might not exist at all.  Handling errors is then necessary when
  the detailed error must be reported to another piece of code: for
  example, as a Lisp error.

  It is not acceptable for Emacs to crash when it is sent invalid data
  by another client, or by Lisp.  As a result, errors must be caught
  around Xlib functions generating requests containing resource
  identifiers that could potentially be invalid, such as window or
  atom identifiers provided in a client message from another program,
  or a child window ID obtained through XTranslateCoordinates that may
  refer to a window that has been deleted in the meantime.

  There are two sets of functions used to perform this "error
  trapping".  Which one should be used depends on what kind of
  processing must be done on the error.  The first consists of the
  functions `x_ignore_errors_for_next_request' and
  `x_stop_ignoring_errors', which ignore errors generated by requests
  made in between a call to the first function and a corresponding
  call to the second.  They should be used for simple asynchronous
  requests that do not require a reply from the X server: using them
  instead of the second set improves performance, as they simply
  record a range of request serials to ignore errors from, instead of
  synchronizing with the X server to handle errors.

  The second set consists of the following functions:

    - x_catch_errors_with_handler
    - x_catch_errors
    - x_uncatch_errors_after_check
    - x_uncatch_errors
    - x_check_errors
    - x_had_errors_p
    - x_clear_errors

  Callers using this set should consult the comment(s) on top of the
  aforementioned functions.  They should not be used when the requests
  being made do not require roundtrips to the X server, and obtaining
  the details of any error generated is unnecessary, as
  `x_uncatch_errors' will always synchronize with the X server, which
  is a potentially slow operation.  */

/* femacs: `src/xterm.c` is the X11 backend, and the frozen configuration does not compile it.

   `docs/baseline/frozen-build-sources.txt` has no `src/xterm.c`: this target's platform object
   variables are empty (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=` in
   `build/emacs-ref/src/Makefile`), so `make -n` never names the file (docs/status.md 3.31).
   What is "live" for it is the X11 target, not this one, and the port has the shape the file's
   own header mirror already has (`femacs/src/include/xterm.rs`, commit b03d883):

     * the whole body carries `#[cfg (feature = "have_x_windows")]` - P-51's arm for the
       non-frozen side, the side whose target supplies Xlib, Xt, Xft, cairo and GTK;
     * every inner condition keeps C's own spelling lowercased as its feature name (`USE_GTK`
       -> `use_gtk`, `HAVE_X_I18N` -> `have_x_i18n`, `USE_X_TOOLKIT` -> `use_x_toolkit`, ...),
       and the arm C compiles on the frozen target is the one that feature disables;
     * the arms outside the frozen reading are carried as text (P-51) instead of dropped, so the
       file stays a 1:1 transcript of the reference (PORTING.md P-42) and the checklist in
       `docs/ports/xterm.md` is the frozen reading's, with the arms outside it tabulated in that
       ledger's Decisions.

   Verification, and what it is worth: `xtask build-tu src/xterm.c` compiles the unit with every
   feature off, so it proves that the text parses and that the lazy arms are legal Rust, and
   nothing beyond that - the same evidence `femacs/src/include/xterm.rs` has (status 3.171).
   The progress reading is `tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs`:
   its `- ` count is what the frozen reading still demands (460 before this slice).  Nothing here
   is reached by the frozen build, and a Lisp test cannot reach it either: the file exists only
   in an X11 build of Emacs.

   Slice 1 (this round) = C 625-990: the header preamble, the two X11 handler typedefs, the first
   file-scope objects with their conditional arms, the two selection-request records, the three
   selection-request functions and the atom-table macro.  What the slice names but does not
   define is in the ledger's Decisions.  */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* C 625-728: the header preamble, in C's order.  A header this tree has a mirror for is
   included where C includes it; the rest are X11/xcb/Xt/Xm/Xaw headers (C 637, 644-645,
   650-683, 688, 691, 724-725, 733, 738, 742, 746-748, 755-765, 785, 788) and the ledger lists
   them instead.  */
include!("include/config.rs");                    // C 625: `#include <config.h>`
include!("../lib/include/stdlib.rs");             // C 626: `#include <stdlib.h>`
include!("../lib/include/math.rs");               // C 627: `#include <math.h>`
include!("../lib/include/signal.rs");             // C 628: `#include <signal.h>`
include!("include/lisp.rs");                      // C 630: `#include "lisp.h"`
include!("include/blockinput.rs");                // C 631: `#include "blockinput.h"`
include!("../lib/include/stdio.rs");              // C 632: `#include "sysstdio.h"`
include!("include/xterm.rs");                     // C 636: `#include "xterm.h"`
include!("include/textconv.rs");                  // C 640: `#include "textconv.h"` (HAVE_X_I18N)
include!("../lib/include/sys_types.in.rs");       // C 688: `#include <sys/types.h>`
include!("include/systime.rs");                   // C 693: `#include "systime.h"`
include!("../lib/include/fcntl.in.rs");           // C 695: `#include <fcntl.h>`
include!("../lib/include/errno.rs");              // C 696: `#include <errno.h>`
include!("../lib/include/sys_stat.in.rs");        // C 697: `#include <sys/stat.h>`
include!("../lib/include/flexmember.rs");         // C 698: `#include <flexmember.h>`
include!("../lib/include/c-ctype.rs");            // C 699: `#include <c-ctype.h>`
include!("../lib/include/byteswap.in.rs");        // C 700: `#include <byteswap.h>`
include!("include/character.rs");                 // C 702: `#include "character.h"`
include!("include/coding.rs");                    // C 703: `#include "coding.h"`
include!("include/composite.rs");                 // C 704: `#include "composite.h"`
include!("include/frame.rs");                     // C 705: `#include "frame.h"`
include!("include/dispextern.rs");                // C 706: `#include "dispextern.h"`
include!("include/xwidget.rs");                   // C 707: `#include "xwidget.h"`
include!("include/fontset.rs");                   // C 708: `#include "fontset.h"`
include!("include/termhooks.rs");                 // C 709: `#include "termhooks.h"`
include!("include/termopts.rs");                  // C 710: `#include "termopts.h"`
include!("include/termchar.rs");                  // C 711: `#include "termchar.h"`
include!("include/emacs-icon.rs");                // C 712: `#include "emacs-icon.h"`
include!("include/intervals.rs");                 // buffer.h 27: `#include "intervals.h"`
include!("include/itree.rs");                     // intervals.h: `itree_node`/`itree_tree`
include!("include/buffer.rs");                    // C 713: `#include "buffer.h"`
include!("include/window.rs");                    // C 714: `#include "window.h"`
include!("include/keyboard.rs");                  // C 715: `#include "keyboard.h"`
include!("include/atimer.rs");                    // C 716: `#include "atimer.h"`
include!("include/font.rs");                      // C 717: `#include "font.h"`
include!("include/xsettings.rs");                 // C 718: `#include "xsettings.h"`
include!("include/sysselect.rs");                 // C 719: `#include "sysselect.h"`
include!("include/menu.rs");                      // C 720: `#include "menu.h"`
include!("include/pdumper.rs");                   // C 721: `#include "pdumper.h"`
include!("../lib/include/unistd.in.rs");          // C 728: `#include <unistd.h>`
include!("include/gtkutil.rs");                   // C 731: `#include "gtkutil.h"` (USE_GTK)
include!("include/xgselect.rs");                  // C 782: `#include <xgselect.h>` (USE_GTK)
include!("include/widget.rs");                    // C 775: `#include "widget.h"` (USE_X_TOOLKIT)

/* C 791-795: the two X11 handler typedefs - C defines them because Xlib does not promise the
   names are in every Xlib header.  `Display` and `XErrorEvent` are Xlib's and no mirror in this
   tree owns them, so the two pointers are opaque and the C prototype stays on the `/// C:`
   line (P-72 rule 3).  */
/// C: typedef int (*Emacs_XErrorHandler) (Display *, XErrorEvent *);
#[cfg (feature = "have_x_windows")]
pub type Emacs_XErrorHandler = Option<unsafe extern "C" fn (*mut c_void, *mut c_void) -> c_int>;
/// C: typedef int (*Emacs_XIOErrorHandler) (Display *);
#[cfg (feature = "have_x_windows")]
pub type Emacs_XIOErrorHandler = Option<unsafe extern "C" fn (*mut c_void) -> c_int>;

/* C 766-768: `#ifndef XtNpickTop` / `#define XtNpickTop "pickTop"`, inside `USE_X_TOOLKIT`
   (750) + `USE_TOOLKIT_SCROLL_BARS` (753) + the `#else` of `USE_MOTIF` (754-769).  The
   `#ifndef` exists because Xaw3d's own header defines the name; the port cannot see the X11
   headers, so the arm is carried under its enclosing features and the ledger records that.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit",
            feature = "use_toolkit_scroll_bars", not (feature = "use_motif")))]
macro_rules! XtNpickTop { () => { "pickTop" } }

/* C 774-778: the toolkit's `initialState` resource name, again behind an `#ifndef` guard.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
macro_rules! XtNinitialState { () => { "initialState" } }

/* C 797-799: `#if defined USE_XCB && defined USE_CAIRO_XCB` -> `USE_CAIRO_XCB_SURFACE`.  C's
   `#define` has no body, so the port's arm defines the name with an empty expansion (the
   `lread.rs` `USE_ANDROID_ASSETS` shape).  */
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "use_cairo_xcb"))]
macro_rules! USE_CAIRO_XCB_SURFACE { () => {} }

/* C 801-805: `#if XCB_SHAPE_MAJOR_VERSION > 1 || (XCB_SHAPE_MAJOR_VERSION == 1 &&
   XCB_SHAPE_MINOR_VERSION >= 1)` -> `HAVE_XCB_SHAPE_INPUT_RECTS`.  The three version macros
   come from the xcb shape headers, which this tree has no mirror for, so the name C derives is
   the feature the arm is carried under.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xcb_shape_input_rects"))]
macro_rules! HAVE_XCB_SHAPE_INPUT_RECTS { () => {} }

/* C 869-881: the three XInput2 request codes, each behind an `#ifndef` (the XI2 headers define
   them once they are new enough to need a different spelling).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
macro_rules! X_XIGrabDevice { () => { 51 } }
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
macro_rules! X_XIUngrabDevice { () => { 52 } }
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
macro_rules! X_XIAllowEvents { () => { 53 } }

/* C 888-889: the keyboard buffer's "next entry" step.  C's `kbd_buffer` is the keyboard.h
   mirror's static array, `KBD_BUFFER_SIZE` its length; the comparison is against the last
   slot, and the `?:` is the port's `if`/`else` (PORTING.md 2.1).  */
#[cfg (feature = "have_x_windows")]
macro_rules! X_NEXT_KBD_EVENT {
  ($ptr:expr) => {
    if $ptr == (&raw mut kbd_buffer).cast::<buffered_input_event> ()
                 .add ((KBD_BUFFER_SIZE - 1) as usize)
      { (&raw mut kbd_buffer).cast::<buffered_input_event> () }
      else { $ptr.add (1) }
  };
}

/* C 883-889, 961-983: the file-scope objects, and the two records they use.  The objects live
   in this module, which is glob-imported below: `include/xterm.rs` declares one of them
   (`x_display_list`, its C 999) at the crate root, so a crate-root definition would be `E0428`
   as soon as the X11 target turns `have_x_windows` on - the `keyboard_globals` shape
   (PORTING.md P-21, the same rule that put `input_pending` in a module).  */
#[cfg (feature = "have_x_windows")]
pub mod xterm_globals {
  use super::*;

  /* C 807-816: `#ifdef USE_GTK` -- the quit event a GTK signal handler cannot deliver itself;
     C's compound literal names only `.kind`, so every other member is its zero value and
     `NIL_IS_ZERO` makes the four `Lisp_Object`s nil (termhooks.rs's `EVENT_INIT` note).  */
  #[cfg (feature = "use_gtk")]
  #[unsafe(no_mangle)]
  pub static mut xg_pending_quit_event: input_event =
    input_event { bits: NO_EVENT as u32, code: 0, modifiers: 0, x: lisp_h_Qnil (),
                  y: lisp_h_Qnil (), timestamp: 0, frame_or_window: lisp_h_Qnil (),
                  arg: lisp_h_Qnil (), device: lisp_h_Qnil () };

  /* C 818-821: `static bool any_help_event_p;`  */
  pub static mut any_help_event_p: bool = false;

  /* C 823-826: the chain of X displays in use.  This is the one object of the module whose
     declaration the mirror already has at the crate root (its C 999).  */
  #[unsafe(no_mangle)]
  pub static mut x_display_list: *mut x_display_info = std::ptr::null_mut ();

  /* C 828-837: `#ifdef USE_X_TOOLKIT` -- the Xt application context and the toolkit
     scroll-bar flag.  `XtAppContext` is Xt's opaque handle and `String` Xt's `char *`, and no
     mirror in this tree owns either, so the first is an opaque pointer with its prototype on
     the `/// C:` line (P-72 rule 3) and the second is `*mut c_char`.  */
  /// C: XtAppContext Xt_app_con;
  #[cfg (feature = "use_x_toolkit")]
  #[unsafe(no_mangle)]
  pub static mut Xt_app_con: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_x_toolkit")]
  pub static mut Xt_default_resources: [*mut c_char; 1] = [std::ptr::null_mut ()];
  #[cfg (feature = "use_x_toolkit")]
  pub static mut toolkit_scroll_bar_interaction: bool = false;

  /* C 839-847: the click that selected a frame is not a command.  */
  pub static mut ignore_next_mouse_click_timeout: Time = 0;
  pub static mut mouse_click_timeout_display: *mut x_display_info = std::ptr::null_mut ();

  /* C 849-851: `static int x_noop_count;`  */
  pub static mut x_noop_count: c_int = 0;

  /* C 853-856: `#ifdef USE_GTK` -- the icon file name.  */
  #[cfg (feature = "use_gtk")]
  pub static mut xg_default_icon_file: Lisp_Object = lisp_h_Qnil ();

  /* C 858-861: `#ifdef HAVE_X_I18N` -- `static char emacs_class[] = EMACS_CLASS;`, and frame.h
     1840 defines `EMACS_CLASS` as the string `Emacs` (six bytes with the NUL).  */
  #[cfg (feature = "have_x_i18n")]
  pub static mut emacs_class: [c_char; 6] =
    [b'E' as c_char, b'm' as c_char, b'a' as c_char, b'c' as c_char, b's' as c_char, 0];

  /* C 863-867: `#ifdef USE_GTK` -- the state a GTK signal handler passes to XTread_socket.  */
  #[cfg (feature = "use_gtk")]
  pub static mut current_count: c_int = 0;
  #[cfg (feature = "use_gtk")]
  pub static mut current_finish: c_int = 0;
  #[cfg (feature = "use_gtk")]
  pub static mut current_hold_quit: *mut input_event = std::ptr::null_mut ();

  /* C 883-885: `static int x_use_pending_selection_requests;`  */
  pub static mut x_use_pending_selection_requests: c_int = 0;

  /* C 970-974: the chain of unprocessed selection requests.  */
  pub static mut pending_selection_requests: *mut x_selection_request_event = std::ptr::null_mut ();
}
#[cfg (feature = "have_x_windows")]
use xterm_globals::*;

/* C 961-968: a deferred selection request.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct x_selection_request_event
{
  /// C 964: `struct selection_input_event se;`
  pub se: selection_input_event,
  /// C 967: `struct x_selection_request_event *next;`
  pub next: *mut x_selection_request_event,
}

/* C 976-983: one atom to intern when a display is connected.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct x_atom_ref
{
  /// C 979: `const char *name;`
  pub name: *const c_char,
  /// C 982: `int offset;`
  pub offset: c_int,
}

/* C 989-990, the `ATOM_REFS_INIT` macro the `x_atom_refs` table at C 987 is written with.  The
   table itself needs all 122 `Xatom_*` members of `struct x_display_info`, so it opens the next
   slice; the macro is here because C defines it outside the table's braces.  */
#[cfg (feature = "have_x_windows")]
macro_rules! ATOM_REFS_INIT {
  ($string:expr, $member:ident) => {
    /* C: { string, offsetof (struct x_display_info, member) }, */
    x_atom_ref { name: $string, offset: core::mem::offset_of! (x_display_info, $member) as c_int }
  };
}

/* C 891: `static void x_push_selection_request (struct selection_input_event *);` - a forward
   declaration of the static C defines at 12573.  Rust has no prototypes and resolves items by
   name, so the port writes the definition where C writes it; until that slice lands the call
   below names a function that exists nowhere, which is what makes this unit's text - not a
   compilation unit - the deliverable (the same state `include/xterm.rs` is in).  */

/* C 893-944: defer selection requests, queueing the ones already in the keyboard buffer.  The
   bitfield read `event->ie.kind` is the mirror's `input_event::bits` unit, and the port keeps
   C's member name in a per-unit accessor (the `keyboard.rs` `input_event_bits` shape,
   PORTING.md P-80).  */
#[cfg (feature = "have_x_windows")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* C 946-950: the counter's other half.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_release_selection_requests ()
{
  x_use_pending_selection_requests -= 1;
}

/* C 952-959: release, and handle what was queued if this was the outermost deferral.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_release_selection_requests_and_flush ()
{
  x_release_selection_requests ();

  if x_use_pending_selection_requests == 0
    {
      /* C: x_handle_pending_selection_requests () - xterm.h 1857 declares it and this tree's
         xterm.h mirror does not carry it yet (the ledger records the gap).  */
      x_handle_pending_selection_requests ();
    }
}

/* C 899-944.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_defer_selection_requests ()
{
  let mut event: *mut buffered_input_event;
  let mut between: bool;

  between = false;

  block_input ();
  if x_use_pending_selection_requests == 0
    {
      event = kbd_fetch_ptr;

      while event != kbd_store_ptr
	{
	  if input_event_bits! (kind, &raw mut (*event).ie) == SELECTION_REQUEST_EVENT
	      || input_event_bits! (kind, &raw mut (*event).ie) == SELECTION_CLEAR_EVENT
	    {
	      x_push_selection_request (&raw mut (*event).sie);

	      /* Mark this selection event as invalid.  */
	      SELECTION_EVENT_DPYINFO! (&raw mut (*event).sie) = std::ptr::null_mut ();

	      /* Move the kbd_fetch_ptr along if doing so would not result in any other
		 events being skipped.  This avoids exhausting the keyboard buffer with
		 some over-enthusiastic clipboard managers.  */
	      if !between
		{
		  kbd_fetch_ptr = X_NEXT_KBD_EVENT! (event);

		  /* `detect_input_pending' will then recompute whether or not pending input
		     events exist.  */
		  input_pending = false;
		}
	    }
	  else
	    { between = true; }

	  event = X_NEXT_KBD_EVENT! (event);
	}
    }

  x_use_pending_selection_requests += 1;
  unblock_input ();
}
