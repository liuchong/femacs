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

   When the visual class is trueColor, the colormap will be indexed
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
   `x_alloc_nearest_color_1', while the optimization for trueColor
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
   in a build of the reference that has X11.

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
include!("../lib/include/string.rs");                 // lisp.h 30: `#include <string.h>` (memcmp)
include!("../lib/include/limits.in.rs");                 // lisp.h 33: `#include <limits.h>` (LONG_MAX)
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

  /* ---- C 1214-1548: the drag-and-drop state; slice 2. ----------------------------------
     `volatile` (C 1548) has no Rust equivalent and stays an ordinary static - the
     `keyboard.rs` note for `interrupt_input_blocked` (PORTING.md 1.1).  The three Xlib
     *pointer* typedefs (`Display`, `XErrorEvent`, `XtAppContext`) are opaque here with the C
     prototype on a `/// C:` line (P-72 rule 3); every other X11 name (`Window`, `Atom`,
     `XRectangle`, `XEvent`, `XWindowAttributes`) keeps C's spelling, which is what this
     file's own header mirror does with `*mut XEvent` and `XRectangle` (its 457, 605).  */

  /* C 1217-1231: the four flags the event loop reads.  The first four are declared by
     `include/xterm.rs` at the crate root, which is why the objects live in this module.  */
  #[unsafe(no_mangle)]
  pub static mut x_dnd_in_progress: bool = false;
  #[unsafe(no_mangle)]
  pub static mut x_dnd_frame: *mut frame = std::ptr::null_mut ();
  #[unsafe(no_mangle)]
  pub static mut x_dnd_finish_frame: *mut frame = std::ptr::null_mut ();
  #[unsafe(no_mangle)]
  pub static mut x_dnd_waiting_for_finish: bool = false;

  /* C 1236-1252: the unsupported-drop state and the tooltip monitor list.  */
  pub static mut x_dnd_run_unsupported_drop_function: bool = false;
  pub static mut x_dnd_unsupported_drop_time: Time = 0;
  pub static mut x_dnd_unsupported_drop_window: Window = 0;
  pub static mut x_dnd_unsupported_drop_data: Lisp_Object = lisp_h_Qnil ();
  pub static mut x_dnd_update_tooltip: bool = false;
  pub static mut x_dnd_monitors: Lisp_Object = lisp_h_Qnil ();
  /// C: static Display *x_dnd_finish_display;
  pub static mut x_dnd_finish_display: *mut c_void = std::ptr::null_mut ();

  /* C 1267-1298: the Motif drop state.  */
  pub static mut x_dnd_waiting_for_motif_finish: c_int = 0;
  pub static mut x_dnd_waiting_for_motif_finish_display: *mut x_display_info = std::ptr::null_mut ();
  pub static mut x_dnd_xm_use_help: bool = false;
  pub static mut x_dnd_motif_setup_p: bool = false;
  pub static mut x_dnd_motif_atom: Atom = 0;
  pub static mut x_dnd_pending_finish_target: Window = 0;
  pub static mut x_dnd_waiting_for_finish_proto: c_int = 0;
  pub static mut x_dnd_allow_current_frame: bool = false;
  pub static mut x_dnd_init_type_lists: bool = false;
  pub static mut x_dnd_return_frame: c_int = 0;
  pub static mut x_dnd_return_frame_object: *mut frame = std::ptr::null_mut ();

  /* C 1314-1341: the last window the pointer moved over.  */
  pub static mut x_dnd_last_seen_window: Window = 0;
  pub static mut x_dnd_last_seen_toplevel: Window = 0;
  pub static mut x_dnd_end_window: Window = 0;
  pub static mut x_dnd_last_protocol_version: c_int = 0;
  pub static mut x_dnd_last_window_is_frame: bool = false;
  pub static mut x_dnd_last_motif_style: c_int = 0;
  pub static mut x_dnd_selection_timestamp: Time = 0;

  /* C 1344-1356: the mouse rectangle and the pending XdndPosition message.  */
  pub static mut x_dnd_mouse_rect_target: Window = 0;
  pub static mut x_dnd_mouse_rect: XRectangle = unsafe { std::mem::zeroed () };
  pub static mut x_dnd_waiting_for_status_window: Window = 0;
  pub static mut x_dnd_pending_send_position: XEvent = unsafe { std::mem::zeroed () };
  pub static mut x_dnd_pending_send_position_button: bool = false;
  pub static mut x_dnd_pending_send_position_root_x: c_int = 0;
  pub static mut x_dnd_pending_send_position_root_y: c_int = 0;
  pub static mut x_dnd_need_send_drop: bool = false;
  pub static mut x_dnd_send_drop_proto: c_int = 0;

  /* C 1384-1407: the chosen action and the selection target list.  */
  pub static mut x_dnd_action: Atom = 0;
  pub static mut x_dnd_action_symbol: Lisp_Object = lisp_h_Qnil ();
  pub static mut x_dnd_wanted_action: Atom = 0;
  pub static mut x_dnd_motif_operations: u8 = 0;
  pub static mut x_dnd_first_motif_operation: u8 = 0;
  pub static mut x_dnd_targets: *mut Atom = std::ptr::null_mut ();
  pub static mut x_dnd_n_targets: c_int = 0;

  /* C 1413-1439: the root window's saved attributes and the movement/wheel state.  */
  pub static mut x_dnd_old_window_attrs: XWindowAttributes = unsafe { std::mem::zeroed () };
  pub static mut x_dnd_unwind_flag: bool = false;
  pub static mut x_dnd_movement_frame: *mut frame = std::ptr::null_mut ();
  pub static mut x_dnd_movement_x: c_int = 0;
  pub static mut x_dnd_movement_y: c_int = 0;
  pub static mut x_dnd_wheel_frame: *mut frame = std::ptr::null_mut ();
  pub static mut x_dnd_wheel_x: c_int = 0;
  pub static mut x_dnd_wheel_y: c_int = 0;
  pub static mut x_dnd_wheel_button: c_int = 0;
  pub static mut x_dnd_wheel_state: c_int = 0;
  pub static mut x_dnd_wheel_time: Time = 0;

  /* C 1441-1444: `#ifdef HAVE_XKB` - the keyboard state during the drag.  */
  #[cfg (feature = "have_xkb")]
  pub static mut x_dnd_keyboard_state: c_uint = 0;

  /* C 1446-1467: the IO-error escape, the recursion depth and the tooltip position.  */
  pub static mut x_dnd_disconnect_handler: sigjmp_buf = [0; 196];
  pub static mut x_dnd_inside_handle_one_xevent: bool = false;
  pub static mut x_dnd_recursion_depth: c_int = 0;
  pub static mut x_dnd_selection_alias_cell: Lisp_Object = lisp_h_Qnil ();
  pub static mut x_dnd_last_tooltip_x: c_int = 0;
  pub static mut x_dnd_last_tooltip_y: c_int = 0;
  pub static mut x_dnd_last_tooltip_valid: bool = false;

  /* C 1469-1476: `#ifdef HAVE_XINPUT2` - the devices doing the drag.  */
  #[cfg (feature = "have_xinput2")]
  pub static mut x_dnd_pointer_device: c_int = 0;
  #[cfg (feature = "have_xinput2")]
  pub static mut x_dnd_keyboard_device: c_int = 0;

  /* C 1535-1548: the stacking-order list, and the Motif drag window's error flag.  */
  pub static mut x_dnd_toplevels: *mut x_client_list_window = std::ptr::null_mut ();
  pub static mut x_dnd_use_toplevels: bool = false;
  pub static mut xm_drag_window_error: *mut bool = std::ptr::null_mut ();
}
#[cfg (feature = "have_x_windows")]
use xterm_globals::*;

/* C 1132-1137: `enum { X_EVENT_NORMAL, X_EVENT_GOTO_OUT, X_EVENT_DROP };` - an anonymous enum,
   so the port writes the three constants (PORTING.md 1.3); `current_finish` holds them.  */
#[cfg (feature = "have_x_windows")]
pub const X_EVENT_NORMAL: c_int = 0;
#[cfg (feature = "have_x_windows")]
pub const X_EVENT_GOTO_OUT: c_int = 1;
#[cfg (feature = "have_x_windows")]
pub const X_EVENT_DROP: c_int = 2;

/* C 1139-1142 and 1144-1160: the XEmbed protocol's flag and message numbers.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xembed_info { XEMBED_MAPPED = 1 }

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xembed_message
{
  XEMBED_EMBEDDED_NOTIFY = 0,
  XEMBED_WINDOW_ACTIVATE = 1,
  XEMBED_WINDOW_DEACTIVATE = 2,
  XEMBED_REQUEST_FOCUS = 3,
  XEMBED_FOCUS_IN = 4,
  XEMBED_FOCUS_OUT = 5,
  XEMBED_FOCUS_NEXT = 6,
  XEMBED_FOCUS_PREV = 7,
  XEMBED_MODALITY_ON = 10,
  XEMBED_MODALITY_OFF = 11,
  XEMBED_REGISTER_ACCELERATOR = 12,
  XEMBED_UNREGISTER_ACCELERATOR = 13,
  XEMBED_ACTIVATE_ACCELERATOR = 14,
}

/* C 1162-1210: the file's own forward declarations of its static functions -
   `x_alloc_nearest_color_1` (1162), `x_raise_frame`/`x_lower_frame` (1163-1164),
   `x_io_error_quitter` (1165), `x_create_terminal` (1166), `x_frame_rehighlight` (1167),
   `x_clip_to_row` (1169), `x_window_to_scroll_bar` (1171), `x_window_to_frame` (1172),
   `x_scroll_bar_report_motion`/`x_horizontal_scroll_bar_report_motion` (1173-1180),
   `x_handle_net_wm_state` (1181), `x_check_fullscreen` (1182), `x_check_expected_move`
   (1183), `x_sync_with_move` (1184), `handle_one_xevent` (1185), `x_dispatch_event` (1189,
   `#if ! (defined USE_X_TOOLKIT || defined USE_MOTIF) && defined USE_GTK`),
   `x_wm_set_window_state`/`x_wm_set_icon_pixmap`/`x_initialize` (1191-1193),
   `x_get_current_wm_state` (1195), `x_update_opaque_region` (1196), `x_filter_event` (1199,
   `#ifdef HAVE_X_I18N`), `x_clean_failable_requests` (1201), `x_tooltip_window_to_frame`
   (1203), `x_get_window_below` (1205), `x_set_input_focus` (1206) and `x_scroll_bar_redraw`
   (1209, `#ifndef USE_TOOLKIT_SCROLL_BARS`).  Rust has no prototypes and resolves items by
   name, so the port writes each definition where C writes it and nothing stands here.  */

/* C 1478-1533: a window that can be the target of a drag-and-drop operation.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct x_client_list_window
{
  /// C 1483: `Window window;`
  pub window: Window,
  /// C 1486: `Display *dpy;`
  pub dpy: *mut c_void,
  /// C 1489: `int x, y;`
  pub x: c_int,
  pub y: c_int,
  /// C 1492: `int width, height;`
  pub width: c_int,
  pub height: c_int,
  /// C 1497: `long previous_event_mask;`
  pub previous_event_mask: c_long,
  /// C 1500: `unsigned long wm_state;`
  pub wm_state: c_ulong,
  /// C 1503: `struct x_client_list_window *next;`
  pub next: *mut x_client_list_window,
  /// C 1506-1509: the frame window's extents.
  pub frame_extents_left: c_int,
  pub frame_extents_right: c_int,
  pub frame_extents_top: c_int,
  pub frame_extents_bottom: c_int,
  /// C 1511-1526: `#ifdef HAVE_XSHAPE` - the input and bounding shapes.
  #[cfg (feature = "have_xshape")]
  pub border_width: c_int,
  #[cfg (feature = "have_xshape")]
  pub n_input_rects: c_int,
  #[cfg (feature = "have_xshape")]
  pub input_rects: *mut XRectangle,
  #[cfg (feature = "have_xshape")]
  pub bounding_rects: *mut XRectangle,
  #[cfg (feature = "have_xshape")]
  pub n_bounding_rects: c_int,
  /// C 1529: `uint8_t xm_protocol_style;`
  pub xm_protocol_style: u8,
  /// C 1532: `bool mapped_p;`
  pub mapped_p: bool,
}

/* C 1550-1559: the byte order of a Motif message.  C *stores a wire byte* in it before it
   compares (`xm_read_targets_table_header`, C 1808), so it is a byte holder rather than a
   closed set, and the port models it as the integer type C's enum is stored as, with the
   members as constants - the `dispextern.rs` `glyph_row_area` shape (`pub type ...
   = c_int` plus `pub const ...`).  Two enum variants with one discriminant would not even be
   expressible (E0081): on this target WORDS_BIGENDIAN is undef, so C's
   `XM_BYTE_ORDER_CUR_FIRST` has the same *value* as `XM_BYTE_ORDER_LSB_FIRST`.  */
#[cfg (feature = "have_x_windows")]
pub type xm_byte_order = c_int;

#[cfg (feature = "have_x_windows")]
pub const XM_BYTE_ORDER_LSB_FIRST: xm_byte_order = b'l' as c_int;   // C 1552: `= 'l'`
#[cfg (feature = "have_x_windows")]
pub const XM_BYTE_ORDER_MSB_FIRST: xm_byte_order = b'B' as c_int;   // C 1553: `= 'B'`

/// C 1554-1558: `#ifndef WORDS_BIGENDIAN XM_BYTE_ORDER_CUR_FIRST = 'l' #else ... = 'B' #endif`.
#[cfg (all (feature = "have_x_windows", not (feature = "words_bigendian")))]
pub const XM_BYTE_ORDER_CUR_FIRST: xm_byte_order = XM_BYTE_ORDER_LSB_FIRST;
#[cfg (all (feature = "have_x_windows", feature = "words_bigendian"))]
pub const XM_BYTE_ORDER_CUR_FIRST: xm_byte_order = XM_BYTE_ORDER_MSB_FIRST;

/* C 1561-1585: `SWAPCARD32`/`SWAPCARD16` in two arms.  Under `ENABLE_CHECKING` (undef here)
   C swaps the bytes through a bitfield struct; the port has no bitfields, so that arm swaps
   the same bytes through a `u32`/`u16` and a `c_char` pointer, and the frozen target takes
   the `#else` arm, which is gnulib's `bswap_32`/`bswap_16`.  */
/* The frozen arm comes first on purpose: `tools/extract-abi.py`'s `macro_body_of` answers for a
   name with the *first* `macro_rules!` it finds, and the arm C compiles here is the `#else` one.  */
#[cfg (all (feature = "have_x_windows", not (feature = "enable_checking")))]
macro_rules! SWAPCARD32 { ($l:expr) => { $l = bswap_32 ($l) }; }
#[cfg (all (feature = "have_x_windows", not (feature = "enable_checking")))]
macro_rules! SWAPCARD16 { ($l:expr) => { $l = bswap_16 ($l) }; }
#[cfg (all (feature = "have_x_windows", feature = "enable_checking"))]
macro_rules! SWAPCARD32 {
  ($l:expr) => {{
    let mut bit32: c_uint = $l;
    let tp: *mut c_char = (&raw mut bit32).cast::<c_char> ();
    let mut n: c_char;
    unsafe {
      n = *tp; *tp = *tp.add (3); *tp.add (3) = n;
      n = *tp.add (1); *tp.add (1) = *tp.add (2); *tp.add (2) = n;
    }
    $l = bit32;
  }};
}
#[cfg (all (feature = "have_x_windows", feature = "enable_checking"))]
macro_rules! SWAPCARD16 {
  ($s:expr) => {{
    let mut bit16: c_uint = $s;
    let tp: *mut c_char = (&raw mut bit16).cast::<c_char> ();
    let mut n: c_char;
    unsafe { n = *tp; *tp = *tp.add (1); *tp.add (1) = n; }
    $s = bit16;
  }};
}

/* C 1587-1685: the Motif drag-and-drop message layouts.  `BYTE`/`CARD16`/`CARD32` are the
   protocol's own widths, and `FLEXIBLE_ARRAY_MEMBER` (C 1599) is the port's `[u32; 0]`
   (PORTING.md P-40).  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_targets_table_header
{
  /// C 1589-1590: `uint8_t byte_order; uint8_t protocol;`
  pub byte_order: u8,
  pub protocol: u8,
  /// C 1592-1593.
  pub target_list_count: u16,
  pub total_data_size: u32,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_targets_table_rec
{
  /// C 1598: `uint16_t n_targets;`
  pub n_targets: u16,
  /// C 1599: `uint32_t targets[FLEXIBLE_ARRAY_MEMBER];`
  pub targets: [u32; 0],
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_drop_start_message
{
  /// C 1604-1611.
  pub reason: u8,
  pub byte_order: u8,
  pub side_effects: u16,
  pub timestamp: u32,
  pub x: u16,
  pub y: u16,
  pub index_atom: u32,
  pub source_window: u32,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_drop_start_reply
{
  /// C 1616-1621.
  pub reason: u8,
  pub byte_order: u8,
  pub side_effects: u16,
  pub better_x: u16,
  pub better_y: u16,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_drag_initiator_info
{
  /// C 1626-1630.
  pub byteorder: u8,
  pub protocol: u8,
  pub table_index: u16,
  pub selection: u32,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_drag_receiver_info
{
  /// C 1635-1642.
  pub byteorder: u8,
  pub protocol: u8,
  pub protocol_style: u8,
  pub unspecified0: u8,
  pub unspecified1: u32,
  pub unspecified2: u32,
  pub unspecified3: u32,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_top_level_enter_message
{
  /// C 1647-1653.
  pub reason: u8,
  pub byteorder: u8,
  pub zero: u16,
  pub timestamp: u32,
  pub source_window: u32,
  pub index_atom: u32,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_drag_motion_message
{
  /// C 1658-1663.
  pub reason: u8,
  pub byteorder: u8,
  pub side_effects: u16,
  pub timestamp: u32,
  pub x: u16,
  pub y: u16,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_drag_motion_reply
{
  /// C 1668-1674.
  pub reason: u8,
  pub byte_order: u8,
  pub side_effects: u16,
  pub timestamp: u32,
  pub better_x: u16,
  pub better_y: u16,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct xm_top_level_leave_message
{
  /// C 1679-1684.
  pub reason: u8,
  pub byteorder: u8,
  pub zero: u16,
  pub timestamp: u32,
  pub source_window: u32,
}

/* C 1687-1761: the Motif drag protocol's bit packers.  C 1694's
   `XM_DRAG_SIDE_EFFECT_OPERATIONS` is commented out in the reference, so it is not ported.  */
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_SIDE_EFFECT {
  ($op:expr, $site:expr, $ops:expr, $act:expr) =>
    { (($op) as c_int | (($site) as c_int) << 4 | (($ops) as c_int) << 8
       | (($act) as c_int) << 12) };
}
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_SIDE_EFFECT_OPERATION { ($effect:expr) => { (($effect) as c_int & 0xf) }; }
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_SIDE_EFFECT_SITE_STATUS {
  ($effect:expr) => { ((($effect) as c_int & 0xf0) >> 4) };
}
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_SIDE_EFFECT_DROP_ACTION {
  ($effect:expr) => { ((($effect) as c_int & 0xf000) >> 12) };
}

/* C 1697-1704: the Motif drag operations.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xm_drag_operation
{
  XM_DRAG_NOOP = 0,
  XM_DRAG_MOVE = 1,
  XM_DRAG_COPY = 2,
  XM_DRAG_LINK = 4,
  XM_DRAG_LINK_REC = 3,
}

#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_OPERATION_IS_LINK {
  ($op:expr) => { (($op) == xm_drag_operation::XM_DRAG_LINK as c_int
                  || ($op) == xm_drag_operation::XM_DRAG_LINK_REC as c_int) };
}

/* C 1709-1714: what the drop target chose to do.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xm_drag_action
{
  XM_DROP_ACTION_DROP = 0,
  XM_DROP_ACTION_DROP_HELP = 1,
  XM_DROP_ACTION_DROP_CANCEL = 2,
}

#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_REASON {
  ($originator:expr, $code:expr) => { (($code) as c_int | (($originator) as c_int) << 7) };
}
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_REASON_ORIGINATOR {
  ($reason:expr) => { if (($reason) as c_int & 0x80) != 0 { 1 } else { 0 } };
}
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_REASON_CODE { ($reason:expr) => { (($reason) as c_int & 0x7f) }; }

/* C 1720-1732: which end of the protocol sent a message.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xm_drag_reason
{
  XM_DRAG_REASON_DROP_START = 5,
  XM_DRAG_REASON_TOP_LEVEL_ENTER = 0,
  XM_DRAG_REASON_TOP_LEVEL_LEAVE = 1,
  XM_DRAG_REASON_DRAG_MOTION = 2,
}

#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xm_drag_originator
{
  XM_DRAG_ORIGINATOR_INITIATOR = 0,
  XM_DRAG_ORIGINATOR_RECEIVER = 1,
}

/* C 1734-1744: the protocol style a receiver advertises.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xm_drag_style
{
  XM_DRAG_STYLE_NONE = 0,
  XM_DRAG_STYLE_DROP_ONLY = 1,
  XM_DRAG_STYLE_DROP_ONLY_REC = 3,
  XM_DRAG_STYLE_DYNAMIC = 5,
  XM_DRAG_STYLE_DYNAMIC_REC = 2,
  XM_DRAG_STYLE_DYNAMIC_REC1 = 4,
}

#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_STYLE_IS_DROP_ONLY {
  ($n:expr) => { (($n) == xm_drag_style::XM_DRAG_STYLE_DROP_ONLY as c_int
                  || ($n) == xm_drag_style::XM_DRAG_STYLE_DROP_ONLY_REC as c_int) };
}
#[cfg (feature = "have_x_windows")]
macro_rules! XM_DRAG_STYLE_IS_DYNAMIC {
  ($n:expr) => { (($n) == xm_drag_style::XM_DRAG_STYLE_DYNAMIC as c_int
                  || ($n) == xm_drag_style::XM_DRAG_STYLE_DYNAMIC_REC as c_int
                  || ($n) == xm_drag_style::XM_DRAG_STYLE_DYNAMIC_REC1 as c_int) };
}

/* C 1752-1757: the drop site's verdict.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub enum xm_drop_site_status
{
  XM_DROP_SITE_VALID = 3,
  XM_DROP_SITE_INVALID = 2,
  XM_DROP_SITE_NONE = 1,
}

/* C 1759-1761: `#define XM_DRAG_PROTOCOL_VERSION 0` - a value, so the port writes it as the
   constant the extractor reads it as (the `STANDARD_EVENT_SET` shape in `include/xterm.rs`).  */
#[cfg (feature = "have_x_windows")]
pub const XM_DRAG_PROTOCOL_VERSION: c_int = 0;

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

/* ---- Slice 3: C 1763-2173 - the Motif side-effect mapping, the targets-table reader and
   writer, the two error handlers, `x_special_window_exists_p` and the drag window.  Xlib's own
   names stay bare (`XChangeProperty`, `None`, `Success`, `XA_WINDOW`, `PropModeReplace`,
   `CopyFromParent`, `InputOnly`, `RetainPermanent`, `DestroyAll`), exactly as this file's own
   header mirror keeps `KeyPressMask`; they resolve on an X11 target and nowhere here.  The
   three libc entry points C uses through its own headers (`qsort`, `sigsetjmp`, `siglongjmp`)
   are declared below - measured, not assumed: a program calling all three links, and `nm -u`
   shows `_qsort`/`_sigsetjmp`/`_siglongjmp` (they are functions, not macros).  */

/* C 1763-1776: the Motif side effect for an XDND action atom.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_side_effect_from_action (dpyinfo: *mut x_display_info, action: Atom) -> u8
{
  if action == (*dpyinfo).Xatom_XdndActionCopy
    { return xm_drag_operation::XM_DRAG_COPY as u8; }
  else if action == (*dpyinfo).Xatom_XdndActionMove
    { return xm_drag_operation::XM_DRAG_MOVE as u8; }
  else if action == (*dpyinfo).Xatom_XdndActionLink
    { return xm_drag_operation::XM_DRAG_LINK as u8; }
  else if action == (*dpyinfo).Xatom_XdndActionAsk
    { return x_dnd_first_motif_operation; }

  return xm_drag_operation::XM_DRAG_NOOP as u8;
}

/* C 1778-1798.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_operations_from_actions (dpyinfo: *mut x_display_info,
				      ask_actions: *mut Atom, n_ask_actions: c_int) -> u8
{
  let mut i: c_int;
  let mut flags: u8;

  flags = 0;

  for i in 0..n_ask_actions
    {
      if *ask_actions.offset (i as isize) == (*dpyinfo).Xatom_XdndActionCopy
	{ flags |= xm_drag_operation::XM_DRAG_COPY as u8; }
      else if *ask_actions.offset (i as isize) == (*dpyinfo).Xatom_XdndActionMove
	{ flags |= xm_drag_operation::XM_DRAG_MOVE as u8; }
      else if *ask_actions.offset (i as isize) == (*dpyinfo).Xatom_XdndActionLink
	{ flags |= xm_drag_operation::XM_DRAG_LINK as u8; }
    }

  return flags;
}

/* C 1800-1823: read the table header out of a wire buffer, in place.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_targets_table_header (mut bytes: *mut u8, length: ptrdiff_t,
					header_return: *mut xm_targets_table_header,
					byteorder_return: *mut xm_byte_order) -> c_int
{
  if length < 8
    { return -1; }

  /* C 1808 chains one assignment (`header_return->byte_order = *byteorder_return = *(bytes++);`);
     Rust has no assignment *expression*, so the value is bound once and stored twice.  */
  let byte_order: xm_byte_order = *bytes as xm_byte_order;
  (*header_return).byte_order = byte_order as u8;
  *byteorder_return = byte_order;
  bytes = bytes.add (1);
  (*header_return).protocol = *bytes;
  bytes = bytes.add (1);

  (*header_return).target_list_count = *(bytes as *mut u16);
  (*header_return).total_data_size = *(bytes.add (2) as *mut u32);

  if (*header_return).byte_order as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
    {
      SWAPCARD16! ((*header_return).target_list_count);
      SWAPCARD32! ((*header_return).total_data_size);
    }

  (*header_return).byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;

  return 8;
}

/* C 1825-1856: read one target list.  The record's `targets` member is the flexible array C
   1599 declares, so its elements are reached through the record's own address (P-40).  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_targets_table_rec (bytes: *mut u8, length: ptrdiff_t,
				     byteorder: xm_byte_order) -> *mut xm_targets_table_rec
{
  let mut nitems: u16;
  let mut i: u16;
  let mut rec: *mut xm_targets_table_rec;

  if length < 2
    { return std::ptr::null_mut (); }

  nitems = *(bytes as *mut u16);

  if byteorder != XM_BYTE_ORDER_CUR_FIRST
    { SWAPCARD16! (nitems); }

  if length < 2 + (nitems as ptrdiff_t) * 4
    { return std::ptr::null_mut (); }

  rec = xmalloc (FLEXSIZEOF! (xm_targets_table_rec, targets, (nitems as usize) * 4))
	  as *mut xm_targets_table_rec;
  (*rec).n_targets = nitems;

  for i in 0..nitems
    {
      FAM_ELEM! ((*rec).targets, i as isize)
	= *(bytes.add (2) as *mut u32).add (i as usize);

      if byteorder != XM_BYTE_ORDER_CUR_FIRST
	{ SWAPCARD32! (FAM_ELEM! ((*rec).targets, i as isize)); }
    }

  return rec;
}

/* C 1858-1881: find the record whose list matches `sorted_targets`.  C's `alloca` is the
   port's stack pool (P-05; `bignum.rs` 428 is the shape).  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_find_targets_table_idx (header: *mut xm_targets_table_header,
				     recs: *mut *mut xm_targets_table_rec,
				     sorted_targets: *mut Atom, ntargets: c_int) -> c_int
{
  let mut j: c_int;
  let mut i: u16;
  let mut targets: *mut u32;

  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  targets = SAFE_ALLOCA! (sa_avail, std::mem::size_of::<u32> () * ntargets as usize) as *mut u32;

  for j in 0..ntargets
    { *targets.offset (j as isize) = *sorted_targets.offset (j as isize) as u32; }

  for i in 0..(*header).target_list_count
    {
      if (*recs.offset (i as isize)).is_null ()
	{ break; }
      if (*(*recs.offset (i as isize))).n_targets == ntargets as u16
	  && memcmp (core::ptr::addr_of! (FAM_ELEM! ((*(*recs.offset (i as isize))).targets, 0))
		       as *const c_void,
		     targets as *const c_void,
		     std::mem::size_of::<u32> () * ntargets as usize) == 0
	{ SAFE_FREE! (sa_count); return i as c_int; }
    }

  SAFE_FREE! (sa_count);
  return -1;
}

/* C 1883-1887: the comparator `qsort` sorts the target atoms with.  C's subtraction is an
   `Atom` (an unsigned long) one, so the port does it at that width and only then narrows.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_atoms_compare (a: *const c_void, b: *const c_void) -> c_int
{
  return (*(a as *const Atom)).wrapping_sub (*(b as *const Atom)) as c_int;
}

/* C 1889-1934: write the whole targets table to the drag window.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_write_targets_table (dpy: *mut c_void, wdesc: Window,
				  targets_table_atom: Atom,
				  header: *mut xm_targets_table_header,
				  recs: *mut *mut xm_targets_table_rec)
{
  use crate::x11_constants::{PropModeAppend, PropModeReplace};

  let mut header_buffer: *mut u8;
  let mut ptr: *mut u8;
  let mut rec_buffer: *mut u8;
  let mut rec_buffer_size: ptrdiff_t;
  let mut i: u16;
  let mut j: u16;

  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  header_buffer = SAFE_ALLOCA! (sa_avail, 8) as *mut u8;
  ptr = header_buffer;

  *header_buffer = (*header).byte_order;
  header_buffer = header_buffer.add (1);
  *header_buffer = (*header).protocol;
  header_buffer = header_buffer.add (1);
  *(header_buffer as *mut u16) = (*header).target_list_count;
  *(header_buffer.add (2) as *mut u32) = (*header).total_data_size;

  rec_buffer = xmalloc (600) as *mut u8;
  rec_buffer_size = 600;

  XChangeProperty (dpy, wdesc, targets_table_atom,
		   targets_table_atom, 8, PropModeReplace,
		   ptr, 8);

  for i in 0..(*header).target_list_count
    {
      if rec_buffer_size < 2 + (*(*recs.offset (i as isize))).n_targets as ptrdiff_t * 4
	{
	  rec_buffer_size = 2 + (*(*recs.offset (i as isize))).n_targets as ptrdiff_t * 4;
	  rec_buffer = xrealloc (rec_buffer as *mut c_void, rec_buffer_size as usize) as *mut u8;
	}

      *(rec_buffer as *mut u16) = (*(*recs.offset (i as isize))).n_targets;

      for j in 0..(*(*recs.offset (i as isize))).n_targets
	{
	  *(rec_buffer.add (2) as *mut u32).add (j as usize)
	    = FAM_ELEM! ((*(*recs.offset (i as isize))).targets, j as isize);
	}

      XChangeProperty (dpy, wdesc, targets_table_atom,
		       targets_table_atom, 8, PropModeAppend,
		       rec_buffer,
		       2 + (*(*recs.offset (i as isize))).n_targets as c_int * 4);
    }

  xfree (rec_buffer as *mut c_void);
  SAFE_FREE! (sa_count);
}

/* C 1936-1958.  C 1950 swaps a CARD32 member with `SWAPCARD16`, which in C narrows to the
   low 16 bits through gnulib's `bswap_16` and stores them back; the port keeps that
   arithmetic instead of "fixing" the reference.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_write_drag_initiator_info (dpy: *mut c_void, wdesc: Window,
					prop_name: Atom, type_name: Atom,
					info: *mut xm_drag_initiator_info)
{
  use crate::x11_constants::PropModeReplace;

  let mut buf: *mut u8;

  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  buf = SAFE_ALLOCA! (sa_avail, 8) as *mut u8;
  /* C 1944-1945's `buf[0]`/`buf[1]` keep their subscripts, so the numbers stay in the body. */
  *buf.offset (0) = (*info).byteorder;
  *buf.offset (1) = (*info).protocol;

  if (*info).byteorder as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
    {
      SWAPCARD16! ((*info).table_index);
      let mut low: u16 = (*info).selection as u16;
      SWAPCARD16! (low);
      (*info).selection = ((*info).selection & !0xffff) | low as u32;
    }

  *(buf.add (2) as *mut u16) = (*info).table_index;
  *(buf.add (4) as *mut u32) = (*info).selection;

  XChangeProperty (dpy, wdesc, prop_name, type_name, 8,
		   PropModeReplace, buf, 8);
  SAFE_FREE! (sa_count);
}

/* C 1960-1967: the X error handler that records that the drag window is bad.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xm_drag_window_error_handler (display: *mut c_void,
						  event: *mut c_void) -> c_int
{
  if !xm_drag_window_error.is_null ()
    { *xm_drag_window_error = true; }

  return 0;
}

/* C 1969-1975: C declares it `_Noreturn int`; the port keeps C's `int` return and its `_Noreturn`
   tail (`siglongjmp` is the libc function declared above, and it is a diverging expression), because
   `XSetIOErrorHandler`'s parameter is the mirror's `Emacs_XIOErrorHandler`
   (`Option<unsafe extern "C" fn (*mut c_void) -> c_int>`, unit 701) - the Slice-14 reading named
   the `-> !` spelling as an `E0308` at C 26783/unit 1803.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xm_drag_window_io_error_handler (dpy: *mut c_void) -> c_int
{
  siglongjmp (&raw mut x_dnd_disconnect_handler, 1);
  /* C declares it `_Noreturn`; `siglongjmp` never returns, so the port gives Rust's `-> !`
     the divergence it needs (`lread.rs` 720's shape).  */
  unreachable! ()
}

/* C 1977-1992: does WINDOW exist on DPYINFO?  Selecting for input from it is what asks.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_special_window_exists_p (dpyinfo: *mut x_display_info, window: Window) -> bool
{
  use crate::x11_constants::StructureNotifyMask;

  let mut rc: bool;

  x_catch_errors ((*dpyinfo).display);
  XSelectInput ((*dpyinfo).display, window,
		StructureNotifyMask);
  rc = !x_had_errors_p ((*dpyinfo).display);
  x_uncatch_errors_after_check ();

  return rc;
}

/* C 2014-2163: create the Motif drag window, or return the one already recorded.  C's comment
   at 1994-2012 is the protocol; the port keeps the statements in C's order.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_get_drag_window_1 (dpyinfo: *mut x_display_info) -> Window
{
  use crate::x11_constants::{CopyFromParent, CWOverrideRedirect, DestroyAll, InputOnly,
                             NoEventMask, None, PropModeReplace, RetainPermanent, Success,
                             XA_WINDOW};

  let mut actual_type: Atom;
  let mut rc: c_int;
  let mut actual_format: c_int;
  let mut nitems: c_ulong;
  let mut bytes_remaining: c_ulong;
  let mut tmp_data: *mut u8 = std::ptr::null_mut ();
  let mut drag_window: Window;
  let mut attrs: XSetWindowAttributes = std::mem::zeroed ();
  let mut temp_display: *mut c_void;
  let mut old_handler: Emacs_XErrorHandler;
  let mut old_io_handler: Emacs_XIOErrorHandler;

  /* This is volatile because GCC mistakenly warns about them being clobbered by longjmp.  */
  let mut error: bool;

  drag_window = None as Window;
  rc = (XGetWindowProperty ((*dpyinfo).display, (*dpyinfo).root_window,
			    (*dpyinfo).Xatom_MOTIF_DRAG_WINDOW,
			    0, 1, false, XA_WINDOW, &raw mut actual_type,
			    &raw mut actual_format, &raw mut nitems, &raw mut bytes_remaining,
			    &raw mut tmp_data) == Success) as c_int;

  if rc != 0 && actual_type == XA_WINDOW
      && actual_format == 32 && nitems == 1
      && !tmp_data.is_null ()
    {
      drag_window = *(tmp_data as *mut Window);

      /* This has the side effect of selecting for StructureNotifyMask, meaning that we
	 will get notifications once it is deleted.  */
      rc = x_special_window_exists_p (dpyinfo, drag_window) as c_int;

      if rc == 0
	{ drag_window = None as Window; }
    }

  if !tmp_data.is_null ()
    { XFree (tmp_data as *mut c_void); }

  if drag_window == None as Window
    {
      block_input ();
      old_io_handler = XSetIOErrorHandler (Some (xm_drag_window_io_error_handler));

      if sigsetjmp (&raw mut x_dnd_disconnect_handler, 1) != 0
	{
	  XSetIOErrorHandler (old_io_handler);
	  unblock_input ();

	  return None as Window;
	}

      unrequest_sigio ();
      temp_display = XOpenDisplay (XDisplayString ((*dpyinfo).display));
      request_sigio ();

      if temp_display.is_null ()
	{
	  XSetIOErrorHandler (old_io_handler);
	  unblock_input ();

	  return None as Window;
	}

      error = false;
      xm_drag_window_error = &raw mut error;

      XSetCloseDownMode (temp_display, RetainPermanent);
      old_handler = XSetErrorHandler (Some (xm_drag_window_error_handler));

      attrs.override_redirect = true;
      drag_window = XCreateWindow (temp_display, DefaultRootWindow (temp_display),
				   -1, -1, 1, 1, 0, CopyFromParent, InputOnly,
				   CopyFromParent, CWOverrideRedirect, &raw mut attrs);

      /* Handle all errors now.  */
      XSync (temp_display as *mut Display, 0);

      /* Some part of the drag window creation process failed, so punt.  Release all
	 resources too.  */
      if error
	{
	  XSetCloseDownMode (temp_display, DestroyAll);
	  drag_window = None as Window;
	}

      xm_drag_window_error = std::ptr::null_mut ();

      /* FIXME: why does XCloseDisplay hang if SIGIO arrives and there are multiple
	 displays?  */
      unrequest_sigio ();
      XCloseDisplay (temp_display);
      request_sigio ();

      XSetErrorHandler (old_handler);
      XSetIOErrorHandler (old_io_handler);

      /* Make sure the drag window created is actually valid for the current display, and
	 the XOpenDisplay above did not accidentally connect to some other display.  */
      if !x_special_window_exists_p (dpyinfo, drag_window)
	{ drag_window = None as Window; }
      unblock_input ();

      if drag_window != None as Window
	{
	  XGrabServer ((*dpyinfo).display);

	  x_catch_errors ((*dpyinfo).display);
	  tmp_data = std::ptr::null_mut ();

	  rc = (XGetWindowProperty ((*dpyinfo).display, (*dpyinfo).root_window,
				    (*dpyinfo).Xatom_MOTIF_DRAG_WINDOW,
				    0, 1, false, XA_WINDOW, &raw mut actual_type,
				    &raw mut actual_format, &raw mut nitems,
				    &raw mut bytes_remaining,
				    &raw mut tmp_data) == Success) as c_int;

	  if rc != 0 && actual_type == XA_WINDOW
	      && actual_format == 32 && nitems == 1
	      && !tmp_data.is_null ()
	      && x_special_window_exists_p (dpyinfo, *(tmp_data as *mut Window))
	    {
	      /* Kill the client now to avoid leaking a client slot, which is a limited
		 resource.  */
	      XKillClient ((*dpyinfo).display, drag_window);
	      drag_window = *(tmp_data as *mut Window);
	    }
	  else
	    { XChangeProperty ((*dpyinfo).display, (*dpyinfo).root_window,
			       (*dpyinfo).Xatom_MOTIF_DRAG_WINDOW,
			       XA_WINDOW, 32, PropModeReplace,
			       &raw mut drag_window as *mut u8, 1); }

	  if !tmp_data.is_null ()
	    { XFree (tmp_data as *mut c_void); }

	  if x_had_errors_p ((*dpyinfo).display)
	    { drag_window = None as Window; }
	  x_uncatch_errors ();

	  XUngrabServer ((*dpyinfo).display);
	}
    }

  return drag_window;
}

/* C 2165-2173.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_get_drag_window (dpyinfo: *mut x_display_info) -> Window
{
  use crate::x11_constants::None;

  if (*dpyinfo).motif_drag_window != None as Window
    { return (*dpyinfo).motif_drag_window; }

  (*dpyinfo).motif_drag_window = xm_get_drag_window_1 (dpyinfo);
  return (*dpyinfo).motif_drag_window;
}

/* The three libc entry points C reaches through its own headers.  `buffer.rs` 132 declares
   `qsort` the same way (the port's `stdlib.rs` mirror does not carry it); `sigsetjmp` and
   `siglongjmp` are what C calls directly at C 2061/1974, and C's own `sys_setjmp` spelling
   (`_setjmp`) does *not* save the signal mask, which is the whole point of C's call here.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: <stdlib.h> 489: `extern void qsort (void *__base, size_t __nmemb, size_t __size,
  /// __compar_fn_t __compar);`
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
		compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
  /// C: <setjmp.h>: `int sigsetjmp (sigjmp_buf, int);`
  pub fn sigsetjmp (env: *mut sigjmp_buf, savemask: c_int) -> c_int;
  /// C: <setjmp.h>: `void siglongjmp (sigjmp_buf, int);`
  pub fn siglongjmp (env: *mut sigjmp_buf, val: c_int);
}

/* ---- Slice 4: C 2175-2612 - the targets-table setup, the drag atom, and the three Motif
   message senders.  X11's macros come from the mirror's `x11_constants` module, imported per
   function (a top-level name would shadow `Option::None`); the prose above records why.  */

/* C 2175-2391: make the drag window's targets table hold TARGETS.  C's `goto
   retry_drag_window` (2192/2224) is the port's labelled `loop` (PORTING.md 2.2; `editfns.rs`'s
   `'retry` is the shape), and C 2316-2319's four `ckd_*` calls keep their short circuit: the
   first is eager and the rest sit in block expressions after `||`, so a store that C would
   never reach does not happen here either (`ccl.rs` 2913 records the same shape).  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_setup_dnd_targets (dpyinfo: *mut x_display_info,
				targets: *mut Atom, ntargets: c_int) -> c_int
{
  use crate::x11_constants::{None, Success};

  let mut drag_window: Window;
  let mut targets_sorted: *mut Atom;
  let mut actual_type: Atom;
  let mut tmp_data: *mut u8 = std::ptr::null_mut ();
  let mut nitems: c_ulong;
  let mut bytes_remaining: c_ulong;
  let mut rc: c_int;
  let mut actual_format: c_int;
  let mut idx: c_int;
  let mut had_errors: bool;
  let mut header: xm_targets_table_header;
  /* C 2186: `xm_targets_table_rec **recs UNINIT;` - the zero `UNINIT` stands for in the frozen
     configuration (conf_post.h 369's `= {0,}`; the `cons_to_unsigned` note row).  */
  let mut recs: *mut *mut xm_targets_table_rec = std::ptr::null_mut ();
  let mut byteorder: xm_byte_order;
  let mut data: *mut u8;
  let mut total_bytes: ptrdiff_t;
  let mut total_items: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut size: u32;
  let mut target_count: u32;

  'retry_drag_window: loop
    {
      drag_window = xm_get_drag_window (dpyinfo);

      if drag_window == None as Window || ntargets > 64
	{ return -1; }

      targets_sorted = xmalloc (std::mem::size_of::<Atom> () * ntargets as usize) as *mut Atom;
      std::ptr::copy_nonoverlapping (targets as *const Atom, targets_sorted,
				     ntargets as usize);
      qsort (targets_sorted as *mut c_void, ntargets as usize,
	     std::mem::size_of::<Atom> (), Some (x_atoms_compare));

      XGrabServer ((*dpyinfo).display);

      x_catch_errors ((*dpyinfo).display);
      rc = (XGetWindowProperty ((*dpyinfo).display, drag_window,
				(*dpyinfo).Xatom_MOTIF_DRAG_TARGETS,
				0, LONG_MAX, false,
				(*dpyinfo).Xatom_MOTIF_DRAG_TARGETS,
				&raw mut actual_type, &raw mut actual_format,
				&raw mut nitems, &raw mut bytes_remaining,
				&raw mut tmp_data) == Success) as c_int;
      had_errors = x_had_errors_p ((*dpyinfo).display);
      x_uncatch_errors_after_check ();

      /* The drag window is probably invalid, so remove our record of it.  */
      if had_errors
	{
	  (*dpyinfo).motif_drag_window = None as Window;
	  XUngrabServer ((*dpyinfo).display);

	  continue 'retry_drag_window;
	}

      if rc != 0 && !tmp_data.is_null () && bytes_remaining == 0
	  && actual_type == (*dpyinfo).Xatom_MOTIF_DRAG_TARGETS
	  && actual_format == 8
	{
	  data = tmp_data;
	  if xm_read_targets_table_header (tmp_data, nitems as ptrdiff_t, &raw mut header,
					   &raw mut byteorder) == 8
	    {
	      data = data.offset (8);
	      nitems -= 8;
	      total_bytes = 0;
	      total_items = 0;

	      /* The extra rec is used to store a new target list if a preexisting one doesn't
		 already exist.  */
	      recs = xmalloc ((header.target_list_count as usize + 1)
			      * std::mem::size_of::<*mut xm_targets_table_rec> ())
		       as *mut *mut xm_targets_table_rec;

	      while total_items < header.target_list_count as ptrdiff_t
		{
		  *recs.offset (total_items)
		    = xm_read_targets_table_rec (data.offset (total_bytes),
						 nitems as ptrdiff_t, byteorder);

		  if (*recs.offset (total_items)).is_null ()
		    { break; }

		  total_bytes += 2 + (*(*recs.offset (total_items))).n_targets as ptrdiff_t * 4;
		  nitems -= 2 + (*(*recs.offset (total_items))).n_targets as c_ulong * 4;
		  total_items += 1;
		}

	      if header.target_list_count as ptrdiff_t != total_items
		  || header.total_data_size as ptrdiff_t != 8 + total_bytes
		{
		  for i in 0..total_items
		    {
		      if !(*recs.offset (i)).is_null ()
			{ xfree (*recs.offset (i) as *mut c_void); }
		      else
			{ break; }
		    }

		  xfree (recs as *mut c_void);

		  rc = 0;
		}
	    }
	  else
	    { rc = 0; }
	}
      else
	{ rc = 0; }

      if !tmp_data.is_null ()
	{ XFree (tmp_data as *mut c_void); }

      /* Now rc means whether or not the target lists weren't updated and shouldn't be written
	 to the drag window.  */

      if rc == 0
	{
	  header.byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;
	  header.protocol = XM_DRAG_PROTOCOL_VERSION as u8;
	  header.target_list_count = 1;
	  header.total_data_size = (8 + 2 + ntargets * 4) as u32;

	  recs = xmalloc (std::mem::size_of::<*mut xm_targets_table_rec> ())
		   as *mut *mut xm_targets_table_rec;
	  *recs = xmalloc (FLEXSIZEOF! (xm_targets_table_rec, targets,
					ntargets as usize * 4))
		    as *mut xm_targets_table_rec;

	  (*(*recs)).n_targets = ntargets as u16;

	  for i in 0..ntargets as ptrdiff_t
	    {
	      FAM_ELEM! ((*(*recs)).targets, i)
		= *targets_sorted.offset (i) as u32;
	    }

	  idx = 0;
	}
      else
	{
	  idx = xm_find_targets_table_idx (&raw mut header, recs, targets_sorted, ntargets);

	  if idx == -1
	    {
	      target_count = header.target_list_count as u32;
	      rc = 0;

	      let (add_value, add_overflow) = header.target_list_count.overflowing_add (1);
	      header.target_list_count = add_value;
	      if add_overflow
		  || { let (mul_value, mul_overflow) = (ntargets as u32).overflowing_mul (4);
		       size = mul_value; mul_overflow }
		  || { let (add_value, add_overflow)
			 = header.total_data_size.overflowing_add (size);
		       header.total_data_size = add_value; add_overflow }
		  || { let (add_value, add_overflow)
			 = header.total_data_size.overflowing_add (2);
		       header.total_data_size = add_value; add_overflow }
		{
		  /* Overflow, remove every entry from the targets table and add one for our
		     current targets list.  This confuses real Motif but not GTK 2.x, and there
		     is no other choice.  */

		  for i in 0..target_count as ptrdiff_t
		    { xfree (*recs.offset (i) as *mut c_void); }

		  xfree (recs as *mut c_void);

		  header.byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;
		  header.protocol = XM_DRAG_PROTOCOL_VERSION as u8;
		  header.target_list_count = 1;
		  header.total_data_size = (8 + 2 + ntargets * 4) as u32;

		  recs = xmalloc (std::mem::size_of::<*mut xm_targets_table_rec> ())
			   as *mut *mut xm_targets_table_rec;
		  *recs = xmalloc (FLEXSIZEOF! (xm_targets_table_rec, targets,
						ntargets as usize * 4))
			    as *mut xm_targets_table_rec;

		  (*(*recs)).n_targets = ntargets as u16;

		  for i in 0..ntargets as ptrdiff_t
		    {
		      FAM_ELEM! ((*(*recs)).targets, i)
			= *targets_sorted.offset (i) as u32;
		    }

		  idx = 0;
		}
	      else
		{
		  *recs.offset (header.target_list_count as isize - 1)
		    = xmalloc (FLEXSIZEOF! (xm_targets_table_rec, targets,
					    ntargets as usize * 4))
			as *mut xm_targets_table_rec;
		  (*(*recs.offset (header.target_list_count as isize - 1))).n_targets
		    = ntargets as u16;

		  for i in 0..ntargets as ptrdiff_t
		    {
		      FAM_ELEM! ((*(*recs
				     .offset (header.target_list_count as isize - 1))).targets,
				 i) = *targets_sorted.offset (i) as u32;
		    }

		  idx = header.target_list_count as c_int - 1;
		}
	    }
	}

      if rc == 0
	{
	  /* Some implementations of Motif DND set the protocol version of just the targets
	     table to 1 without actually changing the data format.  To avoid confusing Motif
	     when that happens, set it back to 0.  There will probably be no more updates to
	     the protocol either.  */
	  header.protocol = XM_DRAG_PROTOCOL_VERSION as u8;

	  x_catch_errors ((*dpyinfo).display);
	  xm_write_targets_table ((*dpyinfo).display, drag_window,
				  (*dpyinfo).Xatom_MOTIF_DRAG_TARGETS,
				  &raw mut header, recs);
	  /* Presumably we got a BadAlloc upon writing the targets table.  */
	  if x_had_errors_p ((*dpyinfo).display)
	    { idx = -1; }
	  x_uncatch_errors_after_check ();
	}

      XUngrabServer ((*dpyinfo).display);

      for i in 0..header.target_list_count as ptrdiff_t
	{ xfree (*recs.offset (i) as *mut c_void); }

      xfree (recs as *mut c_void);
      xfree (targets_sorted as *mut c_void);

      return idx;
    }
}

/* C 2402-2509: allocate the atom used for the Motif selection.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_get_drag_atom_1 (dpyinfo: *mut x_display_info,
			      source_frame: *mut frame) -> Atom
{
  use crate::x11_constants::{None, PropModeAppend, PropModeReplace, Success, XA_ATOM};

  let mut actual_type: Atom;
  let mut atoms: *mut Atom;
  let mut atom: Atom;
  let mut nitems: c_ulong;
  let mut bytes_remaining: c_ulong;
  let mut tmp_data: *mut u8;
  let mut rc: c_int;
  let mut actual_format: c_int;
  let mut i: c_ulong;
  let mut buffer: *mut c_char;
  let mut owner: Window;

  /* Make sure this operation is done atomically.  */
  XGrabServer ((*dpyinfo).display);

  rc = XGetWindowProperty ((*dpyinfo).display, (*dpyinfo).root_window,
			   (*dpyinfo).Xatom_EMACS_DRAG_ATOM,
			   0, LONG_MAX, false, XA_ATOM, &raw mut actual_type,
			   &raw mut actual_format, &raw mut nitems, &raw mut bytes_remaining,
			   &raw mut tmp_data);
  atom = None as Atom;
  /* GCC thinks i is used uninitialized, but it's always initialized if `atoms' exists at that
     particular spot.  */
  i = 0;

  if rc == Success
      && actual_format == 32 && nitems != 0
      && actual_type == XA_ATOM
    {
      atoms = tmp_data as *mut Atom;

      x_catch_errors ((*dpyinfo).display);

      while i < nitems
	{
	  owner = XGetSelectionOwner ((*dpyinfo).display, *atoms.offset (i as isize));

	  if !x_had_errors_p ((*dpyinfo).display)
	      && (owner == None as Window
		  /* If we already own this selection (even if another frame owns it), use it.
		     There is no way of knowing when ownership was asserted, so it still has to
		     be owned again.  */
		  || !x_window_to_frame (dpyinfo, owner as c_int).is_null ())
	    {
	      atom = *atoms.offset (i as isize);

	      break;
	    }

	  i += 1;
	}

      x_uncatch_errors ();
    }

  if !tmp_data.is_null ()
    { XFree (tmp_data as *mut c_void); }

  buffer = (*dpyinfo).motif_drag_atom_name.as_mut_ptr ();

  if atom != None as Atom
    {
      sprintf (buffer, c"_EMACS_ATOM_%lu".as_ptr (), i + 1);
      XSetSelectionOwner ((*dpyinfo).display, atom,
			  FRAME_X_WINDOW! (source_frame),
			  (*dpyinfo).last_user_time);

      /* The selection's last-change time is newer than our last_user_time, so create a new
	 selection instead.  */
      if XGetSelectionOwner ((*dpyinfo).display, atom) != FRAME_X_WINDOW! (source_frame)
	{ atom = None as Atom; }
    }

  while atom == None as Atom
    {
      sprintf (buffer, c"_EMACS_ATOM_%lu".as_ptr (), nitems + 1);
      atom = XInternAtom ((*dpyinfo).display, buffer, false);

      XSetSelectionOwner ((*dpyinfo).display, atom,
			  FRAME_X_WINDOW! (source_frame),
			  (*dpyinfo).last_user_time);

      XChangeProperty ((*dpyinfo).display, (*dpyinfo).root_window,
		       (*dpyinfo).Xatom_EMACS_DRAG_ATOM, XA_ATOM, 32,
		       if rc != Success
			   || (actual_format != 32
			       || actual_type != XA_ATOM)
			 { PropModeReplace } else { PropModeAppend },
		       &raw mut atom as *mut u8, 1);

      actual_format = 32;
      actual_type = XA_ATOM;
      rc = Success;
      nitems += 1;

      /* The selection's last-change time is newer than our last_user_time, so create a new
	 selection (again).  */
      if XGetSelectionOwner ((*dpyinfo).display, atom) != FRAME_X_WINDOW! (source_frame)
	{ atom = None as Atom; }
    }

  (*dpyinfo).motif_drag_atom_time = (*dpyinfo).last_user_time;
  (*dpyinfo).motif_drag_atom_owner = source_frame;

  XUngrabServer ((*dpyinfo).display);
  return atom;
}

/* C 2511-2523.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_get_drag_atom (dpyinfo: *mut x_display_info) -> Atom
{
  use crate::x11_constants::None;

  let mut atom: Atom;

  if (*dpyinfo).motif_drag_atom != None as Atom
    { atom = (*dpyinfo).motif_drag_atom; }
  else
    { atom = xm_get_drag_atom_1 (dpyinfo, x_dnd_frame); }

  (*dpyinfo).motif_drag_atom = atom;
  return atom;
}

/* C 2525-2560.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_setup_drag_info (dpyinfo: *mut x_display_info, source_frame: *mut frame)
{
  use crate::x11_constants::None;

  let mut atom: Atom;
  let mut drag_initiator_info: xm_drag_initiator_info;
  let mut idx: c_int;

  atom = xm_get_drag_atom (dpyinfo);

  if atom == None as Atom
    { return; }

  XSETCAR (x_dnd_selection_alias_cell,
	   x_atom_to_symbol (dpyinfo, atom));
  XSETCDR (x_dnd_selection_alias_cell, QXdndSelection ());

  idx = xm_setup_dnd_targets (dpyinfo, x_dnd_targets, x_dnd_n_targets);

  if idx != -1
    {
      drag_initiator_info.byteorder = XM_BYTE_ORDER_CUR_FIRST as u8;
      drag_initiator_info.protocol = XM_DRAG_PROTOCOL_VERSION as u8;
      drag_initiator_info.table_index = idx as u16;
      drag_initiator_info.selection = atom as u32;

      xm_write_drag_initiator_info ((*dpyinfo).display,
				    FRAME_X_WINDOW! (source_frame), atom,
				    (*dpyinfo).Xatom_MOTIF_DRAG_INITIATOR_INFO,
				    &raw mut drag_initiator_info);

      x_dnd_motif_setup_p = true;
      x_dnd_motif_atom = atom;
    }
}

/* C 2562-2585.  The `XEvent` field writes are C's own `*(uint16_t *) &b[2]` puns; the port
   keeps the cast (`uint16_t` is the notes row's token) and takes the address through
   `addr_of_mut!`, because `msg` is a Rust value here.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_send_drop_message (dpyinfo: *mut x_display_info, source: Window,
				target: Window, dmsg: *mut xm_drop_start_message)
{
  use crate::x11_constants::{ClientMessage, NoEventMask};

  let mut msg: XEvent = std::mem::zeroed ();

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_MOTIF_DRAG_AND_DROP_MESSAGE;
  msg.xclient.format = 8;
  msg.xclient.window = target;
  msg.xclient.data.b[0] = (*dmsg).reason;
  msg.xclient.data.b[1] = (*dmsg).byte_order;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[2]) as *mut u16) = (*dmsg).side_effects;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[4]) as *mut u32) = (*dmsg).timestamp;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[8]) as *mut u16) = (*dmsg).x;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[10]) as *mut u16) = (*dmsg).y;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[12]) as *mut u32) = (*dmsg).index_atom;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[16]) as *mut u32) = (*dmsg).source_window;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent ((*dpyinfo).display, target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
}

/* C 2587-2612.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_send_top_level_enter_message (dpyinfo: *mut x_display_info, source: Window,
					   target: Window,
					   dmsg: *mut xm_top_level_enter_message)
{
  use crate::x11_constants::{ClientMessage, NoEventMask};

  let mut msg: XEvent = std::mem::zeroed ();

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_MOTIF_DRAG_AND_DROP_MESSAGE;
  msg.xclient.format = 8;
  msg.xclient.window = target;
  msg.xclient.data.b[0] = (*dmsg).reason;
  msg.xclient.data.b[1] = (*dmsg).byteorder;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[2]) as *mut u16) = (*dmsg).zero;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[4]) as *mut u32) = (*dmsg).timestamp;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[8]) as *mut u32) = (*dmsg).source_window;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[12]) as *mut u32) = (*dmsg).index_atom;
  msg.xclient.data.b[16] = 0;
  msg.xclient.data.b[17] = 0;
  msg.xclient.data.b[18] = 0;
  msg.xclient.data.b[19] = 0;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent ((*dpyinfo).display, target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
}

/* ---- Slice 5: C 2614-3012 - the Motif message senders and readers, and the drag-and-drop
   toplevel teardown.  Same conventions as slice 4: X11 macros from the mirror's
   `x11_constants` module (imported per function), C's `False`/`True` as Rust's literals, and
   the message puns kept with C's own casts (`uint16_t`/`uint32_t` - the notes rows).  */

/* C 2614-2643.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_send_drag_motion_message (dpyinfo: *mut x_display_info, source: Window,
				       target: Window, dmsg: *mut xm_drag_motion_message)
{
  use crate::x11_constants::{ClientMessage, NoEventMask};

  let mut msg: XEvent = std::mem::zeroed ();

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_MOTIF_DRAG_AND_DROP_MESSAGE;
  msg.xclient.format = 8;
  msg.xclient.window = target;
  msg.xclient.data.b[0] = (*dmsg).reason;
  msg.xclient.data.b[1] = (*dmsg).byteorder;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[2]) as *mut u16) = (*dmsg).side_effects;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[4]) as *mut u32) = (*dmsg).timestamp;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[8]) as *mut u16) = (*dmsg).x;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[10]) as *mut u16) = (*dmsg).y;
  msg.xclient.data.b[12] = 0;
  msg.xclient.data.b[13] = 0;
  msg.xclient.data.b[14] = 0;
  msg.xclient.data.b[15] = 0;
  msg.xclient.data.b[16] = 0;
  msg.xclient.data.b[17] = 0;
  msg.xclient.data.b[18] = 0;
  msg.xclient.data.b[19] = 0;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent ((*dpyinfo).display, target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
}

/* C 2645-2702: Motif's TOP_LEVEL_LEAVE has bitrotted, so C first sends a drag-motion message
   with impossible coordinates to clear any active drop site.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_send_top_level_leave_message (dpyinfo: *mut x_display_info, source: Window,
					   target: Window,
					   dmsg: *mut xm_top_level_leave_message)
{
  use crate::x11_constants::{ClientMessage, NoEventMask};

  let mut msg: XEvent = std::mem::zeroed ();
  let mut mmsg: xm_drag_motion_message;

  /* Motif support for TOP_LEVEL_LEAVE has bitrotted, since these days it assumes every client
     supports the preregister protocol style, but we only support drop-only and dynamic.
     (Interestingly enough LessTif works fine.)  Sending an event with impossible coordinates
     serves to get rid of any active drop site that might still be around in the target drag
     context.  */

  if x_dnd_fix_motif_leave
    {
      mmsg.reason = XM_DRAG_REASON! (xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR
				     as c_int,
				     xm_drag_reason::XM_DRAG_REASON_DRAG_MOTION as c_int) as u8;
      mmsg.byteorder = XM_BYTE_ORDER_CUR_FIRST as u8;
      mmsg.side_effects = XM_DRAG_SIDE_EFFECT! (xm_side_effect_from_action (dpyinfo,
									    x_dnd_wanted_action),
						xm_drop_site_status::XM_DROP_SITE_NONE as c_int,
						x_dnd_motif_operations as c_int,
						xm_drag_action::XM_DROP_ACTION_DROP_CANCEL
						as c_int) as u16;
      mmsg.timestamp = (*dmsg).timestamp;

      /* Use X_SHRT_MAX instead of the max value of uint16_t since that will be interpreted as
	 a plausible position by Motif, and as such breaks if the drop target is beneath that
	 position.  */
      mmsg.x = X_SHRT_MAX as u16;
      mmsg.y = X_SHRT_MAX as u16;

      xm_send_drag_motion_message (dpyinfo, source, target, &raw mut mmsg);
    }

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_MOTIF_DRAG_AND_DROP_MESSAGE;
  msg.xclient.format = 8;
  msg.xclient.window = target;
  msg.xclient.data.b[0] = (*dmsg).reason;
  msg.xclient.data.b[1] = (*dmsg).byteorder;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[2]) as *mut u16) = (*dmsg).zero;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[4]) as *mut u32) = (*dmsg).timestamp;
  *(core::ptr::addr_of_mut! (msg.xclient.data.b[8]) as *mut u32) = (*dmsg).source_window;
  msg.xclient.data.b[12] = 0;
  msg.xclient.data.b[13] = 0;
  msg.xclient.data.b[14] = 0;
  msg.xclient.data.b[15] = 0;
  msg.xclient.data.b[16] = 0;
  msg.xclient.data.b[17] = 0;
  msg.xclient.data.b[18] = 0;
  msg.xclient.data.b[19] = 0;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent ((*dpyinfo).display, target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
}

/* C 2704-2733.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_drop_start_reply (msg: *const XEvent,
				    reply: *mut xm_drop_start_reply) -> c_int
{
  let mut data: *const u8;

  data = core::ptr::addr_of! ((*msg).xclient.data.b[0]);

  if XM_DRAG_REASON_ORIGINATOR! (*data.offset (0))
      != xm_drag_originator::XM_DRAG_ORIGINATOR_RECEIVER as c_int
      || XM_DRAG_REASON_CODE! (*data.offset (0))
	 != xm_drag_reason::XM_DRAG_REASON_DROP_START as c_int
    { return 1; }

  (*reply).reason = *data;
  data = data.add (1);
  (*reply).byte_order = *data;
  data = data.add (1);
  (*reply).side_effects = *(data as *const u16);
  (*reply).better_x = *(data.add (2) as *const u16);
  (*reply).better_y = *(data.add (4) as *const u16);

  if (*reply).byte_order as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
    {
      SWAPCARD16! ((*reply).side_effects);
      SWAPCARD16! ((*reply).better_x);
      SWAPCARD16! ((*reply).better_y);
    }

  (*reply).byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;

  return 0;
}

/* C 2735-2771.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_drop_start_message (msg: *const XEvent,
				      dmsg: *mut xm_drop_start_message) -> c_int
{
  let mut data: *const u8;

  data = core::ptr::addr_of! ((*msg).xclient.data.b[0]);

  if XM_DRAG_REASON_ORIGINATOR! (*data.offset (0))
      != xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR as c_int
      || XM_DRAG_REASON_CODE! (*data.offset (0))
	 != xm_drag_reason::XM_DRAG_REASON_DROP_START as c_int
    { return 1; }

  (*dmsg).reason = *data;
  data = data.add (1);
  (*dmsg).byte_order = *data;
  data = data.add (1);
  (*dmsg).side_effects = *(data as *const u16);
  (*dmsg).timestamp = *(data.add (2) as *const u32);
  (*dmsg).x = *(data.add (6) as *const u16);
  (*dmsg).y = *(data.add (8) as *const u16);
  (*dmsg).index_atom = *(data.add (10) as *const u32);
  (*dmsg).source_window = *(data.add (14) as *const u32);

  if (*dmsg).byte_order as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
    {
      SWAPCARD16! ((*dmsg).side_effects);
      SWAPCARD32! ((*dmsg).timestamp);
      SWAPCARD16! ((*dmsg).x);
      SWAPCARD16! ((*dmsg).y);
      SWAPCARD32! ((*dmsg).index_atom);
      SWAPCARD32! ((*dmsg).source_window);
    }

  (*dmsg).byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;

  return 0;
}

/* C 2773-2827.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_drag_receiver_info (dpyinfo: *mut x_display_info,
				      wdesc: Window, rec: *mut xm_drag_receiver_info) -> c_int
{
  use crate::x11_constants::Success;

  let mut actual_type: Atom;
  let mut rc: c_int;
  let mut actual_format: c_int;
  let mut nitems: c_ulong;
  let mut bytes_remaining: c_ulong;
  let mut tmp_data: *mut u8 = std::ptr::null_mut ();
  let mut data: *mut u8;

  x_catch_errors ((*dpyinfo).display);
  rc = (XGetWindowProperty ((*dpyinfo).display, wdesc,
			    (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO,
			    0, 4, false,
			    (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO,
			    &raw mut actual_type, &raw mut actual_format, &raw mut nitems,
			    &raw mut bytes_remaining,
			    &raw mut tmp_data) == Success) as c_int;

  if x_had_errors_p ((*dpyinfo).display)
      || actual_format != 8 || nitems < 16 || tmp_data.is_null ()
      || actual_type != (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO
    { rc = 0; }
  x_uncatch_errors_after_check ();

  if rc != 0
    {
      data = tmp_data;

      if *data.add (1) as c_int > XM_DRAG_PROTOCOL_VERSION
	{ return 1; }

      (*rec).byteorder = *data;
      (*rec).protocol = *data.add (1);
      (*rec).protocol_style = *data.add (2);
      (*rec).unspecified0 = *data.add (3);
      (*rec).unspecified1 = *((&raw const *data.add (4)) as *const u32);
      (*rec).unspecified2 = *((&raw const *data.add (8)) as *const u32);
      (*rec).unspecified3 = *((&raw const *data.add (12)) as *const u32);

      if (*rec).byteorder as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
	{
	  SWAPCARD32! ((*rec).unspecified1);
	  SWAPCARD32! ((*rec).unspecified2);
	  SWAPCARD32! ((*rec).unspecified3);
	}

      (*rec).byteorder = XM_BYTE_ORDER_CUR_FIRST as u8;
    }

  if !tmp_data.is_null ()
    { XFree (tmp_data as *mut c_void); }

  return (rc == 0) as c_int;
}

/* C 2829-2861.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_drag_motion_message (msg: *const XEvent,
				       dmsg: *mut xm_drag_motion_message) -> c_int
{
  let mut data: *const u8;

  data = core::ptr::addr_of! ((*msg).xclient.data.b[0]);

  if XM_DRAG_REASON_CODE! (*data.offset (0))
      != xm_drag_reason::XM_DRAG_REASON_DRAG_MOTION as c_int
      || XM_DRAG_REASON_ORIGINATOR! (*data.offset (0))
	 != xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR as c_int
    { return 1; }

  (*dmsg).reason = *data;
  data = data.add (1);
  (*dmsg).byteorder = *data;
  data = data.add (1);
  (*dmsg).side_effects = *(data as *const u16);
  (*dmsg).timestamp = *(data.add (2) as *const u32);
  (*dmsg).x = *(data.add (6) as *const u16);
  (*dmsg).y = *(data.add (8) as *const u16);

  if (*dmsg).byteorder as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
    {
      SWAPCARD16! ((*dmsg).side_effects);
      SWAPCARD32! ((*dmsg).timestamp);
      SWAPCARD16! ((*dmsg).x);
      SWAPCARD16! ((*dmsg).y);
    }

  (*dmsg).byteorder = XM_BYTE_ORDER_CUR_FIRST as u8;

  return 0;
}

/* C 2863-2894.  */
#[cfg (feature = "have_x_windows")]
unsafe fn xm_read_drag_motion_reply (msg: *const XEvent,
				     reply: *mut xm_drag_motion_reply) -> c_int
{
  let mut data: *const u8;

  data = core::ptr::addr_of! ((*msg).xclient.data.b[0]);

  if XM_DRAG_REASON_CODE! (*data.offset (0))
      != xm_drag_reason::XM_DRAG_REASON_DRAG_MOTION as c_int
      || XM_DRAG_REASON_ORIGINATOR! (*data.offset (0))
	 != xm_drag_originator::XM_DRAG_ORIGINATOR_RECEIVER as c_int
    { return 1; }

  (*reply).reason = *data;
  data = data.add (1);
  (*reply).byte_order = *data;
  data = data.add (1);
  (*reply).side_effects = *(data as *const u16);
  (*reply).timestamp = *(data.add (2) as *const u32);
  (*reply).better_x = *(data.add (6) as *const u16);
  (*reply).better_y = *(data.add (8) as *const u16);

  if (*reply).byte_order as xm_byte_order != XM_BYTE_ORDER_CUR_FIRST
    {
      SWAPCARD16! ((*reply).side_effects);
      SWAPCARD32! ((*reply).timestamp);
      SWAPCARD16! ((*reply).better_x);
      SWAPCARD16! ((*reply).better_y);
    }

  (*reply).byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;

  return 0;
}

/* C 2896-2913.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_send_xm_leave_for_drop (dpyinfo: *mut x_display_info,
					f: *mut frame, wdesc: Window,
					timestamp: Time)
{
  let mut lmsg: xm_top_level_leave_message;

  lmsg.reason = XM_DRAG_REASON! (xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR as c_int,
				 xm_drag_reason::XM_DRAG_REASON_TOP_LEVEL_LEAVE as c_int) as u8;
  lmsg.byteorder = XM_BYTE_ORDER_CUR_FIRST as u8;
  lmsg.zero = 0;
  lmsg.timestamp = timestamp as u32;
  lmsg.source_window = FRAME_X_WINDOW! (f) as u32;

  if x_dnd_motif_setup_p
    {
      xm_send_top_level_leave_message (dpyinfo, FRAME_X_WINDOW! (f) as Window,
				       wdesc, &raw mut lmsg);
    }
}

/* C 2915-3010: tear the toplevel list down, restoring each window's event mask first if the
   display is still alive.  C's `UNINIT` locals are the port's zero initialisers (Decision 12's
   note), and C's `record_unwind_protect_ptr (xfree, ...)` is the mirror's `xfree_wrapper`, the
   same choice `lisp.rs` 3033 makes.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_free_toplevels (display_alive: bool)
{
  use crate::x11_constants::None;

  let mut last: *mut x_client_list_window;
  let mut tem: *mut x_client_list_window = x_dnd_toplevels;
  let mut n_windows: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut buffer_size: ptrdiff_t;
  let mut destroy_windows: *mut Window = std::ptr::null_mut ();
  let mut prev_masks: *mut c_ulong = std::ptr::null_mut ();
  let mut count: specpdl_ref;
  let mut dpy: *mut c_void = std::ptr::null_mut ();

  if x_dnd_toplevels.is_null ()
    /* Probably called inside an IO error handler.  */
    { return; }

  if display_alive
    {
      buffer_size = 1024;
      destroy_windows = xmalloc (std::mem::size_of::<Window> () * buffer_size as usize)
			  as *mut Window;
      prev_masks = xmalloc (std::mem::size_of::<c_ulong> () * buffer_size as usize)
		     as *mut c_ulong;
      n_windows = 0;
    }

  block_input ();
  while !tem.is_null ()
    {
      last = tem;
      tem = (*tem).next;

      if display_alive
	{
	  n_windows += 1;
	  if n_windows >= buffer_size
	    {
	      buffer_size += 1024;
	      destroy_windows = xrealloc (destroy_windows as *mut c_void,
					  std::mem::size_of::<Window> () * buffer_size as usize)
				  as *mut Window;
	      prev_masks = xrealloc (prev_masks as *mut c_void,
				     std::mem::size_of::<c_ulong> () * buffer_size as usize)
			     as *mut c_ulong;
	    }

	  dpy = (*last).dpy;
	  *prev_masks.offset (n_windows - 1) = (*last).previous_event_mask as c_ulong;
	  *destroy_windows.offset (n_windows - 1) = (*last).window;
	}

      #[cfg (feature = "have_xshape")]
      {
	if (*last).n_input_rects != -1
	  { xfree ((*last).input_rects); }
	if (*last).n_bounding_rects != -1
	  { xfree ((*last).bounding_rects); }
      }

      xfree (last as *mut c_void);
    }

  x_dnd_toplevels = std::ptr::null_mut ();

  if !display_alive
    {
      unblock_input ();
      return;
    }

  count = SPECPDL_INDEX ();
  record_unwind_protect_ptr (Some (xfree_wrapper), destroy_windows as *mut c_void);
  record_unwind_protect_ptr (Some (xfree_wrapper), prev_masks as *mut c_void);

  if display_alive && n_windows != 0
    {
      let dpyinfo: *mut x_display_info = x_dpyinfo (dpy);

      x_ignore_errors_for_next_request (dpyinfo, 0);

      for i in 0..n_windows
	{
	  XSelectInput (dpy, *destroy_windows.offset (i), *prev_masks.offset (i));
	  #[cfg (feature = "have_xshape")]
	  { XShapeSelectInput (dpy, *destroy_windows.offset (i), None); }
	}

      x_stop_ignoring_errors (dpyinfo);
    }

  unbind_to (count, Qnil ());
  unblock_input ();
}

/* ---- Slice 6: C 3013-3614, `x_dnd_compute_toplevels`.  Two arms run through the whole body -
   `#ifndef USE_XCB` (the frozen one) and `USE_XCB` - and inside them `HAVE_XSHAPE`,
   `HAVE_XCB_SHAPE` and `HAVE_XCB_SHAPE_INPUT_RECTS`; every arm is carried under its feature
   (P-51).  C 3461's `#ifdef ShapeInput` asks whether the X11 shape header is new enough to
   define the shape-input selector, and the port's `x11_constants` defines it (measured from
   `X11/extensions/shapeconst.h`), so that block is carried without a gate of its own.  C leaves
   the `#ifndef USE_XCB` locals uninitialised; the port's zero is PORTING.md 1.5.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_compute_toplevels (dpyinfo: *mut x_display_info) -> c_int
{
  use crate::x11_constants::{AnyPropertyType, IsUnmapped, None, PropertyChangeMask,
			     ShapeBounding, ShapeInput, ShapeNotifyMask, StructureNotifyMask,
			     Success, XA_CARDINAL, XA_WINDOW};

  let mut r#type: Atom;
  let mut toplevels: *mut Window;
  let mut format: c_int;
  let mut rc: c_int;
  let mut nitems: c_ulong;
  let mut bytes_after: c_ulong;
  let mut i: c_ulong;
  let mut real_nitems: c_ulong;
  let mut data: *mut u8 = std::ptr::null_mut ();
  let mut frame_extents: [c_int; 4];

  #[cfg (not (feature = "use_xcb"))]
  let mut dest_x: c_int = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut dest_y: c_int = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut wmstate: *mut c_ulong = std::ptr::null_mut ();
  #[cfg (not (feature = "use_xcb"))]
  let mut wmstate_items: c_ulong = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut extent_items: c_ulong = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut wmstate_data: *mut u8 = std::ptr::null_mut ();
  #[cfg (not (feature = "use_xcb"))]
  let mut extent_data: *mut u8 = std::ptr::null_mut ();
  #[cfg (not (feature = "use_xcb"))]
  let mut attrs: XWindowAttributes = std::mem::zeroed ();
  #[cfg (not (feature = "use_xcb"))]
  let mut child: Window = 0;
  #[cfg (not (feature = "use_xcb"))]
  let mut xm_info: xm_drag_receiver_info = std::mem::zeroed ();

  #[cfg (feature = "use_xcb")]
  let mut wmstate: *mut u32 = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut fextents: *mut u32 = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut xmdata: *mut u8 = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut window_attribute_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut translate_coordinate_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut get_property_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut xm_property_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut extent_property_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut get_geometry_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut attrs: c_void = unsafe { std::mem::zeroed () };
  #[cfg (feature = "use_xcb")]
  let mut attrs_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut coordinates_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut property_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut xm_property_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut extent_property_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut geometry_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut error: *mut c_void = std::ptr::null_mut ();

  #[cfg (all (feature = "have_xcb_shape"))]
  let mut bounding_rect_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (all (feature = "have_xcb_shape"))]
  let mut bounding_rect_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (all (feature = "have_xcb_shape"))]
  let mut bounding_rect_iterator: c_void = unsafe { std::mem::zeroed () };

  #[cfg (all (feature = "have_xcb_shape_input_rects"))]
  let mut input_rect_cookies: *mut c_void = std::ptr::null_mut ();
  #[cfg (all (feature = "have_xcb_shape_input_rects"))]
  let mut input_rect_reply: *mut c_void = std::ptr::null_mut ();
  #[cfg (all (feature = "have_xcb_shape_input_rects"))]
  let mut input_rect_iterator: c_void = unsafe { std::mem::zeroed () };

  let mut tem: *mut x_client_list_window;

  #[cfg (all (feature = "have_xshape", not (feature = "have_xcb_shape_input_rects")))]
  let mut count: c_int = 0;
  #[cfg (all (feature = "have_xshape", not (feature = "have_xcb_shape_input_rects")))]
  let mut ordering: c_int = 0;
  #[cfg (all (feature = "have_xshape", not (feature = "have_xcb_shape_input_rects")))]
  let mut rects: *mut XRectangle = std::ptr::null_mut ();

  rc = XGetWindowProperty ((*dpyinfo).display, (*dpyinfo).root_window,
			   (*dpyinfo).Xatom_net_client_list_stacking,
			   0, LONG_MAX, false, XA_WINDOW, &raw mut r#type,
			   &raw mut format, &raw mut nitems, &raw mut bytes_after, &raw mut data);

  if rc != Success
    { return 1; }

  if format != 32 || r#type != XA_WINDOW
    {
      XFree (data as *mut c_void);
      return 1;
    }

  toplevels = data as *mut Window;

  i = 0;
  real_nitems = 0;
  while i < nitems
    {
      /* Some window managers with built in compositors end up putting tooltips in the client
	 list, which is silly.  */
      if x_tooltip_window_to_frame (dpyinfo, *toplevels.offset (i as isize),
				    std::ptr::null_mut ()).is_null ()
	{
	  *toplevels.offset (real_nitems as isize) = *toplevels.offset (i as isize);
	  real_nitems += 1;
	}
      i += 1;
    }

  nitems = real_nitems;

  #[cfg (feature = "use_xcb")]
  {
    USE_SAFE_ALLOCA! (sa_avail, sa_count);

    SAFE_NALLOCA! (sa_avail, window_attribute_cookies, 1, nitems);
    SAFE_NALLOCA! (sa_avail, translate_coordinate_cookies, 1, nitems);
    SAFE_NALLOCA! (sa_avail, get_property_cookies, 1, nitems);
    SAFE_NALLOCA! (sa_avail, xm_property_cookies, 1, nitems);
    SAFE_NALLOCA! (sa_avail, extent_property_cookies, 1, nitems);
    SAFE_NALLOCA! (sa_avail, get_geometry_cookies, 1, nitems);

    #[cfg (feature = "have_xcb_shape")]
    { SAFE_NALLOCA! (sa_avail, bounding_rect_cookies, 1, nitems); }

    #[cfg (feature = "have_xcb_shape_input_rects")]
    { SAFE_NALLOCA! (sa_avail, input_rect_cookies, 1, nitems); }

    i = 0;
    while i < nitems
      {
	*(window_attribute_cookies as *mut c_void) = xcb_get_window_attributes (
	  (*dpyinfo).xcb_connection, *toplevels.offset (i as isize) as xcb_window_t);
	*(translate_coordinate_cookies as *mut c_void) = xcb_translate_coordinates (
	  (*dpyinfo).xcb_connection, *toplevels.offset (i as isize) as xcb_window_t,
	  (*dpyinfo).root_window as xcb_window_t, 0, 0);
	*(get_property_cookies as *mut c_void) = xcb_get_property (
	  (*dpyinfo).xcb_connection, 0, *toplevels.offset (i as isize) as xcb_window_t,
	  (*dpyinfo).Xatom_wm_state as xcb_atom_t, 0, 0, 2);
	*(xm_property_cookies as *mut c_void) = xcb_get_property (
	  (*dpyinfo).xcb_connection, 0, *toplevels.offset (i as isize) as xcb_window_t,
	  (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO as xcb_atom_t,
	  (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO as xcb_atom_t, 0, 4);
	*(extent_property_cookies as *mut c_void) = xcb_get_property (
	  (*dpyinfo).xcb_connection, 0, *toplevels.offset (i as isize) as xcb_window_t,
	  (*dpyinfo).Xatom_net_frame_extents as xcb_atom_t,
	  XA_CARDINAL as xcb_atom_t, 0, 4);
	*(get_geometry_cookies as *mut c_void) = xcb_get_geometry (
	  (*dpyinfo).xcb_connection, *toplevels.offset (i as isize) as xcb_window_t);

	#[cfg (feature = "have_xcb_shape")]
	{
	  *(bounding_rect_cookies as *mut c_void) = xcb_shape_get_rectangles (
	    (*dpyinfo).xcb_connection, *toplevels.offset (i as isize) as xcb_window_t,
	    XCB_SHAPE_SK_BOUNDING);
	}

	#[cfg (feature = "have_xcb_shape_input_rects")]
	{
	  if (*dpyinfo).xshape_major > 1
	      || ((*dpyinfo).xshape_major == 1
		  && (*dpyinfo).xshape_minor >= 1)
	    {
	      *(input_rect_cookies as *mut c_void) = xcb_shape_get_rectangles (
		(*dpyinfo).xcb_connection, *toplevels.offset (i as isize) as xcb_window_t,
		XCB_SHAPE_SK_INPUT);
	    }
	}
	i += 1;
      }
  }

  /* Actually right because _NET_CLIENT_LIST_STACKING has bottom-up order.  */
  i = 0;
  while i < nitems
    {
      frame_extents[0] = 0;
      frame_extents[1] = 0;
      frame_extents[2] = 0;
      frame_extents[3] = 0;

      #[cfg (not (feature = "use_xcb"))]
      {
	x_catch_errors ((*dpyinfo).display);
	rc = (XGetWindowAttributes ((*dpyinfo).display,
				    *toplevels.offset (i as isize), &raw mut attrs) != 0
	      && !x_had_errors_p ((*dpyinfo).display)) as c_int;

	if rc != 0
	  {
	    rc = (XTranslateCoordinates ((*dpyinfo).display, *toplevels.offset (i as isize),
					 attrs.root, -attrs.border_width,
					 -attrs.border_width, &raw mut dest_x,
					 &raw mut dest_y, &raw mut child) != 0
		  && !x_had_errors_p ((*dpyinfo).display)) as c_int;
	  }
	if rc != 0
	  {
	    rc = ((XGetWindowProperty ((*dpyinfo).display,
				       *toplevels.offset (i as isize),
				       (*dpyinfo).Xatom_wm_state,
				       0, 2, false, AnyPropertyType,
				       &raw mut r#type, &raw mut format, &raw mut wmstate_items,
				       &raw mut bytes_after, &raw mut wmstate_data)
		   == Success)
		  && !x_had_errors_p ((*dpyinfo).display)
		  && !wmstate_data.is_null () && wmstate_items == 2
		  && format == 32) as c_int;
	  }

	if XGetWindowProperty ((*dpyinfo).display, *toplevels.offset (i as isize),
			       (*dpyinfo).Xatom_net_frame_extents,
			       0, 4, false, XA_CARDINAL, &raw mut r#type,
			       &raw mut format, &raw mut extent_items, &raw mut bytes_after,
			       &raw mut extent_data) == Success
	    && !x_had_errors_p ((*dpyinfo).display)
	    && !extent_data.is_null () && extent_items >= 4 && format == 32
	  {
	    frame_extents[0] = *(extent_data as *mut c_ulong).offset (0) as c_int;
	    frame_extents[1] = *(extent_data as *mut c_ulong).offset (1) as c_int;
	    frame_extents[2] = *(extent_data as *mut c_ulong).offset (2) as c_int;
	    frame_extents[3] = *(extent_data as *mut c_ulong).offset (3) as c_int;
	  }

	if !extent_data.is_null ()
	  { XFree (extent_data as *mut c_void); }

	x_uncatch_errors ();
      }

      #[cfg (feature = "use_xcb")]
      {
	rc = true as c_int;

	attrs_reply = xcb_get_window_attributes_reply ((*dpyinfo).xcb_connection,
						       window_attribute_cookies, &raw mut error);

	if attrs_reply.is_null ()
	  {
	    rc = false as c_int;
	    free (error);
	  }

	coordinates_reply = xcb_translate_coordinates_reply ((*dpyinfo).xcb_connection,
							    translate_coordinate_cookies,
							    &raw mut error);

	if coordinates_reply.is_null ()
	  {
	    rc = false as c_int;
	    free (error);
	  }

	property_reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
						  get_property_cookies, &raw mut error);

	if property_reply.is_null ()
	  {
	    rc = false as c_int;
	    free (error);
	  }

	/* These requests don't set rc on failure because they aren't required.  */

	xm_property_reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
						    xm_property_cookies, &raw mut error);

	if xm_property_reply.is_null ()
	  { free (error); }

	extent_property_reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
							extent_property_cookies,
							&raw mut error);

	if extent_property_reply.is_null ()
	  { free (error); }
	else
	  {
	    if xcb_get_property_value_length (extent_property_reply) == 16
		&& (*extent_property_reply).format == 32
		&& (*extent_property_reply).r#type == XA_CARDINAL
	      {
		fextents = xcb_get_property_value (extent_property_reply);
		frame_extents[0] = *fextents.offset (0) as c_int;
		frame_extents[1] = *fextents.offset (1) as c_int;
		frame_extents[2] = *fextents.offset (2) as c_int;
		frame_extents[3] = *fextents.offset (3) as c_int;
	      }

	    free (extent_property_reply);
	  }

	if !property_reply.is_null ()
	    && (xcb_get_property_value_length (property_reply) != 8
		|| (*property_reply).format != 32)
	  { rc = false as c_int; }

	geometry_reply = xcb_get_geometry_reply ((*dpyinfo).xcb_connection,
						 get_geometry_cookies, &raw mut error);

	if geometry_reply.is_null ()
	  {
	    rc = false as c_int;
	    free (error);
	  }
      }

      if rc != 0
	{
	  #[cfg (feature = "use_xcb")]
	  {
	    wmstate = xcb_get_property_value (property_reply) as *mut u32;
	    attrs = *attrs_reply;
	  }
	  #[cfg (not (feature = "use_xcb"))]
	  {
	    wmstate = wmstate_data as *mut c_ulong;
	  }

	  tem = xmalloc (std::mem::size_of::<x_client_list_window> ())
		  as *mut x_client_list_window;
	  (*tem).window = *toplevels.offset (i as isize);
	  (*tem).dpy = (*dpyinfo).display;
	  (*tem).frame_extents_left = frame_extents[0];
	  (*tem).frame_extents_right = frame_extents[1];
	  (*tem).frame_extents_top = frame_extents[2];
	  (*tem).frame_extents_bottom = frame_extents[3];

	  #[cfg (not (feature = "use_xcb"))]
	  {
	    (*tem).x = dest_x;
	    (*tem).y = dest_y;
	    (*tem).width = attrs.width + attrs.border_width;
	    (*tem).height = attrs.height + attrs.border_width;
	    (*tem).mapped_p = attrs.map_state != IsUnmapped;
	  }
	  #[cfg (feature = "use_xcb")]
	  {
	    (*tem).x = (*coordinates_reply).dst_x - (*geometry_reply).border_width;
	    (*tem).y = (*coordinates_reply).dst_y - (*geometry_reply).border_width;
	    (*tem).width = (*geometry_reply).width + (*geometry_reply).border_width;
	    (*tem).height = (*geometry_reply).height + (*geometry_reply).border_width;
	    (*tem).mapped_p = attrs.map_state != XCB_MAP_STATE_UNMAPPED;
	  }
	  (*tem).next = x_dnd_toplevels;
	  (*tem).previous_event_mask = attrs.your_event_mask;
	  (*tem).wm_state = *wmstate.offset (0);
	  (*tem).xm_protocol_style = xm_drag_style::XM_DRAG_STYLE_NONE as u8;

	  #[cfg (not (feature = "use_xcb"))]
	  {
	    if xm_read_drag_receiver_info (dpyinfo, *toplevels.offset (i as isize),
					   &raw mut xm_info) == 0
	      { (*tem).xm_protocol_style = xm_info.protocol_style; }
	  }
	  #[cfg (feature = "use_xcb")]
	  {
	    if !xm_property_reply.is_null ()
		&& (*xm_property_reply).format == 8
		&& ((*xm_property_reply).r#type
		    == (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO)
		&& xcb_get_property_value_length (xm_property_reply) >= 4
	      {
		xmdata = xcb_get_property_value (xm_property_reply);

		if *xmdata.offset (1) as c_int <= XM_DRAG_PROTOCOL_VERSION
		  { (*tem).xm_protocol_style = *xmdata.offset (2); }
	      }
	  }

	  #[cfg (feature = "have_xshape")]
	  {
	    #[cfg (not (feature = "use_xcb"))]
	    { (*tem).border_width = attrs.border_width; }
	    #[cfg (feature = "use_xcb")]
	    { (*tem).border_width = (*geometry_reply).border_width; }
	    (*tem).n_bounding_rects = -1;
	    (*tem).n_input_rects = -1;

	    if (*dpyinfo).xshape_supported_p
	      {
		x_ignore_errors_for_next_request (dpyinfo, 0);
		XShapeSelectInput ((*dpyinfo).display,
				   *toplevels.offset (i as isize),
				   ShapeNotifyMask);
		x_stop_ignoring_errors (dpyinfo);

		#[cfg (not (feature = "have_xcb_shape"))]
		{
		  x_catch_errors ((*dpyinfo).display);
		  rects = XShapeGetRectangles ((*dpyinfo).display,
					       *toplevels.offset (i as isize),
					       ShapeBounding,
					       &raw mut count, &raw mut ordering);
		  rc = x_had_errors_p ((*dpyinfo).display) as c_int;
		  x_uncatch_errors_after_check ();

		  /* Does XShapeGetRectangles allocate anything upon an error?  */
		  if rc == 0
		    {
		      (*tem).n_bounding_rects = count;
		      (*tem).bounding_rects = xmalloc (
			std::mem::size_of::<XRectangle> () * count as usize) as *mut XRectangle;
		      std::ptr::copy_nonoverlapping (rects, (*tem).bounding_rects,
						     count as usize);

		      XFree (rects as *mut c_void);
		    }
		}
		#[cfg (feature = "have_xcb_shape")]
		{
		  bounding_rect_reply = xcb_shape_get_rectangles_reply (
		    (*dpyinfo).xcb_connection, bounding_rect_cookies, &raw mut error);

		  if !bounding_rect_reply.is_null ()
		    {
		      bounding_rect_iterator
			= xcb_shape_get_rectangles_rectangles_iterator (bounding_rect_reply);
		      (*tem).n_bounding_rects = bounding_rect_iterator.rem + 1;
		      (*tem).bounding_rects = xmalloc (
			((*tem).n_bounding_rects
			 * std::mem::size_of::<XRectangle> () as c_int) as usize)
			as *mut XRectangle;
		      (*tem).n_bounding_rects = 0;

		      while bounding_rect_iterator.rem != 0
			{
			  (*tem).bounding_rects.offset ((*tem).n_bounding_rects as isize).x
			    = (*bounding_rect_iterator.data).x;
			  (*tem).bounding_rects.offset ((*tem).n_bounding_rects as isize).y
			    = (*bounding_rect_iterator.data).y;
			  (*tem).bounding_rects.offset ((*tem).n_bounding_rects as isize).width
			    = (*bounding_rect_iterator.data).width;
			  (*tem).bounding_rects.offset ((*tem).n_bounding_rects as isize).height
			    = (*bounding_rect_iterator.data).height;

			  (*tem).n_bounding_rects += 1;
			  xcb_rectangle_next (&raw mut bounding_rect_iterator);
			}

		      free (bounding_rect_reply);
		    }
		  else
		    { free (error); }
		}

		#[cfg (feature = "have_xcb_shape_input_rects")]
		{
		  if (*dpyinfo).xshape_major > 1
		      || ((*dpyinfo).xshape_major == 1
			  && (*dpyinfo).xshape_minor >= 1)
		    {
		      input_rect_reply = xcb_shape_get_rectangles_reply (
			(*dpyinfo).xcb_connection, input_rect_cookies, &raw mut error);

		      if !input_rect_reply.is_null ()
			{
			  input_rect_iterator
			    = xcb_shape_get_rectangles_rectangles_iterator (input_rect_reply);
			  (*tem).n_input_rects = input_rect_iterator.rem + 1;
			  (*tem).input_rects = xmalloc (
			    ((*tem).n_input_rects
			     * std::mem::size_of::<XRectangle> () as c_int) as usize)
			    as *mut XRectangle;
			  (*tem).n_input_rects = 0;

			  while input_rect_iterator.rem != 0
			    {
			      (*tem).input_rects.offset ((*tem).n_input_rects as isize).x
				= (*input_rect_iterator.data).x;
			      (*tem).input_rects.offset ((*tem).n_input_rects as isize).y
				= (*input_rect_iterator.data).y;
			      (*tem).input_rects.offset ((*tem).n_input_rects as isize).width
				= (*input_rect_iterator.data).width;
			      (*tem).input_rects.offset ((*tem).n_input_rects as isize).height
				= (*input_rect_iterator.data).height;

			      (*tem).n_input_rects += 1;
			      xcb_rectangle_next (&raw mut input_rect_iterator);
			    }

			  free (input_rect_reply);
			}
		      else
			{ free (error); }
		    }
		}
		#[cfg (not (feature = "have_xcb_shape_input_rects"))]
		{
		  /* C 3461's `#ifdef ShapeInput`: the port's `x11_constants` defines it, so the
		     block is carried (see this slice's note).  */
		  if (*dpyinfo).xshape_major > 1
		      || ((*dpyinfo).xshape_major == 1
			  && (*dpyinfo).xshape_minor >= 1)
		    {
		      x_catch_errors ((*dpyinfo).display);
		      rects = XShapeGetRectangles ((*dpyinfo).display,
						   *toplevels.offset (i as isize), ShapeInput,
						   &raw mut count, &raw mut ordering);
		      rc = x_had_errors_p ((*dpyinfo).display) as c_int;
		      x_uncatch_errors_after_check ();

		      /* Does XShapeGetRectangles allocate anything upon an error?  */
		      if rc == 0
			{
			  (*tem).n_input_rects = count;
			  (*tem).input_rects = xmalloc (
			    std::mem::size_of::<XRectangle> () * count as usize)
			    as *mut XRectangle;
			  std::ptr::copy_nonoverlapping (rects, (*tem).input_rects,
							 count as usize);

			  XFree (rects as *mut c_void);
			}
		    }
		}
	      }

	    /* Handle the common case where the input shape equals the bounding shape.  */

	    if (*tem).n_input_rects != -1
		&& (*tem).n_bounding_rects == (*tem).n_input_rects
		&& memcmp ((*tem).bounding_rects as *const c_void,
			   (*tem).input_rects as *const c_void,
			   ((*tem).n_input_rects as usize)
			   * std::mem::size_of::<XRectangle> ()) == 0
	      {
		xfree ((*tem).input_rects as *mut c_void);
		(*tem).n_input_rects = -1;
	      }

	    /* And the common case where there is no input rect and the bounding rect equals the
	       window dimensions.  */

	    #[cfg (not (feature = "use_xcb"))]
	    let dims_ok = (*tem).n_input_rects == -1
	      && (*tem).n_bounding_rects == 1
	      && (*tem).bounding_rects.offset (0).width == attrs.width + attrs.border_width
	      && (*tem).bounding_rects.offset (0).height == attrs.height + attrs.border_width
	      && (*tem).bounding_rects.offset (0).x == -attrs.border_width
	      && (*tem).bounding_rects.offset (0).y == -attrs.border_width;
	    #[cfg (feature = "use_xcb")]
	    let dims_ok = (*tem).n_input_rects == -1
	      && (*tem).n_bounding_rects == 1
	      && (*tem).bounding_rects.offset (0).width
		 == (*geometry_reply).width + (*geometry_reply).border_width
	      && (*tem).bounding_rects.offset (0).height
		 == (*geometry_reply).height + (*geometry_reply).border_width
	      && (*tem).bounding_rects.offset (0).x == -(*geometry_reply).border_width
	      && (*tem).bounding_rects.offset (0).y == -(*geometry_reply).border_width;

	    if dims_ok
	      {
		xfree ((*tem).bounding_rects as *mut c_void);
		(*tem).n_bounding_rects = -1;
	      }
	  }

	  x_ignore_errors_for_next_request (dpyinfo, 0);
	  XSelectInput ((*dpyinfo).display, *toplevels.offset (i as isize),
			(attrs.your_event_mask
			 | StructureNotifyMask
			 | PropertyChangeMask));
	  x_stop_ignoring_errors (dpyinfo);

	  x_dnd_toplevels = tem;
	}
      else
	{
	  #[cfg (feature = "have_xcb_shape")]
	  {
	    if (*dpyinfo).xshape_supported_p
	      {
		bounding_rect_reply = xcb_shape_get_rectangles_reply (
		  (*dpyinfo).xcb_connection, bounding_rect_cookies, &raw mut error);

		if !bounding_rect_reply.is_null ()
		  { free (bounding_rect_reply); }
		else
		  { free (error); }
	      }
	  }

	  #[cfg (feature = "have_xcb_shape_input_rects")]
	  {
	    if (*dpyinfo).xshape_supported_p
		&& ((*dpyinfo).xshape_major > 1
		    || ((*dpyinfo).xshape_major == 1
			&& (*dpyinfo).xshape_minor >= 1))
	      {
		input_rect_reply = xcb_shape_get_rectangles_reply (
		  (*dpyinfo).xcb_connection, input_rect_cookies, &raw mut error);

		if !input_rect_reply.is_null ()
		  { free (input_rect_reply); }
		else
		  { free (error); }
	      }
	  }
	}

      #[cfg (feature = "use_xcb")]
      {
	if !attrs_reply.is_null ()
	  { free (attrs_reply); }

	if !coordinates_reply.is_null ()
	  { free (coordinates_reply); }

	if !property_reply.is_null ()
	  { free (property_reply); }

	if !xm_property_reply.is_null ()
	  { free (xm_property_reply); }

	if !geometry_reply.is_null ()
	  { free (geometry_reply); }
      }

      #[cfg (not (feature = "use_xcb"))]
      {
	if !wmstate_data.is_null ()
	  {
	    XFree (wmstate_data as *mut c_void);
	    wmstate_data = std::ptr::null_mut ();
	  }
      }
      i += 1;
    }

  #[cfg (feature = "use_xcb")]
  { SAFE_FREE! (); }

  if !data.is_null ()
    { XFree (data as *mut c_void); }

  return 0;
}

/* ---- Slice 7: C 3616-4466 - the drag-and-drop target-window discovery (the io-error escape,
   the two helpers C keeps inside its own conditions, the toplevel walk, the wm-state/protocol
   query, the unsupported-drop fallback and the window walk itself).  The conventions are slice
   6's: every configuration arm is carried under its feature (P-51); the X11 macros the mirror's
   `x11_constants` module does not carry stay bare (`NormalState`, `IsViewable`, `XA_PRIMARY`,
   `ButtonPress`/`ButtonRelease` and their two masks - the Decision 15 ruling, with the measured
   values in this ledger's Evidence); C's uninitialised locals take the port's zero (PORTING.md
   1.5); C's prototypes are not transcribed (Rust resolves items by name - the C 1162-1210 note's
   rule), except the two this unit takes from another *unit's* header, which the one function
   that calls them declares (the `font.rs` 413-418 shape).  */

/* C 3616-3624: C declares it `_Noreturn int`, which the port writes `-> !` (PORTING.md 5.2);
   the `USE_GTK` half is C's other arm (P-51), and `siglongjmp` is the libc function declared
   above.  */
/* C 3622-3636: C's `_Noreturn int` handler, kept with C's `int` return for the reason the
   `xm_drag_window_io_error_handler` note above records (Slice 14's reading).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_dnd_io_error_handler (display: *mut c_void) -> c_int
{
  #[cfg (feature = "use_gtk")]
  { emacs_abort () }
  #[cfg (not (feature = "use_gtk"))]
  {
    siglongjmp (&raw mut x_dnd_disconnect_handler, 1);
    /* C's arm is `_Noreturn` and C's return type is `int`; `siglongjmp` is the diverging
       expression (the `xm_drag_window_io_error_handler` shape above).  */
    unreachable! ()
  }
}

/* C 3626: `#define X_DND_SUPPORTED_VERSION 5` - a value, so the port writes the constant the
   extractor reads it as (the `XM_DRAG_PROTOCOL_VERSION` shape, C 1759-1761).  */
#[cfg (feature = "have_x_windows")]
pub const X_DND_SUPPORTED_VERSION: c_int = 5;

/* C 3628-3630: three forward declarations (`x_dnd_get_window_proto` 3628, `x_dnd_get_window_proxy`
   3629 and `x_dnd_update_state` 3630).  Rust has no prototypes and resolves items by name, so
   each definition stands where C writes it and nothing stands here.  */

/* C 3632-3696: `#ifdef USE_XCB` - the xcb half of the proxy/protocol query, dead in the frozen
   configuration and carried under its feature (P-51).  The xcb type names stay bare (this file's
   convention for the X11-header names it cannot see), and C's uninitialised cookies take the
   port's zero (PORTING.md 1.5).  */

#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
unsafe fn x_dnd_get_proxy_proto (dpyinfo: *mut x_display_info, wdesc: Window,
				 proxy_out: *mut Window, proto_out: *mut c_int)
{
  use crate::x11_constants::{None, XA_ATOM, XA_WINDOW};

  let mut xdnd_proto_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  let mut xdnd_proxy_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  let mut reply: *mut xcb_get_property_reply_t = std::ptr::null_mut ();
  let mut error: *mut xcb_generic_error_t = std::ptr::null_mut ();

  if !proxy_out.is_null ()
    { *proxy_out = None as Window; }

  if !proto_out.is_null ()
    { *proto_out = -1; }

  if !proxy_out.is_null ()
    {
      xdnd_proxy_cookie = xcb_get_property ((*dpyinfo).xcb_connection, 0,
					    wdesc as xcb_window_t,
					    (*dpyinfo).Xatom_XdndProxy as xcb_atom_t,
					    XA_WINDOW, 0, 1);
    }

  if !proto_out.is_null ()
    {
      xdnd_proto_cookie = xcb_get_property ((*dpyinfo).xcb_connection, 0,
					    wdesc as xcb_window_t,
					    (*dpyinfo).Xatom_XdndAware as xcb_atom_t,
					    XA_ATOM, 0, 1);
    }

  if !proxy_out.is_null ()
    {
      reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
				      xdnd_proxy_cookie, &raw mut error);

      if reply.is_null ()
	{ free (error as *mut c_void); }
      else
	{
	  if (*reply).format == 32
	      && (*reply).r#type == XA_WINDOW
	      && (xcb_get_property_value_length (reply) >= 4)
	    { *proxy_out = *(xcb_get_property_value (reply) as *mut xcb_window_t); }

	  free (reply as *mut c_void);
	}
    }

  if !proto_out.is_null ()
    {
      reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
				      xdnd_proto_cookie, &raw mut error);

      if reply.is_null ()
	{ free (error as *mut c_void); }
      else
	{
	  if (*reply).format == 32
	      && (*reply).r#type == XA_ATOM
	      && (xcb_get_property_value_length (reply) >= 4)
	    { *proto_out = *(xcb_get_property_value (reply) as *mut xcb_atom_t) as c_int; }

	  free (reply as *mut c_void);
	}
    }
}

/* C 3698-3718: `#ifdef HAVE_XSHAPE` - the point-in-rectangles test, dead in the frozen
   configuration and carried under its feature (P-51).  `XRectangle`'s fields are `short` and
   `unsigned short`, so C's `tem->x + tem->width` is int arithmetic; the port writes the two
   promotions C's integer promotion performs.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xshape"))]
unsafe fn x_dnd_get_target_window_2 (rects: *mut XRectangle, nrects: c_int,
				     x: c_int, y: c_int) -> bool
{
  let mut i: c_int;
  let mut tem: *mut XRectangle;

  i = 0;
  while i < nrects
    {
      tem = rects.offset (i as isize);

      if x >= (*tem).x as c_int && y >= (*tem).y as c_int
	  && x < (*tem).x as c_int + (*tem).width as c_int
	  && y < (*tem).y as c_int + (*tem).height as c_int
	{ return true; }

      i += 1;
    }

  return false;
}

/* C 3720-3817: the toplevel search.  C's `for` increment runs on its `continue` too, so the
   port keeps the increment after a labelled `'body:` block (PORTING.md 2.2, the
   `overlay_arrow_in_current_buffer_p` shape of deviation 133) and names the loop `'toplevels`
   for C's two `break`s - a bare `break` inside a labelled block needs a label (E0695, measured
   in this slice's first x11-arm probe).  The `#ifdef HAVE_XSHAPE` half of C 3777 is a named
   binding so the `if` C writes once stays one statement (the `dims_ok` shape of Decision 15).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_get_target_window_1 (dpyinfo: *mut x_display_info, root_x: c_int,
				     root_y: c_int, motif_out: *mut c_int,
				     extents_p: *mut bool) -> Window
{
  use crate::x11_constants::None;

  let mut tem: *mut x_client_list_window;
  let mut chosen: *mut x_client_list_window = std::ptr::null_mut ();

  /* Loop through x_dnd_toplevels until we find the toplevel where
     root_x and root_y are.  */

  *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;

  tem = x_dnd_toplevels;
  'toplevels: while !tem.is_null ()
    {
      'body:
	{
	  if !(*tem).mapped_p || (*tem).wm_state != NormalState as c_ulong
	    { break 'body; }

	  /* Test if the coordinates are inside the window's frame
	     extents, and return None in that case.  */

	  *extents_p = true;
	  if root_x > (*tem).x - (*tem).frame_extents_left
	      && root_x < (*tem).x
	      && root_y > (*tem).y - (*tem).frame_extents_top
	      && root_y < ((*tem).y + (*tem).height - 1
			   + (*tem).frame_extents_bottom)
	    { return None as Window; }

	  if root_x > (*tem).x + (*tem).width
	      && root_x < ((*tem).x + (*tem).width - 1
			   + (*tem).frame_extents_right)
	      && root_y > (*tem).y - (*tem).frame_extents_top
	      && root_y < ((*tem).y + (*tem).height - 1
			   + (*tem).frame_extents_bottom)
	    { return None as Window; }

	  if root_y > (*tem).y - (*tem).frame_extents_top
	      && root_y < (*tem).y
	      && root_x > (*tem).x - (*tem).frame_extents_left
	      && root_x < ((*tem).x + (*tem).width - 1
			   + (*tem).frame_extents_right)
	    { return None as Window; }

	  if root_y > (*tem).y + (*tem).height
	      && root_y < ((*tem).y + (*tem).height - 1
			   + (*tem).frame_extents_bottom)
	      && root_x >= (*tem).x - (*tem).frame_extents_left
	      && root_x < ((*tem).x + (*tem).width - 1
			   + (*tem).frame_extents_right)
	    { return None as Window; }
	  *extents_p = false;

	  if root_x >= (*tem).x && root_y >= (*tem).y
	      && root_x < (*tem).x + (*tem).width
	      && root_y < (*tem).y + (*tem).height
	    {
	      /* C 3776-3778's `#ifdef HAVE_XSHAPE` half; without the shape extension C's `if`
		 has no condition at all.  */
	      #[cfg (feature = "have_xshape")]
	      let bounding_only = (*tem).n_bounding_rects == -1;
	      #[cfg (not (feature = "have_xshape"))]
	      let bounding_only = true;

	      if bounding_only
		{
		  chosen = tem;
		  break 'toplevels;
		}

	      #[cfg (feature = "have_xshape")]
	      {
		if x_dnd_get_target_window_2 ((*tem).bounding_rects,
					      (*tem).n_bounding_rects,
					      (*tem).border_width + root_x - (*tem).x,
					      (*tem).border_width + root_y - (*tem).y)
		  {
		    if (*tem).n_input_rects == -1
			|| x_dnd_get_target_window_2 ((*tem).input_rects,
						      (*tem).n_input_rects,
						      ((*tem).border_width
						       + root_x - (*tem).x),
						      ((*tem).border_width
						       + root_y - (*tem).y))
		      {
			chosen = tem;
			break 'toplevels;
		      }
		  }
	      }
	    }
	}
      tem = (*tem).next;
    }

  if !chosen.is_null ()
    {
      *motif_out = if x_dnd_disable_motif_protocol
	{ xm_drag_style::XM_DRAG_STYLE_NONE as c_int }
	else
	  { (*chosen).xm_protocol_style as c_int };
      return (*chosen).window;
    }
  else
    { *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int; }

  return None as Window;
}

/* C 3819-3980: the wm-state and protocol query.  Both of C's arms are carried - the `#ifndef
   USE_XCB` one (live in the frozen reading) and the `#else` one, whose xcb type names stay bare
   and whose `free (error), rc = false;` is C's comma operator, which Rust has no spelling for
   (the two statements stand on their own lines; the `magic_db` note's class).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_get_wm_state_and_proto (dpyinfo: *mut x_display_info, window: Window,
					wmstate_out: *mut c_int, proto_out: *mut c_int,
					motif_out: *mut c_int, proxy_out: *mut Window) -> c_int
{
  #[cfg (not (feature = "use_xcb"))]
  use crate::x11_constants::{AnyPropertyType, Success};
  #[cfg (feature = "use_xcb")]
  use crate::x11_constants::{None, XA_ATOM, XA_WINDOW};

  #[cfg (not (feature = "use_xcb"))]
  let mut r#type: Atom;
  #[cfg (not (feature = "use_xcb"))]
  let mut format: c_int;
  #[cfg (not (feature = "use_xcb"))]
  let mut nitems: c_ulong;
  #[cfg (not (feature = "use_xcb"))]
  let mut bytes_after: c_ulong;
  #[cfg (not (feature = "use_xcb"))]
  let mut data: *mut u8 = std::ptr::null_mut ();
  #[cfg (not (feature = "use_xcb"))]
  let mut xm_info: xm_drag_receiver_info = std::mem::zeroed ();

  #[cfg (feature = "use_xcb")]
  let mut wmstate_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  #[cfg (feature = "use_xcb")]
  let mut xdnd_proto_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  #[cfg (feature = "use_xcb")]
  let mut xdnd_proxy_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  #[cfg (feature = "use_xcb")]
  let mut xm_style_cookie: xcb_get_property_cookie_t = std::mem::zeroed ();
  #[cfg (feature = "use_xcb")]
  let mut reply: *mut xcb_get_property_reply_t = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut error: *mut xcb_generic_error_t = std::ptr::null_mut ();
  #[cfg (feature = "use_xcb")]
  let mut xmdata: *mut u8 = std::ptr::null_mut ();

  let mut rc: c_int;

  #[cfg (not (feature = "use_xcb"))]
  {
    x_catch_errors ((*dpyinfo).display);
    rc = (XGetWindowProperty ((*dpyinfo).display, window,
			      (*dpyinfo).Xatom_wm_state,
			      0, 2, false, AnyPropertyType,
			      &raw mut r#type, &raw mut format, &raw mut nitems,
			      &raw mut bytes_after, &raw mut data)
	  == Success
	  && !x_had_errors_p ((*dpyinfo).display)
	  && !data.is_null () && nitems == 2 && format == 32) as c_int;
    x_uncatch_errors ();

    if rc != 0
      { *wmstate_out = *(data as *mut c_ulong) as c_int; }

    *proto_out = x_dnd_get_window_proto (dpyinfo, window);

    if xm_read_drag_receiver_info (dpyinfo, window, &raw mut xm_info) == 0
      { *motif_out = xm_info.protocol_style as c_int; }
    else
      { *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int; }

    *proxy_out = x_dnd_get_window_proxy (dpyinfo, window);

    if !data.is_null ()
      { XFree (data as *mut c_void); }
  }

  #[cfg (feature = "use_xcb")]
  {
    rc = 1;

    wmstate_cookie = xcb_get_property ((*dpyinfo).xcb_connection, 0,
				       window as xcb_window_t,
				       (*dpyinfo).Xatom_wm_state as xcb_atom_t,
				       0, 0, 2);
    xdnd_proto_cookie = xcb_get_property ((*dpyinfo).xcb_connection, 0,
					  window as xcb_window_t,
					  (*dpyinfo).Xatom_XdndAware as xcb_atom_t,
					  XA_ATOM, 0, 1);
    xdnd_proxy_cookie = xcb_get_property ((*dpyinfo).xcb_connection, 0,
					  window as xcb_window_t,
					  (*dpyinfo).Xatom_XdndProxy as xcb_atom_t,
					  XA_WINDOW, 0, 1);
    xm_style_cookie
      = xcb_get_property ((*dpyinfo).xcb_connection, 0,
			  window as xcb_window_t,
			  (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO as xcb_atom_t,
			  (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO as xcb_atom_t,
			  0, 4);

    reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
				    wmstate_cookie, &raw mut error);

    if reply.is_null ()
      {
	free (error as *mut c_void);
	rc = 0;
      }
    else
      {
	if (*reply).format != 32
	    || xcb_get_property_value_length (reply) != 8
	  { rc = 0; }
	else
	  { *wmstate_out = *(xcb_get_property_value (reply) as *mut u32) as c_int; }

	free (reply as *mut c_void);
      }

    reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
				    xdnd_proto_cookie, &raw mut error);

    *proto_out = -1;
    if reply.is_null ()
      { free (error as *mut c_void); }
    else
      {
	if (*reply).format == 32
	    && xcb_get_property_value_length (reply) >= 4
	  { *proto_out = *(xcb_get_property_value (reply) as *mut u32) as c_int; }

	free (reply as *mut c_void);
      }

    *proxy_out = None as Window;
    reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
				    xdnd_proxy_cookie, &raw mut error);

    if reply.is_null ()
      { free (error as *mut c_void); }
    else
      {
	if (*reply).format == 32
	    && (*reply).r#type == XA_WINDOW
	    && (xcb_get_property_value_length (reply) >= 4)
	  { *proxy_out = *(xcb_get_property_value (reply) as *mut xcb_window_t); }

	free (reply as *mut c_void);
      }

    *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;

    reply = xcb_get_property_reply ((*dpyinfo).xcb_connection,
				    xm_style_cookie, &raw mut error);

    if reply.is_null ()
      { free (error as *mut c_void); }
    else
      {
	if (*reply).format == 8
	    && (*reply).r#type == (*dpyinfo).Xatom_MOTIF_DRAG_RECEIVER_INFO
	    && xcb_get_property_value_length (reply) >= 4
	  {
	    xmdata = xcb_get_property_value (reply);
	    *motif_out = *xmdata.offset (2) as c_int;
	  }

	free (reply as *mut c_void);
      }
  }

  return rc;
}

/* From the XDND protocol specification:

   Dropping on windows that do not support XDND

   Since middle clicking is the universal shortcut for pasting
   in X, one can drop data into a window that does not support
   XDND by:

   1. After the mouse has been released to trigger the drop,
   obtain ownership of XA_PRIMARY.

   2. Send a ButtonPress event and then a ButtonRelease event to
   the deepest subwindow containing the mouse to simulate a
   middle click.  The times for these events should be the time
   of the actual button release +1 and +2, respectively.  These
   values will not be used by anybody else, so one can
   unambiguously recognize the resulting `XConvertSelection'
   request.

   3. If a request for XA_PRIMARY arrives bearing the timestamp
   of either the ButtonPress or the ButtonRelease event, treat
   it as a request for XdndSelection.  Note that you must use
   the X data types instead of the MIME types in this case.
   (e.g. XA_STRING instead of text/plain).  */

/* C 3985-4079.  `XA_PRIMARY`, `ButtonPress`, `ButtonRelease` and the two button masks are
   X11-header names the mirror's `x11_constants` module does not carry yet, so they stay bare
   (Decision 15's ruling; the measured values are in this ledger's Evidence).  Two declarations
   C takes from *other units'* headers sit in the body: `frame.h`'s
   `decode_window_system_frame` (`frame.c` defines it) and `xterm.h`'s `x_own_selection`
   (`xselect.c` defines it) - this unit's include graph reaches no mirror that carries either,
   and the transcription lands with the unit that needs it (the `include/xterm.rs` prototype
   note; the `font.rs` 413-418 and `keyboard.rs` `haiku_handle_selection_clear` shapes).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_do_unsupported_drop (dpyinfo: *mut x_display_info, frame: Lisp_Object,
				     value: Lisp_Object, targets: Lisp_Object,
				     target_window: Window, root_x: c_int,
				     root_y: c_int, before: Time)
{
  use crate::x11_constants::None;

  unsafe extern "C" {
    /// C: frame.h 1474-1475: `extern WINDOW_SYSTEM_RETURN struct frame *decode_window_system_frame (Lisp_Object);`
    pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
    /// C: xterm.h 1947-1948: `extern void x_own_selection (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Time);`
    pub fn x_own_selection (selection_name: Lisp_Object, selection_value: Lisp_Object,
			    frame: Lisp_Object, dnd_data: Lisp_Object, timestamp: Time);
  }

  let mut event: XEvent = std::mem::zeroed ();
  let mut dest_x: c_int;
  let mut dest_y: c_int;
  let mut child_return: Window = 0;
  let mut child: Window;
  let mut owner: Window;
  let mut current_value: Lisp_Object;
  let mut f: *mut frame;

  f = decode_window_system_frame (frame);

  if NILP (value)
    { return; }

  if !x_dnd_use_unsupported_drop
    { return; }

  event.xbutton.serial = 0;
  event.xbutton.send_event = true;
  event.xbutton.display = (*dpyinfo).display;
  event.xbutton.root = (*dpyinfo).root_window;
  event.xbutton.x_root = root_x;
  event.xbutton.y_root = root_y;

  x_catch_errors ((*dpyinfo).display);

  child = (*dpyinfo).root_window;
  dest_x = root_x;
  dest_y = root_y;

  while XTranslateCoordinates ((*dpyinfo).display, (*dpyinfo).root_window,
			       child, root_x, root_y, &raw mut dest_x, &raw mut dest_y,
			       &raw mut child_return) != 0
      && child_return != None as Window
    { child = child_return; }

  x_uncatch_errors ();

  if !CONSP (value)
    { return; }

  current_value = assq_no_quit (QPRIMARY (),
				(*(*dpyinfo).terminal).Vselection_alist);

  if !NILP (current_value)
    { current_value = XCAR (XCDR (current_value)); }

  x_own_selection (QPRIMARY (), current_value, frame,
		   XCAR (XCDR (value)), before);

  owner = XGetSelectionOwner ((*dpyinfo).display, XA_PRIMARY);

  /* If we didn't successfully obtain selection ownership, refrain
     from generating events that will insert something else.  */

  if owner != FRAME_X_WINDOW! (f)
    { return; }

  /* mouse-drag-and-drop-region will immediately deactivate the mark
     after this is set.  Make sure the primary selection is not
     clobbered in that case by setting `deactivate-mark' to
     Qdont_save.  */
  Vdeactivate_mark = Qdont_save ();

  event.xbutton.window = child;
  event.xbutton.subwindow = None as Window;
  event.xbutton.x = dest_x;
  event.xbutton.y = dest_y;
  event.xbutton.state = 0;
  event.xbutton.button = 2;
  event.xbutton.same_screen = true;

  (*dpyinfo).pending_dnd_time = before;

  event.xbutton.r#type = ButtonPress;
  event.xbutton.time = before + 1;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent ((*dpyinfo).display, child,
	      true, ButtonPressMask, &raw mut event);

  event.xbutton.r#type = ButtonRelease;
  event.xbutton.time = before + 2;

  XSendEvent ((*dpyinfo).display, child,
	      true, ButtonReleaseMask, &raw mut event);
  x_stop_ignoring_errors (dpyinfo);

  x_dnd_action_symbol = QXdndActionPrivate ();

  return;
}

/* C 4081-4125.  C's `alloca` is the port's stack pool (PORTING.md P-05).  `x_get_atom_name` is
   declared in xterm.h 1844 and defined in xterm.c 29984, so the port resolves it by name.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_send_unsupported_drop (dpyinfo: *mut x_display_info, target_window: Window,
				       root_x: c_int, root_y: c_int, before: Time)
{
  let mut targets: Lisp_Object;
  let mut arg: Lisp_Object;
  let mut i: c_int;
  let mut atom_names: *mut *mut c_char;
  let mut name: *mut c_char;

  targets = Qnil ();
  atom_names = sa_stack_alloc (std::mem::size_of::<*mut c_char> ()
			       * x_dnd_n_targets as usize) as *mut *mut c_char;

  if XGetAtomNames ((*dpyinfo).display, x_dnd_targets,
		    x_dnd_n_targets, atom_names) == 0
    { return; }

  i = x_dnd_n_targets;
  while i > 0
    {
      targets = Fcons (build_string (*atom_names.offset ((i - 1) as isize)),
		       targets);
      XFree (*atom_names.offset ((i - 1) as isize) as *mut c_void);
      i -= 1;
    }

  name = x_get_atom_name (dpyinfo, x_dnd_wanted_action,
			  std::ptr::null_mut ());

  if !name.is_null ()
    {
      arg = intern (name);
      xfree (name as *mut c_void);
    }
  else
    { arg = Qnil (); }

  x_dnd_run_unsupported_drop_function = true;
  x_dnd_unsupported_drop_time = before;
  x_dnd_unsupported_drop_window = target_window;
  x_dnd_unsupported_drop_data
    = list! (assq_no_quit (QXdndSelection (),
			   (*(*dpyinfo).terminal).Vselection_alist),
	     targets, arg, make_fixnum (root_x as EMACS_INT),
	     make_fixnum (root_y as EMACS_INT));

  x_dnd_waiting_for_finish = true;
  x_dnd_finish_display = (*dpyinfo).display;
}

/* C 4127-4137.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_fill_empty_target (proto_out: *mut c_int, motif_out: *mut c_int,
				   toplevel_out: *mut Window, was_frame: *mut bool) -> Window
{
  use crate::x11_constants::None;

  *proto_out = -1;
  *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;
  *toplevel_out = None as Window;
  *was_frame = false;

  return None as Window;
}

/* C 4139-4466: the target-window search - the toplevel list first (with the composite overlay
   window between it and the root-window proxy), then the child-window walk.  C's
   `#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOMPOSITE_MINOR > 2)` names an X11
   header's major/minor pair, which this tree cannot see, so that arm is carried under the
   derived feature `have_xcomposite_overlay` beside `have_xcomposite` - the
   `have_xcb_shape_input_rects` shape of Decision 3.  The fields that arm reads
   (`composite_supported_p`, `composite_major`, `composite_minor`, xterm.h 922-924) are not in
   the mirror yet either; the gap is recorded in this round's report.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_get_target_window (dpyinfo: *mut x_display_info, root_x: c_int,
				   root_y: c_int, proto_out: *mut c_int,
				   motif_out: *mut c_int, toplevel_out: *mut Window,
				   was_frame: *mut bool) -> Window
{
  use crate::x11_constants::None;

  let mut child_return: Window;
  let mut child: Window;
  let mut proxy: Window;
  let mut dest_x_return: c_int;
  let mut dest_y_return: c_int;
  let mut rc: c_int;
  let mut proto: c_int;
  let mut motif: c_int;
  let mut parent_x: c_int;
  let mut parent_y: c_int;
  let mut extents_p: bool;

  #[cfg (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay"))]
  let mut overlay_window: Window = 0;
  #[cfg (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay"))]
  let mut attrs: XWindowAttributes = std::mem::zeroed ();

  let mut wmstate: c_int;
  let mut tooltip: *mut frame;
  let mut f: *mut frame;
  let mut unrelated: bool;

  child_return = (*dpyinfo).root_window;
  dest_x_return = root_x;
  dest_y_return = root_y;

  proto = -1;
  *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;
  *toplevel_out = None as Window;
  *was_frame = false;

  if x_dnd_use_toplevels
    {
      extents_p = false;
      child = x_dnd_get_target_window_1 (dpyinfo, root_x,
					 root_y, motif_out,
					 &raw mut extents_p);

      if !x_dnd_allow_current_frame
	  && FRAME_X_WINDOW! (x_dnd_frame) == child
	{ *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int; }

      f = x_top_window_to_frame (dpyinfo, child);

      *toplevel_out = child;

      if child != None as Window
	{
	  if !f.is_null ()
	    {
	      *was_frame = true;
	      *proto_out = -1;
	      *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;

	      return child;
	    }

	  #[cfg (not (feature = "use_xcb"))]
	  { proxy = x_dnd_get_window_proxy (dpyinfo, child); }
	  #[cfg (feature = "use_xcb")]
	  { x_dnd_get_proxy_proto (dpyinfo, child, &raw mut proxy, proto_out); }

	  if proxy != None as Window
	    {
	      proto = x_dnd_get_window_proto (dpyinfo, proxy);

	      if proto != -1
		{
		  *proto_out = proto;
		  return proxy;
		}
	    }

	  #[cfg (not (feature = "use_xcb"))]
	  { *proto_out = x_dnd_get_window_proto (dpyinfo, child); }
	  return child;
	}

      if extents_p
	{
	  *proto_out = -1;
	  *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;
	  *toplevel_out = None as Window;

	  return None as Window;
	}

      /* Then look at the composite overlay window.  */
      #[cfg (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay"))]
      {
	if (*dpyinfo).composite_supported_p
	    && ((*dpyinfo).composite_major > 0
		|| (*dpyinfo).composite_minor > 2)
	  {
	    if XGetSelectionOwner ((*dpyinfo).display,
				   (*dpyinfo).Xatom_NET_WM_CM_Sn) != None as Window
	      {
		x_catch_errors ((*dpyinfo).display);
		XGrabServer ((*dpyinfo).display);
		overlay_window = XCompositeGetOverlayWindow ((*dpyinfo).display,
							     (*dpyinfo).root_window);
		XCompositeReleaseOverlayWindow ((*dpyinfo).display,
						(*dpyinfo).root_window);
		XUngrabServer ((*dpyinfo).display);

		if !x_had_errors_p ((*dpyinfo).display)
		  {
		    XGetWindowAttributes ((*dpyinfo).display, overlay_window,
					  &raw mut attrs);

		    if attrs.map_state == IsViewable
		      {
			proxy = x_dnd_get_window_proxy (dpyinfo, overlay_window);

			if proxy != None as Window
			  {
			    proto = x_dnd_get_window_proto (dpyinfo, proxy);

			    if proto != -1
			      {
				*proto_out = proto;
				*toplevel_out = overlay_window;
				x_uncatch_errors_after_check ();

				return proxy;
			      }
			  }
		      }
		  }
		x_uncatch_errors_after_check ();
	      }
	  }
      }

      /* Now look for an XdndProxy on the root window.  */

      proxy = x_dnd_get_window_proxy (dpyinfo, (*dpyinfo).root_window);

      if proxy != None as Window
	{
	  proto = x_dnd_get_window_proto (dpyinfo, (*dpyinfo).root_window);

	  if proto != -1
	    {
	      *toplevel_out = (*dpyinfo).root_window;
	      *proto_out = proto;
	      return proxy;
	    }
	}

      /* No toplevel was found and the overlay and root windows were
	 not proxies, so return None.  */
      *proto_out = -1;
      *toplevel_out = (*dpyinfo).root_window;
      return None as Window;
    }

  /* Not strictly necessary, but satisfies GCC.  */
  child = (*dpyinfo).root_window;

  while child_return != None as Window
    {
      child = child_return;
      parent_x = dest_x_return;
      parent_y = dest_y_return;

      x_catch_errors ((*dpyinfo).display);
      rc = XTranslateCoordinates ((*dpyinfo).display, (*dpyinfo).root_window,
				  child_return, root_x, root_y, &raw mut dest_x_return,
				  &raw mut dest_y_return, &raw mut child_return);

      if x_had_errors_p ((*dpyinfo).display) || rc == 0
	{
	  x_uncatch_errors_after_check ();
	  break;
	}

      if child_return != 0
	{
	  /* If child_return is a tooltip frame, look beneath it.  We
	     never want to drop anything onto a tooltip frame.  */

	  tooltip = x_tooltip_window_to_frame (dpyinfo, child_return,
					       &raw mut unrelated);

	  if !tooltip.is_null () || unrelated
	    { child_return = x_get_window_below ((*dpyinfo).display, child_return,
						 parent_x, parent_y,
						 &raw mut dest_x_return,
						 &raw mut dest_y_return); }

	  if child_return == 0
	    {
	      x_uncatch_errors ();
	      break;
	    }

	  f = x_top_window_to_frame (dpyinfo, child_return);

	  if !f.is_null ()
	    {
	      *proto_out = -1;
	      *motif_out = xm_drag_style::XM_DRAG_STYLE_NONE as c_int;
	      *toplevel_out = child_return;
	      *was_frame = true;

	      return child_return;
	    }

	  if x_dnd_get_wm_state_and_proto (dpyinfo, child_return,
					   &raw mut wmstate, &raw mut proto,
					   &raw mut motif, &raw mut proxy) != 0
	      /* `proto' and `motif' are set by x_dnd_get_wm_state
		 even if getting the wm state failed.  */
	      || proto != -1 || motif != xm_drag_style::XM_DRAG_STYLE_NONE as c_int
	    {
	      *proto_out = proto;
	      *motif_out = if x_dnd_disable_motif_protocol
		{ xm_drag_style::XM_DRAG_STYLE_NONE as c_int }
		else
		  { motif };
	      *toplevel_out = child_return;
	      x_uncatch_errors ();

	      return child_return;
	    }

	  if proxy != None as Window
	    {
	      proto = x_dnd_get_window_proto (dpyinfo, proxy);

	      if proto != -1
		{
		  *proto_out = proto;
		  *toplevel_out = child_return;

		  x_uncatch_errors ();
		  return proxy;
		}
	    }
	}

      x_uncatch_errors ();
    }

  /* C writes this body twice - once inside the composite arm's
     `if (child != dpyinfo->root_window)` and once after it; the port keeps both copies.  */
  #[cfg (not (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay")))]
  {
    if child != None as Window
      {
	proxy = x_dnd_get_window_proxy (dpyinfo, child);

	if proxy != 0
	  {
	    proto = x_dnd_get_window_proto (dpyinfo, proxy);

	    if proto != -1
	      {
		*proto_out = proto;
		*toplevel_out = child;
		return proxy;
	      }
	  }
      }

    *proto_out = x_dnd_get_window_proto (dpyinfo, child);
    return child;
  }

  #[cfg (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay"))]
  {
    if child != (*dpyinfo).root_window
      {
	if child != None as Window
	  {
	    proxy = x_dnd_get_window_proxy (dpyinfo, child);

	    if proxy != 0
	      {
		proto = x_dnd_get_window_proto (dpyinfo, proxy);

		if proto != -1
		  {
		    *proto_out = proto;
		    *toplevel_out = child;
		    return proxy;
		  }
	      }
	  }

	*proto_out = x_dnd_get_window_proto (dpyinfo, child);
	return child;
      }
    else if (*dpyinfo).composite_supported_p
	&& ((*dpyinfo).composite_major > 0
	    || (*dpyinfo).composite_minor > 2)
      {
	/* Only do this if a compositing manager is present.  */
	if XGetSelectionOwner ((*dpyinfo).display,
			       (*dpyinfo).Xatom_NET_WM_CM_Sn) != None as Window
	  {
	    x_catch_errors ((*dpyinfo).display);
	    XGrabServer ((*dpyinfo).display);
	    overlay_window = XCompositeGetOverlayWindow ((*dpyinfo).display,
							 (*dpyinfo).root_window);
	    XCompositeReleaseOverlayWindow ((*dpyinfo).display,
					    (*dpyinfo).root_window);
	    XUngrabServer ((*dpyinfo).display);

	    if !x_had_errors_p ((*dpyinfo).display)
	      {
		XGetWindowAttributes ((*dpyinfo).display, overlay_window, &raw mut attrs);

		if attrs.map_state == IsViewable
		  {
		    proxy = x_dnd_get_window_proxy (dpyinfo, overlay_window);

		    if proxy != None as Window
		      {
			proto = x_dnd_get_window_proto (dpyinfo, proxy);

			if proto != -1
			  {
			    *proto_out = proto;
			    *toplevel_out = overlay_window;
			    x_uncatch_errors_after_check ();

			    return proxy;
			  }
		      }
		  }
	      }
	    x_uncatch_errors_after_check ();
	  }
      }

    if child != None as Window
      {
	proxy = x_dnd_get_window_proxy (dpyinfo, child);

	if proxy != 0
	  {
	    proto = x_dnd_get_window_proto (dpyinfo, proxy);

	    if proto != -1
	      {
		*toplevel_out = child;
		*proto_out = proto;
		return proxy;
	      }
	  }
      }

    *proto_out = x_dnd_get_window_proto (dpyinfo, child);
    *toplevel_out = child;
    return child;
  }
}

/* ---- Slice 8: C 4469-6989 - the Xdnd senders and the self-drop path, the miscellaneous
   event/graphics-extension functions (`x_flush`, the GC-ext-data helpers, the XInput2 device
   plumbing, the cairo and XRender drawing helpers), the Display Iterators' rectangles, and the
   frame-property functions up to `x_set_frame_alpha`.  Slices 3-7's conventions hold: every
   configuration arm is carried under its feature (P-51), Xlib's own names stay bare as this
   file's header mirror keeps them (they resolve on an X11 target and nowhere here), C's
   uninitialised locals take the port's zero (PORTING.md 1.5), C's `alloca` is the port's stack
   pool (PORTING.md P-05) and the widenings C performs implicitly are written out (Decision 14).
   The `.xclient`/`.xbutton` field writes are C's own shape, which is also slices 2 and 5's: the
   mirror's `XEvent` is opaque, so those members are one recorded class of the flip arm's errors
   (Decision 19 and this round's ledger entry).  */

/* C 4468-4508: the XdndProxy query.  C's `(Window *) tmp_data` read is a pointer pun, and the
   `if (tmp_data)` tests are the port's `is_null`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_get_window_proxy (dpyinfo: *mut x_display_info, wdesc: Window) -> Window
{
  use crate::x11_constants::{None, Success, XA_WINDOW};

  let mut rc: c_int;
  let mut actual_format: c_int;
  let mut actual_size: c_ulong;
  let mut bytes_remaining: c_ulong;
  let mut tmp_data: *mut u8 = std::ptr::null_mut ();
  let mut attrs: XWindowAttributes = std::mem::zeroed ();
  let mut actual_type: Atom;
  let mut proxy: Window;

  proxy = None as Window;
  x_catch_errors ((*dpyinfo).display);
  rc = XGetWindowProperty ((*dpyinfo).display, wdesc,
			   (*dpyinfo).Xatom_XdndProxy,
			   0, 1, false, XA_WINDOW,
			   &raw mut actual_type, &raw mut actual_format,
			   &raw mut actual_size, &raw mut bytes_remaining,
			   &raw mut tmp_data);

  if !x_had_errors_p ((*dpyinfo).display)
      && rc == Success
      && !tmp_data.is_null ()
      && actual_type == XA_WINDOW
      && actual_format == 32
      && actual_size == 1
    {
      proxy = *(tmp_data as *mut Window);

      /* Verify the proxy window exists.  */
      XGetWindowAttributes ((*dpyinfo).display, proxy, &raw mut attrs);

      if x_had_errors_p ((*dpyinfo).display)
	{ proxy = None as Window; }
    }

  if !tmp_data.is_null ()
    { XFree (tmp_data as *mut c_void); }
  x_uncatch_errors_after_check ();

  return proxy;
}

/* C 4510-4542: the XdndAware property's version byte.  `min` is the lisp.rs `min (isize,
   isize)`, so both arguments are the port's written-out widening; `had_errors` is C's `bool`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_get_window_proto (dpyinfo: *mut x_display_info, wdesc: Window) -> c_int
{
  use crate::x11_constants::{None, Success, XA_ATOM};

  let mut actual: Atom;
  let mut value: Atom;
  let mut tmp_data: *mut u8 = std::ptr::null_mut ();
  let mut rc: c_int;
  let mut format: c_int;
  let mut n: c_ulong;
  let mut left: c_ulong;
  let mut had_errors: bool;

  if wdesc == None as Window || (x_dnd_allow_current_frame == false
				 && wdesc == FRAME_OUTER_WINDOW! (x_dnd_frame))
    { return -1; }

  x_catch_errors ((*dpyinfo).display);
  rc = XGetWindowProperty ((*dpyinfo).display, wdesc, (*dpyinfo).Xatom_XdndAware,
			   0, 1, false, XA_ATOM, &raw mut actual, &raw mut format,
			   &raw mut n, &raw mut left,
			   &raw mut tmp_data);
  had_errors = x_had_errors_p ((*dpyinfo).display);
  x_uncatch_errors_after_check ();

  if had_errors || rc != Success || actual != XA_ATOM || format != 32 || n < 1
      || tmp_data.is_null ()
    {
      if !tmp_data.is_null ()
	{ XFree (tmp_data as *mut c_void); }
      return -1;
    }

  value = *(tmp_data as *mut Atom);
  XFree (tmp_data as *mut c_void);

  return min (X_DND_SUPPORTED_VERSION as isize, value as isize) as c_int;
}

/* C 4544-4580: the XdndEnter message.  C's `(unsigned int)` shift is written out as `c_uint`
   (the type is the port's own spelling of C's cast), and the `?:` is the port's `if`/`else`
   (PORTING.md 2.1).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_send_enter (f: *mut frame, target: Window, toplevel: Window,
			    supported: c_int)
{
  use crate::x11_constants::{ClientMessage, NoEventMask, PropModeReplace, XA_ATOM};

  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut i: c_int;
  let mut msg: XEvent = std::mem::zeroed ();

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_XdndEnter;
  msg.xclient.format = 32;
  msg.xclient.window = toplevel;
  msg.xclient.data.l[0] = FRAME_X_WINDOW! (f) as c_long;
  msg.xclient.data.l[1] = (((min (X_DND_SUPPORTED_VERSION as isize,
				 supported as isize) as c_int) as c_uint) << 24)
			   | (if x_dnd_n_targets > 3 { 1u32 } else { 0u32 }) as c_uint;
  msg.xclient.data.l[2] = 0;
  msg.xclient.data.l[3] = 0;
  msg.xclient.data.l[4] = 0;

  for i in 0..(min (3, x_dnd_n_targets as isize) as c_int)
    { msg.xclient.data.l[(i + 2) as usize] = *x_dnd_targets.offset (i as isize); }

  if x_dnd_n_targets > 3 && x_dnd_init_type_lists == false
    {
      XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
		       (*dpyinfo).Xatom_XdndTypeList, XA_ATOM, 32,
		       PropModeReplace, x_dnd_targets as *mut u8,
		       x_dnd_n_targets);
    }

  /* Now record that the type list has already been set (if required),
     so we don't have to set it again.  */
  x_dnd_init_type_lists = true;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent (FRAME_X_DISPLAY! (f), target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
}

/* C 4582-4655: the XdndPosition message, including the two waits (`x_dnd_pending_send_position`
   keeps the message and the mouse-rectangle test suppresses the send).  C's `|=` targets are
   `long` fields, so the two shifts are widened and the `state & 0xff` mask is C's.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_send_position (f: *mut frame, target: Window, toplevel: Window,
			       supported: c_int, root_x: c_ushort,
			       root_y: c_ushort, timestamp: Time, action: Atom,
			       button: c_int, state: c_uint)
{
  use crate::x11_constants::{ClientMessage, NoEventMask};

  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut msg: XEvent = std::mem::zeroed ();

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_XdndPosition;
  msg.xclient.format = 32;
  msg.xclient.window = toplevel;
  msg.xclient.data.l[0] = FRAME_X_WINDOW! (f) as c_long;
  msg.xclient.data.l[1] = 0;

  /* This is problematic because it's not specified in the
     freedesktop.org copy of the protocol specification, but the copy
     maintained by the original author of the protocol specifies it
     for all versions.  Since at least one program supports these
     flags, but uses protocol v4 (and not v5), set them for all
     protocol versions.  */
  if button >= 4 && button <= 7
    {
      msg.xclient.data.l[1] |= (1 << 10);
      msg.xclient.data.l[1] |= ((button - 4) as c_long) << 8;
    }
  else if button != 0
    { return; }

  msg.xclient.data.l[1] |= (state & 0xff) as c_long;

  msg.xclient.data.l[2] = (((root_x as c_uint) << 16) | root_y as c_uint) as c_long;
  msg.xclient.data.l[3] = 0;
  msg.xclient.data.l[4] = 0;

  if supported >= 3
    { msg.xclient.data.l[3] = timestamp as c_long; }

  if supported >= 4
    { msg.xclient.data.l[4] = action as c_long; }

  if x_dnd_waiting_for_status_window == target
    {
      x_dnd_pending_send_position = msg;
      x_dnd_pending_send_position_button = button != 0;
      x_dnd_pending_send_position_root_x = root_x as c_int;
      x_dnd_pending_send_position_root_y = root_y as c_int;
    }
  else
    {
      if target == x_dnd_mouse_rect_target
	  && x_dnd_mouse_rect.width != 0
	  && x_dnd_mouse_rect.height != 0
	  /* Ignore the mouse rectangle if we're supposed to be sending a
	     button press instead.  */
	  && button == 0
	{
	  if root_x as c_int >= x_dnd_mouse_rect.x as c_int
	      && (root_x as c_int) < (x_dnd_mouse_rect.x as c_int
				      + x_dnd_mouse_rect.width as c_int)
	      && root_y as c_int >= x_dnd_mouse_rect.y as c_int
	      && (root_y as c_int) < (x_dnd_mouse_rect.y as c_int
				      + x_dnd_mouse_rect.height as c_int)
	    { return; }
	}

      x_ignore_errors_for_next_request (dpyinfo, 0);
      XSendEvent (FRAME_X_DISPLAY! (f), target, false, NoEventMask, &raw mut msg);
      x_stop_ignoring_errors (dpyinfo);

      x_dnd_waiting_for_status_window = target;
    }
}

/* C 4657-4679: the XdndLeave message.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_send_leave (f: *mut frame, target: Window, toplevel: Window)
{
  use crate::x11_constants::{ClientMessage, NoEventMask, None};

  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut msg: XEvent = std::mem::zeroed ();

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_XdndLeave;
  msg.xclient.format = 32;
  msg.xclient.window = toplevel;
  msg.xclient.data.l[0] = FRAME_X_WINDOW! (f) as c_long;
  msg.xclient.data.l[1] = 0;
  msg.xclient.data.l[2] = 0;
  msg.xclient.data.l[3] = 0;
  msg.xclient.data.l[4] = 0;

  x_dnd_waiting_for_status_window = None as Window;
  x_dnd_pending_send_position.r#type = 0;

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent (FRAME_X_DISPLAY! (f), target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
}

/* C 4681-4713: the XdndDrop message; the early return is C's `x_dnd_action == None`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_send_drop (f: *mut frame, target: Window, toplevel: Window,
			   timestamp: Time, supported: c_int) -> bool
{
  use crate::x11_constants::{ClientMessage, NoEventMask, None};

  let mut dpyinfo: *mut x_display_info;
  let mut msg: XEvent = std::mem::zeroed ();

  if x_dnd_action == None as Atom
    {
      x_dnd_send_leave (f, target, toplevel);
      return false;
    }

  dpyinfo = FRAME_DISPLAY_INFO! (f);

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type = (*dpyinfo).Xatom_XdndDrop;
  msg.xclient.format = 32;
  msg.xclient.window = toplevel;
  msg.xclient.data.l[0] = FRAME_X_WINDOW! (f) as c_long;
  msg.xclient.data.l[1] = 0;
  msg.xclient.data.l[2] = 0;
  msg.xclient.data.l[3] = 0;
  msg.xclient.data.l[4] = 0;

  if supported >= 1
    { msg.xclient.data.l[2] = timestamp as c_long; }

  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent (FRAME_X_DISPLAY! (f), target, false, NoEventMask, &raw mut msg);
  x_stop_ignoring_errors (dpyinfo);
  return true;
}

/* C 4715-4726: send the drop now, or note that it has to follow the status message.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_do_drop (target: Window, toplevel: Window, supported: c_int) -> bool
{
  if x_dnd_waiting_for_status_window != target
    { return x_dnd_send_drop (x_dnd_frame, target, toplevel,
			      x_dnd_selection_timestamp, supported); }

  x_dnd_need_send_drop = true;
  x_dnd_send_drop_proto = supported;

  return true;
}

/* C 4728-4741: replace the selection targets the drag offers.  C's `sizeof *targets` is the
   port's `size_of::<Atom> ()` (the `sizeof` family) and `memcpy` its `copy_nonoverlapping`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_dnd_targets (targets: *mut Atom, ntargets: c_int)
{
  if !x_dnd_targets.is_null ()
    { xfree (x_dnd_targets as *mut c_void); }

  block_input ();
  x_dnd_targets = xmalloc (std::mem::size_of::<Atom> () * ntargets as usize) as *mut Atom;
  x_dnd_n_targets = ntargets;

  std::ptr::copy_nonoverlapping (targets, x_dnd_targets, ntargets as usize);
  unblock_input ();
}

/* C 4743-4755: free them.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_free_dnd_targets ()
{
  if x_dnd_targets.is_null ()
    { return; }

  xfree (x_dnd_targets as *mut c_void);
  x_dnd_targets = std::ptr::null_mut ();
  x_dnd_n_targets = 0;
}

/* Clear some Lisp variables after the drop finishes, so they are
   freed by the GC.  */

/* C 4757-4762.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_clear_dnd_variables ()
{
  x_dnd_monitors = Qnil ();
  x_dnd_unsupported_drop_data = Qnil ();
}

/* C 4764-4776.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_free_dnd_toplevels ()
{
  if x_dnd_use_toplevels == false || x_dnd_toplevels.is_null ()
    { return; }

  /* If the display is deleted, x_dnd_toplevels will already be
     NULL, so we can always assume the display is alive here.  */

  x_dnd_free_toplevels (true);
}

/* Restore event masks and window properties changed during a
   drag-and-drop operation, after it finishes.  */
/* C 4778-4802.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_restore_events_after_dnd (f: *mut frame, wa: *mut XWindowAttributes)
{
  let mut dpyinfo: *mut x_display_info;

  dpyinfo = FRAME_DISPLAY_INFO! (f);

  /* Restore the old event mask.  */
  XSelectInput ((*dpyinfo).display, (*dpyinfo).root_window,
		(*wa).your_event_mask);
  #[cfg (feature = "have_xkb")]
  {
    if (*dpyinfo).supports_xkb
      { XkbSelectEvents ((*dpyinfo).display, XkbUseCoreKbd,
			 XkbStateNotifyMask, 0); }
  }
  /* Delete the Motif drag initiator info if it was set up.  */
  if x_dnd_motif_setup_p
    { XDeleteProperty ((*dpyinfo).display, FRAME_X_WINDOW! (f),
		       x_dnd_motif_atom); }

  /* Remove any type list set as well.  */
  if x_dnd_init_type_lists && x_dnd_n_targets > 3
    { XDeleteProperty ((*dpyinfo).display, FRAME_X_WINDOW! (f),
		       (*dpyinfo).Xatom_XdndTypeList); }
}

/* C 4804-4864: `#ifdef HAVE_XINPUT2` - the early cancellation of a drag, dead in the frozen
   reading and carried under its feature (P-51).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn x_dnd_cancel_dnd_early ()
{
  use crate::x11_constants::None;

  let mut f: *mut frame;
  let mut dmsg: xm_drop_start_message = std::mem::zeroed ();

  eassert! (x_dnd_frame.is_null () == false && x_dnd_in_progress);

  f = x_dnd_frame;

  if x_dnd_last_seen_window != None as Window
      && x_dnd_last_protocol_version != -1
    { x_dnd_send_leave (x_dnd_frame,
			x_dnd_last_seen_window,
			x_dnd_last_seen_toplevel); }
  else if x_dnd_last_seen_window != None as Window
	   && XM_DRAG_STYLE_IS_DROP_ONLY! (x_dnd_last_motif_style) == false
	   && x_dnd_last_motif_style != xm_drag_style::XM_DRAG_STYLE_NONE as c_int
	   && x_dnd_motif_setup_p
    {
      dmsg.reason = XM_DRAG_REASON! (xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR
				     as c_int,
				     xm_drag_reason::XM_DRAG_REASON_DROP_START
				     as c_int) as u8;
      dmsg.byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;
      dmsg.timestamp = (*FRAME_DISPLAY_INFO! (f)).last_user_time as u32;
      dmsg.side_effects
	= XM_DRAG_SIDE_EFFECT! (xm_side_effect_from_action (FRAME_DISPLAY_INFO! (f),
							   x_dnd_wanted_action),
			       xm_drop_site_status::XM_DROP_SITE_VALID as c_int,
			       x_dnd_motif_operations as c_int,
			       xm_drag_action::XM_DROP_ACTION_DROP_CANCEL
			       as c_int) as u16;
      dmsg.x = 0;
      dmsg.y = 0;
      dmsg.index_atom = x_dnd_motif_atom as u32;
      dmsg.source_window = FRAME_X_WINDOW! (f) as u32;

      x_dnd_send_xm_leave_for_drop (FRAME_DISPLAY_INFO! (f), f,
				    x_dnd_last_seen_window,
				    (*FRAME_DISPLAY_INFO! (f)).last_user_time);
      xm_send_drop_message (FRAME_DISPLAY_INFO! (f), FRAME_X_WINDOW! (f),
			    x_dnd_last_seen_window, &raw mut dmsg);
    }

  x_dnd_last_seen_window = None as Window;
  x_dnd_last_seen_toplevel = None as Window;
  x_dnd_in_progress = false;
  x_dnd_waiting_for_finish = false;
  x_dnd_return_frame_object = std::ptr::null_mut ();
  x_dnd_movement_frame = std::ptr::null_mut ();
  x_dnd_wheel_frame = std::ptr::null_mut ();
  x_dnd_frame = std::ptr::null_mut ();
  x_dnd_action = None as Atom;
  x_dnd_action_symbol = Qnil ();
}

/* C 4866-4919: the unwind-protect cleanup.  C's `void *frame` parameter is the port's cast, and
   `FRAME_DISPLAY_INFO (f)->grabbed` is the one macro expansion C writes with the field.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_cleanup_drag_and_drop (frame: *mut c_void)
{
  use crate::x11_constants::None;

  let f: *mut frame = frame as *mut frame;
  let mut dmsg: xm_drop_start_message = std::mem::zeroed ();

  if x_dnd_unwind_flag == false
    { return; }

  if x_dnd_in_progress
    {
      eassert! (x_dnd_frame.is_null () == false);

      block_input ();
      if x_dnd_last_seen_window != None as Window
	  && x_dnd_last_protocol_version != -1
	{ x_dnd_send_leave (x_dnd_frame, x_dnd_last_seen_window,
			    x_dnd_last_seen_toplevel); }
      else if x_dnd_last_seen_window != None as Window
	       && XM_DRAG_STYLE_IS_DROP_ONLY! (x_dnd_last_motif_style) == false
	       && x_dnd_last_motif_style != xm_drag_style::XM_DRAG_STYLE_NONE as c_int
	       && x_dnd_motif_setup_p
	{
	  dmsg.reason = XM_DRAG_REASON! (xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR
					 as c_int,
					 xm_drag_reason::XM_DRAG_REASON_DROP_START
					 as c_int) as u8;
	  dmsg.byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;
	  dmsg.timestamp = (*FRAME_DISPLAY_INFO! (f)).last_user_time as u32;
	  dmsg.side_effects
	    = XM_DRAG_SIDE_EFFECT! (xm_side_effect_from_action (FRAME_DISPLAY_INFO! (f),
							       x_dnd_wanted_action),
				   xm_drop_site_status::XM_DROP_SITE_VALID as c_int,
				   x_dnd_motif_operations as c_int,
				   xm_drag_action::XM_DROP_ACTION_DROP_CANCEL
				   as c_int) as u16;
	  dmsg.x = 0;
	  dmsg.y = 0;
	  dmsg.index_atom = x_dnd_motif_atom as u32;
	  dmsg.source_window = FRAME_X_WINDOW! (f) as u32;

	  x_dnd_send_xm_leave_for_drop (FRAME_DISPLAY_INFO! (f), f,
					x_dnd_last_seen_window,
					(*FRAME_DISPLAY_INFO! (f)).last_user_time);
	  xm_send_drop_message (FRAME_DISPLAY_INFO! (f), FRAME_X_WINDOW! (f),
				x_dnd_last_seen_window, &raw mut dmsg);
	}
      unblock_input ();

      x_dnd_end_window = x_dnd_last_seen_window;
      x_dnd_last_seen_window = None as Window;
      x_dnd_last_seen_toplevel = None as Window;
      x_dnd_in_progress = false;
    }

  x_dnd_waiting_for_finish = false;

  (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;
  #[cfg (feature = "use_gtk")]
  { current_hold_quit = std::ptr::null_mut (); }
  x_dnd_return_frame_object = std::ptr::null_mut ();
  x_dnd_movement_frame = std::ptr::null_mut ();
  x_dnd_wheel_frame = std::ptr::null_mut ();
  x_dnd_frame = std::ptr::null_mut ();

  x_restore_events_after_dnd (f, &raw mut x_dnd_old_window_attrs);
}

/* Primitives for simplified drag-and-drop tracking when items are
   being dragged between frames comprising the same Emacs session.  */

/* C 4937-4956.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_note_self_position (dpyinfo: *mut x_display_info, target: Window,
				    root_x: c_ushort, root_y: c_ushort)
{
  let mut f: *mut frame;
  let mut dest_x: c_int;
  let mut dest_y: c_int;

  f = x_top_window_to_frame (dpyinfo, target);

  if !f.is_null ()
    {
      x_translate_coordinates (f, root_x as c_int, root_y as c_int,
			       &raw mut dest_x, &raw mut dest_y);

      x_dnd_movement_frame = f;
      x_dnd_movement_x = dest_x;
      x_dnd_movement_y = dest_y;

      return;
    }
}

/* C 4958-4984.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_note_self_wheel (dpyinfo: *mut x_display_info, target: Window,
				 root_x: c_ushort, root_y: c_ushort,
				 button: c_int, state: c_uint, time: Time)
{
  let mut f: *mut frame;
  let mut dest_x: c_int;
  let mut dest_y: c_int;

  if button < 4 || button > 7
    { return; }

  f = x_top_window_to_frame (dpyinfo, target);

  if !f.is_null ()
    {
      x_translate_coordinates (f, root_x as c_int, root_y as c_int,
			       &raw mut dest_x, &raw mut dest_y);

      x_dnd_wheel_frame = f;
      x_dnd_wheel_x = dest_x;
      x_dnd_wheel_y = dest_y;
      x_dnd_wheel_button = button;
      x_dnd_wheel_state = state as c_int;
      x_dnd_wheel_time = time;

      return;
    }
}

/* C 4986-5071: the self-drop path.  C's `alloca` is the port's stack pool (PORTING.md P-05;
   the `x_dnd_send_unsupported_drop` shape), `EVENT_INIT`/`XSETINT`/`XSETFRAME` are the mirrors'
   macros, and the `DRAG_N_DROP_EVENT` is the `event_kind` constant.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_dnd_note_self_drop (dpyinfo: *mut x_display_info, target: Window,
				root_x: c_ushort, root_y: c_ushort,
				timestamp: Time)
{
  use crate::x11_constants::None;

  let mut ie: input_event = std::mem::zeroed ();
  let mut f: *mut frame;
  let mut lval: Lisp_Object;
  let mut atom_names: *mut *mut c_char;
  let mut name: *mut c_char;
  let mut win_x: c_int;
  let mut win_y: c_int;
  let mut i: c_int;

  if x_dnd_allow_current_frame == false
      && (FRAME_OUTER_WINDOW! (x_dnd_frame)
	  == target)
    { return; }

  f = x_top_window_to_frame (dpyinfo, target);

  if f.is_null ()
    { return; }

  if NILP (Vx_dnd_native_test_function)
    { return; }

  x_translate_coordinates (f, root_x as c_int, root_y as c_int,
			   &raw mut win_x, &raw mut win_y);

  /* Emacs can't respond to DND events inside the nested event loop,
     so when dragging items to itself, call the test function
     manually.  */

  XSETFRAME! (lval, f);
  x_dnd_action = None as Atom;
  x_dnd_action_symbol
    = safe_calln! (Vx_dnd_native_test_function,
		   Fposn_at_x_y (make_fixnum (win_x as EMACS_INT),
				 make_fixnum (win_y as EMACS_INT),
				 lval, Qnil ()),
		   x_atom_to_symbol (dpyinfo,
				     x_dnd_wanted_action));

  if SYMBOLP (x_dnd_action_symbol) == false
    { return; }

  EVENT_INIT (&raw mut ie);

  /* C 5038: `ie.kind = DRAG_N_DROP_EVENT;` - the port's `input_event` packs `kind` into
     `bits` (PORTING.md 1.3), so the store is the masked write (`xselect.rs` 3610's shape).  */
  ie.bits = (ie.bits & !INPUT_EVENT_KIND_MASK)
    | ((DRAG_N_DROP_EVENT as u32) & INPUT_EVENT_KIND_MASK);
  XSETFRAME! (ie.frame_or_window, f);

  lval = Qnil ();
  atom_names = sa_stack_alloc (std::mem::size_of::<*mut c_char> ()
			       * x_dnd_n_targets as usize) as *mut *mut c_char;
  name = x_get_atom_name (dpyinfo, x_dnd_wanted_action, std::ptr::null_mut ());

  if XGetAtomNames ((*dpyinfo).display, x_dnd_targets,
		    x_dnd_n_targets, atom_names) == 0
    {
      xfree (name as *mut c_void);
      return;
    }

  i = x_dnd_n_targets;
  while i != 0
    {
      lval = Fcons (intern (*atom_names.offset ((i - 1) as isize)), lval);
      XFree (*atom_names.offset ((i - 1) as isize) as *mut c_void);
      i -= 1;
    }

  lval = Fcons (assq_no_quit (QXdndSelection (),
			      (*FRAME_TERMINAL! (f)).Vselection_alist),
		lval);
  lval = Fcons (intern (name), lval);
  lval = Fcons (QXdndSelection (), lval);
  ie.arg = lval;
  ie.timestamp = timestamp;

  XSETINT! (ie.x, win_x as EMACS_INT);
  XSETINT! (ie.y, win_y as EMACS_INT);

  xfree (name as *mut c_void);
  kbd_buffer_store_event (&raw mut ie as *mut c_void);
}

/* ---- C 5073-5300: the flush, the Xdbe/XRender surface helpers, the toolkit position test and
   the opaque region, the GC-ext-data helpers and the two macro families slice 8's USE_CAIRO arm
   needs.  This file's first section banner (`Miscellaneous X event and graphics extension
   functions`) and the `Debugging` page banner C writes around them are carried as comments.  */

/* C 5073-5085.  `FRAME_X_P` is `include/frame.rs`'s function (frame.h 933-963's predicate).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_flush (f: *mut frame)
{
  eassert! (f.is_null () == false && FRAME_X_P (f));
  /* Don't call XFlush when it is not safe to redisplay; the X
     connection may be broken.  */
  if !NILP (Vinhibit_redisplay)
    { return; }

  block_input ();
  XFlush (FRAME_X_DISPLAY! (f));
  unblock_input ();
}

/* C 5087-5103: `#ifdef HAVE_XDBE`.  `font_drop_xrender_surfaces` is declared font.h 932 and
   defined font.c 5832, and this unit's include graph reaches no mirror that carries the
   prototype, so the one function that calls it declares it (the `font.rs` 413-418 shape, used
   the same way at C 3982-3988) - P-26: it moves back when `include/font.rs` carries font.h 932's
   prototype block.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]
unsafe fn x_drop_xrender_surfaces (f: *mut frame)
{
  use crate::x11_constants::None;

  unsafe extern "C" {
    /// C: font.h 932: `extern void font_drop_xrender_surfaces (struct frame *f);`
    pub fn font_drop_xrender_surfaces (f: *mut frame);
  }

  font_drop_xrender_surfaces (f);

  #[cfg (feature = "have_xrender")]
  {
    if f.is_null () == false && FRAME_X_DOUBLE_BUFFERED_P! (f)
	&& FRAME_X_PICTURE! (f) != None
      {
	XRenderFreePicture (FRAME_X_DISPLAY! (f),
			    FRAME_X_PICTURE! (f));
	FRAME_X_PICTURE! (f) = None;
      }
  }
}

/* C 5105-5118: `#ifdef HAVE_XRENDER`.  C's definition is not `static`, so the port keeps it an
   exported one (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[unsafe(no_mangle)]
pub unsafe fn x_xr_ensure_picture (f: *mut frame)
{
  use crate::x11_constants::{CPClipMask, None};

  if FRAME_X_PICTURE! (f) == None && FRAME_X_PICTURE_FORMAT! (f).is_null () == false
    {
      let mut attrs: XRenderPictureAttributes = std::mem::zeroed ();
      attrs.clip_mask = None;
      let fmt: *mut XRenderPictFormat = FRAME_X_PICTURE_FORMAT! (f);

      FRAME_X_PICTURE! (f) = XRenderCreatePicture (FRAME_X_DISPLAY! (f),
						   FRAME_X_RAW_DRAWABLE! (f),
						   fmt, CPClipMask, &raw mut attrs);
    }
}

/* C 5120-5156: `<Debugging>`, and C 5127-5153's `#if false` - the `record_event` recorder, dead
   text in every configuration and carried as C's own text inside a comment (the `cm.rs` 86
   reading).  `tools/cinventory.py`'s dead-branch map marks every line of it `False`, which is
   why `record_event`, `event_record` and `event_record_index` are in no checklist.  */
/*
   #if false

   /* This is a function useful for recording debugging information about
      the sequence of occurrences in this file.  */

   struct record
   {
     char *locus;
     int type;
   };

   struct record event_record[100];

   int event_record_index;

   void
   record_event (char *locus, int type)
   {
     if (event_record_index == countof (event_record))
       event_record_index = 0;

     event_record[event_record_index].locus = locus;
     event_record[event_record_index].type = type;
     event_record_index++;
   }

   #endif
*/

/* C 5159-5194: the three toolkit arms.  C's `#ifdef USE_GTK` / `#elif defined USE_X_TOOLKIT` /
   `#else` are the port's three mutually exclusive cfgs (P-51); the GTK arm's two locals are its
   own block's, and C's `x +=`/`y +=` are why the two parameters are `mut`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_toolkit_position (f: *mut frame, mut x: c_int, mut y: c_int,
			      menu_bar_p: *mut bool, tool_bar_p: *mut bool)
{
  #[cfg (feature = "use_gtk")]
  {
    let mut test_rect: GdkRectangle = std::mem::zeroed ();
    let mut scale: c_int;

    y += (FRAME_MENUBAR_HEIGHT! (f)
	  + FRAME_TOOLBAR_TOP_HEIGHT! (f));
    x += FRAME_TOOLBAR_LEFT_WIDTH! (f);

    if FRAME_EXTERNAL_MENU_BAR (f)
      { *menu_bar_p = (x >= 0 && x < FRAME_PIXEL_WIDTH (f)
		       && y >= 0 && y < FRAME_MENUBAR_HEIGHT! (f)); }

    if (*FRAME_X_OUTPUT! (f)).toolbar_widget.is_null () == false
      {
	scale = xg_get_scale (f);
	test_rect.x = (x / scale) as c_short;
	test_rect.y = (y / scale) as c_short;
	test_rect.width = 1;
	test_rect.height = 1;

	*tool_bar_p = gtk_widget_intersect ((*FRAME_X_OUTPUT! (f)).toolbar_widget,
					    &raw mut test_rect, std::ptr::null_mut ());
      }
  }
  #[cfg (all (not (feature = "use_gtk"), feature = "use_x_toolkit"))]
  {
    *menu_bar_p = (x > 0 && x < FRAME_PIXEL_WIDTH (f)
		   && (y < 0 && y >= -FRAME_MENUBAR_HEIGHT! (f)));
  }
  #[cfg (all (not (feature = "use_gtk"), not (feature = "use_x_toolkit")))]
  {
    *menu_bar_p = (WINDOWP ((*f).menu_bar_window)
		   && (x > 0 && x < FRAME_PIXEL_WIDTH (f)
		       && (y > 0 && y < FRAME_MENU_BAR_HEIGHT (f))));
  }
}

/* C 5196-5243: the `_NET_WM_OPAQUE_REGION` property, and the `#else` arm's GTK child-frame
   workaround (the `xfns.rs` 1078-1113 shape: C's `if (…) A; #ifndef HAVE_GTK3 else B; #else else
   if (…) C; else D; #endif` is the port's `if`/`else` with one `#[cfg]` block per arm).  C's
   `configure->xconfigure.width` keeps C's spelling; the mirror's `XEvent` is opaque, so that
   member is the same recorded class as the `.xclient` writes above.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_update_opaque_region (f: *mut frame, configure: *mut XEvent)
{
  use crate::x11_constants::{PropModeReplace, XA_CARDINAL};

  let mut opaque_region: [c_ulong; 4] = [0, 0,
					(if configure.is_null () == false
					 { (*configure).xconfigure.width as c_ulong }
					 else
					   { FRAME_PIXEL_WIDTH (f) as c_ulong }),
					(if configure.is_null () == false
					 { (*configure).xconfigure.height as c_ulong }
					 else
					   { FRAME_PIXEL_HEIGHT (f) as c_ulong })];

  if (*FRAME_DISPLAY_INFO! (f)).alpha_bits == 0
    { return; }

  if (*f).alpha_background < 1.0
    {
      XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
		       (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_opaque_region,
		       XA_CARDINAL, 32, PropModeReplace,
		       std::ptr::null_mut (), 0);
    }
  else
    {
      /* C 5215-5220: the `#ifndef HAVE_GTK3` arm.  */
      #[cfg (not (feature = "have_gtk3"))]
      {
	XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
			 (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_opaque_region,
			 XA_CARDINAL, 32, PropModeReplace,
			 &raw mut opaque_region as *mut c_uchar, 4);
      }

      /* C 5221-5239: the `#else` arm's `else if`/`else`.  */
      #[cfg (feature = "have_gtk3")]
      {
	if FRAME_TOOLTIP_P (f)
	  {
	    XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
			     (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_opaque_region,
			     XA_CARDINAL, 32, PropModeReplace,
			     &raw mut opaque_region as *mut c_uchar, 4);
	  }
	else
	  {
	    /* This causes child frames to not update correctly for an
	       unknown reason.  (bug#55779) */
	    if FRAME_PARENT_FRAME (f).is_null ()
	      {
		let object_class: *mut GObjectClass
		  = G_OBJECT_GET_CLASS! (FRAME_GTK_OUTER_WIDGET! (f));
		let mut class: *mut GtkWidgetClass = GTK_WIDGET_CLASS! (object_class);

		if (*class).style_updated.is_some ()
		  { ((*class).style_updated.unwrap_unchecked ())
		      (FRAME_GTK_OUTER_WIDGET! (f)); }
	      }
	  }
      }
    }
}

/* C 5245-5287: `#if defined USE_CAIRO || defined HAVE_XRENDER` - the GC extension data, dead in
   the frozen reading and carried under its feature (P-51).  C's `object.gc = gc` is an
   `XEDataObject` union member write, and `dpyinfo->ext_codes` is one of the `x_display_info`
   members this mirror does not carry yet (Decision 19's list, xterm.h 838).  */
#[cfg (all (feature = "have_x_windows",
	    any (feature = "use_cairo", feature = "have_xrender")))]
unsafe extern "C" fn x_gc_free_ext_data_private (extension: *mut XExtData) -> c_int
{
  xfree ((*extension).private_data as *mut c_void);

  return 0;
}

#[cfg (all (feature = "have_x_windows",
	    any (feature = "use_cairo", feature = "have_xrender")))]
unsafe fn x_gc_get_ext_data (f: *mut frame, gc: GC, create_if_not_found_p: c_int)
			     -> *mut x_gc_ext_data
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut object: XEDataObject = std::mem::zeroed ();
  let mut head: *mut *mut XExtData;
  let mut ext_data: *mut XExtData;

  object.gc = gc;
  head = XEHeadOfExtensionList (object);
  ext_data = XFindOnExtensionList (head, (*(*dpyinfo).ext_codes).extension);
  if ext_data.is_null ()
    {
      if create_if_not_found_p == 0
	{ return std::ptr::null_mut (); }
      else
	{
	  ext_data = xzalloc (std::mem::size_of::<XExtData> ()) as *mut XExtData;
	  (*ext_data).number = (*(*dpyinfo).ext_codes).extension;
	  (*ext_data).private_data = xzalloc (std::mem::size_of::<x_gc_ext_data> ());
	  (*ext_data).free_private = Some (x_gc_free_ext_data_private);
	  XAddToExtensionList (head, ext_data);
	}
    }
  return (*ext_data).private_data as *mut x_gc_ext_data;
}

/* C 5281-5286: the extension codes the GC callbacks are keyed by.  */
#[cfg (all (feature = "have_x_windows",
	    any (feature = "use_cairo", feature = "have_xrender")))]
unsafe fn x_extension_initialize (dpyinfo: *mut x_display_info)
{
  let ext_codes: *mut XExtCodes = XAddExtension ((*dpyinfo).display);

  (*dpyinfo).ext_codes = ext_codes;
}

/* C 5289-5297: `#ifdef USE_CAIRO` - the three cairo members of `x_output`, carried as the
   macros C writes (P-51's arm for the non-frozen side).  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
macro_rules! FRAME_CR_CONTEXT {
  ($f:expr) => { ((*(*$f).output_data.x).cr_context) };
}
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
macro_rules! FRAME_CR_SURFACE_DESIRED_WIDTH {
  ($f:expr) => { ((*(*$f).output_data.x).cr_surface_desired_width) };
}
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
macro_rules! FRAME_CR_SURFACE_DESIRED_HEIGHT {
  ($f:expr) => { ((*(*$f).output_data.x).cr_surface_desired_height) };
}

/* X input extension device and event mask management functions.  */

/* C 5303-5328: `#ifdef HAVE_XINPUT2`, and inside it C 5305's
   `# if defined USE_X_TOOLKIT || !(defined USE_GTK && defined HAVE_GTK3)`.  The definition is
   C's non-`static` one, so the port exports it (PORTING.md 0.2); dead in the frozen reading.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    any (feature = "use_x_toolkit",
		 not (all (feature = "use_gtk", feature = "have_gtk3")))))]
#[unsafe(no_mangle)]
pub unsafe fn xi_frame_selected_for (f: *mut frame, event: c_ulong) -> bool
{
  let mut masks: *mut XIEventMask;
  let mut i: c_int;

  masks = (*FRAME_X_OUTPUT! (f)).xi_masks;

  if masks.is_null ()
    { return false; }

  for i in 0..(*FRAME_X_OUTPUT! (f)).num_xi_masks
    {
      if (*masks.offset (i as isize)).mask_len >= XIMaskLen (event)
	  && XIMaskIsSet ((*masks.offset (i as isize)).mask, event)
	{ return true; }
    }

  return false;
}

/* Convert XI2 button state IN to a standard X button modifier
   mask, and place it in OUT.  */

/* C 5331-5352.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn xi_convert_button_state (in_: *mut XIButtonState, out: *mut c_uint)
{
  let mut i: c_int;

  if (*in_).mask_len != 0
    {
      for i in 1..=8
	{
	  if XIMaskIsSet ((*in_).mask, i)
	    { *out |= (Button1Mask as c_uint) << (i - 1); }
	}
    }
}

/* Return the modifier state in XEV as a standard X modifier mask.
   This should be used for non-keyboard events, where the group does
   not matter.  */

/* C 5353-5368: C's `#ifdef USE_GTK static #endif` selects the storage class.  The port keeps a
   plain `unsafe fn`: the reference's *inventory* has no entry for the name, because the whole
   `#ifdef HAVE_XINPUT2` span is dead in the frozen reading, so an exported symbol here is
   `extract-abi`'s "invented exported symbol" (`comp.rs` 506's `load_gccjit_if_necessary`
   precedent, `docs/ports/comp.md`'s `## Evidence` 4).  A configuration with `have_xinput2` and
   without `use_gtk` is the one C links this name in, and its `Cargo.toml` row is this round's
   request.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn xi_convert_event_state (xev: *mut XIDeviceEvent) -> c_uint
{
  let mut mods: c_uint;
  let mut buttons: c_uint;

  mods = (*xev).mods.effective;
  buttons = 0;

  xi_convert_button_state (&raw mut (*xev).buttons, &raw mut buttons);

  return mods | buttons;
}

/* Like the above.  However, buttons are not converted, while the
   group is.  This should be used for key events being passed to the
   likes of input methods and Xt.  */

/* C 5370-5378.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn xi_convert_event_keyboard_state (xev: *mut XIDeviceEvent) -> c_uint
{
  return (((*xev).mods.effective & !(1 << 13 | 1 << 14))
	  | ((*xev).group.effective << 13));
}

/* Free all XI2 devices on DPYINFO.  */

/* C 5380-5427: the two `HAVE_XINPUT2_1`/`_2` members C guards.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn x_free_xi_devices (dpyinfo: *mut x_display_info)
{
  #[cfg (feature = "have_xinput2_2")]
  let mut tem: *mut xi_touch_point_t;
  #[cfg (feature = "have_xinput2_2")]
  let mut last: *mut xi_touch_point_t;

  block_input ();

  if (*dpyinfo).num_devices != 0
    {
      let mut i: c_int = 0;
      while i < (*dpyinfo).num_devices
	{
	  #[cfg (feature = "have_xinput2_1")]
	  { xfree ((*((*dpyinfo).devices.offset (i as isize))).valuators as *mut c_void); }

	  #[cfg (feature = "have_xinput2_2")]
	  {
	    tem = (*((*dpyinfo).devices.offset (i as isize))).touchpoints;
	    while !tem.is_null ()
	      {
		last = tem;
		tem = (*tem).next;
		xfree (last as *mut c_void);
	      }
	  }
	  i += 1;
	}

      xfree ((*dpyinfo).devices as *mut c_void);
      (*dpyinfo).devices = std::ptr::null_mut ();
      (*dpyinfo).num_devices = 0;
    }

  unblock_input ();
}

/* C 5429-5441: `#ifdef HAVE_XINPUT2_1` - the record `xi_populate_device_from_info` builds.  C
   defines it in this file (not xterm.h), so it lives in this unit.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_1"))]
#[repr(C)]
pub struct xi_known_valuator
{
  /* The current value of this valuator.  */
  pub current_value: f64,
  /* The number of the valuator.  */
  pub number: c_int,
  /* The next valuator whose value we already know.  */
  pub next: *mut xi_known_valuator,
}

/* Populate the scroll valuator at INDEX in DEVICE with the scroll
   valuator information provided in INFO.

   The information consists of:

     - whether or not the valuator is horizontal.

     - whether or not the valuator's value is currently unknown,
       until the next XI_Motion event is received or the valuator's
       value is restored by the caller upon encountering valuator
       class data.

     - what the current value of the valuator is.  This is set to
       DBL_MIN for debugging purposes, but can be any value, as
       invalid_p is currently true.

     - the increment, which defines the amount of movement equal to a
       single unit of scrolling.  For example, if the increment is
       2.0, then a WHEEL_DOWN or WHEEL_UP event will be sent every
       time the valuator value changes by 2.0, unless
       mwheel-coalesce-scroll-events is nil.

     - the number used in XI_Motion events and elsewhere to identify
       the valuator.  */

/* C 5455-5467: the two bitfields C assigns (`horizontal`, `invalid_p`) are members this mirror
   does not carry yet (its `xi_scroll_valuator_t` records them as comments, xterm.h 238-256), so
   the port keeps C's spelling and the gap is recorded in this ledger.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_1"))]
unsafe fn xi_populate_scroll_valuator (device: *mut xi_device_t,
				       index: c_int, info: *mut XIScrollClassInfo)
{
  let mut valuator: *mut xi_scroll_valuator_t;

  valuator = &raw mut *(*device).valuators.offset (index as isize);
  (*valuator).horizontal
    = ((*info).scroll_type == XIScrollTypeHorizontal);
  (*valuator).invalid_p = true;
  (*valuator).emacs_value = DBL_MIN;
  (*valuator).increment = (*info).increment;
  (*valuator).number = (*info).number;
}

/* C 5472-5706: the device records.  C's `USE_SAFE_ALLOCA`/`SAFE_ALLOCA`/`SAFE_FREE` are the
   lisp.rs macros (`sa_avail`/`sa_count`), `xnmalloc` the allocator, and the two class arms
   carry `HAVE_XINPUT2_2` and `HAVE_XINPUT2_1`; the `switch` is the port's labelled match (the
   `keyboard.rs` 4580 shape) so C's `break`s survive as `break 'switch`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn xi_populate_device_from_info (dpyinfo: *mut x_display_info,
					xi_device: *mut xi_device_t,
					device: *mut XIDeviceInfo)
{
  #[cfg (feature = "have_xinput2_1")]
  let mut values: *mut xi_known_valuator;
  #[cfg (feature = "have_xinput2_1")]
  let mut tem: *mut xi_known_valuator;
  #[cfg (feature = "have_xinput2_1")]
  let mut actual_valuator_count: c_int;
  #[cfg (feature = "have_xinput2_1")]
  let mut c: c_int;
  #[cfg (feature = "have_xinput2_1")]
  let mut info: *mut XIScrollClassInfo;
  #[cfg (feature = "have_xinput2_1")]
  let mut valuator_info: *mut XIValuatorClassInfo;
  #[cfg (feature = "have_xinput2_2")]
  let mut touch_info: *mut XITouchClassInfo;

  #[cfg (feature = "have_xinput2_1")]
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  /* Initialize generic information about the device: its ID, which
     buttons are currently pressed and thus presumably actively
     grabbing the device, what kind of device it is (master pointer,
     master keyboard, slave pointer, slave keyboard, or floating
     slave), and its attachment.

     Here is a brief description of what device uses and attachments
     are.  Under XInput 2, user input from individual input devices is
     multiplexed into specific seats before being delivered, with each
     seat corresponding to a single on-screen mouse pointer and having
     its own keyboard focus.  Each seat consists of two virtual
     devices: the master keyboard and the master pointer, the first of
     which is used to report all keyboard input, with the other used
     to report all other input.

     Input from each physical device (mouse, trackpad or keyboard) is
     then associated with that slave device's paired master device.
     For example, moving the device "Logitech USB Optical Mouse",
     enslaved by the master pointer device "Virtual core pointer",
     will result in movement of the mouse pointer associated with that
     master device's seat.  If the pointer moves over an Emacs frame,
     then the frame will receive XI_Enter and XI_Motion events from
     that master pointer.

     Likewise, keyboard input from the device "AT Translated Set 2
     keyboard", enslaved by the master keyboard device "Virtual core
     keyboard", will be reported to its seat's input focus window.

     The device use describes what the device is.  The meanings of
     MasterPointer, MasterKeyboard, SlavePointer and SlaveKeyboard
     should be obvious.  FloatingSlave means the device is a slave
     device that is not associated with any seat, and thus generates
     no input.

     The device attachment is a device ID whose meaning varies
     depending on the device's use.  If a device is a master device,
     then its attachment is the device ID of the other device in its
     seat (the master keyboard for master pointer devices and vice
     versa.)  Otherwise, it is the ID of the master device the slave
     device is attached to.  For slave devices not attached to any
     seat, its value is undefined.

     Emacs receives ordinary pointer and keyboard events from the
     master devices associated with each seat, discarding events from
     slave devices.  However, multiplexing events from touch devices
     onto a master device poses problems: if both dependent and direct
     touch devices are attached to the same master pointer device, the
     coordinate space of touch events sent from that seat becomes
     ambiguous.  In addition, the X server does not send TouchEnd
     events to cancel ongoing touch sequences if the slave device that
     is their source is detached.  As a result of these ambiguities,
     touch events are processed from and recorded onto their slave
     devices instead.  */

  (*xi_device).device_id = (*device).deviceid;
  (*xi_device).grab = 0;
  (*xi_device).r#use = (*device).r#use;
  (*xi_device).name = build_string ((*device).name);
  (*xi_device).attachment = (*device).attachment;

  /* Clear the list of active touch points on the device, which are
     individual touches tracked by a touchscreen.  */

  #[cfg (feature = "have_xinput2_2")]
  {
    (*xi_device).touchpoints = std::ptr::null_mut ();
    (*xi_device).direct_p = false;
  }

  #[cfg (feature = "have_xinput2_1")]
  {
    if (*dpyinfo).xi2_version == 0
      {
	/* Skip everything below as there are no classes of interest on
	   XI 2.0 servers.  */
	(*xi_device).valuators = std::ptr::null_mut ();
	(*xi_device).scroll_valuator_count = 0;

	SAFE_FREE! (sa_count);
	return;
      }

    actual_valuator_count = 0;
    (*xi_device).valuators = xnmalloc ((*device).num_classes as isize,
				       std::mem::size_of::<xi_scroll_valuator_t> ()
				       as isize) as *mut xi_scroll_valuator_t;
    values = std::ptr::null_mut ();

    /* Initialize device info based on a list of "device classes".
       Device classes are little pieces of information associated with a
       device.  Emacs is interested in scroll valuator information and
       touch handling information, which respectively describe the axes
       (if any) along which the device's scroll wheel rotates, and how
       the device reports touch input.  */

    c = 0;
    while c < (*device).num_classes
      {
	'ci: {
	match (*(*device).classes.offset (c as isize)).r#type
	  {
	    XIScrollClass =>
	      {
		info = (*device).classes.offset (c as isize) as *mut XIScrollClassInfo;
		xi_populate_scroll_valuator (xi_device, actual_valuator_count,
					     info);
		actual_valuator_count += 1;
		break 'ci;
	      }

	    XIValuatorClass =>
	      {
		valuator_info = (*device).classes.offset (c as isize)
				as *mut XIValuatorClassInfo;
		tem = SAFE_ALLOCA! (sa_avail, std::mem::size_of::<xi_known_valuator> ())
		      as *mut xi_known_valuator;

		/* Avoid restoring bogus values if some driver
		   accidentally specifies relative values in scroll
		   valuator classes how the input extension spec says they
		   should be, but allow restoring values when a value is
		   set, which is how the input extension actually
		   behaves.  */

		if (*valuator_info).value == 0.0
		    && (*valuator_info).mode != XIModeAbsolute
		  { break 'ci; }

		(*tem).next = values;
		(*tem).number = (*valuator_info).number;
		(*tem).current_value = (*valuator_info).value;

		values = tem;
		break 'ci;
	      }

	    #[cfg (feature = "have_xinput2_2")]
	    XITouchClass =>
	      {
		  touch_info = (*device).classes.offset (c as isize)
			       as *mut XITouchClassInfo;

		  /* touch_info->mode indicates the coordinate space that
		     this device reports in its touch events.

		     DirectTouch means that the device uses a coordinate
		     space that corresponds to locations on the screen.  It
		     is set by touch screen devices which are overlaid
		     over the raster itself.

		     The other value (DependentTouch) means that the device
		     uses a separate abstract coordinate space corresponding
		     to its own surface.  Emacs ignores events from these
		     devices because it does not support recognizing touch
		     gestures from surfaces other than the screen.

		     Master devices may report multiple touch classes for
		     attached slave devices, leaving the nature of touch
		     events they send ambiguous.  The problem of
		     discriminating between these events is bypassed
		     entirely through only processing touch events from the
		     slave devices where they originate.  */

		  if (*touch_info).mode == XIDirectTouch
		    { (*xi_device).direct_p = true; }
		  else
		    { (*xi_device).direct_p = false; }
	      }
	    _ => { break 'ci; }   /* C's `default: break;` */
	  }
	}
	c += 1;
      }

    (*xi_device).scroll_valuator_count = actual_valuator_count;

    /* Now look through all the valuators whose values are already known
       and populate our client-side records with their current
       values.  */

    tem = values;
    while !values.is_null ()
      {
	c = 0;
	while c < (*xi_device).scroll_valuator_count
	  {
	    if (*(*xi_device).valuators.offset (c as isize)).number == (*tem).number
	      {
		(*(*xi_device).valuators.offset (c as isize)).invalid_p = false;
		(*(*xi_device).valuators.offset (c as isize)).current_value
		  = (*tem).current_value;
		(*(*xi_device).valuators.offset (c as isize)).emacs_value = 0.0;
	      }
	    c += 1;
	  }

	values = (*values).next;
      }

    SAFE_FREE! (sa_count);
  }
}

/* Populate our client-side record of all devices, which includes
   basic information about the device and also touchscreen tracking
   information and scroll valuators.

   Keeping track of scroll valuators is required in order to support
   scroll wheels that report information in a fashion more detailed
   than a single turn of a "step" in the wheel.

   When the input extension is being utilized, the states of the mouse
   wheels on each axis are stored as absolute values inside
   "valuators" attached to each mouse device.  To obtain the delta of
   the scroll wheel from a motion event (which is used to report that
   some valuator has changed), it is necessary to iterate over every
   valuator that changed, and compare its previous value to the
   current value of the valuator.

   Each individual valuator also has an "interval", which is the
   amount you must divide that delta by in order to obtain a delta in
   the terms of scroll units.

   This delta however is still intermediate, to make driver
   implementations easier.  The XInput developers recommend (and most
   programs use) the following algorithm to convert from scroll unit
   deltas to pixel deltas by which the display must actually be
   scrolled:

     pixels_scrolled = pow (window_height, 2.0 / 3.0) * delta;  */

/* C 5708-5740: C's `&dpyinfo->devices[actual_devices++]` reads the old count and then increments
   it, which the port writes as the call plus the increment (the same value reaches the call).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn x_cache_xi_devices (dpyinfo: *mut x_display_info)
{
  let mut ndevices: c_int;
  let mut actual_devices: c_int;
  let mut i: c_int;
  let mut infos: *mut XIDeviceInfo;

  actual_devices = 0;
  block_input ();
  x_free_xi_devices (dpyinfo);
  infos = XIQueryDevice ((*dpyinfo).display,
			 XIAllDevices,
			 &raw mut ndevices);

  if ndevices == 0
    {
      XIFreeDeviceInfo (infos);
      unblock_input ();
      return;
    }

  (*dpyinfo).devices = xcalloc (ndevices as usize,
				std::mem::size_of::<xi_device_t> ()) as *mut xi_device_t;

  for i in 0..ndevices
    {
      if (*infos.offset (i as isize)).enabled
	{
	  xi_populate_device_from_info (dpyinfo,
					&raw mut *(*dpyinfo).devices
					.offset (actual_devices as isize),
					&raw mut *infos.offset (i as isize));
	  actual_devices += 1;
	}
    }

  (*dpyinfo).num_devices = actual_devices;
  XIFreeDeviceInfo (infos);
  unblock_input ();
}

/* Return the delta of the scroll valuator VALUATOR_NUMBER under
   DEVICE in the display DPYINFO with VALUE.  The valuator's valuator
   will be set to VALUE afterwards.  In case no scroll valuator is
   found, or if the valuator state is invalid (see the comment under
   XI_Enter in handle_one_xevent).  Otherwise, the valuator is
   returned in VALUATOR_RETURN.  */

/* C 5742-5792: `#ifdef HAVE_XINPUT2_1`.  `DBL_MAX` is C's `<float.h>` maximum, and `sv->invalid_p`
   is one of the two bitfield members the mirror does not carry yet (`xi_populate_scroll_valuator`'s
   note above).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_1"))]
unsafe fn x_get_scroll_valuator_delta (dpyinfo: *mut x_display_info,
				       device: *mut xi_device_t,
				       valuator_number: c_int, value: f64,
				       valuator_return: *mut *mut xi_scroll_valuator_t)
				      -> f64
{
  let mut sv: *mut xi_scroll_valuator_t;
  let mut delta: f64;
  let mut i: c_int;

  for i in 0..(*device).scroll_valuator_count
    {
      sv = &raw mut *(*device).valuators.offset (i as isize);

      if (*sv).number == valuator_number
	{
	  *valuator_return = sv;

	  if (*sv).increment == 0.0
	    { return DBL_MAX; }

	  if (*sv).invalid_p
	    {
	      (*sv).current_value = value;
	      (*sv).invalid_p = false;

	      return DBL_MAX;
	    }
	  else
	    {
	      delta = ((*sv).current_value - value) / (*sv).increment;
	      (*sv).current_value = value;

	      return delta;
	    }
	}
    }

  *valuator_return = std::ptr::null_mut ();
  return DBL_MAX;
}

/* C 5794-5811: C's definition is not `static`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[unsafe(no_mangle)]
pub unsafe fn xi_device_from_id (dpyinfo: *mut x_display_info, deviceid: c_int)
				 -> *mut xi_device_t
{
  for i in 0..(*dpyinfo).num_devices
    {
      if (*(*dpyinfo).devices.offset (i as isize)).device_id == deviceid
	{ return &raw mut *(*dpyinfo).devices.offset (i as isize); }
    }

  return std::ptr::null_mut ();
}

/* Record a touch sequence with the identifier DETAIL from the given
   FRAME on the specified DEVICE.  Round X and Y and record them as its
   current position, assign an identifier to the touch sequence suitable
   for reporting to Lisp, and return the same.  */

/* C 5813-5848: `#ifdef HAVE_XINPUT2_2`.  C's `static EMACS_INT local_detail;` is a function
   static, which the port writes where C declares it; the post-increment is written out as the
   store plus the increment (`x_get_scroll_valuator_delta`'s note), and `lrint` is math.h's
   rounding function, which no mirror in this graph carries yet (recorded in this ledger).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_2"))]
unsafe fn xi_link_touch_point (device: *mut xi_device_t,
			       detail: c_int, x: f64, y: f64,
			       frame: *mut frame) -> EMACS_INT
{
  let mut touchpoint: *mut xi_touch_point_t;
  static mut local_detail: EMACS_INT = 0;

  /* Assign an identifier suitable for reporting to Lisp.  On builds
     with 64-bit Lisp_Object, this is largely a theoretical problem, but
     CARD32s easily overflow 32-bit systems, as they are not specific to
     X clients (e.g. Emacs) but grow uniformly across all of them.  */

  if FIXNUM_OVERFLOW_P (local_detail)
    { local_detail = 0; }

  touchpoint = xmalloc (std::mem::size_of::<xi_touch_point_t> ()) as *mut xi_touch_point_t;
  (*touchpoint).next = (*device).touchpoints;
  (*touchpoint).x = lrint (x) as c_int;
  (*touchpoint).y = lrint (y) as c_int;
  (*touchpoint).number = detail;
  (*touchpoint).local_detail = local_detail;
  local_detail += 1;
  (*touchpoint).frame = frame;
  (*touchpoint).ownership = xi_touch_ownership::TOUCH_OWNERSHIP_NONE;
  (*device).touchpoints = touchpoint;
  return (*touchpoint).local_detail;
}

/* Free and remove the touch sequence with the identifier DETAIL.
   DEVICE is the device in which the touch sequence should be
   recorded.  If such a touch sequence exists, return its local
   identifier in *LOCAL_DETAIL.

   Value is 0 if no touch sequence by that identifier exists inside
   DEVICE, 1 if a touch sequence has been found but is not owned by
   Emacs, and 2 otherwise.  */

/* C 5850-5883: C's `for (last = NULL, tem = …; tem; last = tem, tem = tem->next)` is the two
   comma-operator lists, so the port writes the init before the loop and the increment after the
   body's `if` (`x_dnd_compute_toplevels`' shape: the increment runs on `continue` too, and this
   body has none).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_2"))]
unsafe fn xi_unlink_touch_point (detail: c_int, device: *mut xi_device_t,
				 local_detail: *mut EMACS_INT) -> c_int
{
  let mut last: *mut xi_touch_point_t;
  let mut tem: *mut xi_touch_point_t;
  let mut ownership: xi_touch_ownership;

  last = std::ptr::null_mut ();
  tem = (*device).touchpoints;
  while !tem.is_null ()
    {
      if (*tem).number == detail
	{
	  if last.is_null ()
	    { (*device).touchpoints = (*tem).next; }
	  else
	    { (*last).next = (*tem).next; }

	  ownership = (*tem).ownership;
	  *local_detail = (*tem).local_detail;
	  xfree (tem as *mut c_void);

	  if ownership == xi_touch_ownership::TOUCH_OWNERSHIP_SELF
	    { return 2; }

	  return 1;
	}

      last = tem;
      tem = (*tem).next;
    }

  return 0;
}

/* Unlink all touch points associated with the frame F.
   This is done upon unmapping or destroying F's window, because
   touch point delivery after that point is undefined.  */

/* C 5885-5913: C's `for (next = &device->touchpoints; (last = *next);)` re-reads `*next` for its
   condition, which the port writes as the loop's last statement (`next` is C's pointer to the
   link, so an unlinked point leaves the following one there).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_2"))]
unsafe fn xi_unlink_touch_points (f: *mut frame)
{
  let mut device: *mut xi_device_t;
  let mut next: *mut *mut xi_touch_point_t;
  let mut last: *mut xi_touch_point_t;
  let mut i: c_int;

  for i in 0..(*FRAME_DISPLAY_INFO! (f)).num_devices
    {
      device = &raw mut *(*FRAME_DISPLAY_INFO! (f)).devices.offset (i as isize);

      /* Now unlink all touch points on DEVICE matching F.  */

      next = &raw mut (*device).touchpoints;
      last = *next;
      while !last.is_null ()
	{
	  if (*last).frame == f
	    {
	      *next = (*last).next;
	      xfree (last as *mut c_void);
	    }
	  else
	    { next = &raw mut (*last).next; }

	  last = *next;
	}
    }
}

/* Return the data associated with a touch sequence DETAIL recorded by
   `xi_link_touch_point' from DEVICE, or NULL if it can't be
   found.  */

/* C 5915-5931.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_2"))]
unsafe fn xi_find_touch_point (device: *mut xi_device_t, detail: c_int)
			      -> *mut xi_touch_point_t
{
  let mut point: *mut xi_touch_point_t;

  point = (*device).touchpoints;
  while !point.is_null ()
    {
      if (*point).number == detail
	{ return point; }

      point = (*point).next;
    }

  return std::ptr::null_mut ();
}

/* C 5933-5963: `#ifdef HAVE_XINPUT2_1`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_1"))]
unsafe fn xi_reset_scroll_valuators_for_device_id (dpyinfo: *mut x_display_info,
						   id: c_int)
{
  let mut device: *mut xi_device_t;
  let mut valuator: *mut xi_scroll_valuator_t;
  let mut i: c_int;

  device = xi_device_from_id (dpyinfo, id);

  if device.is_null ()
    { return; }

  if (*device).scroll_valuator_count == 0
    { return; }

  for i in 0..(*device).scroll_valuator_count
    {
      valuator = &raw mut *(*device).valuators.offset (i as isize);
      (*valuator).invalid_p = true;
      (*valuator).emacs_value = 0.0;
    }

  return;
}

/* Cairo context, X rendering extension, and GC auxiliary data
   management functions.  */

/* C 5966-6451: `#ifdef USE_CAIRO` - the cairo drawing helpers, dead in the frozen reading and
   carried under their feature (P-51).  The cairo type names and functions are cairo.h's, which
   this graph reaches through `include/gtk.rs` (the mirror this unit pulls under `use_gtk`); C's
   chained assignment in `x_begin_cr_clip` is the port's store plus read (PORTING.md 2.1), and the
   `#ifdef USE_CAIRO_XCB_SURFACE` that selects one `if`'s arm is a named condition
   (`bounding_only`'s shape, Decision 15).  */

/* C 5969-5978.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_cr_destroy_frame_context (f: *mut frame)
{
  if FRAME_CR_CONTEXT! (f).is_null () == false
    {
      cairo_destroy (FRAME_CR_CONTEXT! (f));
      FRAME_CR_CONTEXT! (f) = std::ptr::null_mut ();
    }
}

/* C 5979-5990.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_cr_update_surface_desired_size (f: *mut frame, width: c_int, height: c_int)
{
  if FRAME_CR_SURFACE_DESIRED_WIDTH! (f) != width
      || FRAME_CR_SURFACE_DESIRED_HEIGHT! (f) != height
    {
      x_cr_destroy_frame_context (f);
      FRAME_CR_SURFACE_DESIRED_WIDTH! (f) = width;
      FRAME_CR_SURFACE_DESIRED_HEIGHT! (f) = height;
    }
}

/* C 5991-6009.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_cr_gc_clip (cr: *mut cairo_t, f: *mut frame, gc: GC)
{
  if gc.is_null () == false
    {
      let gc_ext: *mut x_gc_ext_data = x_gc_get_ext_data (f, gc, 0);

      if gc_ext.is_null () == false && (*gc_ext).n_clip_rects != 0
	{
	  for i in 0..(*gc_ext).n_clip_rects
	    {
	      cairo_rectangle (cr, (*gc_ext).clip_rects[i as usize].x as f64,
			       (*gc_ext).clip_rects[i as usize].y as f64,
			       (*gc_ext).clip_rects[i as usize].width as f64,
			       (*gc_ext).clip_rects[i as usize].height as f64);
	    }
	  cairo_clip (cr);
	}
    }
}

/* C 6010-6041.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_begin_cr_clip (f: *mut frame, gc: GC) -> *mut cairo_t
{
  let mut cr: *mut cairo_t = FRAME_CR_CONTEXT! (f);

  if cr.is_null ()
    {
      let width: c_int = FRAME_CR_SURFACE_DESIRED_WIDTH! (f);
      let height: c_int = FRAME_CR_SURFACE_DESIRED_HEIGHT! (f);
      let mut surface: *mut cairo_surface_t;

      /* C 6019-6026's `#ifdef USE_CAIRO_XCB_SURFACE` is the arm of one `if`, so the port names
	 the arm's condition (`bounding_only`'s shape, Decision 15).  */
      #[cfg (all (feature = "use_xcb", feature = "use_cairo_xcb"))]
      let xcb_surface_p: bool = (*FRAME_DISPLAY_INFO! (f)).xcb_visual.is_null () == false;
      #[cfg (not (all (feature = "use_xcb", feature = "use_cairo_xcb")))]
      let xcb_surface_p: bool = false;

      if xcb_surface_p
	{
	  surface = cairo_xcb_surface_create ((*FRAME_DISPLAY_INFO! (f)).xcb_connection,
					      FRAME_X_RAW_DRAWABLE! (f) as xcb_drawable_t,
					      (*FRAME_DISPLAY_INFO! (f)).xcb_visual,
					      width, height);
	}
      else
	{
	  surface = cairo_xlib_surface_create (FRAME_X_DISPLAY! (f),
					       FRAME_X_RAW_DRAWABLE! (f),
					       FRAME_X_VISUAL! (f),
					       width, height);
	}

      FRAME_CR_CONTEXT! (f) = cairo_create (surface);
      cr = FRAME_CR_CONTEXT! (f);
      cairo_surface_destroy (surface);
    }
  cairo_save (cr);
  x_cr_gc_clip (cr, f, gc);

  return cr;
}

/* C 6042-6051.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_end_cr_clip (f: *mut frame)
{
  cairo_restore (FRAME_CR_CONTEXT! (f));
  #[cfg (feature = "have_xdbe")]
  {
    if FRAME_X_DOUBLE_BUFFERED_P! (f)
      { x_mark_frame_dirty (f); }
  }
}

/* C 6052-6081.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_set_cr_source_with_gc_foreground (f: *mut frame, gc: GC,
						  respect_alpha_background: bool)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let mut xgcv: XGCValues = std::mem::zeroed ();
  let mut color: XColor = std::mem::zeroed ();
  let mut depth: c_uint;

  XGetGCValues (FRAME_X_DISPLAY! (f), gc, GCForeground, &raw mut xgcv);
  color.pixel = xgcv.foreground;
  x_query_colors (f, &raw mut color, 1);
  depth = (*FRAME_DISPLAY_INFO! (f)).n_planes;

  if (*f).alpha_background < 1.0 && depth == 32
      && respect_alpha_background
    {
      cairo_set_source_rgba (FRAME_CR_CONTEXT! (f), color.red as f64 / 65535.0,
			     color.green as f64 / 65535.0, color.blue as f64 / 65535.0,
			     (*f).alpha_background);

      cairo_set_operator (FRAME_CR_CONTEXT! (f), CAIRO_OPERATOR_SOURCE);
    }
  else
    {
      cairo_set_source_rgb (FRAME_CR_CONTEXT! (f), color.red as f64 / 65535.0,
			    color.green as f64 / 65535.0, color.blue as f64 / 65535.0);
      cairo_set_operator (FRAME_CR_CONTEXT! (f), CAIRO_OPERATOR_OVER);
    }
}

/* C 6082-6114.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_set_cr_source_with_gc_background (f: *mut frame, gc: GC,
						  respect_alpha_background: bool)
{
  use crate::x11_constants::GCBackground;

  let mut xgcv: XGCValues = std::mem::zeroed ();
  let mut color: XColor = std::mem::zeroed ();
  let mut depth: c_uint;

  XGetGCValues (FRAME_X_DISPLAY! (f), gc, GCBackground, &raw mut xgcv);
  color.pixel = xgcv.background;

  x_query_colors (f, &raw mut color, 1);

  depth = (*FRAME_DISPLAY_INFO! (f)).n_planes;

  if (*f).alpha_background < 1.0 && depth == 32
      && respect_alpha_background
    {
      cairo_set_source_rgba (FRAME_CR_CONTEXT! (f), color.red as f64 / 65535.0,
			     color.green as f64 / 65535.0, color.blue as f64 / 65535.0,
			     (*f).alpha_background);

      cairo_set_operator (FRAME_CR_CONTEXT! (f), CAIRO_OPERATOR_SOURCE);
    }
  else
    {
      cairo_set_source_rgb (FRAME_CR_CONTEXT! (f), color.red as f64 / 65535.0,
			    color.green as f64 / 65535.0, color.blue as f64 / 65535.0);
      cairo_set_operator (FRAME_CR_CONTEXT! (f), CAIRO_OPERATOR_OVER);
    }
}

/* C 6116-6125: the two cairo user-data keys, and the destructor of the Pixmap surface.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
pub static mut xlib_surface_key: cairo_user_data_key_t = unsafe { std::mem::zeroed () };
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
pub static mut saved_drawable_key: cairo_user_data_key_t = unsafe { std::mem::zeroed () };

#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_cr_destroy_xlib_surface (xlib_surface: *mut cairo_surface_t)
{
  if xlib_surface.is_null () == false
    {
      XFreePixmap (cairo_xlib_surface_get_display (xlib_surface),
		   cairo_xlib_surface_get_drawable (xlib_surface));
      cairo_surface_destroy (xlib_surface);
    }
}

/* C 6127-6193: C's `switch (cairo_surface_get_type (surface))` with the `USE_CAIRO_XCB_SURFACE`
   label sharing the XLIB case's body (the two labels' one body is the port's two arms, each
   carrying the same statements - C's fall-through), and its `break` leaves the switch, so the
   match sits in a labelled block (`keyboard.rs` 4630's shape).  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_try_cr_xlib_drawable (f: *mut frame, gc: GC) -> bool
{
  let mut cr: *mut cairo_t = FRAME_CR_CONTEXT! (f);
  if cr.is_null ()
    { return true; }

  let surface: *mut cairo_surface_t = cairo_get_target (cr);
  'surface_type: {
  match cairo_surface_get_type (surface)
    {
      CAIRO_SURFACE_TYPE_XLIB =>
	{
	  cairo_surface_flush (surface);
	  return true;
	}

      #[cfg (all (feature = "use_xcb", feature = "use_cairo_xcb"))]
      CAIRO_SURFACE_TYPE_XCB =>
	{
	  cairo_surface_flush (surface);
	  return true;
	}

      CAIRO_SURFACE_TYPE_IMAGE =>
	{ break 'surface_type; }

      _ =>
	{ return false; }
    }
  }

  /* FRAME_CR_CONTEXT (f) is an image surface we can not draw into
     directly with Xlib.  Set up a Pixmap so we can copy back the
     result later in x_end_cr_xlib_drawable.  */
  let mut xlib_surface: *mut cairo_surface_t
    = cairo_get_user_data (cr, &raw mut xlib_surface_key) as *mut cairo_surface_t;
  let width: c_int = FRAME_CR_SURFACE_DESIRED_WIDTH! (f);
  let height: c_int = FRAME_CR_SURFACE_DESIRED_HEIGHT! (f);
  let mut pixmap: Pixmap;
  if xlib_surface.is_null () == false
      && cairo_xlib_surface_get_width (xlib_surface) == width
      && cairo_xlib_surface_get_height (xlib_surface) == height
    { pixmap = cairo_xlib_surface_get_drawable (xlib_surface); }
  else
    {
      pixmap = XCreatePixmap (FRAME_X_DISPLAY! (f), FRAME_X_RAW_DRAWABLE! (f),
			      width as c_uint, height as c_uint,
			      DefaultDepthOfScreen (FRAME_X_SCREEN! (f)) as c_uint);
      xlib_surface = cairo_xlib_surface_create (FRAME_X_DISPLAY! (f),
						pixmap, FRAME_X_VISUAL! (f),
						width, height);
      cairo_set_user_data (cr, &raw mut xlib_surface_key, xlib_surface as *mut c_void,
			   Some (x_cr_destroy_xlib_surface
				 as unsafe extern "C" fn (*mut cairo_surface_t)));
    }

  let buf: *mut cairo_t = cairo_create (xlib_surface);
  cairo_set_source_surface (buf, surface, 0.0, 0.0);
  let mut matrix: cairo_matrix_t = std::mem::zeroed ();
  cairo_get_matrix (cr, &raw mut matrix);
  cairo_pattern_set_matrix (cairo_get_source (cr), &raw mut matrix);
  cairo_set_operator (buf, CAIRO_OPERATOR_SOURCE);
  x_cr_gc_clip (buf, f, gc);
  cairo_paint (buf);
  cairo_destroy (buf);

  cairo_set_user_data (cr, &raw mut saved_drawable_key,
		       (WINDOW_HANDLE_UINTPTR (FRAME_X_RAW_DRAWABLE! (f))
			as *mut c_void),
		       None);
  FRAME_X_RAW_DRAWABLE! (f) = pixmap;
  cairo_surface_flush (xlib_surface);

  return true;
}

/* C 6194-6232: the other half of the Pixmap dance.  C's `(uintptr_t)` cast and the two
   `cairo_get_user_data` reads keep C's shape; `cairo_set_user_data`'s two `NULL`s are the port's
   `None`.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_end_cr_xlib_drawable (f: *mut frame, gc: GC)
{
  let mut cr: *mut cairo_t = FRAME_CR_CONTEXT! (f);
  if cr.is_null ()
    { return; }

  let mut saved_drawable: Drawable
    = cairo_get_user_data (cr, &raw mut saved_drawable_key) as uintptr_t as Drawable;
  let mut surface: *mut cairo_surface_t
    = (if saved_drawable != 0
       { cairo_get_user_data (cr, &raw mut xlib_surface_key) as *mut cairo_surface_t }
       else
	 { cairo_get_target (cr) });
  let gc_ext: *mut x_gc_ext_data = x_gc_get_ext_data (f, gc, 0);
  if gc_ext.is_null () == false && (*gc_ext).n_clip_rects != 0
    {
      for i in 0..(*gc_ext).n_clip_rects
	{
	  cairo_surface_mark_dirty_rectangle (surface,
					      (*gc_ext).clip_rects[i as usize].x as f64,
					      (*gc_ext).clip_rects[i as usize].y as f64,
					      (*gc_ext).clip_rects[i as usize].width as f64,
					      (*gc_ext).clip_rects[i as usize].height as f64);
	}
    }
  else
    { cairo_surface_mark_dirty (surface); }
  if saved_drawable == 0
    { return; }

  cairo_save (cr);
  cairo_set_source_surface (cr, surface, 0.0, 0.0);
  cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);
  x_cr_gc_clip (cr, f, gc);
  cairo_paint (cr);
  cairo_restore (cr);

  FRAME_X_RAW_DRAWABLE! (f) = saved_drawable;
  cairo_set_user_data (cr, &raw mut saved_drawable_key, std::ptr::null_mut (),
		       None);
}

/* Fringe bitmaps.  */

/* C 6233-6234: C's two file-scope objects.  They are this file's own (no mirror declares them),
   so they live here where C defines them.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
pub static mut max_fringe_bmp: c_int = 0;
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
pub static mut fringe_bmp: *mut *mut cairo_pattern_t = std::ptr::null_mut ();

/* C 6236-6270.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_cr_define_fringe_bitmap (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)
{
  let mut i: c_int;
  let mut stride: c_int;
  let mut surface: *mut cairo_surface_t;
  let mut pattern: *mut cairo_pattern_t;
  let mut data: *mut u8;

  if which >= max_fringe_bmp
    {
      i = max_fringe_bmp;
      max_fringe_bmp = which + 20;
      fringe_bmp = xrealloc (fringe_bmp as *mut c_void,
			     max_fringe_bmp as usize
			     * std::mem::size_of::<*mut cairo_pattern_t> ())
		   as *mut *mut cairo_pattern_t;
      while i < max_fringe_bmp
	{
	  *fringe_bmp.offset (i as isize) = std::ptr::null_mut ();
	  i += 1;
	}
    }

  block_input ();

  surface = cairo_image_surface_create (CAIRO_FORMAT_A1, wd, h);
  stride = cairo_image_surface_get_stride (surface);
  data = cairo_image_surface_get_data (surface);

  for i in 0..h
    {
      *(data as *mut c_ushort) = *bits.offset (i as isize);
      data = data.add (stride as usize);
    }

  cairo_surface_mark_dirty (surface);
  pattern = cairo_pattern_create_for_surface (surface);
  cairo_surface_destroy (surface);

  unblock_input ();

  *fringe_bmp.offset (which as isize) = pattern;
}

/* C 6272-6285.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_cr_destroy_fringe_bitmap (which: c_int)
{
  if which >= max_fringe_bmp
    { return; }

  if *fringe_bmp.offset (which as isize) != std::ptr::null_mut ()
    {
      block_input ();
      cairo_pattern_destroy (*fringe_bmp.offset (which as isize));
      unblock_input ();
    }
  *fringe_bmp.offset (which as isize) = std::ptr::null_mut ();
}

/* C 6287-6321.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_cr_draw_image (f: *mut frame, gc: GC, image: *mut cairo_pattern_t,
			   src_x: c_int, src_y: c_int, width: c_int, height: c_int,
			   dest_x: c_int, dest_y: c_int, overlay_p: bool)
{
  let cr: *mut cairo_t = x_begin_cr_clip (f, gc);

  if overlay_p
    { cairo_rectangle (cr, dest_x as f64, dest_y as f64, width as f64, height as f64); }
  else
    {
      x_set_cr_source_with_gc_background (f, gc, false);
      cairo_rectangle (cr, dest_x as f64, dest_y as f64, width as f64, height as f64);
      cairo_fill_preserve (cr);
    }

  cairo_translate (cr, (dest_x - src_x) as f64, (dest_y - src_y) as f64);

  let mut surface: *mut cairo_surface_t;
  cairo_pattern_get_surface (image, &raw mut surface);
  let format: cairo_format_t = cairo_image_surface_get_format (surface);
  if format != CAIRO_FORMAT_A8 && format != CAIRO_FORMAT_A1
    {
      cairo_set_source (cr, image);
      cairo_fill (cr);
    }
  else
    {
      x_set_cr_source_with_gc_foreground (f, gc, false);
      cairo_clip (cr);
      cairo_mask (cr, image);
    }

  x_end_cr_clip (f);
}

/* C 6323-6337.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_cr_draw_frame (cr: *mut cairo_t, f: *mut frame)
{
  let mut width: c_int;
  let mut height: c_int;

  width = FRAME_PIXEL_WIDTH (f);
  height = FRAME_PIXEL_HEIGHT (f);

  let saved_cr: *mut cairo_t = FRAME_CR_CONTEXT! (f);
  FRAME_CR_CONTEXT! (f) = cr;
  x_clear_area (f, 0, 0, width, height);
  expose_frame (f, 0, 0, width, height);
  FRAME_CR_CONTEXT! (f) = saved_cr;
}

/* C 6338-6347.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe extern "C" fn x_cr_accumulate_data (closure: *mut c_void,
					   data: *const c_uchar,
					   length: c_uint) -> cairo_status_t
{
  let acc: *mut Lisp_Object = closure as *mut Lisp_Object;

  *acc = Fcons (make_unibyte_string (data as *const c_char, length as ptrdiff_t), *acc);

  return CAIRO_STATUS_SUCCESS;
}

/* C 6349-6355.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe extern "C" fn x_cr_destroy (cr: *mut c_void)
{
  block_input ();
  cairo_destroy (cr as *mut cairo_t);
  unblock_input ();
}

/* C 6357-6451: the four cairo surface families C guards with `CAIRO_HAS_*`, which are cairo.h's
   own feature macros this tree cannot see, so each gets a derived feature (the
   `have_xcb_shape_input_rects` shape of Decision 3): `cairo_has_pdf_surface`,
   `cairo_has_png_functions`, `cairo_has_ps_surface` and `cairo_has_svg_surface`.  C adds one
   `else if` per live guard and its trailing `abort ()` is the arm the last live guard reaches, so
   the port keeps one `if`/`else` chain with the compile-time half of each guard a `cfg`-defined
   condition plus its own `#[cfg]` body (so a cairo build without that surface never names the
   function it lacks).  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[unsafe(no_mangle)]
pub unsafe fn x_cr_export_frames (frames: Lisp_Object, surface_type: cairo_surface_type_t)
				  -> Lisp_Object
{
  let mut f: *mut frame;
  let mut surface: *mut cairo_surface_t;
  let mut cr: *mut cairo_t;
  let mut width: c_int;
  let mut height: c_int;
  let mut surface_set_size_func: Option<unsafe extern "C" fn (*mut cairo_surface_t, f64, f64)>
    = None;
  let mut acc: Lisp_Object = Qnil ();
  let count: specpdl_ref = SPECPDL_INDEX ();

  redisplay_preserve_echo_area (31);

  f = XFRAME (XCAR (frames));
  frames = XCDR (frames);
  width = FRAME_PIXEL_WIDTH (f);
  height = FRAME_PIXEL_HEIGHT (f);

  block_input ();

  #[cfg (feature = "cairo_has_pdf_surface")]
  let pdf_arm_p: bool = surface_type == CAIRO_SURFACE_TYPE_PDF;
  #[cfg (not (feature = "cairo_has_pdf_surface"))]
  let pdf_arm_p: bool = false;
  #[cfg (feature = "cairo_has_png_functions")]
  let png_arm_p: bool = surface_type == CAIRO_SURFACE_TYPE_IMAGE;
  #[cfg (not (feature = "cairo_has_png_functions"))]
  let png_arm_p: bool = false;
  #[cfg (feature = "cairo_has_ps_surface")]
  let ps_arm_p: bool = surface_type == CAIRO_SURFACE_TYPE_PS;
  #[cfg (not (feature = "cairo_has_ps_surface"))]
  let ps_arm_p: bool = false;
  #[cfg (feature = "cairo_has_svg_surface")]
  let svg_arm_p: bool = surface_type == CAIRO_SURFACE_TYPE_SVG;
  #[cfg (not (feature = "cairo_has_svg_surface"))]
  let svg_arm_p: bool = false;

  if pdf_arm_p
    {
      #[cfg (feature = "cairo_has_pdf_surface")]
      {
	surface = cairo_pdf_surface_create_for_stream (Some (x_cr_accumulate_data),
						       &raw mut acc,
						       width as f64, height as f64);
	surface_set_size_func = Some (cairo_pdf_surface_set_size);
      }
    }
  else if png_arm_p
    {
      #[cfg (feature = "cairo_has_png_functions")]
      { surface = cairo_image_surface_create (CAIRO_FORMAT_RGB24, width, height); }
    }
  else if ps_arm_p
    {
      #[cfg (feature = "cairo_has_ps_surface")]
      {
	surface = cairo_ps_surface_create_for_stream (Some (x_cr_accumulate_data),
						      &raw mut acc,
						      width as f64, height as f64);
	surface_set_size_func = Some (cairo_ps_surface_set_size);
      }
    }
  else if svg_arm_p
    {
      #[cfg (feature = "cairo_has_svg_surface")]
      {
	surface = cairo_svg_surface_create_for_stream (Some (x_cr_accumulate_data),
						       &raw mut acc,
						       width as f64, height as f64);
      }
    }
  else
    { abort (); }

  cr = cairo_create (surface);
  cairo_surface_destroy (surface);
  record_unwind_protect_ptr (Some (x_cr_destroy), cr as *mut c_void);

  loop
    {
      let saved_cr: *mut cairo_t = FRAME_CR_CONTEXT! (f);
      FRAME_CR_CONTEXT! (f) = cr;
      x_clear_area (f, 0, 0, width, height);
      expose_frame (f, 0, 0, width, height);
      FRAME_CR_CONTEXT! (f) = saved_cr;

      if NILP (frames)
	{ break; }

      cairo_surface_show_page (surface);
      f = XFRAME (XCAR (frames));
      frames = XCDR (frames);
      width = FRAME_PIXEL_WIDTH (f);
      height = FRAME_PIXEL_HEIGHT (f);
      if surface_set_size_func.is_some ()
	{ (surface_set_size_func.unwrap ()) (surface, width as f64, height as f64); }

      unblock_input ();
      maybe_quit ();
      block_input ();
    }

  #[cfg (feature = "cairo_has_png_functions")]
  {
    if surface_type == CAIRO_SURFACE_TYPE_IMAGE
      {
	cairo_surface_flush (surface);
	cairo_surface_write_to_png_stream (surface, Some (x_cr_accumulate_data),
					   &raw mut acc);
      }
  }
  unblock_input ();

  unbind_to (count, Qnil ());

  return CALLN! (Fapply, Qconcat (), Fnreverse (acc));
}

/* C 6453-6477: `#if defined HAVE_XRENDER` - the extended clip.  Both definitions are C's
   non-`static` ones, so the port exports them (PORTING.md 0.2).  `x_xr_reset_ext_clip`'s
   `{ .clip_mask = None }` is a one-member compound literal, which the port writes as the zeroed
   record plus the store (`x_xr_ensure_picture`'s shape above).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[unsafe(no_mangle)]
pub unsafe fn x_xr_apply_ext_clip (f: *mut frame, gc: GC)
{
  use crate::x11_constants::None;

  eassert! (FRAME_X_PICTURE! (f) != None);

  let data: *mut x_gc_ext_data = x_gc_get_ext_data (f, gc, 1);

  if (*data).n_clip_rects != 0
    { XRenderSetPictureClipRectangles (FRAME_X_DISPLAY! (f),
				       FRAME_X_PICTURE! (f),
				       0, 0, (*data).clip_rects,
				       (*data).n_clip_rects); }
}

/* C 6467-6477.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[unsafe(no_mangle)]
pub unsafe fn x_xr_reset_ext_clip (f: *mut frame)
{
  use crate::x11_constants::{CPClipMask, None};

  let mut attrs: XRenderPictureAttributes = std::mem::zeroed ();
  attrs.clip_mask = None;

  XRenderChangePicture (FRAME_X_DISPLAY! (f),
			FRAME_X_PICTURE! (f),
			CPClipMask, &raw mut attrs);
}

/* C 6479-6493: C's `#if defined USE_CAIRO || defined HAVE_XRENDER` block is the GC bookkeeping
   the cairo and XRender drawing share; `Unsorted` is an X11 value this mirror has no row for
   (recorded in this ledger), and C's `memcpy (…, sizeof (XRectangle) * n)` is the port's
   byte-count `copy_nonoverlapping`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_clip_rectangles (f: *mut frame, gc: GC, rectangles: *mut XRectangle, n: c_int)
{
  XSetClipRectangles (FRAME_X_DISPLAY! (f), gc, 0, 0, rectangles, n, Unsorted);
  #[cfg (any (feature = "use_cairo", feature = "have_xrender"))]
  {
    eassert! (n >= 0 && (n as usize) <= MAX_CLIP_RECTS);

    {
      let gc_ext: *mut x_gc_ext_data = x_gc_get_ext_data (f, gc, 1);

      (*gc_ext).n_clip_rects = n;
      std::ptr::copy_nonoverlapping (rectangles as *const u8,
				     (&raw mut (*gc_ext).clip_rects) as *mut u8,
				     std::mem::size_of::<XRectangle> () * n as usize);
    }
  }
}

/* C 6495-6509.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_reset_clip_rectangles (f: *mut frame, gc: GC)
{
  use crate::x11_constants::None;

  XSetClipMask (FRAME_X_DISPLAY! (f), gc, None);
  #[cfg (any (feature = "use_cairo", feature = "have_xrender"))]
  {
    {
      let gc_ext: *mut x_gc_ext_data = x_gc_get_ext_data (f, gc, 0);

      if gc_ext.is_null () == false
	{ (*gc_ext).n_clip_rects = 0; }
    }
  }
}

/* C 6511-6569: `#ifdef HAVE_XRENDER` and, for the first, C 6512's
   `# if !defined USE_CAIRO && (RENDER_MAJOR > 0 || RENDER_MINOR >= 2)`.  The version test names
   Xrender.h's `RENDER_MAJOR`/`RENDER_MINOR` (a header this tree has no mirror for), so the port
   carries it under the derived feature `have_xrender_0_2` (the `have_xcomposite_overlay` shape of
   Decision 18).  C's `65535 * f->alpha_background` is a `double` product stored into the
   `unsigned short` member, so the port writes the conversion.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xrender",
	    not (feature = "use_cairo"), feature = "have_xrender_0_2"))]
unsafe fn x_xrender_color_from_gc_foreground (f: *mut frame, gc: GC,
					      color: *mut XRenderColor,
					      apply_alpha_background: bool)
{
  use crate::x11_constants::GCForeground;

  let mut xgcv: XGCValues = std::mem::zeroed ();
  let mut xc: XColor = std::mem::zeroed ();

  XGetGCValues (FRAME_X_DISPLAY! (f), gc, GCForeground, &raw mut xgcv);
  xc.pixel = xgcv.foreground;
  x_query_colors (f, &raw mut xc, 1);

  (*color).alpha = (if apply_alpha_background
		    { 65535.0 * (*f).alpha_background }
		    else
		      { 65535.0 }) as c_ushort;

  if (*color).alpha == 65535
    {
      (*color).red = xc.red;
      (*color).blue = xc.blue;
      (*color).green = xc.green;
    }
  else
    {
      (*color).red = ((xc.red as c_uint * (*color).alpha as c_uint) / 65535) as c_ushort;
      (*color).blue = ((xc.blue as c_uint * (*color).alpha as c_uint) / 65535) as c_ushort;
      (*color).green = ((xc.green as c_uint * (*color).alpha as c_uint) / 65535) as c_ushort;
    }
}

/* C 6541-6569: C's non-`static` definition.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xrender"))]
#[unsafe(no_mangle)]
pub unsafe fn x_xrender_color_from_gc_background (f: *mut frame, gc: GC,
						  color: *mut XRenderColor,
						  apply_alpha_background: bool)
{
  use crate::x11_constants::GCBackground;

  let mut xgcv: XGCValues = std::mem::zeroed ();
  let mut xc: XColor = std::mem::zeroed ();

  XGetGCValues (FRAME_X_DISPLAY! (f), gc, GCBackground, &raw mut xgcv);
  xc.pixel = xgcv.background;
  x_query_colors (f, &raw mut xc, 1);

  (*color).alpha = (if apply_alpha_background
		    { 65535.0 * (*f).alpha_background }
		    else
		      { 65535.0 }) as c_ushort;

  if (*color).alpha == 65535
    {
      (*color).red = xc.red;
      (*color).blue = xc.blue;
      (*color).green = xc.green;
    }
  else
    {
      (*color).red = ((xc.red as c_uint * (*color).alpha as c_uint) / 65535) as c_ushort;
      (*color).blue = ((xc.blue as c_uint * (*color).alpha as c_uint) / 65535) as c_ushort;
      (*color).green = ((xc.green as c_uint * (*color).alpha as c_uint) / 65535) as c_ushort;
    }
}

/* C 6571-6683: the two drawing arms and the XRender path.  `have_xrender_0_2` and
   `have_xrender_0_10` are the two version tests (the note above), and C's
   `#if RENDER_MAJOR > 0 || (RENDER_MINOR >= 10)` block carries its own locals, which the port
   declares in that block.  `FRAME_CHECK_XR_VERSION` is the mirror's macro (`include/xterm.rs`,
   C 1385-1390).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_fill_rectangle (f: *mut frame, gc: GC, x: c_int, y: c_int,
			    width: c_int, height: c_int,
			    respect_alpha_background: bool)
{
  #[cfg (feature = "use_cairo")]
  {
    use crate::x11_constants::{GCForeground, None};

    let dpy: *mut Display = FRAME_X_DISPLAY! (f);
    let cr: *mut cairo_t;
    let mut xgcv: XGCValues = std::mem::zeroed ();

    cr = x_begin_cr_clip (f, gc);
    XGetGCValues (dpy, gc, GCFillStyle | GCStipple, &raw mut xgcv);
    if xgcv.fill_style == FillSolid
	/* Invalid resource ID (one or more of the three most
	   significant bits set to 1) is obtained if the GCStipple
	   component has never been explicitly set.  It should be
	   regarded as Pixmap of unspecified size filled with ones.  */
	|| (xgcv.stipple & ((7 as Pixmap) << (std::mem::size_of::<Pixmap> () * 8 - 3))) != 0
      {
	x_set_cr_source_with_gc_foreground (f, gc, respect_alpha_background);
	cairo_rectangle (cr, x as f64, y as f64, width as f64, height as f64);
	cairo_fill (cr);
      }
    else
      {
	eassert! (xgcv.fill_style == FillOpaqueStippled);
	eassert! (xgcv.stipple != None);
	x_set_cr_source_with_gc_background (f, gc, respect_alpha_background);
	cairo_rectangle (cr, x as f64, y as f64, width as f64, height as f64);
	cairo_fill_preserve (cr);

	let pattern: *mut cairo_pattern_t = x_bitmap_stipple (f, xgcv.stipple);
	if pattern.is_null () == false
	  {
	    x_set_cr_source_with_gc_foreground (f, gc, respect_alpha_background);
	    cairo_clip (cr);
	    cairo_mask (cr, pattern);
	  }
      }
    x_end_cr_clip (f);
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    #[cfg (all (feature = "have_xrender", feature = "have_xrender_0_2"))]
    {
      if respect_alpha_background
	  && (*f).alpha_background != 1.0
	  && (*FRAME_DISPLAY_INFO! (f)).alpha_bits != 0
	  && FRAME_CHECK_XR_VERSION! (f, 0, 2)
	{
	  x_xr_ensure_picture (f);

	  if FRAME_X_PICTURE! (f) != None
	    {
	      let mut xc: XRenderColor = std::mem::zeroed ();

	      #[cfg (feature = "have_xrender_0_10")]
	      let mut xgcv: XGCValues = std::mem::zeroed ();
	      #[cfg (feature = "have_xrender_0_10")]
	      let mut attrs: XRenderPictureAttributes = std::mem::zeroed ();
	      #[cfg (feature = "have_xrender_0_10")]
	      let mut alpha: XRenderColor = std::mem::zeroed ();
	      #[cfg (feature = "have_xrender_0_10")]
	      let mut stipple: Picture = 0;
	      #[cfg (feature = "have_xrender_0_10")]
	      let mut fill: Picture = 0;

	      x_xr_apply_ext_clip (f, gc);

	      #[cfg (feature = "have_xrender_0_10")]
	      {
		XGetGCValues (FRAME_X_DISPLAY! (f),
			      gc, GCFillStyle | GCStipple, &raw mut xgcv);

		if xgcv.fill_style == FillOpaqueStippled
		    && FRAME_CHECK_XR_VERSION! (f, 0, 10)
		  {
		    x_xrender_color_from_gc_background (f, gc, &raw mut alpha, true);
		    x_xrender_color_from_gc_foreground (f, gc, &raw mut xc, true);
		    attrs.repeat = RepeatNormal;

		    stipple = XRenderCreatePicture (FRAME_X_DISPLAY! (f),
						    xgcv.stipple,
						    XRenderFindStandardFormat (FRAME_X_DISPLAY! (f),
									       PictStandardA1),
						    CPRepeat, &raw mut attrs);

		    XRenderFillRectangle (FRAME_X_DISPLAY! (f), PictOpSrc,
					  FRAME_X_PICTURE! (f),
					  &raw mut alpha, x, y, width, height);

		    fill = XRenderCreateSolidFill (FRAME_X_DISPLAY! (f), &raw mut xc);

		    XRenderComposite (FRAME_X_DISPLAY! (f), PictOpOver, fill, stipple,
				      FRAME_X_PICTURE! (f), 0, 0, x, y, x, y, width, height);

		    XRenderFreePicture (FRAME_X_DISPLAY! (f), stipple);
		    XRenderFreePicture (FRAME_X_DISPLAY! (f), fill);
		  }
		else
		  {
		    x_xrender_color_from_gc_foreground (f, gc, &raw mut xc, true);
		    XRenderFillRectangle (FRAME_X_DISPLAY! (f),
					  PictOpSrc, FRAME_X_PICTURE! (f),
					  &raw mut xc, x, y, width, height);
		  }
	      }

	      #[cfg (not (feature = "have_xrender_0_10"))]
	      {
		x_xrender_color_from_gc_foreground (f, gc, &raw mut xc, true);
		XRenderFillRectangle (FRAME_X_DISPLAY! (f),
				      PictOpSrc, FRAME_X_PICTURE! (f),
				      &raw mut xc, x, y, width, height);
	      }
	      x_xr_reset_ext_clip (f);
	      x_mark_frame_dirty (f);

	      return;
	    }
	}
    }

    XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
		    gc, x, y, width, height);
  }
}

/* Graphics primitives.  */

/* C 6685-6731.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_clear_rectangle (f: *mut frame, gc: GC, x: c_int, y: c_int,
			     width: c_int, height: c_int,
			     respect_alpha_background: bool)
{
  #[cfg (feature = "use_cairo")]
  {
    let cr: *mut cairo_t;

    cr = x_begin_cr_clip (f, gc);
    x_set_cr_source_with_gc_background (f, gc, respect_alpha_background);
    cairo_rectangle (cr, x as f64, y as f64, width as f64, height as f64);
    cairo_fill (cr);
    x_end_cr_clip (f);
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    use crate::x11_constants::{GCBackground, GCForeground};

    #[cfg (all (feature = "have_xrender", feature = "have_xrender_0_2"))]
    {
      if respect_alpha_background
	  && (*f).alpha_background != 1.0
	  && (*FRAME_DISPLAY_INFO! (f)).alpha_bits != 0
	  && FRAME_CHECK_XR_VERSION! (f, 0, 2)
	{
	  x_xr_ensure_picture (f);

	  if FRAME_X_PICTURE! (f) != None
	    {
	      let mut xc: XRenderColor = std::mem::zeroed ();

	      x_xr_apply_ext_clip (f, gc);
	      x_xrender_color_from_gc_background (f, gc, &raw mut xc, true);
	      XRenderFillRectangle (FRAME_X_DISPLAY! (f),
				    PictOpSrc, FRAME_X_PICTURE! (f),
				    &raw mut xc, x, y, width, height);
	      x_xr_reset_ext_clip (f);
	      x_mark_frame_dirty (f);

	      return;
	    }
	}
    }

    /* C 6719-6731: C's `#else` tail.  */
    let mut xgcv: XGCValues = std::mem::zeroed ();
    let dpy: *mut Display = FRAME_X_DISPLAY! (f);
    XGetGCValues (dpy, gc, GCBackground | GCForeground, &raw mut xgcv);
    XSetForeground (dpy, gc, xgcv.background);
    XFillRectangle (dpy, FRAME_X_DRAWABLE! (f),
		    gc, x, y, width, height);
    XSetForeground (dpy, gc, xgcv.foreground);
  }
}

/* C 6733-6749.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_rectangle (f: *mut frame, gc: GC, x: c_int, y: c_int,
			    width: c_int, height: c_int)
{
  #[cfg (feature = "use_cairo")]
  {
    let cr: *mut cairo_t;

    cr = x_begin_cr_clip (f, gc);
    x_set_cr_source_with_gc_foreground (f, gc, false);
    cairo_rectangle (cr, x as f64 + 0.5, y as f64 + 0.5, width as f64, height as f64);
    cairo_set_line_width (cr, 1.0);
    cairo_stroke (cr);
    x_end_cr_clip (f);
  }
  #[cfg (not (feature = "use_cairo"))]
  {
    XDrawRectangle (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
		    gc, x, y, width, height);
  }
}

/* C 6751-6777: the `#ifndef USE_GTK` arm whose condition C's `#ifdef HAVE_XDBE` extends.  The
   port names that disjunct (`bounding_only`'s shape, Decision 15) so C's one `if`/`else` stays
   one statement per configuration.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_clear_window (f: *mut frame)
{
  #[cfg (feature = "use_cairo")]
  {
    let cr: *mut cairo_t;

    cr = x_begin_cr_clip (f, std::ptr::null_mut ());
    x_set_cr_source_with_gc_background (f, (*(*f).output_data.x as *mut x_output).normal_gc,
					true);
    cairo_paint (cr);
    x_end_cr_clip (f);
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    #[cfg (not (feature = "use_gtk"))]
    {
      #[cfg (feature = "have_xdbe")]
      let double_buffered_p: bool = FRAME_X_DOUBLE_BUFFERED_P! (f);
      #[cfg (not (feature = "have_xdbe"))]
      let double_buffered_p: bool = false;

      if (*f).alpha_background != 1.0
	  || double_buffered_p
	{ x_clear_area (f, 0, 0, FRAME_PIXEL_WIDTH (f),
			FRAME_PIXEL_HEIGHT (f)); }
      else
	{ XClearWindow (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f)); }
    }
    #[cfg (feature = "use_gtk")]
    {
      x_clear_area (f, 0, 0, FRAME_PIXEL_WIDTH (f),
		    FRAME_PIXEL_HEIGHT (f));
    }
  }
}

/* C 6775-6865: `#ifdef USE_CAIRO` - the relief trapezoids, the corners and the horizontal wave,
   dead in the frozen reading and carried under their feature (P-51).  C's `enum corners` is in
   this file (not a header), so it is this unit's record; `M_PI_2` is math.h's constant and
   `cairo_arc` takes `double`s, so the port writes the widening.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_fill_trapezoid_for_relief (f: *mut frame, gc: GC, x: c_int, y: c_int,
				       width: c_int, height: c_int, top_p: c_int)
{
  let cr: *mut cairo_t;

  cr = x_begin_cr_clip (f, gc);
  x_set_cr_source_with_gc_foreground (f, gc, false);
  cairo_move_to (cr, (if top_p != 0 { x } else { x + height }) as f64, y as f64);
  cairo_line_to (cr, x as f64, (y + height) as f64);
  cairo_line_to (cr, (if top_p != 0 { x + width - height } else { x + width }) as f64,
		 (y + height) as f64);
  cairo_line_to (cr, (x + width) as f64, y as f64);
  cairo_fill (cr);
  x_end_cr_clip (f);
}

#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
#[repr(C)]
pub enum corners
{
  CORNER_BOTTOM_RIGHT,	/* 0 -> pi/2 */
  CORNER_BOTTOM_LEFT,		/* pi/2 -> pi */
  CORNER_TOP_LEFT,		/* pi -> 3pi/2 */
  CORNER_TOP_RIGHT,		/* 3pi/2 -> 2pi */
  CORNER_LAST,
}

/* C 6804-6835: C's `xm = x - margin, xc = xm + radius;` is the comma operator, so the port puts
   the two stores on their own lines (the `magic_db` row's class).  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_erase_corners_for_relief (f: *mut frame, gc: GC, x: c_int, y: c_int,
				      width: c_int, height: c_int,
				      radius: f64, margin: f64, corners: c_int)
{
  let cr: *mut cairo_t;
  let mut i: c_int;

  cr = x_begin_cr_clip (f, gc);
  x_set_cr_source_with_gc_background (f, gc, false);
  for i in 0..(corners::CORNER_LAST as c_int)
    {
      if corners & (1 << i) != 0
	{
	  let mut xm: f64;
	  let mut ym: f64;
	  let mut xc: f64;
	  let mut yc: f64;

	  if i == corners::CORNER_TOP_LEFT as c_int
	      || i == corners::CORNER_BOTTOM_LEFT as c_int
	    {
	      xm = x as f64 - margin;
	      xc = xm + radius;
	    }
	  else
	    {
	      xm = (x + width) as f64 + margin;
	      xc = xm - radius;
	    }
	  if i == corners::CORNER_TOP_LEFT as c_int
	      || i == corners::CORNER_TOP_RIGHT as c_int
	    {
	      ym = y as f64 - margin;
	      yc = ym + radius;
	    }
	  else
	    {
	      ym = (y + height) as f64 + margin;
	      yc = ym - radius;
	    }

	  cairo_move_to (cr, xm, ym);
	  cairo_arc (cr, xc, yc, radius, (i as f64) * M_PI_2, ((i + 1) as f64) * M_PI_2);
	}
    }
  cairo_clip (cr);
  cairo_rectangle (cr, x as f64, y as f64, width as f64, height as f64);
  cairo_fill (cr);
  x_end_cr_clip (f);
}

/* C 6837-6863: C's `xoffset` is an `int` and `dx`/`dy` are `double`s, so the port writes the
   conversions where C converts implicitly; `--n` is the port's `n -= 1`.  */
#[cfg (all (feature = "have_x_windows", feature = "use_cairo"))]
unsafe fn x_draw_horizontal_wave (f: *mut frame, gc: GC, x: c_int, y: c_int,
				  width: c_int, height: c_int, wave_length: c_int)
{
  let cr: *mut cairo_t;
  let mut dx: f64 = wave_length as f64;
  let mut dy: f64 = (height - 1) as f64;
  let mut xoffset: c_int;
  let mut n: c_int;

  cr = x_begin_cr_clip (f, gc);
  x_set_cr_source_with_gc_foreground (f, gc, false);
  cairo_rectangle (cr, x as f64, y as f64, width as f64, height as f64);
  cairo_clip (cr);

  if x >= 0
    {
      xoffset = x % (wave_length * 2);
      if xoffset == 0
	{ xoffset = wave_length * 2; }
    }
  else
    { xoffset = x % (wave_length * 2) + wave_length * 2; }
  n = (width + xoffset) / wave_length + 1;
  if xoffset > wave_length
    {
      xoffset -= wave_length;
      n -= 1;
      y += height - 1;
      dy = -dy;
    }

  cairo_move_to (cr, (x - xoffset) as f64 + 0.5, y as f64 + 0.5);
  loop
    {
      n -= 1;
      if n < 0
	{ break; }
      cairo_rel_line_to (cr, dx, dy);
      dy = -dy;
    }
  cairo_set_line_width (cr, 1.0);
  cairo_stroke (cr);
  x_end_cr_clip (f);
}

/* Return the struct x_display_info corresponding to DPY,
   when it is guaranteed that one will correspond.  */

/* C 6883-6893: C's `for (…; ; …)` never tests, so the port writes the `loop` its body already
   is (the `emit_pdump_object_functions` shape).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_dpyinfo (dpy: *mut Display) -> *mut x_display_info
{
  let mut dpyinfo: *mut x_display_info = x_display_list;
  loop
    {
      if (*dpyinfo).display == dpy
	{ return dpyinfo; }
      dpyinfo = (*dpyinfo).next;
    }
}

/* Return the struct x_display_info corresponding to DPY,
   or a null pointer if none corresponds.  */

/* C 6895-6905.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_display_info_for_display (dpy: *mut Display) -> *mut x_display_info
{
  let mut dpyinfo: *mut x_display_info;

  dpyinfo = x_display_list;
  while !dpyinfo.is_null ()
    {
      if (*dpyinfo).display == dpy
	{ return dpyinfo; }

      dpyinfo = (*dpyinfo).next;
    }

  return 0 as *mut x_display_info;
}

/* C 6907-6928.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_find_topmost_parent (f: *mut frame) -> Window
{
  use crate::x11_constants::None;

  let x: *mut x_output = (*f).output_data.x as *mut x_output;
  let mut win: Window = None as Window;
  let mut wi: Window = (*x).parent_desc;
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);

  while wi != (*FRAME_DISPLAY_INFO! (f)).root_window
    {
      let mut root: Window;
      let mut children: *mut Window;
      let mut nchildren: c_uint;

      win = wi;
      if XQueryTree (dpy, win, &raw mut root, &raw mut wi, &raw mut children,
		     &raw mut nchildren) != 0
	{ XFree (children as *mut c_void); }
      else
	{ break; }
    }

  return win;
}

/* C 6929: `#define OPAQUE 0xffffffff` - a value, so the port writes the constant the extractor
   reads it as (the `X_DND_SUPPORTED_VERSION` shape).  */
#[cfg (feature = "have_x_windows")]
pub const OPAQUE: c_ulong = 0xffffffff;

/* C 6932-6989.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_frame_alpha (f: *mut frame)
{
  use crate::x11_constants::{None, PropModeReplace, XA_CARDINAL};

  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);
  let win: Window = FRAME_OUTER_WINDOW! (f);
  let mut alpha: c_double = 1.0;
  let mut alpha_min: c_double = 1.0;
  let mut opac: c_ulong;
  let mut parent: Window;

  if (*dpyinfo).highlight_frame == f
    { alpha = (*f).alpha[0]; }
  else
    { alpha = (*f).alpha[1]; }

  if alpha < 0.0
    { return; }

  if FLOATP (Vframe_alpha_lower_limit)
    { alpha_min = XFLOAT_DATA (Vframe_alpha_lower_limit); }
  else if FIXNUMP (Vframe_alpha_lower_limit)
    { alpha_min = (XFIXNUM (Vframe_alpha_lower_limit)) as c_double / 100.0; }

  if alpha > 1.0
    { alpha = 1.0; }
  else if alpha < alpha_min && alpha_min <= 1.0
    { alpha = alpha_min; }

  opac = (alpha * OPAQUE as c_double) as c_ulong;

  /* If there is a parent from the window manager, put the property there
     also, to work around broken window managers that fail to do that.
     Do this unconditionally as this function is called on reparent when
     alpha has not changed on the frame.  */

  x_ignore_errors_for_next_request (dpyinfo, 0);

  if FRAME_PARENT_FRAME (f).is_null ()
    {
      parent = x_find_topmost_parent (f);

      if parent != None as Window
	{
	  XChangeProperty (dpy, parent,
			   (*dpyinfo).Xatom_net_wm_window_opacity,
			   XA_CARDINAL, 32, PropModeReplace,
			   &raw mut opac as *mut u8, 1);
	}
    }

  XChangeProperty (dpy, win, (*dpyinfo).Xatom_net_wm_window_opacity,
		   XA_CARDINAL, 32, PropModeReplace,
		   &raw mut opac as *mut u8, 1);
  x_stop_ignoring_errors (dpyinfo);
}

/* ---- C 6990-7410: the `Starting and ending an update` section and C 6994's
   `#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME` block - the
   frame-synchronization machinery (the event wait, the two monotonic-time readers, the
   frame-time recorder, the two drawing hooks and the counter/fence plumbing).  Three gates,
   each the lowercased name of one macro of that condition (P-51): `have_xsync`, `not use_gtk`
   and `have_clock_gettime`; C 7249's inner `#ifdef HAVE_XSYNCTRIGGERFENCE` is
   `have_xsynctriggerfence`, and the two `#ifdef FRAME_DEBUG` blocks (C 7101-7106, 7271-7273)
   are `frame_debug` - the `glyph_debug` shape `docs/ports/dispnew.md` 220-224 uses for a
   source-level debug knob the configure script knows nothing about.  Every function of this
   block is dead in the frozen reading (HAVE_XSYNC and HAVE_XSYNCTRIGGERFENCE are both `undef`
   in docs/baseline/config/darwin-arm64.macros.tsv), so none of them is in the checklist and no
   arm of this tree compiles one: the text is carried (P-51), not verified.  */

/* C 6990-6993 is the section banner (`Starting and ending an update`): a comment in C, carried
   as one here.  */

/* C 6996-7047: `x_if_event`.  `Bool`/`XPointer` are Xlib's own aliases (`include/xlib.rs` 198,
   202), `Display` its opaque record (xlib.rs 858-861); C's callback parameter is spelled as the
   port's function-pointer option (`widget.rs` 332-336's shape) and the call passes C's own
   function name from the other side.  `fd_set`, `FD_ZERO`/`FD_SET` and `pselect` are
   `include/sysselect.rs`'s (`lib/include/sys_select.rs` 39, 52-56, 87, 111).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_if_event (dpy: *mut Display, event_return: *mut XEvent,
		      predicate: Option<unsafe extern "C" fn (*mut Display, *mut XEvent,
							       XPointer) -> Bool>,
		      arg: XPointer, timeout: timespec) -> c_int
{
  let mut current_time: timespec;
  let mut target: timespec;
  let fd: c_int;
  let mut fds: fd_set;

  fd = ConnectionNumber (dpy);
  current_time = current_timespec ();
  target = timespec_add (current_time, timeout);

  /* Check if an event is already in the queue.  If it is, avoid
     syncing.  */
  if XCheckIfEvent (dpy, event_return, predicate, arg) != 0
    { return 0; }

  loop
    {
      /* Get events into the queue.  */
      XSync (dpy, 0);

      /* Look for an event again.  */
      if XCheckIfEvent (dpy, event_return, predicate, arg) != 0
	{ return 0; }

      /* Calculate the timeout.  */
      current_time = current_timespec ();
      timeout = timespec_sub (target, current_time);

      /* If not, wait for some input to show up on the X connection,
	 or for the timeout to elapse.  */
      FD_ZERO (&raw mut fds);
      FD_SET (fd, &raw mut fds);

      /* If this fails due to an IO error, XSync will call the IO
	 error handler.  */
      pselect (fd + 1, &raw mut fds, std::ptr::null_mut (), std::ptr::null_mut (),
	       &raw mut timeout, std::ptr::null_mut ());

      /* Timeout elapsed.  */
      current_time = current_timespec ();
      if timespec_cmp (target, current_time) < 0
	{ return 1; }
    }
}

/* C 7049-7066: `x_sync_get_monotonic_time`.  `ckd_sub` is `lib/include/intprops-internal.rs`
   (PORTING.md P-15); C's ternary is the port's `if`/`else` (PORTING.md 2.1).  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_get_monotonic_time (dpyinfo: *mut x_display_info,
				     timestamp: uint_fast64_t) -> uint_fast64_t
{
  if (*dpyinfo).server_time_monotonic_p
    { return timestamp; }

  /* This means we haven't yet initialized the server time offset.  */
  if !(*dpyinfo).server_time_offset
    { return 0; }

  /* C: `uint_fast64_t t; return ckd_sub (&t, timestamp, dpyinfo->server_time_offset) ? 0 : t;`
     - the P-15 `ckd_sub`/`overflowing_sub` pair (docs/ports/token-aliases.tsv 10): the wrapped
     difference is stored either way and the flag is what the ternary reads.  */
  let mut t: uint_fast64_t = 0;
  let (t_, sub_overflow) = timestamp.overflowing_sub ((*dpyinfo).server_time_offset);
  t = t_;
  if sub_overflow
    { return 0; }
  t
}

/* C 7068-7079: `x_sync_current_monotonic_time`.  `monotonic_coarse_timespec` is timefns.c's
   (`include/systime.rs` 81); C's `||` of two ckd_* calls keeps C's two-operand shape.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_current_monotonic_time () -> uint_fast64_t
{
  let time: timespec = monotonic_coarse_timespec ();
  /* C: `return ((ckd_mul (&t, time.tv_sec, 1000000) || ckd_add (&t, t, time.tv_nsec / 1000))
     ? 0 : t);` - the `ckd_mul`/`overflowing_mul` and `ckd_add`/`overflowing_add` pairs
     (P-15); C's operands are converted to the destination's type, and the destination is only
     read when neither operation overflowed.  */
  let mut t: uint_fast64_t = 0;
  let (t_prod, mul_overflow) = (time.tv_sec as uint_fast64_t).overflowing_mul (1000000);
  t = t_prod;
  let (t_sum, add_overflow) = t.overflowing_add (time.tv_nsec as uint_fast64_t / 1000);
  t = t_sum;
  if mul_overflow || add_overflow
    { 0 }
  else
    { t }
}

/* C 7081-7107: `x_sync_note_frame_times`, with C 7101-7106's `#ifdef FRAME_DEBUG` arm carried
   under `frame_debug`.  C's `event->xclient` is a read of the mirror's *opaque* `XEvent`
   (`include/xlib.rs` 1181, the 33 owed members of `docs/ports/include-xlib.md`) - the class
   slices 2, 5 and 8 already carry.  The debug arm's format string is C's concatenation of two
   `PRIuFAST64` expansions (`lib/include/inttypes.in.rs` 615-619), which this unit's include
   graph does not reach: the arm is not compiled by any arm of this tree (the ledger records
   the gap), and the port keeps C's two-part text with the macro that spells it.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_note_frame_times (dpyinfo: *mut x_display_info,
				   f: *mut frame, event: *mut XEvent)
{
  let mut low: uint_fast64_t;
  let mut high: uint_fast64_t;
  let mut time: uint_fast64_t;
  let output: *mut x_output;

  low = (*event).xclient.data.l[2];
  high = (*event).xclient.data.l[3];
  output = FRAME_X_OUTPUT! (f);

  time = x_sync_get_monotonic_time (dpyinfo, low | (high << 32));

  /* C: `if (!time || !output->temp_frame_time || ckd_sub (&output->last_frame_time, time,
     output->temp_frame_time)) output->last_frame_time = 0;` - the `ckd_sub` pair (P-15); the
     store C only performs when both terms are non-zero is inside the guard, so the read of the
     result and the zeroing keep C's order.  */
  let mut sub_overflow = false;
  if time != 0 && (*output).temp_frame_time != 0
    {
      let (last_frame_time_, overflow) = time.overflowing_sub ((*output).temp_frame_time);
      (*output).last_frame_time = last_frame_time_;
      sub_overflow = overflow;
    }

  if time == 0 || !(*output).temp_frame_time || sub_overflow
    { (*output).last_frame_time = 0; }

  #[cfg (feature = "frame_debug")]
  {
    let last_frame_ms: uint_fast64_t = (*output).last_frame_time / 1000;
    fprintf (stderr,
	     concat! ("Drawing the last frame took: %", PRIuFAST64! (), " ms (%",
		      PRIuFAST64! (), ")\n").as_ptr () as *const c_char,
	     last_frame_ms, time);
  }
}

/* C 7109-7126: `x_sync_is_frame_drawn_event`.  C's `True`/`False` are `include/xterm.rs` 76/79's
   constants; the port writes Rust's literals (slice 5's rule) because the token check
   case-folds.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_is_frame_drawn_event (dpy: *mut Display, event: *mut XEvent,
				       user_data: XPointer) -> Bool
{
  let f: *mut frame;
  let dpyinfo: *mut x_display_info;

  f = user_data as *mut frame;
  dpyinfo = FRAME_DISPLAY_INFO! (f);

  if (*event).r#type == ClientMessage
      && ((*event).xclient.message_type
	  == (*dpyinfo).Xatom_net_wm_frame_drawn)
      && (*event).xclient.window == FRAME_OUTER_WINDOW! (f)
    { return true as Bool; }

  return false as Bool;
}

/* C 7128-7182: `x_sync_wait_for_frame_drawn_event`.  `make_timespec` is `include/systime.rs`
   52's; `store_frame_param` and `Quse_frame_synchronization` are keyboard.c's and
   `include/lisp.rs`'s, both reached through this unit's include graph.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_wait_for_frame_drawn_event (f: *mut frame)
{
  let mut event: XEvent = std::mem::zeroed ();
  let dpyinfo: *mut x_display_info;

  if !FRAME_X_WAITING_FOR_DRAW! (f)
      /* The compositing manager can't draw a frame if it is
	 unmapped.  */
      || !FRAME_VISIBLE_P! (f)
    { return; }

  dpyinfo = FRAME_DISPLAY_INFO! (f);

  /* Wait for the frame drawn message to arrive.  */
  if x_if_event (FRAME_X_DISPLAY! (f), &raw mut event,
		 Some (x_sync_is_frame_drawn_event), f as XPointer,
		 make_timespec (1, 0)) != 0
    {
      /* The first time a draw hangs, treat it as a random fluctuation
	 on the part of the compositor.  If the next draw continues to
	 hang, disable frame synchronization.  */
      if FRAME_X_DRAW_JUST_HUNG! (f)
	{
	  fprintf (stderr, c"Warning: compositing manager spent more than 1 second drawing a frame.  Frame synchronization has been disabled\n".as_ptr ());
	  (*FRAME_X_OUTPUT! (f)).use_vsync_p = false;

	  /* Remove the compositor bypass property from the outer
	     window.  */
	  XDeleteProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_bypass_compositor);

	  /* Also change the frame parameter to reflect the new
	     state.  */
	  store_frame_param (f, Quse_frame_synchronization, Qnil);
	}
      else
	{
	  fprintf (stderr, c"Warning: compositing manager spent more than 1 second drawing a frame.  Frame synchronization will be disabled if this happens again\n".as_ptr ());
	  FRAME_X_DRAW_JUST_HUNG! (f) = true;
	}
    }
  else
    { x_sync_note_frame_times (dpyinfo, f, &raw mut event); }

  FRAME_X_WAITING_FOR_DRAW! (f) = false;
}

/* C 7184-7247: `x_sync_update_begin`.  C's `eassert` is the prelude's `eassert!`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_update_begin (f: *mut frame)
{
  use crate::x11_constants::None;

  let mut value: XSyncValue;
  let mut add: XSyncValue;
  let mut overflow: Bool;

  if FRAME_X_EXTENDED_COUNTER! (f) == None as XSyncCounter
    { return; }

  value = FRAME_X_COUNTER_VALUE! (f);

  if (*FRAME_X_OUTPUT! (f)).ext_sync_end_pending_p
    {
      FRAME_X_COUNTER_VALUE! (f)
	= (*FRAME_X_OUTPUT! (f)).resize_counter_value;

      value = FRAME_X_COUNTER_VALUE! (f);

      if XSyncValueLow32 (value) % 2
	{
	  XSyncIntToValue (&raw mut add, 1);
	  XSyncValueAdd (&raw mut value, value, add, &raw mut overflow);

	  if overflow
	    { XSyncIntToValue (&raw mut value, 0); }
	}

      (*FRAME_X_OUTPUT! (f)).ext_sync_end_pending_p = false;
    }

  /* Since a frame is already in progress, there is no point in
     continuing.  */
  if XSyncValueLow32 (value) % 2
    { return; }

  /* Wait for the last frame to be drawn before drawing this one.  */
  x_sync_wait_for_frame_drawn_event (f);

  /* Make a note of the time at which we started to draw this
     frame.  */
  (*FRAME_X_OUTPUT! (f)).temp_frame_time = x_sync_current_monotonic_time ();

  /* Since Emacs needs a non-urgent redraw, ensure that value % 4 ==
     1.  Later, add 3 to create the even counter value.  */
  if XSyncValueLow32 (value) % 4 == 2
    { XSyncIntToValue (&raw mut add, 3); }
  else
    { XSyncIntToValue (&raw mut add, 1); }

  XSyncValueAdd (&raw mut FRAME_X_COUNTER_VALUE! (f),
		 value, add, &raw mut overflow);

  if overflow
    { XSyncIntToValue (&raw mut FRAME_X_COUNTER_VALUE! (f), 3); }

  eassert! (XSyncValueLow32 (FRAME_X_COUNTER_VALUE! (f)) % 4 == 1);

  XSyncSetCounter (FRAME_X_DISPLAY! (f),
		   FRAME_X_EXTENDED_COUNTER! (f),
		   FRAME_X_COUNTER_VALUE! (f));
}

/* C 7249-7334: `#ifdef HAVE_XSYNCTRIGGERFENCE` - `x_sync_trigger_fence` (with C 7271-7273's
   `FRAME_DEBUG` arm), `x_sync_init_fences` and `x_sync_free_fences`.  C's `x_sync_init_fences`
   has no `static`, so the port keeps it an exported one (PORTING.md 0.2) - the `x_xr_ensure_picture`
   shape of slice 8.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime", feature = "have_xsynctriggerfence"))]
unsafe fn x_sync_trigger_fence (f: *mut frame, value: XSyncValue)
{
  let mut n: uint_fast64_t;
  let mut low: uint_fast64_t;
  let mut high: uint_fast64_t;
  let mut idx: uint_fast64_t;

  /* Sync fences aren't supported by the X server.  */
  if (*FRAME_DISPLAY_INFO! (f)).xsync_major < 3
      || ((*FRAME_DISPLAY_INFO! (f)).xsync_major == 3
	  && (*FRAME_DISPLAY_INFO! (f)).xsync_minor < 1)
    { return; }

  low = XSyncValueLow32 (value);
  high = XSyncValueHigh32 (value);

  n = low | (high << 32);
  idx = (n / 4) % 2;

  #[cfg (feature = "frame_debug")]
  { fprintf (stderr, c"Triggering synchronization fence: %lu\n".as_ptr (), idx); }

  XSyncTriggerFence (FRAME_X_DISPLAY! (f),
		     (*FRAME_X_OUTPUT! (f)).sync_fences[idx as usize]);
}

/* C 7279-7320: `x_sync_init_fences`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime", feature = "have_xsynctriggerfence"))]
#[unsafe(no_mangle)]
pub unsafe fn x_sync_init_fences (f: *mut frame)
{
  use crate::x11_constants::{None, PropModeReplace, XA_CARDINAL};

  let output: *mut x_output;
  let dpyinfo: *mut x_display_info;

  output = FRAME_X_OUTPUT! (f);
  dpyinfo = FRAME_DISPLAY_INFO! (f);

  /* Sync fences aren't supported by the X server.  */
  if (*dpyinfo).xsync_major < 3
      || ((*dpyinfo).xsync_major == 3
	  && (*dpyinfo).xsync_minor < 1)
    { return; }

  /* Suppress errors around XSyncCreateFence requests, since its
     implementations on certain X servers erroneously reject valid
     drawables, such as the frame's inner window.  (bug#69762) */

  x_catch_errors ((*dpyinfo).display);
  (*output).sync_fences[0]
    = XSyncCreateFence (FRAME_X_DISPLAY! (f),
			/* The drawable given below is only used to
			   determine the screen on which the fence is
			   created.  */
			FRAME_X_WINDOW! (f),
			false);
  (*output).sync_fences[1]
    = XSyncCreateFence (FRAME_X_DISPLAY! (f),
			FRAME_X_WINDOW! (f),
			false);
  if x_had_errors_p ((*dpyinfo).display)
    { (*output).sync_fences[1] = None as XSyncFence; (*output).sync_fences[0] = None as XSyncFence; }
  x_uncatch_errors_after_check ();

  XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_OUTER_WINDOW! (f),
		   (*dpyinfo).Xatom_net_wm_sync_fences, XA_CARDINAL,
		   32, PropModeReplace,
		   &raw mut (*output).sync_fences as *mut u8, 2);
}

/* C 7322-7332: `x_sync_free_fences`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime", feature = "have_xsynctriggerfence"))]
unsafe fn x_sync_free_fences (f: *mut frame)
{
  use crate::x11_constants::None;

  if (*FRAME_X_OUTPUT! (f)).sync_fences[0] != None as XSyncFence
    { XSyncDestroyFence (FRAME_X_DISPLAY! (f),
			 (*FRAME_X_OUTPUT! (f)).sync_fences[0]); }

  if (*FRAME_X_OUTPUT! (f)).sync_fences[1] != None as XSyncFence
    { XSyncDestroyFence (FRAME_X_DISPLAY! (f),
			 (*FRAME_X_OUTPUT! (f)).sync_fences[1]); }
}

/* C 7336-7379: `x_sync_update_finish`, with C 7369-7371's inner `HAVE_XSYNCTRIGGERFENCE` arm
   gated on its own feature inside the statement list.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_update_finish (f: *mut frame)
{
  use crate::x11_constants::None;

  let mut value: XSyncValue;
  let mut add: XSyncValue;
  let mut overflow: Bool;

  if FRAME_X_EXTENDED_COUNTER! (f) == None as XSyncCounter
    { return; }

  value = FRAME_X_COUNTER_VALUE! (f);

  if XSyncValueLow32 (value) % 2 == 0
    { return; }

  if (XSyncValueLow32 (value) % 4) == 1
    /* This means the frame is non-urgent and should be drawn at the
       next redraw point.  */
    { XSyncIntToValue (&raw mut add, 3); }
  else
    /* Otherwise, the frame is urgent and should be drawn as soon as
       possible.  */
    { XSyncIntToValue (&raw mut add, 1); }

  XSyncValueAdd (&raw mut FRAME_X_COUNTER_VALUE! (f),
		 value, add, &raw mut overflow);

  if overflow
    { XSyncIntToValue (&raw mut FRAME_X_COUNTER_VALUE! (f), 0); }

  /* Trigger any sync fences if necessary.  */
  #[cfg (feature = "have_xsynctriggerfence")]
  { x_sync_trigger_fence (f, FRAME_X_COUNTER_VALUE! (f)); }

  XSyncSetCounter (FRAME_X_DISPLAY! (f),
		   FRAME_X_EXTENDED_COUNTER! (f),
		   FRAME_X_COUNTER_VALUE! (f));

  if (*FRAME_OUTPUT_DATA! (f)).use_vsync_p
    { FRAME_X_WAITING_FOR_DRAW! (f) = true; }
}

/* C 7381-7409: `x_sync_handle_frame_drawn`, and C 7410's `#endif`.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"),
	    feature = "have_clock_gettime"))]
unsafe fn x_sync_handle_frame_drawn (dpyinfo: *mut x_display_info,
				     message: *mut XEvent, f: *mut frame)
{
  let mut value: XSyncValue;
  let mut counter: XSyncValue;

  if FRAME_OUTER_WINDOW! (f) == (*message).xclient.window
    {
      counter = FRAME_X_COUNTER_VALUE! (f);

      /* Check that the counter in the message is the same as the
	 counter in the frame.  */
      XSyncIntsToValue (&raw mut value,
			(*message).xclient.data.l[0] & 0xffffffff,
			(*message).xclient.data.l[1] & 0xffffffff);

      if XSyncValueEqual (value, counter)
	{ FRAME_X_WAITING_FOR_DRAW! (f) = false; }

      /* As long as a _NET_WM_FRAME_DRAWN message arrives, we know
	 that the compositor is still sending events, so avoid timing
	 out.  */
      FRAME_X_DRAW_JUST_HUNG! (f) = false;
    }

  x_sync_note_frame_times (dpyinfo, f, message);
}

/* ---- C 7412-7524: `x_update_begin`, the two window-border drawing hooks and the divider.  The
   three are the file gate's own (no checklist box is under a condition), so this is the first
   text of the slice the plain `have_x_windows` probe reaches.  */

/* C 7412-7438: `x_update_begin`.  C's condition pair is an `#if`-half (`HAVE_XDBE`'s
   `!FRAME_X_DOUBLE_BUFFERED_P (f)` tests the *call*) so the port carries both arms: with Xdbe
   the call is guarded, without it C has no `if` at all.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_update_begin (f: *mut frame)
{
  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
	      feature = "have_clock_gettime"))]
  {
    /* If F is double-buffered, we can make the entire frame center
       around XdbeSwapBuffers.  */
    #[cfg (feature = "have_xdbe")]
    if !FRAME_X_DOUBLE_BUFFERED_P! (f)
      { x_sync_update_begin (f); }
    #[cfg (not (feature = "have_xdbe"))]
    { x_sync_update_begin (f); }
  }

  /* The `#else` arm is C's own text: `Nothing to do.` inside C's own comment markers.  */

  #[cfg (feature = "have_xdbe")]
  {
    if FRAME_X_DOUBLE_BUFFERED_P! (f)
      /* The frame is no longer complete, as it is in the midst of an
	 update.  */
      { FRAME_X_COMPLETE_P! (f) = false; }
  }
}

/* C 7440-7459: `x_draw_vertical_window_border`.  C's `f->output_data.x` is
   `frame.rs` 35-38's `*mut c_void` member, so each read casts it, the `x_find_topmost_parent`
   shape (this file, C 6907-6928).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_vertical_window_border (w: *mut window, x: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let mut face: *mut face;

  face = FACE_FROM_ID_OR_NULL (f, VERTICAL_BORDER_FACE_ID);
  if !face.is_null ()
    { XSetForeground (FRAME_X_DISPLAY! (f),
		      (*((*f).output_data.x as *mut x_output)).normal_gc,
		      (*face).foreground); }

  #[cfg (feature = "use_cairo")]
  { x_fill_rectangle (f, (*((*f).output_data.x as *mut x_output)).normal_gc, x, y0, 1, y1 - y0,
		      false); }

  #[cfg (not (feature = "use_cairo"))]
  { XDrawLine (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
	       (*((*f).output_data.x as *mut x_output)).normal_gc, x, y0, x, y1); }
}

/* C 7461-7524: `x_draw_window_divider`.  C's three `?:` are the port's `if`/`else`
   (PORTING.md 2.1), and the repeated `f->output_data.x` reads take the cast of the comment
   above.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_window_divider (w: *mut window, x0: c_int, x1: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let face: *mut face = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FACE_ID);
  let face_first: *mut face
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID);
  let face_last: *mut face
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_LAST_PIXEL_FACE_ID);
  let mut color: c_ulong = if !face.is_null ()
    { (*face).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let mut color_first: c_ulong = if !face_first.is_null ()
    { (*face_first).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let mut color_last: c_ulong = if !face_last.is_null ()
    { (*face_last).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let display: *mut Display = FRAME_X_DISPLAY! (f);
  let x: *mut x_output = (*f).output_data.x as *mut x_output;

  if (y1 - y0 > x1 - x0) && (x1 - x0 >= 3)
    /* A vertical divider, at least three pixels wide: Draw first and
       last pixels differently.  */
    {
      XSetForeground (display, (*x).normal_gc, color_first);
      x_fill_rectangle (f, (*x).normal_gc,
			x0, y0, 1, y1 - y0,
			(*f).borders_respect_alpha_background);
      XSetForeground (display, (*x).normal_gc, color);
      x_fill_rectangle (f, (*x).normal_gc,
			x0 + 1, y0, x1 - x0 - 2, y1 - y0,
			(*f).borders_respect_alpha_background);
      XSetForeground (display, (*x).normal_gc, color_last);
      x_fill_rectangle (f, (*x).normal_gc,
			x1 - 1, y0, 1, y1 - y0,
			(*f).borders_respect_alpha_background);
    }
  else if (x1 - x0 > y1 - y0) && (y1 - y0 >= 3)
    /* A horizontal divider, at least three pixels high: Draw first and
       last pixels differently.  */
    {
      XSetForeground (display, (*x).normal_gc, color_first);
      x_fill_rectangle (f, (*x).normal_gc,
			x0, y0, x1 - x0, 1,
			(*f).borders_respect_alpha_background);
      XSetForeground (display, (*x).normal_gc, color);
      x_fill_rectangle (f, (*x).normal_gc,
			x0, y0 + 1, x1 - x0, y1 - y0 - 2,
			(*f).borders_respect_alpha_background);
      XSetForeground (display, (*x).normal_gc, color_last);
      x_fill_rectangle (f, (*x).normal_gc,
			x0, y1 - 1, x1 - x0, 1,
			(*f).borders_respect_alpha_background);
    }
  else
    {
    /* In any other case do not draw the first and last pixels
       differently.  */
      XSetForeground (display, (*x).normal_gc, color);
      x_fill_rectangle (f, (*x).normal_gc,
			x0, y0, x1 - x0, y1 - y0,
			(*f).borders_respect_alpha_background);
    }
}

/* ---- C 7526-7809: the double-buffered flip, the two update hooks, the internal border and
   C 7752's `x_after_update_window_line`.  */

/* C 7526-7570: `#ifdef HAVE_XDBE` - `show_back_buffer`.  `XdbeSwapInfo`/`XdbeSwapBuffers`/
   `XdbeCopied` are `<X11/extensions/Xdbe.h>`'s and have no mirror (the ledger's gap list);
   C's `sizeof (swap_info)` is `std::mem::size_of_val` (the `sizeof` family of
   docs/ports/token-aliases.tsv 15) because the type behind it has no Rust name here.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]
unsafe fn show_back_buffer (f: *mut frame)
{
  let mut swap_info: XdbeSwapInfo = std::mem::zeroed ();
  #[cfg (feature = "use_cairo")]
  let mut cr: *mut cairo_t;

  if FRAME_X_DOUBLE_BUFFERED_P! (f)
    {
      #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
		  feature = "have_clock_gettime"))]
      {
	/* Wait for drawing of the previous frame to complete before
	   displaying this new frame.  */
	x_sync_wait_for_frame_drawn_event (f);

	/* Begin a new frame.  */
	x_sync_update_begin (f);
      }

      #[cfg (feature = "use_cairo")]
      {
	cr = FRAME_CR_CONTEXT! (f);
	if !cr.is_null ()
	  { cairo_surface_flush (cairo_get_target (cr)); }
      }
      memset (&raw mut swap_info as *mut c_void, 0, std::mem::size_of_val (&swap_info));
      swap_info.swap_window = FRAME_X_WINDOW! (f);
      swap_info.swap_action = XdbeCopied;
      XdbeSwapBuffers (FRAME_X_DISPLAY! (f), &raw mut swap_info, 1);

      #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
		  feature = "have_clock_gettime"))]
      {
	/* Finish the frame here.  */
	x_sync_update_finish (f);
      }
    }

  FRAME_X_NEED_BUFFER_FLIP! (f) = false;
}

/* C 7572-7598: `x_flip_and_flush`.  `NILP` is a function (include/blockinput.rs' neighbour,
   lisp.h's own); C's `#ifdef HAVE_XDBE` run is one gated block.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_flip_and_flush (f: *mut frame)
{
  /* Flipping buffers requires a working connection to the X server,
     which isn't always present if `inhibit-redisplay' is t, since
     this can be called from the IO error handler.  */
  if !NILP (Vinhibit_redisplay)
      /* This has to work for tooltip frames, however, and redisplay
	 cannot happen when they are being flushed anyway.  (bug#55519) */
      && !FRAME_TOOLTIP_P (f)
    { return; }

  block_input ();
  #[cfg (feature = "have_xdbe")]
  {
    if FRAME_X_NEED_BUFFER_FLIP! (f)
      { show_back_buffer (f); }

    /* The frame is complete again as its contents were just
       flushed.  */
    FRAME_X_COMPLETE_P! (f) = true;
  }
  x_flush (f);
  unblock_input ();
}

/* C 7600-7625: `x_update_end`.  C's two `#if` runs are the cairo/Xdbe pair and the
   XSYNC+Xdbe pair of `x_update_begin`; both arms are carried (P-51).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_update_end (f: *mut frame)
{
  /* Mouse highlight may be displayed again.  */
  (*MOUSE_HL_INFO! (f)).mouse_face_defer = false;

  #[cfg (all (feature = "use_cairo", feature = "have_xdbe"))]
  {
    if !FRAME_X_DOUBLE_BUFFERED_P! (f) && !FRAME_CR_CONTEXT! (f).is_null ()
      { cairo_surface_flush (cairo_get_target (FRAME_CR_CONTEXT! (f))); }
  }

  /* If double buffering is disabled, finish the update here.
     Otherwise, finish the update when the back buffer is next
     displayed.  */
  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
	      feature = "have_clock_gettime"))]
  {
    #[cfg (feature = "have_xdbe")]
    if !FRAME_X_DOUBLE_BUFFERED_P! (f)
      { x_sync_update_finish (f); }
    #[cfg (not (feature = "have_xdbe"))]
    { x_sync_update_finish (f); }
  }
}

/* C 7627-7672: `XTframe_up_to_date`.  C 7646-7670's `HAVE_XSYNC` run holds two arms - the plain
   one and `HAVE_GTK3`'s Gdk frame-clock arm - so the port carries them as the two halves of an
   `#ifndef HAVE_GTK3`/`#else` pair.  */
#[cfg (feature = "have_x_windows")]
unsafe fn XTframe_up_to_date (f: *mut frame)
{
  use crate::x11_constants::None;

  eassert! (FRAME_X_P (f));
  block_input ();
  FRAME_MOUSE_UPDATE! (f);

  #[cfg (feature = "have_xdbe")]
  {
    if !buffer_flipping_blocked_p ()
	&& FRAME_X_NEED_BUFFER_FLIP! (f)
      { show_back_buffer (f); }

    /* The frame is now complete, as its contents have been drawn.  */
    FRAME_X_COMPLETE_P! (f) = true;
  }

  #[cfg (all (feature = "have_xsync", not (feature = "have_gtk3")))]
  {
    if (*FRAME_X_OUTPUT! (f)).sync_end_pending_p
	&& FRAME_X_BASIC_COUNTER! (f) != None as XSyncCounter
      {
	XSyncSetCounter (FRAME_X_DISPLAY! (f),
			 FRAME_X_BASIC_COUNTER! (f),
			 (*FRAME_X_OUTPUT! (f)).pending_basic_counter_value);
	(*FRAME_X_OUTPUT! (f)).sync_end_pending_p = false;
      }
  }

  #[cfg (all (feature = "have_xsync", feature = "have_gtk3"))]
  {
    if (*FRAME_X_OUTPUT! (f)).xg_sync_end_pending_p
      {
	let widget: *mut GtkWidget = FRAME_GTK_OUTER_WIDGET! (f);
	let window: *mut GdkWindow = gtk_widget_get_window (widget);
	eassert! (!window.is_null ());
	let clock: *mut GdkFrameClock = gdk_window_get_frame_clock (window);
	eassert! (!clock.is_null ());

	gdk_frame_clock_request_phase (clock,
				       GDK_FRAME_CLOCK_PHASE_AFTER_PAINT);
	(*FRAME_X_OUTPUT! (f)).xg_sync_end_pending_p = false;
      }
  }
  unblock_input ();
}

/* C 7674-7685: `#ifdef HAVE_XDBE` - `XTbuffer_flipping_unblocked_hook`.  It is dead in the
   frozen reading and has no checklist box (HAVE_XDBE is `undef`), so it is carried (P-51) and
   not demanded.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xdbe"))]
unsafe fn XTbuffer_flipping_unblocked_hook (f: *mut frame)
{
  block_input ();

  if FRAME_X_NEED_BUFFER_FLIP! (f)
    { show_back_buffer (f); }

  unblock_input ();
}

/* C 7687-7743: `x_clear_under_internal_border` (C's `/** … */` doc comment is carried as
   prose).  C's nested `?:` pair is the port's nested `if`/`else` (PORTING.md 2.1) and C's two
   `NULL`s become `std::ptr::null_mut ()` (the `null` family of token-aliases.tsv 94).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_clear_under_internal_border (f: *mut frame)
{
  if FRAME_INTERNAL_BORDER_WIDTH (f) > 0
    {
      let border: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);
      let width: c_int = FRAME_PIXEL_WIDTH (f);
      let height: c_int = FRAME_PIXEL_HEIGHT (f);
      let margin: c_int = FRAME_TOP_MARGIN_HEIGHT (f);
      let bottom_margin: c_int = FRAME_BOTTOM_MARGIN_HEIGHT (f);
      let face_id: c_int = if !FRAME_PARENT_FRAME (f).is_null ()
	{ if !NILP (Vface_remapping_alist)
	    { lookup_basic_face (std::ptr::null_mut (), f, CHILD_FRAME_BORDER_FACE_ID) }
	  else
	    { CHILD_FRAME_BORDER_FACE_ID } }
	else
	{ if !NILP (Vface_remapping_alist)
	    { lookup_basic_face (std::ptr::null_mut (), f, INTERNAL_BORDER_FACE_ID) }
	  else
	    { INTERNAL_BORDER_FACE_ID } };
      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);

      if !face.is_null ()
	{
	  let color: c_ulong = (*face).background;
	  let display: *mut Display = FRAME_X_DISPLAY! (f);
	  let gc: GC = (*((*f).output_data.x as *mut x_output)).normal_gc;

	  XSetForeground (display, gc, color);
	  x_fill_rectangle (f, gc, 0, margin, width, border,
			    (*f).borders_respect_alpha_background);
	  x_fill_rectangle (f, gc, 0, 0, border, height,
			    (*f).borders_respect_alpha_background);
	  x_fill_rectangle (f, gc, width - border, 0, border, height,
			    (*f).borders_respect_alpha_background);
	  x_fill_rectangle (f, gc, 0, height - bottom_margin - border,
			    width, border,
			    (*f).borders_respect_alpha_background);
	  XSetForeground (display, gc, FRAME_FOREGROUND_PIXEL (f));
	}
      else
	{
	  x_clear_area (f, 0, 0, border, height);
	  x_clear_area (f, 0, margin, width, border);
	  x_clear_area (f, width - border, 0, border, height);
	  x_clear_area (f, 0, height - bottom_margin - border,
			width, border);
	}
    }
}

/* The one window bit this slice's text names (`w->pseudo_window_p`, C 7756).  `window.rs` has
   no dispatcher for the nineteen flags yet, so the unit carries the arm it uses - the
   `dispnew.rs` 390-425 shape, whose `WINDOW_PSEUDO_WINDOW_P_SHIFT` is the mirror's measured
   position.  The member name stays the macro's first argument, so the ported text keeps C's
   token (P-80).  */
#[cfg (feature = "have_x_windows")]
macro_rules! window_bits {
  (pseudo_window_p, $w:expr) => { ((*($w)).bits >> WINDOW_PSEUDO_WINDOW_P_SHIFT) & 1 != 0 };
  (pseudo_window_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }
	else { (*($w)).bits & !(1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }; };
}

/* C 7745-7809: `x_after_update_window_line`.  C's two comma expressions are `if`-condition
   blocks so that each assignment happens exactly where C's `&&` chain reaches it and exactly
   once (the `dims_ok` shape of slice 8, `docs/ports/fringe.md` deviation 5 for the other
   order).  The `#ifdef USE_X_TOOLKIT` half is a gated block, both arms carried (P-51).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_after_update_window_line (w: *mut window, desired_row: *mut glyph_row)
{
  eassert! (!w.is_null ());

  if !glyph_row_bits! (mode_line_p, desired_row) && !window_bits! (pseudo_window_p, w)
    { glyph_row_bits! (redraw_fringe_bitmaps_p, desired_row, true); }

  #[cfg (feature = "use_x_toolkit")]
  {
    /* When a window has disappeared, make sure that no rest of
       full-width rows stays visible in the internal border.  Could
       check here if updated window is the leftmost/rightmost window,
       but I guess it's not worth doing since vertically split windows
       are almost never used, internal border is rarely set, and the
       overhead is very small.  */
    {
      let f: *mut frame;
      let mut width: c_int;
      let mut height: c_int;

      if windows_or_buffers_changed != 0
	  && glyph_row_bits! (full_width_p, desired_row)
	  && {
	       f = XFRAME ((*w).frame);
	       width = FRAME_INTERNAL_BORDER_WIDTH (f);
	       width != 0
	     }
	  && {
	       height = (*desired_row).visible_height;
	       height > 0
	     }
	{
	  let y: c_int = WINDOW_TO_FRAME_PIXEL_Y! (w, max (0, (*desired_row).y as isize) as c_int);
	  let face_id: c_int =
	    if !FRAME_PARENT_FRAME (f).is_null ()
	    { if !NILP (Vface_remapping_alist)
		{ lookup_basic_face (std::ptr::null_mut (), f, CHILD_FRAME_BORDER_FACE_ID) }
	      else
		{ CHILD_FRAME_BORDER_FACE_ID } }
	    else
	    { if !NILP (Vface_remapping_alist)
		{ lookup_basic_face (std::ptr::null_mut (), f, INTERNAL_BORDER_FACE_ID) }
	      else
		{ INTERNAL_BORDER_FACE_ID } };
	  let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);

	  if !face.is_null ()
	    {
	      let color: c_ulong = (*face).background;
	      let display: *mut Display = FRAME_X_DISPLAY! (f);
	      let gc: GC = (*((*f).output_data.x as *mut x_output)).normal_gc;

	      XSetForeground (display, gc, color);
	      x_fill_rectangle (f, gc, 0, y, width, height, true);
	      x_fill_rectangle (f, gc, FRAME_PIXEL_WIDTH (f) - width, y,
				width, height, true);
	      XSetForeground (display, gc, FRAME_FOREGROUND_PIXEL (f));
	    }
	  else
	    {
	      x_clear_area (f, 0, y, width, height);
	      x_clear_area (f, FRAME_PIXEL_WIDTH (f) - width, y, width, height);
	    }
	}
    }
  }
}

/* ---- C 7811-8062: the premultiplied pixel helper, `x_draw_fringe_bitmap` and C 8052's `Glyph
   display` banner with the three forward declarations under it.  */

/* C 7811-7849: `#ifndef USE_CAIRO` - `x_premultiply_pixel`.  C's three `*= alpha` are compound
   assignments of a 16-bit member (the port writes the double conversion the C compiler
   performs), and C's `(unsigned long) (alpha * 65535)` is the port's `as c_ulong`
   (PORTING.md 2.1's cast rule).  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe fn x_premultiply_pixel (dpyinfo: *mut x_display_info,
			       color: *mut XColor, alpha: c_double) -> c_ulong
{
  let mut pixel: c_ulong;

  eassert! ((*dpyinfo).alpha_bits != 0);

  /* Multiply the RGB channels.  */
  (*color).red = ((*color).red as c_double * alpha) as c_ushort;
  (*color).green = ((*color).green as c_double * alpha) as c_ushort;
  (*color).blue = ((*color).blue as c_double * alpha) as c_ushort;

  /* First, allocate a fully opaque pixel.  */
  pixel = x_make_truecolor_pixel (dpyinfo, (*color).red as c_int,
				  (*color).green as c_int,
				  (*color).blue as c_int);

  /* Next, erase the alpha component.  */
  pixel &= !(*dpyinfo).alpha_mask;

  /* And add an alpha channel.  */
  pixel |= (((alpha * 65535.0) as c_ulong
	     >> (16 - (*dpyinfo).alpha_bits))
	    << (*dpyinfo).alpha_offset);

  pixel
}

/* C 7851-8050: `x_draw_fringe_bitmap`.  Three shapes worth naming: C 7864's `p->bx >= 0` and
   the two `p->cursor_p`/`p->overlay_p` tests are `bool_bf` members of
   `draw_fringe_bitmap_params`, so the port reads them with the mirror's
   `get_cursor_p`/`get_overlay_p` (the `fringe.rs` 991-992 pair's readers; the ledger's
   `macro-body-notes.tsv` row carries the two member names the C body spells and the port
   cannot); C 7946's `goto undo_clip` (whose label C puts at 8046, inside the `#else` arm) is
   the port's labelled `'undo_clip` block; and C 7888/7916's `#ifdef USE_CAIRO`/`#else` pair is
   two gated blocks, so the `#else` arm's `undo_clip:` label stays inside the arm C puts it in
   (the arm without the label has no `goto`).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_fringe_bitmap (w: *mut window, row: *mut glyph_row,
				p: *mut draw_fringe_bitmap_params)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let display: *mut Display = FRAME_X_DISPLAY! (f);
  let gc: GC = (*((*f).output_data.x as *mut x_output)).normal_gc;
  let face: *mut face = (*p).face;
  let mut clip_rect: XRectangle;

  /* Must clip because of partially visible lines.  */
  x_clip_to_row (w, row, ANY_AREA, gc, &raw mut clip_rect);

  if (*p).bx >= 0 && !get_overlay_p (p)
    {
      /* In case the same realized face is used for fringes and
	 for something displayed in the text (e.g. face `region' on
	 mono-displays, the fill style may have been changed to
	 FillSolid in x_draw_glyph_string_background.  */
      if (*face).stipple != 0
	{
	  XSetFillStyle (display, (*face).gc as GC, FillOpaqueStippled);
	  x_fill_rectangle (f, (*face).gc as GC, (*p).bx, (*p).by, (*p).nx, (*p).ny,
			    true);
	  XSetFillStyle (display, (*face).gc as GC, FillSolid);

	  glyph_row_bits! (stipple_p, row, true);
	}
      else
	{
	  XSetBackground (display, (*face).gc as GC, (*face).background);
	  x_clear_rectangle (f, (*face).gc as GC, (*p).bx, (*p).by, (*p).nx, (*p).ny,
			     true);
	  XSetForeground (display, (*face).gc as GC, (*face).foreground);
	}
    }

  #[cfg (feature = "use_cairo")]
  {
    if (*p).which != 0
	&& (*p).which < max_fringe_bmp
	&& (*p).which < max_used_fringe_bitmap
      {
	let mut gcv: XGCValues = std::mem::zeroed ();

	XGetGCValues (display, gc, GCForeground | GCBackground, &raw mut gcv as *mut c_void);
	XSetForeground (display, gc,
			if get_cursor_p (p)
			{ if get_overlay_p (p) { (*face).background }
			  else { (*((*f).output_data.x as *mut x_output)).cursor_pixel } }
			else { (*face).foreground });
	XSetBackground (display, gc, (*face).background);
	if (*fringe_bmp.offset ((*p).which as isize)).is_null ()
	  {
	    /* This fringe bitmap is known to fringe.c, but lacks the
	       cairo_pattern_t pattern which shadows that bitmap.  This
	       is typical to define-fringe-bitmap being called when the
	       selected frame was not a GUI frame, for example, when
	       packages that define fringe bitmaps are loaded by a
	       daemon Emacs.  Create the missing pattern now.  */
	    gui_define_fringe_bitmap (f, (*p).which);
	  }
	x_cr_draw_image (f, gc, *fringe_bmp.offset ((*p).which as isize), 0, (*p).dh,
			 (*p).wd, (*p).h, (*p).x, (*p).y, get_overlay_p (p));
	XSetForeground (display, gc, gcv.foreground);
	XSetBackground (display, gc, gcv.background);
      }
  }

  #[cfg (not (feature = "use_cairo"))]
  'undo_clip: {
    if (*p).which != 0
      {
	let drawable: Drawable = FRAME_X_DRAWABLE! (f);
	let mut bits: *mut c_char;
	let mut pixmap: Pixmap;
	let mut clipmask: Pixmap = None as Pixmap;
	let depth: c_int = (*FRAME_DISPLAY_INFO! (f)).n_planes;
	let mut gcv: XGCValues = std::mem::zeroed ();
	let mut background: c_ulong = (*face).background;
	let mut bg: XColor = std::mem::zeroed ();
	#[cfg (feature = "have_xrender")]
	let mut picture: Picture = None as Picture;
	#[cfg (feature = "have_xrender")]
	let mut attrs: XRenderPictureAttributes = std::mem::zeroed ();

	#[cfg (feature = "have_xrender")]
	{ memset (&raw mut attrs as *mut c_void, 0, std::mem::size_of_val (&attrs)); }
	let mut image_rect: XRectangle;
	let mut dest: XRectangle;
	let mut px: c_int;
	let mut py: c_int;
	let mut pwidth: c_int;
	let mut pheight: c_int;

	/* Intersect the destination rectangle with that of the row.
	   Setting a clip mask overrides the clip rectangles provided by
	   x_clip_to_row, so clipping must be performed by hand.  */

	image_rect.x = (*p).x as c_short;
	image_rect.y = (*p).y as c_short;
	image_rect.width = (*p).wd as c_ushort;
	image_rect.height = (*p).h as c_ushort;

	if !gui_intersect_rectangles (&raw mut clip_rect, &raw mut image_rect, &raw mut dest)
	  /* The entire destination rectangle falls outside the row.  */
	  { break 'undo_clip; }

	/* Extrapolate the source rectangle from the difference between
	   the destination and image rectangles.  */

	px = dest.x as c_int - image_rect.x as c_int;
	py = dest.y as c_int - image_rect.y as c_int;
	pwidth = dest.width as c_int;
	pheight = dest.height as c_int;

	if (*p).wd > 8
	  { bits = (*p).bits.offset ((*p).dh as isize) as *mut c_char; }
	else
	  { bits = ((*p).bits as *mut c_char).offset ((*p).dh as isize); }

	if (*FRAME_DISPLAY_INFO! (f)).alpha_bits != 0
	    && (*f).alpha_background < 1.0
	  {
	    /* Extend the background color with an alpha channel
	       according to f->alpha_background.  */
	    bg.pixel = background;
	    x_query_colors (f, &raw mut bg, 1);

	    background
	      = x_premultiply_pixel (FRAME_DISPLAY_INFO! (f),
				     &raw mut bg,
				     (*f).alpha_background);
	  }

	/* Draw the bitmap.  I believe these small pixmaps can be cached
	   by the server.  */
	pixmap = XCreatePixmapFromBitmapData (display, drawable, bits, (*p).wd, (*p).h,
					      if get_cursor_p (p)
					      { if get_overlay_p (p) { (*face).background }
						else { (*((*f).output_data.x as *mut x_output)).cursor_pixel } }
					      else { (*face).foreground },
					      background, depth);

	#[cfg (feature = "have_xrender")]
	{
	  if !FRAME_X_PICTURE_FORMAT! (f).is_null ()
	      && { x_xr_ensure_picture (f); !FRAME_X_PICTURE! (f).is_null () }
	    { picture = XRenderCreatePicture (display, pixmap,
					      FRAME_X_PICTURE_FORMAT! (f),
					      0, &raw mut attrs); }
	}

	if get_overlay_p (p)
	  {
	    clipmask = XCreatePixmapFromBitmapData (display,
						    (*FRAME_DISPLAY_INFO! (f)).root_window,
						    bits, (*p).wd, (*p).h,
						    1, 0, 1);

	    #[cfg (feature = "have_xrender")]
	    {
	      if picture != None as Picture
		{
		  attrs.clip_mask = clipmask;
		  attrs.clip_x_origin = (*p).x;
		  attrs.clip_y_origin = (*p).y;

		  XRenderChangePicture (display, FRAME_X_PICTURE! (f),
					CPClipMask | CPClipXOrigin | CPClipYOrigin,
					&raw mut attrs);
		}
	      else
		{
		  gcv.clip_mask = clipmask;
		  gcv.clip_x_origin = (*p).x;
		  gcv.clip_y_origin = (*p).y;
		  XChangeGC (display, gc, GCClipMask | GCClipXOrigin | GCClipYOrigin,
			     &raw mut gcv);
		}
	    }
	    /* C's `#else` half: without HAVE_XRENDER the statement is the else body itself, so
	       the port carries it as this arm's text (P-51).  */
	    #[cfg (not (feature = "have_xrender"))]
	    {
	      gcv.clip_mask = clipmask;
	      gcv.clip_x_origin = (*p).x;
	      gcv.clip_y_origin = (*p).y;
	      XChangeGC (display, gc, GCClipMask | GCClipXOrigin | GCClipYOrigin, &raw mut gcv);
	    }
	  }

	#[cfg (feature = "have_xrender")]
	{
	  if picture != None as Picture
	    {
	      x_xr_apply_ext_clip (f, gc);
	      XRenderComposite (display, PictOpSrc, picture,
				None as Picture, FRAME_X_PICTURE! (f),
				px, py, px, py, dest.x as c_int, dest.y as c_int,
				pwidth, pheight);
	      x_xr_reset_ext_clip (f);

	      XRenderFreePicture (display, picture);
	    }
	  else
	    { XCopyArea (display, pixmap, drawable, gc, px, py,
			 pwidth, pheight, dest.x as c_int, dest.y as c_int); }
	}
	#[cfg (not (feature = "have_xrender"))]
	{ XCopyArea (display, pixmap, drawable, gc, px, py,
		     pwidth, pheight, dest.x as c_int, dest.y as c_int); }
	XFreePixmap (display, pixmap);

	if get_overlay_p (p)
	  {
	    gcv.clip_mask = 0 as Pixmap;
	    XChangeGC (display, gc, GCClipMask, &raw mut gcv);
	    XFreePixmap (display, clipmask);
	  }
      }
  }

  x_reset_clip_rectangles (f, gc);
}

/* C 8052-8062: the `Glyph display` section banner (a comment in C, carried as one here) and
   C 8056-8062's three forward declarations - `x_alloc_lighter_color` (defined at C 9591),
   `x_scroll_bar_clear` (C 17777) and, inside `#ifdef GLYPH_DEBUG`, `x_check_font` (C 30433).
   PORTING.md has no item for a prototype (C 891's `x_push_selection_request` was slice 1's
   precedent): the definitions are later slices, and the one *call* C 8347 makes through the
   debug guard is kept as C's `IF_DEBUG (x_check_font (…))`.  */


/* ---- C 8064-8307: the last-user-time bookkeeping and its three helpers.  */

/* C 8064-8176: `x_display_set_last_user_time`.  The seven `ckd_*` calls are written as the
   P-15 `overflowing_*` pairs *inside* C's `&&`/`||` chains, so each store happens exactly where
   C's short-circuit puts it (the block-expression shape of `callproc.rs` 1859/`editfns.rs`
   2468); C's `#ifndef USE_GTK` and `ENABLE_CHECKING` halves are gated, both arms carried.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_display_set_last_user_time (dpyinfo: *mut x_display_info, mut time: Time,
					send_event: bool, set_property: bool)
{
  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
	      feature = "have_clock_gettime"))]
  let mut monotonic_time: uint_fast64_t;
  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
	      feature = "have_clock_gettime"))]
  let mut monotonic_ms: uint_fast64_t;
  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
	      feature = "have_clock_gettime"))]
  let mut diff_ms: int_fast64_t;

  #[cfg (not (feature = "use_gtk"))]
  let mut focus_frame: *mut frame;
  #[cfg (not (feature = "use_gtk"))]
  let old_time: Time;

  #[cfg (not (feature = "use_gtk"))]
  {
    focus_frame = (*dpyinfo).x_focus_frame;
    old_time = (*dpyinfo).last_user_time;
  }

  #[cfg (feature = "enable_checking")]
  { eassert! (time <= X_ULONG_MAX); }

  if !send_event || time > (*dpyinfo).last_user_time
    { (*dpyinfo).last_user_time = time; }

  #[cfg (all (feature = "have_xsync", not (feature = "use_gtk"),
	      feature = "have_clock_gettime"))]
  {
    if !send_event
      {
	/* See if the current CLOCK_MONOTONIC time is reasonably close
	   to the X server time.  */
	monotonic_time = x_sync_current_monotonic_time ();
	monotonic_ms = monotonic_time / 1000;

	let mut server_time_monotonic_p = false;
	if monotonic_time != 0
	  {
	    let (diff_ms_, sub_overflow)
	      = (time as int_fast64_t).overflowing_sub (monotonic_ms as int_fast64_t);
	    diff_ms = diff_ms_;
	    server_time_monotonic_p
	      = !sub_overflow
		&& -500 < diff_ms && diff_ms < 500;
	  }
	(*dpyinfo).server_time_monotonic_p = server_time_monotonic_p;

	if !(*dpyinfo).server_time_monotonic_p
	  {
	    /* Compute an offset that can be subtracted from the server
	       time to estimate the monotonic time on the X server.  */

	    if monotonic_time == 0
		|| {
		     let (offset_, mul_overflow)
		       = (time as int_fast64_t).overflowing_mul (1000);
		     (*dpyinfo).server_time_offset = offset_;
		     mul_overflow
		   }
		|| {
		     let (offset_, sub_overflow)
		       = (*dpyinfo).server_time_offset
			   .overflowing_sub (monotonic_time as int_fast64_t);
		     (*dpyinfo).server_time_offset = offset_;
		     sub_overflow
		   }
	      { (*dpyinfo).server_time_offset = 0; }

	    /* If the server time is reasonably close to the monotonic
	       time after the latter is truncated to CARD32, simply make
	       the offset that between the server time in ms and the
	       actual time in ms.  */

	    monotonic_ms = monotonic_ms & 0xffffffff;
	    let (diff_ms_, sub_overflow)
	      = (time as int_fast64_t).overflowing_sub (monotonic_ms as int_fast64_t);
	    diff_ms = diff_ms_;
	    if !sub_overflow && -500 < diff_ms && diff_ms < 500
	      {
		/* The server timestamp overflowed.  Make the time
		   offset exactly how much it overflowed by.  */

		if {
		     let (offset_, overflow)
		       = ((monotonic_time / 1000) as int_fast64_t)
			   .overflowing_sub (monotonic_ms as int_fast64_t);
		     (*dpyinfo).server_time_offset = offset_;
		     overflow
		   }
		    || {
			 let (offset_, overflow)
			   = (*dpyinfo).server_time_offset.overflowing_mul (1000);
			 (*dpyinfo).server_time_offset = offset_;
			 overflow
		       }
		    || {
			 let (offset_, overflow)
			   = (0 as int_fast64_t)
			       .overflowing_sub ((*dpyinfo).server_time_offset);
			 (*dpyinfo).server_time_offset = offset_;
			 overflow
		       }
		  { (*dpyinfo).server_time_offset = 0; }
	      }
	  }
      }
  }

  #[cfg (not (feature = "use_gtk"))]
  {
    /* Don't waste bandwidth if the time hasn't actually changed.  */
    if !focus_frame.is_null () && old_time != (*dpyinfo).last_user_time
	&& set_property
      {
	time = (*dpyinfo).last_user_time;

	while !FRAME_PARENT_FRAME (focus_frame).is_null ()
	  { focus_frame = FRAME_PARENT_FRAME (focus_frame); }

	if (*(FRAME_X_OUTPUT! (focus_frame) as *mut x_output)).user_time_window
	    != None as Window
	  {
	    XChangeProperty ((*dpyinfo).display,
			     (*(FRAME_X_OUTPUT! (focus_frame) as *mut x_output)).user_time_window,
			     (*dpyinfo).Xatom_net_wm_user_time,
			     XA_CARDINAL, 32, PropModeReplace,
			     &raw mut time as *mut u8, 1);
	  }
      }
  }
}

/* C 8178-8196: `#ifdef USE_GTK` - `x_set_gtk_user_time`.  `gtk_widget_get_window` and
   `gdk_x11_window_set_user_time` are GTK's own (no mirror: the ledger's gap list), so the arm
   names them as C does.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe fn x_set_gtk_user_time (f: *mut frame, time: Time)
{
  let widget: *mut GtkWidget;
  let window: *mut GdkWindow;

  widget = FRAME_GTK_OUTER_WIDGET! (f);
  window = gtk_widget_get_window (widget);

  /* This widget isn't realized yet.  */
  if window.is_null ()
    { return; }

  gdk_x11_window_set_user_time (window, time);
}

/* C 8198-8218: `#if !defined USE_GTK || defined HAVE_XFIXES` - `x_create_special_window`.
   C's `attrs` is a one-member compound literal (`{ .override_redirect = True }`), which the
   port writes as the zeroed record plus the store (the `x_premultiply`… the `xmenu.rs` 3854
   shape).  */
#[cfg (all (feature = "have_x_windows",
	    any (not (feature = "use_gtk"), feature = "have_xfixes")))]
unsafe fn x_create_special_window (dpyinfo: *mut x_display_info,
				   parent_window: Window) -> Window
{
  use crate::x11_constants::{CopyFromParent, CWOverrideRedirect, InputOnly};

  let mut attrs: XSetWindowAttributes = std::mem::zeroed ();

  attrs.override_redirect = true as Bool;

  return XCreateWindow ((*dpyinfo).display, parent_window,
			-1, -1, 1, 1, 0, CopyFromParent, InputOnly,
			CopyFromParent, CWOverrideRedirect, &raw mut attrs);
}

/* C 8220-8300: `#ifndef USE_GTK` - `x_update_frame_user_time_window`.  `x_wm_supports` is
   xterm.h's (its mirror declares it), `Vx_no_window_manager` is lisp.h's; C's
   `&(Time) {0}` compound literal is a bound local (the `xmenu.rs` 3854 shape).  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_gtk")))]
unsafe fn x_update_frame_user_time_window (f: *mut frame)
{
  use crate::x11_constants::{None, PropModeReplace, XA_CARDINAL, XA_WINDOW};

  let output: *mut x_output;
  let dpyinfo: *mut x_display_info;

  output = FRAME_X_OUTPUT! (f) as *mut x_output;
  dpyinfo = FRAME_DISPLAY_INFO! (f);

  if !NILP (Vx_no_window_manager)
      || !x_wm_supports (f, (*dpyinfo).Xatom_net_wm_user_time)
    {
      if (*output).user_time_window != None as Window
	  && (*output).user_time_window != FRAME_OUTER_WINDOW! (f)
	{
	  XDestroyWindow ((*dpyinfo).display, (*output).user_time_window);
	  XDeleteProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_user_time_window);
	}
      else
	{ XDeleteProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_user_time); }

      (*output).user_time_window = None as Window;
      return;
    }

  if !x_wm_supports (f, (*dpyinfo).Xatom_net_wm_user_time_window)
    {
      if (*output).user_time_window == None as Window
	{ (*output).user_time_window = FRAME_OUTER_WINDOW! (f); }
      else if (*output).user_time_window != FRAME_OUTER_WINDOW! (f)
	{
	  XDestroyWindow ((*dpyinfo).display,
			  (*output).user_time_window);
	  XDeleteProperty ((*dpyinfo).display,
			   FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_user_time_window);
	  (*output).user_time_window = FRAME_OUTER_WINDOW! (f);
	}
    }
  else
    {
      if (*output).user_time_window == FRAME_OUTER_WINDOW! (f)
	  || (*output).user_time_window == None as Window
	{
	  /* Create a "user time" window that is used to report user
	     activity on a given frame.  This is used in preference to
	     _NET_WM_USER_TIME, as using a separate window allows the
	     window manager to express interest in other properties
	     while only reading the user time when necessary, thereby
	     improving battery life by not involving the window
	     manager in each key press.  */

	  (*output).user_time_window
	    = x_create_special_window (dpyinfo, FRAME_X_WINDOW! (f));

	  if FRAME_NO_FOCUS_ON_MAP (f) != 0
	    /* If the user time is zero, which is the case with
	       `no-focus-on-map', then preserve that value by copying
	       it to the new user time window.  */
	    {
	      /* C's `&(Time) {0}` is a compound literal; the port binds the value and passes
		 its address (the `xmenu.rs` 3854 shape).  */
	      let mut user_time_zero: Time = 0;
	      XChangeProperty ((*dpyinfo).display, (*output).user_time_window,
			       (*dpyinfo).Xatom_net_wm_user_time, XA_CARDINAL, 32,
			       PropModeReplace, &raw mut user_time_zero as *mut u8, 1);
	    }

	  XDeleteProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_user_time);
	  XChangeProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),
			   (*dpyinfo).Xatom_net_wm_user_time_window,
			   XA_WINDOW, 32, PropModeReplace,
			   &raw mut (*output).user_time_window as *mut u8, 1);
	}
    }
}

/* C 8302-8307: `x_set_last_user_time_from_lisp`.  C's definition carries no `static`, so the
   port keeps it exported (PORTING.md 0.2).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_set_last_user_time_from_lisp (dpyinfo: *mut x_display_info,
					      time: Time)
{
  x_display_set_last_user_time (dpyinfo, time, true, true);
}

/* The two records and the bit accessor this slice's glyph-string text needs, which
   `include/dispextern.rs` keeps behind `have_window_system` (`glyph_string`, `bits` and the
   `glyph_string_bits!` family) and `dispextern_face_records` (`draw_fringe_bitmap_params`):
   this unit is compiled with the file gate only, so it carries its own live copies - the
   `term.rs` 322-345/`dispnew.rs` 3600 precedent and status 3.220's ruling.  The
   `draw_fringe_bitmap_params` copy repeats the mirror's measured offsets (its `abi_shim.rs`
   probe: `sizeof` 64, `flags`@52, `face`@56, and the two `bool_bf` bits in that byte); the
   `glyph_string` copy is C's declaration order with **no** asserts, because the mirror's own
   note records the record's layout as unmeasured on this target - the ledger repeats that, and
   the unit's copy is a temporary until the record group is reachable (its Owed list).  */

/// C dispextern.h 2215-2240: `struct draw_fringe_bitmap_params`.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct draw_fringe_bitmap_params
{
  pub which: c_int,                 // offset 0
  pub bits: *mut c_ushort,          // offset 8
  pub wd: c_int,                    // offset 16
  pub h: c_int,                     // offset 20
  pub dh: c_int,                    // offset 24
  pub x: c_int,                     // offset 28
  pub y: c_int,                     // offset 32
  pub bx: c_int,                    // offset 36
  pub nx: c_int,                    // offset 40
  pub by: c_int,                    // offset 44
  pub ny: c_int,                    // offset 48
  /// C 2230-2231's two `bool_bf : 1` members, stored as one byte (PORTING.md 1.3/P-80).
  pub flags: u8,                    // offset 52
  pub face: *mut face,              // offset 56
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<draw_fringe_bitmap_params> () == 64);
  core::assert! (std::mem::align_of::<draw_fringe_bitmap_params> () == 8);
  core::assert! (std::mem::offset_of! (draw_fringe_bitmap_params, flags) == 52);
  core::assert! (std::mem::offset_of! (draw_fringe_bitmap_params, face) == 56);
};
/// C: `bool_bf cursor_p : 1;` (dispextern.h 2230) - bit 0 of the byte at 52.
#[cfg (feature = "have_x_windows")]
pub const DRAW_FRINGE_BITMAP_CURSOR_P: u8 = 1 << 0;
/// C: `bool_bf overlay_p : 1;` (dispextern.h 2231) - bit 1 of the byte at 52.
#[cfg (feature = "have_x_windows")]
pub const DRAW_FRINGE_BITMAP_OVERLAY_P: u8 = 1 << 1;
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn get_cursor_p (p: *const draw_fringe_bitmap_params) -> bool
{ ((*p).flags & DRAW_FRINGE_BITMAP_CURSOR_P) != 0 }
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn set_cursor_p (p: *mut draw_fringe_bitmap_params, value: bool)
{
  if value { (*p).flags |= DRAW_FRINGE_BITMAP_CURSOR_P }
  else { (*p).flags &= !DRAW_FRINGE_BITMAP_CURSOR_P }
}
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn get_overlay_p (p: *const draw_fringe_bitmap_params) -> bool
{ ((*p).flags & DRAW_FRINGE_BITMAP_OVERLAY_P) != 0 }
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn set_overlay_p (p: *mut draw_fringe_bitmap_params, value: bool)
{
  if value { (*p).flags |= DRAW_FRINGE_BITMAP_OVERLAY_P }
  else { (*p).flags &= !DRAW_FRINGE_BITMAP_OVERLAY_P }
}

/// C dispextern.h 1363-1506: `struct glyph_string`, in C's declaration order (the fields this
/// unit's text reads are `x`/`y`/`ybase`/`width`/`background_width`/`height`/the two
/// overhangs/`f`/`w`/`row`/`char2b`/`nchars`/`hl`/`face`/`font`/`cmp`/`cmp_id`/`cmp_from`/
/// `cmp_to`/`bits`/`gc`/`first_glyph`/`img`/`slice`/`clip`/`num_clips`; the rest keep C's
/// place in the order).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct glyph_string
{
  pub x: c_int,                        // C 1365
  pub y: c_int,                        // C 1368
  pub ybase: c_int,
  pub width: c_int,                    // C 1371
  pub background_width: c_int,         // C 1374
  pub height: c_int,                   // C 1379
  pub left_overhang: c_int,            // C 1384
  pub right_overhang: c_int,           // C 1389
  pub f: *mut frame,                   // C 1392
  pub w: *mut window,                  // C 1395
  pub row: *mut glyph_row,             // C 1399
  pub area: glyph_row_area,            // C 1402
  pub char2b: *mut c_uint,             // C 1407
  pub nchars: c_int,                   // C 1408
  pub hl: draw_glyphs_face,            // C 1411
  pub face: *mut c_void,               /// C: `struct face *face;` (dispextern.h 1414)
  pub font: *mut font,                 // C 1417
  pub cmp: *mut c_void,                /// C: `struct composition *cmp;` (composite.h 148)
  pub cmp_id: ptrdiff_t,               // C 1424
  pub cmp_from: c_int,                 // C 1427
  pub cmp_to: c_int,
  pub bits: u32,                       // C 1431-1461: the six `bool_bf`/`unsigned` members
  pub gc: *mut c_void,                 /// C: `GC gc;` (dispextern.h 1465) - untyped (P-72)
  pub first_glyph: *mut glyph,         // C 1480
  pub img: *mut image,                 // C 1483
  pub xwidget: *mut c_void,            /// C: `struct xwidget *xwidget;` (dispextern.h 1486)
  pub slice: glyph_slice,              // C 1489
  pub clip_head: *mut glyph_string,    // C 1494
  pub clip_tail: *mut glyph_string,
  pub clip: [c_int; 2],                /// C: `NativeRectangle clip[2];` (dispextern.h 1497)
  pub num_clips: c_int,                // C 1500
  pub underline_position: c_int,       // C 1502
  pub underline_thickness: c_int,      // C 1504
  pub next: *mut glyph_string,         // C 1506
  pub prev: *mut glyph_string,
}

/* `glyph_string::bits`, C 1431-1461's six members at the positions `include/dispextern.rs`
   1760-1780 documents (four `bool_bf` in bits 0-3, `unsigned for_overlaps : 3` in bits 4-6 and
   one more `bool_bf` in bit 7); the member name stays the macro's first argument (P-80).  */
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P: u32 = 1 << 0;   // C 1431
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_BACKGROUND_FILLED_P: u32 = 1 << 1;        // C 1434
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_FONT_NOT_FOUND_P: u32 = 1 << 2;           // C 1439
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_STIPPLED_P: u32 = 1 << 3;                 // C 1443
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_FOR_OVERLAPS_SHIFT: u32 = 4;              // C 1456
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_FOR_OVERLAPS_WIDTH: u32 = 3;              // C 1456
#[cfg (feature = "have_x_windows")]
pub const GLYPH_STRING_PADDING_P: u32 = 1 << 7;                  // C 1461
#[cfg (feature = "have_x_windows")]
macro_rules! glyph_string_bits {
  (extends_to_end_of_line_p, $s:expr) =>
    { (($s).bits & GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P) != 0 };
  (extends_to_end_of_line_p, $s:expr, $value:expr) => {{
    if $value { ($s).bits |= GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P }
    else { ($s).bits &= ! GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P } }};
  (background_filled_p, $s:expr) =>
    { (($s).bits & GLYPH_STRING_BACKGROUND_FILLED_P) != 0 };
  (background_filled_p, $s:expr, $value:expr) => {{
    if $value { ($s).bits |= GLYPH_STRING_BACKGROUND_FILLED_P }
    else { ($s).bits &= ! GLYPH_STRING_BACKGROUND_FILLED_P } }};
  (font_not_found_p, $s:expr) => { (($s).bits & GLYPH_STRING_FONT_NOT_FOUND_P) != 0 };
  (font_not_found_p, $s:expr, $value:expr) => {{
    if $value { ($s).bits |= GLYPH_STRING_FONT_NOT_FOUND_P }
    else { ($s).bits &= ! GLYPH_STRING_FONT_NOT_FOUND_P } }};
  (stippled_p, $s:expr) => { (($s).bits & GLYPH_STRING_STIPPLED_P) != 0 };
  (stippled_p, $s:expr, $value:expr) => {{
    if $value { ($s).bits |= GLYPH_STRING_STIPPLED_P }
    else { ($s).bits &= ! GLYPH_STRING_STIPPLED_P } }};
  (for_overlaps, $s:expr) =>
    { ((($s).bits >> GLYPH_STRING_FOR_OVERLAPS_SHIFT)
       & ((1 << GLYPH_STRING_FOR_OVERLAPS_WIDTH) - 1)) as c_int };
  (for_overlaps, $s:expr, $value:expr) => {{
    let mask: u32 = (1 << GLYPH_STRING_FOR_OVERLAPS_WIDTH) - 1;
    ($s).bits = (($s).bits & ! (mask << GLYPH_STRING_FOR_OVERLAPS_SHIFT))
      | (((($value) as u32) & mask) << GLYPH_STRING_FOR_OVERLAPS_SHIFT); }};
  (padding_p, $s:expr) => { (($s).bits & GLYPH_STRING_PADDING_P) != 0 };
  (padding_p, $s:expr, $value:expr) => {{
    if $value { ($s).bits |= GLYPH_STRING_PADDING_P }
    else { ($s).bits &= ! GLYPH_STRING_PADDING_P } }};
}

/* ---- C 8309-8560: the glyph-string GC helpers, the two clipping setters, the overhang
   computation and C 8530's two small clear helpers.  */

/* The `XGCValues` record this group reads and writes.  `<X11/Xlib.h>` 177-203's record is not
   in `include/xlib.rs` (that mirror keeps `XGCValues` in a comment only, because the one
   prototype it carries takes an untyped pointer), so the unit declares it at the use site - the
   `src/xftfont.rs` 132-171 shape, whose clang readings against this machine's `xorgproto`/
   `libX11` headers (`/opt/homebrew/include`) are repeated here: `sizeof` **128**, align **8**,
   `foreground`@16, `background`@24, `line_width`@32, `graphics_exposures`@72.  The ledger's
   Owed list asks `include/xlib.rs` to take it.  */
/// C: Xlib.h 177-203: `typedef struct { int function; unsigned long plane_mask; unsigned long
/// foreground; unsigned long background; int line_width; … } XGCValues;`
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGCValues
{
  pub function: c_int,                          // Xlib.h 178
  pub plane_mask: c_ulong,                      // Xlib.h 179
  pub foreground: c_ulong,                      // Xlib.h 180
  pub background: c_ulong,                      // Xlib.h 181
  pub line_width: c_int,                        // Xlib.h 182
  pub line_style: c_int,                        // Xlib.h 183
  pub cap_style: c_int,                         // Xlib.h 184-185
  pub join_style: c_int,                        // Xlib.h 186
  pub fill_style: c_int,                        // Xlib.h 187-188
  pub fill_rule: c_int,                         // Xlib.h 189
  pub arc_mode: c_int,                          // Xlib.h 190
  pub tile: Pixmap,                             // Xlib.h 191
  pub stipple: Pixmap,                          // Xlib.h 192
  pub ts_x_origin: c_int,                       // Xlib.h 193
  pub ts_y_origin: c_int,                       // Xlib.h 194
  pub font: Font,                               // Xlib.h 195
  pub subwindow_mode: c_int,                    // Xlib.h 196
  pub graphics_exposures: Bool,                 // Xlib.h 197
  pub clip_x_origin: c_int,                     // Xlib.h 198
  pub clip_y_origin: c_int,                     // Xlib.h 199
  pub clip_mask: Pixmap,                        // Xlib.h 200
  pub dash_offset: c_int,                       // Xlib.h 201
  pub dashes: c_char,                           // Xlib.h 202
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<XGCValues> () == 128);
  core::assert! (std::mem::align_of::<XGCValues> () == 8);
  core::assert! (std::mem::offset_of! (XGCValues, foreground) == 16);
  core::assert! (std::mem::offset_of! (XGCValues, background) == 24);
  core::assert! (std::mem::offset_of! (XGCValues, line_width) == 32);
};

/* The two dispextern.h prototypes this group calls and no mirror in this graph carries, each
   declared at its use site with C's own line (PORTING.md P-55; the `fringe.rs` 846-850 and
   `androidfont.rs` 111-113 shape).  `NativeRectangle` is `XRectangle` on the X11 arm.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: dispextern.h 3661-3662: `int get_glyph_string_clip_rects (struct glyph_string *,
  ///                                        NativeRectangle *, int);`
  pub fn get_glyph_string_clip_rects (s: *mut glyph_string, rects: *mut XRectangle, n: c_int)
    -> c_int;
  /// C: dispextern.h 3810: `void prepare_face_for_display (struct frame *, struct face *);`
  pub fn prepare_face_for_display (f: *mut frame, face: *mut face);
}

/* C 8310-8363: `x_set_cursor_gc`.  `s->face` is the mirror's `*mut c_void` member, so the port
   binds it with C's cast (the `w32term.rs` 1869 shape); `s->cmp` is C's `struct composition *`
   and the null test is the `null` family of token-aliases.tsv 94; C's `f->output_data.x->…`
   reads go through the same cast; and the two mask constants are `c_int`s, so `mask`'s
   `unsigned long` is C's one conversion written out (`as c_ulong`).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_cursor_gc (s: *mut glyph_string)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let face: *mut face = (*s).face as *mut face;

  if (*s).font == FRAME_FONT! ((*s).f)
      && (*face).background == FRAME_BACKGROUND_PIXEL ((*s).f)
      && (*face).foreground == FRAME_FOREGROUND_PIXEL ((*s).f)
      && (*s).cmp.is_null ()
    { (*s).gc = (*( (*(*s).f).output_data.x as *mut x_output )).cursor_gc as *mut c_void; }
  else
    {
      /* Cursor on non-default face: must merge.  */
      let mut xgcv: XGCValues = std::mem::zeroed ();
      let mut mask: c_ulong;
      let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

      xgcv.background = (*( (*(*s).f).output_data.x as *mut x_output )).cursor_pixel;
      xgcv.foreground = (*face).background;

      /* If the glyph would be invisible, try a different foreground.  */
      if xgcv.foreground == xgcv.background
	{ xgcv.foreground = (*face).foreground; }
      if xgcv.foreground == xgcv.background
	{ xgcv.foreground
	    = (*( (*(*s).f).output_data.x as *mut x_output )).cursor_foreground_pixel; }
      if xgcv.foreground == xgcv.background
	{ xgcv.foreground = (*face).foreground; }

      /* Make sure the cursor is distinct from text in this face.  */
      if xgcv.background == (*face).background
	  && xgcv.foreground == (*face).foreground
	{
	  xgcv.background = (*face).foreground;
	  xgcv.foreground = (*face).background;
	}

      IF_DEBUG! (x_check_font ((*s).f, (*s).font));
      xgcv.graphics_exposures = false as Bool;
      xgcv.line_width = 1;
      mask = (GCForeground | GCBackground
	      | GCGraphicsExposures
	      | GCLineWidth) as c_ulong;

      if !(*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc.is_null ()
	{
	  XChangeGC (display, (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc,
		     mask, &raw mut xgcv as *mut c_void);
	}
      else
	{
	  (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc
	    = XCreateGC (display, FRAME_X_DRAWABLE! ((*s).f), mask, &raw mut xgcv as *mut c_void);
	}

      (*s).gc = (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc;
    }
}

/* C 8365-8401: `x_set_mouse_face_gc`.  C's `s->face->font` is the mirror's `*mut c_void`
   member cast to `struct font *` (the w32 sibling's line).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_mouse_face_gc (s: *mut glyph_string)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let face: *mut face = (*s).face as *mut face;

  if (*s).font == (*face).font as *mut font
    { (*s).gc = (*face).gc; }
  else
    {
      /* Otherwise construct scratch_cursor_gc with values from FACE
	 except for FONT.  */
      let mut xgcv: XGCValues = std::mem::zeroed ();
      let mut mask: c_ulong;
      let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

      xgcv.background = (*face).background;
      xgcv.foreground = (*face).foreground;
      xgcv.graphics_exposures = false as Bool;
      xgcv.line_width = 1;

      mask = (GCForeground | GCBackground
	      | GCGraphicsExposures
	      | GCLineWidth) as c_ulong;

      if !(*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc.is_null ()
	{
	  XChangeGC (display, (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc,
		     mask, &raw mut xgcv as *mut c_void);
	}
      else
	{
	  (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc
	    = XCreateGC (display, FRAME_X_DRAWABLE! ((*s).f), mask, &raw mut xgcv as *mut c_void);
	}

      (*s).gc = (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc;

    }
  eassert! ((*s).gc != 0 as *mut c_void);
}

/* C 8403-8412: `x_set_mode_line_face_gc`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_mode_line_face_gc (s: *mut glyph_string)
{
  (*s).gc = (*( (*s).face as *mut face )).gc;
}

/* C 8414-8455: `x_set_glyph_string_gc`.  The five `s->hl` comparisons are C's
   `enum draw_glyphs_face` against `dispextern.rs`'s constants, the four `s->stippled_p`
   writes are the record's bitfield run (`glyph_string_bits!`, named after the member), and
   C's `s->face->stipple != 0` keeps C's own `0` (`stipple` is a `ptrdiff_t` member).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_glyph_string_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;

  prepare_face_for_display ((*s).f, face);

  if (*s).hl == DRAW_NORMAL_TEXT
    {
      (*s).gc = (*face).gc;
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else if (*s).hl == DRAW_INVERSE_VIDEO
    {
      x_set_mode_line_face_gc (s);
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else if (*s).hl == DRAW_CURSOR
    {
      x_set_cursor_gc (s);
      glyph_string_bits! (stippled_p, (*s), false);
    }
  else if (*s).hl == DRAW_MOUSE_FACE
    {
      x_set_mouse_face_gc (s);
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else if (*s).hl == DRAW_IMAGE_RAISED
	   || (*s).hl == DRAW_IMAGE_SUNKEN
    {
      (*s).gc = (*face).gc;
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else
    { emacs_abort (); }

  /* GC must have been set.  */
  eassert! ((*s).gc != 0 as *mut c_void);
}

/* C 8457-8470: `x_set_glyph_string_clipping`.  C's `XRectangle *r = s->clip;` reads the
   mirror's degraded `NativeRectangle clip[2]` (`[c_int; 2]`) through a cast, the
   `w32term.rs` 2026-2028 shape (PORTING.md P-72).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_glyph_string_clipping (s: *mut glyph_string)
{
  let r: *mut XRectangle = (&raw mut (*s).clip) as *mut XRectangle;
  let n: c_int = get_glyph_string_clip_rects (s, r, 2);

  if n > 0
    { x_set_clip_rectangles ((*s).f, (*s).gc as GC, r, n); }
  (*s).num_clips = n;
}

/* C 8472-8489: `x_set_glyph_string_clipping_exactly`.  C's `dst->clip[0] = r;` stores through
   the same degraded-array cast, and C's `[0]` subscript is the pointer's `.add (0)` with C's
   own index.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_set_glyph_string_clipping_exactly (src: *mut glyph_string, dst: *mut glyph_string)
{
  let mut r: XRectangle = std::mem::zeroed ();

  r.x = (*src).x as c_short;
  r.width = (*src).width as c_ushort;
  r.y = (*src).y as c_short;
  r.height = (*src).height as c_ushort;
  *((&raw mut (*dst).clip) as *mut XRectangle).add (0) = r;
  (*dst).num_clips = 1;
  x_set_clip_rectangles ((*dst).f, (*dst).gc as GC, &raw mut r, 1);
}

/* C 8491-8524: `x_compute_glyph_string_overhangs`.  `s->first_glyph->type` is the glyph
   record's bitfield run read through `glyph_bits!`, the two `font->driver->…` calls are the
   unwrapped function-pointer fields of the mirror's `font_driver` (`include/font.rs` 134 keeps
   `driver` untyped, P-72 rule 3), `composition_gstring_width`'s two `int` members widen to its
   `ptrdiff_t` parameters, C's `s->cmp->{rbearing,pixel_width,lbearing}` are `short`s widened to
   the `int` overhang members, and C's four `?:` are the port's `if`/`else`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_compute_glyph_string_overhangs (s: *mut glyph_string)
{
  if (*s).cmp.is_null ()
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)
    {
      let mut metrics: font_metrics = std::mem::zeroed ();

      if glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH
	{
	  let font: *mut font = (*s).font;
	  ((*((*font).driver as *const font_driver)).text_extents.unwrap ())
	    (font, (*s).char2b, (*s).nchars, &raw mut metrics);
	}
      else
	{
	  let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);

	  composition_gstring_width (gstring, (*s).cmp_from as ptrdiff_t,
				     (*s).cmp_to as ptrdiff_t,
				     &raw mut metrics as *mut c_void);
	}
      (*s).right_overhang = if metrics.rbearing > metrics.width
	{ (metrics.rbearing - metrics.width) as c_int } else { 0 };
      (*s).left_overhang = if metrics.lbearing < 0
	{ (- metrics.lbearing) as c_int } else { 0 };
    }
  else if !(*s).cmp.is_null ()
    {
      (*s).right_overhang = ((*( (*s).cmp as *mut composition )).rbearing
			     - (*( (*s).cmp as *mut composition )).pixel_width) as c_int;
      (*s).left_overhang = (- (*( (*s).cmp as *mut composition )).lbearing) as c_int;
    }
}

/* C 8526-8533: `x_clear_glyph_string_rect`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_clear_glyph_string_rect (s: *mut glyph_string, x: c_int, y: c_int,
				     w: c_int, h: c_int)
{
  x_clear_rectangle ((*s).f, (*s).gc as GC, x, y, w, h, (*s).hl != DRAW_CURSOR);
}

/* C 8535-8559: `#ifndef USE_CAIRO` - `x_clear_point`.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe fn x_clear_point (f: *mut frame, gc: GC, x: c_int, y: c_int,
			 respect_alpha_background: bool)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let mut xgcv: XGCValues = std::mem::zeroed ();
  let dpy: *mut Display;

  dpy = FRAME_X_DISPLAY! (f);

  if (*f).alpha_background != 1.0
      && respect_alpha_background
    {
      x_clear_rectangle (f, gc, x, y, 1, 1, true);
      return;
    }

  XGetGCValues (dpy, gc, (GCBackground | GCForeground) as c_ulong,
		&raw mut xgcv as *mut c_void);
  XSetForeground (dpy, gc, xgcv.background);
  XDrawPoint (dpy, FRAME_X_DRAWABLE! (f), gc, x, y);
  XSetForeground (dpy, gc, xgcv.foreground);
}

/* ---- C 8561-8896: the four glyph-string drawers.  */

/* C dispextern.h 1787-1945: `struct face`, the X11 arm's layout.  This unit reads
   `gc`/`stipple`/`foreground`/`background`/`box_color`/`font`/the two box widths and the bit
   run, so those are named and the rest is padding at its measured offset; every number is the
   `x11` column of `docs/baseline/face-layout.tsv` (`sizeof` 296, align 8, `gc`@168,
   `stipple`@176, `foreground`@184, `background`@192, `box_color`@224, `font`@232, the run at
   256, `hash`@264, the four pointers to 296).  The *mirror*'s own copy
   (`include/dispextern.rs` 2187) is behind `dispextern_face_records`, which no configuration
   sets, and status 3.220's ruling is that the units keep their live typed copies - the
   `term.rs` 322-345 and `dispnew.rs` 3600 shapes.  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct face
{
  pub _pad_lface: [u8; 160],        // 1789-1794: `Lisp_Object lface[LFACE_VECTOR_SIZE]`
  pub id: c_int,                    // 1795, offset 160
  pub _pad_id: [u8; 4],
  pub gc: *mut c_void,              /// C: `GC gc;` (1799) - X11's `struct _XGC *`, untyped for
                                    /// the same reason `include/dispextern.rs` leaves it so
  pub stipple: ptrdiff_t,           // 1810, offset 176
  pub foreground: c_ulong,          // 1816, offset 184
  pub background: c_ulong,          // 1819, offset 192
  pub underline_color: c_ulong,     // 1823
  pub overline_color: c_ulong,      // 1826
  pub strike_through_color: c_ulong, // 1829
  pub box_color: c_ulong,           // 1832, offset 224
  pub font: *mut c_void,            /// C: `struct font *font;` (1845) - `include/font.rs` is
                                    /// the type's owner and this unit reaches it, but C's
                                    /// member is spelled as the mirror's does (P-72 rule 3)
  pub fontset: c_int,               // 1838
  pub box_vertical_line_width: c_int,     // 1848
  pub box_horizontal_line_width: c_int,   // 1849
  pub underline_pixels_above_descent_line: c_int,   // C 1850
  pub bits: u32,                    // 1851-1861: the `bool_bf`/`ENUM_BF` run at offset 256
  pub _pad_bits: [u8; 4],           // C's padding before `uintptr_t hash`
  pub hash: uintptr_t,              // 1930, offset 264
  pub next: *mut face,              // 1933
  pub prev: *mut face,              // 1934
  pub ascii_face: *mut face,        // 1935
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (std::mem::size_of::<face> () == 296);
  core::assert! (std::mem::align_of::<face> () == 8);
  core::assert! (std::mem::offset_of! (face, gc) == 168);
  core::assert! (std::mem::offset_of! (face, stipple) == 176);
  core::assert! (std::mem::offset_of! (face, foreground) == 184);
  core::assert! (std::mem::offset_of! (face, background) == 192);
  core::assert! (std::mem::offset_of! (face, box_color) == 224);
  core::assert! (std::mem::offset_of! (face, font) == 232);
  core::assert! (std::mem::offset_of! (face, bits) == 256);
  core::assert! (std::mem::offset_of! (face, hash) == 264);
};

/* The three bitfield families these four functions read and no mirror names: `struct face`'s
   `bool_bf` run (`include/dispextern.rs` 2224 stores it as one `bits: u32`), `glyph`'s two
   union sub-records (dispextern.rs 417-465 carry the units and their masks) and the two member
   names of each.  Each accessor keeps the member name as its first argument, so the ported text
   keeps C's token (P-80, the `w32term.rs` 1696-1745 and `term.rs` 840-851 copies of the same
   positions; the ledger records the sharing).  */
/// C: dispextern.h 1893-1913: `ENUM_BF (face_box_type) box : 2;` … `bool_bf
/// use_box_color_for_shadows_p : 1;` … `bool_bf overstrike : 1;`
#[cfg (feature = "have_x_windows")]
pub const FACE_BOX_SHIFT: u32 = 0;
#[cfg (feature = "have_x_windows")]
pub const FACE_BOX_MASK: u32 = 0x3;
#[cfg (feature = "have_x_windows")]
pub const FACE_USE_BOX_COLOR_FOR_SHADOWS_P: u32 = 1 << 5;
#[cfg (feature = "have_x_windows")]
pub const FACE_OVERSTRIKE: u32 = 1 << 21;
/* The six members slice 10's `x_draw_glyph_string`/`x_draw_image_relief` text reads and this
   unit's copy did not name yet, at the positions dispextern.h 1893-1935 gives them (the
   `font.rs` 232-253 row of the same record; `include/dispextern.rs`'s own copy is behind
   `dispextern_face_records`, so the mirror row is owed - P-26/P-80).  */
#[cfg (feature = "have_x_windows")]
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
#[cfg (feature = "have_x_windows")]
pub const FACE_UNDERLINE_MASK: u32 = 0x7;
#[cfg (feature = "have_x_windows")]
pub const FACE_OVERLINE_P: u32 = 1 << 6;
#[cfg (feature = "have_x_windows")]
pub const FACE_STRIKE_THROUGH_P: u32 = 1 << 7;
#[cfg (feature = "have_x_windows")]
pub const FACE_UNDERLINE_DEFAULTED_P: u32 = 1 << 10;
#[cfg (feature = "have_x_windows")]
pub const FACE_OVERLINE_COLOR_DEFAULTED_P: u32 = 1 << 11;
#[cfg (feature = "have_x_windows")]
pub const FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P: u32 = 1 << 12;
#[cfg (feature = "have_x_windows")]
pub const FACE_UNDERLINE_AT_DESCENT_LINE_P: u32 = 1 << 14;
#[cfg (feature = "have_x_windows")]
macro_rules! face_bits {
  (box, $f:expr) => { (((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT) as c_int };
  (underline, $f:expr) =>
    { (((*$f).bits & FACE_UNDERLINE_MASK) >> FACE_UNDERLINE_SHIFT) as c_int };
  (use_box_color_for_shadows_p, $f:expr) =>
    { ((*$f).bits & FACE_USE_BOX_COLOR_FOR_SHADOWS_P) != 0 };
  (overline_p, $f:expr) => { ((*$f).bits & FACE_OVERLINE_P) != 0 };
  (strike_through_p, $f:expr) => { ((*$f).bits & FACE_STRIKE_THROUGH_P) != 0 };
  (underline_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_DEFAULTED_P) != 0 };
  (overline_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_OVERLINE_COLOR_DEFAULTED_P) != 0 };
  (strike_through_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P) != 0 };
  (underline_at_descent_line_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_AT_DESCENT_LINE_P) != 0 };
  (overstrike, $f:expr) => { ((*$f).bits & FACE_OVERSTRIKE) != 0 };
}

/// C: dispextern.h 597-598: `bool_bf automatic : 1; unsigned id : 31;`
#[cfg (feature = "have_x_windows")]
macro_rules! glyph_u_cmp_bits {
  (automatic, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_AUTOMATIC) != 0) };
  (id, $g:expr) => { (((*$g).u.cmp.bits >> GLYPH_U_CMP_ID_SHIFT) as ptrdiff_t) };
}

/// C: dispextern.h 626-632: `unsigned method : 2; bool_bf for_no_font : 1; unsigned len : 4;
/// unsigned ch : 25;`
#[cfg (feature = "have_x_windows")]
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
  (for_no_font, $g:expr) => { ((*$g).u.glyphless.bits & GLYPH_U_GLYPHLESS_FOR_NO_FONT) != 0 };
  (len, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_LEN_SHIFT) & 0xf) as c_int };
  (ch, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_CH_SHIFT) & 0x1ffffff) as c_int };
}

/* C 8561-8606: `x_draw_glyph_string_background`.  C's `max (…)` is lisp.h's `max (EMACS_INT,
   EMACS_INT)` (`include/lisp.rs` 3136), so the two `int` operands widen and the result narrows
   the way `this file's` other `max`/`min` sites do; `FONT_HEIGHT`/`FONT_TOO_HIGH` are
   `include/font.rs` 258's and `include/dispextern.rs` 2669's macros.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_glyph_string_background (s: *mut glyph_string, force_p: bool)
{

  /* Nothing to do if background has already been drawn or if it
     shouldn't be drawn in the first place.  */
  if !glyph_string_bits! (background_filled_p, (*s))
    {
      let box_line_width: c_int = max ((*((*s).face as *mut face)).box_horizontal_line_width as isize,
				       0) as c_int;

      if glyph_string_bits! (stippled_p, (*s))
	{
	  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

	  /* Fill background with a stipple pattern.  */
	  XSetFillStyle (display, (*s).gc as GC, FillOpaqueStippled);
	  x_fill_rectangle ((*s).f, (*s).gc as GC, (*s).x,
			    (*s).y + box_line_width,
			    (*s).background_width,
			    (*s).height - 2 * box_line_width,
			    (*s).hl != DRAW_CURSOR);
	  XSetFillStyle (display, (*s).gc as GC, FillSolid);
	  glyph_string_bits! (background_filled_p, (*s), true);
	}
      else if FONT_HEIGHT! ((*s).font) < (*s).height - 2 * box_line_width
	       /* When xdisp.c ignores FONT_HEIGHT, we cannot trust
		  font dimensions, since the actual glyphs might be
		  much smaller.  So in that case we always clear the
		  rectangle with background color.  */
	       || FONT_TOO_HIGH! ((*s).font)
	       || glyph_string_bits! (font_not_found_p, (*s))
	       || glyph_string_bits! (extends_to_end_of_line_p, (*s))
	       || force_p
	{
	  x_clear_glyph_string_rect (s, (*s).x, (*s).y + box_line_width,
				     (*s).background_width,
				     (*s).height - 2 * box_line_width);
	  glyph_string_bits! (background_filled_p, (*s), true);
	}
    }
}

/* C 8608-8701: `x_draw_glyph_string_foreground`.  The `#ifdef USE_CAIRO` run is C's `if` whose
   condition the arm supplies, so the port carries C's one statement as the arm's `if` plus the
   `#else`-side body again under `not use_cairo` (both arms, P-51).  `font->driver->draw` is the
   unwrapped field, and `VCENTER_BASELINE_OFFSET` is `include/dispextern.rs`'s macro.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_glyph_string_foreground (s: *mut glyph_string)
{
  let mut i: c_int;
  let mut x: c_int;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if face_bits! (box, (*((*s).face as *mut face))) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max ((*((*s).face as *mut face)).box_vertical_line_width as isize,
			0) as c_int; }
  else
    { x = (*s).x; }

  /* Draw characters of S as rectangles if S's font could not be
     loaded.  */
  if glyph_string_bits! (font_not_found_p, (*s))
    {
      i = 0;
      while i < (*s).nchars
	{
	  let g: *mut glyph = (*s).first_glyph.offset (i as isize);
	  x_draw_rectangle ((*s).f,
			    (*s).gc as GC, x, (*s).y, (*g).pixel_width - 1,
			    (*s).height - 1);
	  x += (*g).pixel_width as c_int;
	  i += 1;
	}
    }
  else
    {
      let font: *mut font = (*s).font;

      #[cfg (feature = "use_cairo")]
      let cairo_arm_ok = !EQ ((*( (*font).driver as *const font_driver )).r#type, Qx)
	|| x_try_cr_xlib_drawable ((*s).f, (*s).gc as GC);
      #[cfg (not (feature = "use_cairo"))]
      let cairo_arm_ok = true;

      if cairo_arm_ok
	{
	  let mut boff: c_int = (*font).baseline_offset;
	  let mut y: c_int;

	  if (*font).vertical_centering
	    { boff = VCENTER_BASELINE_OFFSET! (font, (*s).f) - boff; }

	  y = (*s).ybase - boff;
	  if glyph_string_bits! (for_overlaps, (*s))
	      || (glyph_string_bits! (background_filled_p, (*s))
		  && (*s).hl != DRAW_CURSOR)
	    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, 0, (*s).nchars, x, y, false); }
	  else
	    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, 0, (*s).nchars, x, y, true); }
	  if face_bits! (overstrike, (*((*s).face as *mut face)))
	    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, 0, (*s).nchars, x + 1, y, false); }

	  #[cfg (feature = "use_cairo")]
	  {
	    if EQ ((*( (*font).driver as *const font_driver )).r#type, Qx)
	      { x_end_cr_xlib_drawable ((*s).f, (*s).gc as GC); }
	  }
	}
      else
	{
	  /* Fallback for the case that no Xlib Drawable is available
	     for drawing text with X core fonts.  */
	  if !(glyph_string_bits! (for_overlaps, (*s))
	       || (glyph_string_bits! (background_filled_p, (*s))
		   && (*s).hl != DRAW_CURSOR))
	    {
	      let box_line_width: c_int
		= max ((*((*s).face as *mut face)).box_horizontal_line_width as isize,
		       0) as c_int;

	      if glyph_string_bits! (stippled_p, (*s))
		{
		  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

		  /* Fill background with a stipple pattern.  */
		  XSetFillStyle (display, (*s).gc as GC, FillOpaqueStippled);
		  x_fill_rectangle ((*s).f, (*s).gc as GC, (*s).x,
				    (*s).y + box_line_width,
				    (*s).background_width,
				    (*s).height - 2 * box_line_width,
				    false);
		  XSetFillStyle (display, (*s).gc as GC, FillSolid);
		}
	      else
		{ x_clear_glyph_string_rect (s, (*s).x, (*s).y + box_line_width,
					     (*s).background_width,
					     (*s).height - 2 * box_line_width); }
	    }
	  i = 0;
	  while i < (*s).nchars
	    {
	      let g: *mut glyph = (*s).first_glyph.offset (i as isize);
	      x_draw_rectangle ((*s).f,
				(*s).gc as GC, x, (*s).y, (*g).pixel_width - 1,
				(*s).height - 1);
	      x += (*g).pixel_width as c_int;
	      i += 1;
	    }
	}
    }
}

/* C 8703-8810: `x_draw_composite_glyph_string_foreground`.  C's `s->first_glyph->u.cmp.
   automatic` is the `glyph_u_cmp_bits!` reader named after the member, `COMPOSITION_GLYPH` and
   the six `LGLYPH_*` readers are `include/composite.rs`'s, and C's `for (…)` loops that drive
   two counters keep C's own `i, j` in a `while` (the loop's increment is written where C puts
   it).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_composite_glyph_string_foreground (s: *mut glyph_string)
{

  let mut i: c_int;
  let mut j: c_int;
  let mut x: c_int;
  let font: *mut font = (*s).font;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if !(*s).face.is_null () && face_bits! (box, (*((*s).face as *mut face))) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max ((*((*s).face as *mut face)).box_vertical_line_width as isize,
			0) as c_int; }
  else
    { x = (*s).x; }

  /* S is a glyph string for a composition.  S->cmp_from is the index
     of the first character drawn for glyphs of this composition.
     S->cmp_from == 0 means we are drawing the very first character of
     this composition.  */

  /* Draw a rectangle for the composition if the font for the very
     first character of the composition could not be loaded.  */
  if glyph_string_bits! (font_not_found_p, (*s))
    {
      if (*s).cmp_from == 0
	{ x_draw_rectangle ((*s).f, (*s).gc as GC, x, (*s).y,
			    (*s).width - 1, (*s).height - 1); }
    }
  else
    {
      #[cfg (feature = "use_cairo")]
      let cairo_arm_ok = !EQ ((*( (*font).driver as *const font_driver )).r#type, Qx)
	|| x_try_cr_xlib_drawable ((*s).f, (*s).gc as GC);
      #[cfg (not (feature = "use_cairo"))]
      let cairo_arm_ok = true;

      if cairo_arm_ok
	{
	  if !glyph_u_cmp_bits! (automatic, (*(*s).first_glyph))
	    {
	      let y: c_int = (*s).ybase;

	      i = 0;
	      j = (*s).cmp_from;
	      while i < (*s).nchars
		{
		  /* TAB in a composition means display glyphs with
		     padding space on the left or right.  */
		  if COMPOSITION_GLYPH ((*(*s).cmp as *mut composition), j as ptrdiff_t) != b'\t' as isize
		    {
		      let cmp: *mut composition = (*s).cmp as *mut composition;
		      let xx: c_int
			= x + *(*cmp).offsets.offset ((j * 2) as isize) as c_int;
		      let yy: c_int
			= y - *(*cmp).offsets.offset ((j * 2 + 1) as isize) as c_int;

		      ((*((*font).driver as *const font_driver)).draw.unwrap ())
			(s, j, j + 1, xx, yy, false);
		      if face_bits! (overstrike, (*((*s).face as *mut face)))
			{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
			    (s, j, j + 1, xx + 1, yy, false); }
		    }

		  i += 1;
		  j += 1;
		}
	    }
	  else
	    {
	      let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);
	      let mut glyph: Lisp_Object;
	      let y: c_int = (*s).ybase;
	      let mut width: c_int = 0;

	      i = (*s).cmp_from;
	      j = (*s).cmp_from;
	      while i < (*s).cmp_to
		{
		  glyph = LGSTRING_GLYPH (gstring, i as isize);
		  if NILP (LGLYPH_ADJUSTMENT (glyph))
		    { width += LGLYPH_WIDTH (glyph) as c_int; }
		  else
		    {
		      let mut xoff: c_int;
		      let mut yoff: c_int;
		      let mut wadjust: c_int;

		      if j < i
			{
			  ((*((*font).driver as *const font_driver)).draw.unwrap ())
			    (s, j, i, x, y, false);
			  if face_bits! (overstrike, (*((*s).face as *mut face)))
			    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
				(s, j, i, x + 1, y, false); }
			  x += width;
			}
		      xoff = LGLYPH_XOFF (glyph) as c_int;
		      yoff = LGLYPH_YOFF (glyph) as c_int;
		      wadjust = LGLYPH_WADJUST (glyph) as c_int;
		      ((*((*font).driver as *const font_driver)).draw.unwrap ())
			(s, i, i + 1, x + xoff, y + yoff, false);
		      if face_bits! (overstrike, (*((*s).face as *mut face)))
			{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
			    (s, i, i + 1, x + xoff + 1, y + yoff, false); }
		      x += wadjust;
		      j = i + 1;
		      width = 0;
		    }
		  i += 1;
		}
	      if j < i
		{
		  ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, i, x, y, false);
		  if face_bits! (overstrike, (*((*s).face as *mut face)))
		    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
			(s, j, i, x + 1, y, false); }
		}
	    }

	  #[cfg (feature = "use_cairo")]
	  {
	    if EQ ((*( (*font).driver as *const font_driver )).r#type, Qx)
	      { x_end_cr_xlib_drawable ((*s).f, (*s).gc as GC); }
	  }
	}
      else
	{
	  /* Fallback for the case that no Xlib Drawable is available
	     for drawing text with X core fonts.  */
	  if (*s).cmp_from == 0
	    { x_draw_rectangle ((*s).f, (*s).gc as GC, x, (*s).y,
				(*s).width - 1, (*s).height - 1); }
	}
    }
}

/* C 8812-8896: `x_draw_glyphless_glyph_string_foreground`.  C's `unsigned char2b[8]` is an
   `unsigned int` array (`char2b` is the name C gives the implicit-`int` declaration) and
   `s->char2b = char2b` its decay; C 8834-8838's `#ifdef GCC_LINT` pair is carried under that
   feature (`GCC_LINT` is 1 in the frozen table, so the `= 1` arm is the live one); C 8841-8867's
   five `glyph->u.glyphless.*` reads are the `glyphless_bits!` accessors named after the member;
   `eassume` is the prelude's macro, `sprintf`'s `%0*X` the one C writes.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_glyphless_glyph_string_foreground (s: *mut glyph_string)
{
  let mut glyph: *mut glyph = (*s).first_glyph;
  let mut char2b: [c_uint; 8] = [0; 8];
  let mut x: c_int;
  let mut i: c_int;
  let mut j: c_int;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if !(*s).face.is_null () && face_bits! (box, (*((*s).face as *mut face))) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max ((*((*s).face as *mut face)).box_vertical_line_width as isize,
			0) as c_int; }
  else
    { x = (*s).x; }

  (*s).char2b = char2b.as_mut_ptr ();

  i = 0;
  while i < (*s).nchars
    {
      /* C 8834-8838's `#ifdef GCC_LINT` pair: the frozen table has `GCC_LINT` 1 (conf_post.h
	 486 defines it for every build), so the first arm is the text this target compiles and
	 the `#else` arm is carried under `not gcc_lint`.  */
      #[cfg (feature = "gcc_lint")]
      const PACIFY_GCC_BUG_81401: c_int = 1;
      #[cfg (not (feature = "gcc_lint"))]
      const PACIFY_GCC_BUG_81401: c_int = 0;
      let mut buf: [c_char; 7 + PACIFY_GCC_BUG_81401 as usize] =
	[0; 7 + PACIFY_GCC_BUG_81401 as usize];
      let mut str: *mut c_char = std::ptr::null_mut ();
      let len: c_int = glyphless_bits! (len, (*glyph));

      if glyphless_bits! (method, (*glyph)) == GLYPHLESS_DISPLAY_ACRONYM
	{
	  if len > 0
	      && CHAR_TABLE_P (Vglyphless_char_display)
	      && (CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (Vglyphless_char_display))
		  >= 1)
	    {
	      let mut acronym: Lisp_Object
		= if !glyphless_bits! (for_no_font, (*glyph))
		  { CHAR_TABLE_REF (Vglyphless_char_display,
				    glyphless_bits! (ch, (*glyph))) }
		  else { (*(XCHAR_TABLE (Vglyphless_char_display))).extras[0] };
	      if CONSP (acronym)
		{ acronym = XCAR (acronym); }
	      if STRINGP (acronym)
		{ str = SSDATA (acronym); }
	    }
	}
      else if glyphless_bits! (method, (*glyph)) == GLYPHLESS_DISPLAY_HEX_CODE
	{
	  let ch: c_uint = glyphless_bits! (ch, (*glyph)) as c_uint;
	  eassume! (ch as c_int <= MAX_CHAR);
	  sprintf (buf.as_mut_ptr (), c"%0*X".as_ptr (), if ch < 0x10000 { 4 } else { 6 },
		   ch);
	  str = buf.as_mut_ptr ();
	}

      if !str.is_null ()
	{
	  let upper_len: c_int = (len + 1) / 2;

	  /* It is assured that all LEN characters in STR is ASCII.  */
	  j = 0;
	  while j < len
	    {
	      char2b[j as usize]
		= ((*((*(*s).font).driver as *const font_driver)).encode_char.unwrap ())
		    ((*s).font, *str.offset (j as isize) as c_int) & 0xFFFF;
	      j += 1;
	    }
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, upper_len,
	     x + (*glyph).slice.glyphless.upper_xoff as c_int,
	     (*s).ybase + (*glyph).slice.glyphless.upper_yoff as c_int,
	     false);
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, upper_len, len,
	     x + (*glyph).slice.glyphless.lower_xoff as c_int,
	     (*s).ybase + (*glyph).slice.glyphless.lower_yoff as c_int,
	     false);
	}
      if glyphless_bits! (method, (*glyph)) != GLYPHLESS_DISPLAY_THIN_SPACE
	{ x_draw_rectangle ((*s).f, (*s).gc as GC,
			    x, (*s).ybase - (*glyph).ascent,
			    (*glyph).pixel_width - 1,
			    (*glyph).ascent + (*glyph).descent - 1); }
      x += (*glyph).pixel_width as c_int;
      glyph = glyph.offset (1);
      i += 1;
   }

  /* Defend against hypothetical bad code elsewhere that uses
     s->char2b after this function returns.  */
  (*s).char2b = std::ptr::null_mut ();
}

/* ---- C 8898-9126: `#ifdef USE_X_TOOLKIT` (the four color functions and the two Xt converter
   hooks) and C 9097's `x_color_cells`.  Nothing in this tree turns the Xt arm on (`USE_X_TOOLKIT`
   and `USE_LUCID` are both `undef` in the frozen table), so the whole group is text-only: the
   plain `have_x_windows` probe compiles none of it, and the ledger records that no reading below
   reaches it.  */

/* The three Xt names the arm's signatures need and no mirror carries.  `XtPointer`/`Boolean`
   *are* mirrored (`include/widget.rs` 182-185, reached through C 775's `"widget.h"`); these three
   are not, and this machine has no libXt headers (`/opt/homebrew/include/X11` carries Xlib's
   only) to measure a record from - so the arm names the scalars C uses and the ledger records the
   gap.  */
/// C: `<X11/Intrinsic.h>`: `typedef unsigned int Cardinal;`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type Cardinal = c_uint;
/// C: `<X11/Intrinsic.h>`: `typedef struct _XtAppStruct *XtAppContext;`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type XtAppContext = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef char *String;` - the `xfns.rs` 1183 shape, declared under the
/// arm's own gate so the unit's other text keeps the prelude's `String`.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type String = *mut c_char;

/* C 8900-8932: `#ifdef USE_LUCID` - `x_frame_of_widget`.  `FOR_EACH_FRAME` is
   `include/frame.rs` 800-808's macro (the `androidterm.rs` 282 call shape), `XtDisplay`/
   `XtIsTopLevelShell`/`XtParent` are Xt's (no mirror) and `x_dpyinfo` is this file's.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_lucid"))]
unsafe fn x_frame_of_widget (widget: Widget) -> *mut frame
{
  let mut widget: Widget = widget;
  let dpyinfo: *mut x_display_info;
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  dpyinfo = x_dpyinfo (XtDisplay (widget));

  /* Find the top-level shell of the widget.  Note that this function
     can be called when the widget is not yet realized, so XtWindow
     (widget) == 0.  That's the reason we can't simply use
     x_any_window_to_frame.  */
  while XtIsTopLevelShell (widget) == 0
    { widget = XtParent (widget); }

  /* Look for a frame with that top-level widget.  Allocate the color
     on that frame to get the right gamma correction value.  */
  FOR_EACH_FRAME! (tail, frame, {
    f = XFRAME (frame);
    if FRAME_X_P (f)
	&& FRAME_DISPLAY_INFO! (f) == dpyinfo
	&& (*( (*((*f).output_data.x as *mut x_output)) )).widget == widget
      { return f; }
  });
  emacs_abort ();
}

/* C 8934-8947: `x_alloc_lighter_color_for_widget`.  C's definition has no `static`, so the port
   keeps it exported (PORTING.md 0.2).  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_lucid"))]
#[unsafe(no_mangle)]
pub unsafe fn x_alloc_lighter_color_for_widget (widget: Widget, display: *mut Display,
						cmap: Colormap, pixel: *mut c_ulong,
						factor: c_double, delta: c_int) -> bool
{
  let f: *mut frame = x_frame_of_widget (widget);
  x_alloc_lighter_color (f, display, cmap, pixel, factor, delta)
}

/* C 8952-8967: the converter's two file-scope objects.  `cvt_string_to_pixel_value` is a plain
   `Pixel`; `cvt_string_to_pixel_args` is an `XtConvertArgRec[2]` whose element record cannot be
   laid out here (no libXt headers on this machine), so C's own initializer is carried as text
   below rather than guessed at (P-86 clause 3's text-only reading; the ledger names the missing
   toolchain and asks for the record).  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub static mut cvt_string_to_pixel_value: Pixel = 0;

/*
   C 8955-8961, C's text (its element record has no measured spelling here):

   static XtConvertArgRec cvt_string_to_pixel_args[] =
     {
       {XtWidgetBaseOffset, (XtPointer) offsetof (WidgetRec, core.screen),
	sizeof (Screen *)},
       {XtWidgetBaseOffset, (XtPointer) offsetof (WidgetRec, core.colormap),
	sizeof (Colormap)}
     };
*/

/* C 8970-9058: `cvt_string_to_pixel`.  `XrmValue`/`Colormap`/`Pixel`/`Screen` are mirrored
   (`include/xlib.rs` 253, 882, 543's neighbour and 153) and the Xt/Xlib entry points named here
   (`XtAppWarningMsg`, `XtDisplayToApplicationContext`, `XtDefaultBackground`,
   `WhitePixelOfScreen`, `DisplayOfScreen`, `XParseColor`) have no mirror: the arm names them as
   C does.  C's `*closure_ret = (XtPointer) False` keeps C's cast (`False` is
   `include/xterm.rs` 79's constant, `XtPointer` `*mut c_char`).  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe fn cvt_string_to_pixel (dpy: *mut Display, args: *mut XrmValue, nargs: *mut Cardinal,
			       from: *mut XrmValue, to: *mut XrmValue,
			       closure_ret: *mut XtPointer) -> Boolean
{

  let mut screen: *mut Screen;
  let mut cmap: Colormap;
  let mut pixel: Pixel;
  let mut color_name: String;
  let mut color: XColor = std::mem::zeroed ();

  if *nargs != 2
    {
      XtAppWarningMsg (XtDisplayToApplicationContext (dpy),
		       c"wrongParameters".as_ptr (), c"cvt_string_to_pixel".as_ptr (),
		       c"XtToolkitError".as_ptr (),
		       c"Screen and colormap args required".as_ptr (),
		       std::ptr::null_mut (), std::ptr::null_mut ());
      return false as Boolean;
    }

  screen = *((*args.add (0)).addr as *mut *mut Screen);
  cmap = *((*args.add (1)).addr as *mut Colormap);
  color_name = (*from).addr;

  if strcmp (color_name, XtDefaultBackground) == 0
    {
      *closure_ret = False as XtPointer;
      pixel = WhitePixelOfScreen (screen);
    }
  else if strcmp (color_name, XtDefaultForeground) == 0
    {
      *closure_ret = False as XtPointer;
      pixel = BlackPixelOfScreen (screen);
    }
  else if XParseColor (dpy, cmap, color_name, &raw mut color) != 0
	   && x_alloc_nearest_color_1 (dpy, cmap, &raw mut color)
    {
      pixel = color.pixel;
      *closure_ret = true as XtPointer;
    }
  else
    {
      let mut params: [String; 1] = [std::ptr::null_mut ()];
      let mut nparams: Cardinal = 1;

      params[0] = color_name;
      XtAppWarningMsg (XtDisplayToApplicationContext (dpy),
		       c"badValue".as_ptr (), c"cvt_string_to_pixel".as_ptr (),
		       c"XtToolkitError".as_ptr (), c"Invalid color '%s'".as_ptr (),
		       params.as_mut_ptr (), &raw mut nparams);
      return false as Boolean;
    }

  if !(*to).addr.is_null ()
    {
      if (*to).size < std::mem::size_of::<Pixel> () as c_uint
	{
	  (*to).size = std::mem::size_of::<Pixel> () as c_uint;
	  return false as Boolean;
	}

      *((*to).addr as *mut Pixel) = pixel;
    }
  else
    {
      cvt_string_to_pixel_value = pixel;
      (*to).addr = &raw mut cvt_string_to_pixel_value as XtPointer;
    }

  (*to).size = std::mem::size_of::<Pixel> () as c_uint;
  return true as Boolean;
}

/* C 9061-9092: `cvt_pixel_dtor`.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe fn cvt_pixel_dtor (app: XtAppContext, to: XrmValuePtr, closure: XtPointer,
			  args: XrmValuePtr, nargs: *mut Cardinal)
{
  if *nargs != 2
    {
      XtAppWarningMsg (app, c"wrongParameters".as_ptr (), c"cvt_pixel_dtor".as_ptr (),
		       c"XtToolkitError".as_ptr (),
		       c"Screen and colormap arguments required".as_ptr (),
		       std::ptr::null_mut (), std::ptr::null_mut ());
    }
  else if !closure.is_null ()
    {
      /* We did allocate the pixel, so free it.  */
      let screen: *mut Screen = *((*args).addr as *mut *mut Screen);
      let cmap: Colormap = *((*args.add (1)).addr as *mut Colormap);
      x_free_dpy_colors (DisplayOfScreen (screen), screen, cmap,
			 (*to).addr as *mut c_ulong, 1);
    }
}

/* C 9097-9126: `x_color_cells`.  `xnmalloc` is `include/lisp.rs`'s, and C's two assignments
   into the array widen an `int` to `XColor.pixel`'s `unsigned long` (the one conversion C
   performs, written out).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_color_cells (dpy: *mut Display, ncells: *mut c_int) -> *const XColor
{
  let dpyinfo: *mut x_display_info = x_dpyinfo (dpy);

  if (*dpyinfo).color_cells.is_null ()
    {
      let ncolor_cells: c_int = (*dpyinfo).visual_info.colormap_size;
      let mut i: c_int;

      (*dpyinfo).color_cells = xnmalloc (ncolor_cells as isize,
					 std::mem::size_of_val (&*(*dpyinfo).color_cells) as isize)
	as *mut XColor;
      (*dpyinfo).ncolor_cells = ncolor_cells;

      i = 0;
      while i < ncolor_cells
	{
	  (*((*dpyinfo).color_cells).offset (i as isize)).pixel = i as c_ulong;
	  i += 1;
	}

      XQueryColors (dpy, (*dpyinfo).cmap,
		    (*dpyinfo).color_cells, ncolor_cells);
    }

  *ncells = (*dpyinfo).ncolor_cells;
  return (*dpyinfo).color_cells;
}

/* ---- C 9128-9370: the color readers and the name cache.  */

/* The five prototypes this group calls, none of which any mirror in this graph carries, each
   declared at its use site with C's own line (PORTING.md P-55; the `fringe.rs` 846-850 shape).
   `x_free_colors`, `parse_color_spec` and `gamma_correct` are dispextern.h's, `x_free_dpy_colors`
   is xterm.h 1980's (the mirror does not carry it), and `register_color` is xfaces.c 422's (its
   prototype lives inside that file's `#ifdef DEBUG_X_COLORS` block).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: dispextern.h 3791: `void x_free_colors (struct frame *, unsigned long *, int);`
  pub fn x_free_colors (f: *mut frame, pixels: *mut c_ulong, npixels: c_int);
  /// C: dispextern.h 3798-3799: `bool parse_color_spec (const char *, unsigned short *,
  ///                                    unsigned short *, unsigned short *);`
  pub fn parse_color_spec (spec: *const c_char, r: *mut c_ushort, g: *mut c_ushort,
			   b: *mut c_ushort) -> bool;
  /// C: dispextern.h 3839: `void gamma_correct (struct frame *, XColor *);`
  pub fn gamma_correct (f: *mut frame, color: *mut XColor);
  /// C: xterm.h 1980-1981: `extern void x_free_dpy_colors (Display *, Screen *, Colormap,
  ///                              unsigned long *, int);`
  pub fn x_free_dpy_colors (dpy: *mut Display, screen: *mut Screen, cmap: Colormap,
			    pixels: *mut c_ulong, npixels: c_int);
}

/* C 9129-9200: `x_query_colors`.  C's `0xffffffff / rmask` is an `unsigned int` division (the
   port writes the literal as `c_uint` so the division stays the width C uses), the shifts and
   masks are `c_uint`s, and C's two `eassert`s keep C's `pixel < dpyinfo->ncolor_cells`
   comparison.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_query_colors (f: *mut frame, colors: *mut XColor, ncolors: c_int)
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut i: c_int;

  if (*dpyinfo).red_bits > 0
    {
      /* For TrueColor displays, we can decompose the RGB value
	 directly.  */
      let mut rmult: c_uint;
      let mut gmult: c_uint;
      let mut bmult: c_uint;
      let rmask: c_uint;
      let gmask: c_uint;
      let bmask: c_uint;

      rmask = (1 << (*dpyinfo).red_bits) - 1;
      gmask = (1 << (*dpyinfo).green_bits) - 1;
      bmask = (1 << (*dpyinfo).blue_bits) - 1;
      /* If we're widening, for example, 8 bits in the pixel value to
	 16 bits for the separate-color representation, we want to
	 extrapolate the lower bits based on those bits available --
	 in other words, we'd like 0xff to become 0xffff instead of
	 the 0xff00 we'd get by just zero-filling the lower bits.

         We generate a 32-bit scaled-up value and shift it, in case
         the bit count doesn't divide 16 evenly (e.g., when dealing
         with a 3-3-2 bit RGB display), to get more of the lower bits
         correct.

         Should we cache the multipliers in dpyinfo?  Maybe
         special-case the 8-8-8 common case?  */
      rmult = 0xffffffff_u32 as c_uint / rmask;
      gmult = 0xffffffff_u32 as c_uint / gmask;
      bmult = 0xffffffff_u32 as c_uint / bmask;

      i = 0;
      while i < ncolors
	{
	  let mut r: c_uint;
	  let mut g: c_uint;
	  let mut b: c_uint;
	  let pixel: c_ulong = (*colors.offset (i as isize)).pixel;

	  r = ((pixel >> (*dpyinfo).red_offset) & rmask as c_ulong) as c_uint;
	  g = ((pixel >> (*dpyinfo).green_offset) & gmask as c_ulong) as c_uint;
	  b = ((pixel >> (*dpyinfo).blue_offset) & bmask as c_ulong) as c_uint;

	  (*colors.offset (i as isize)).red = ((r * rmult) >> 16) as c_ushort;
	  (*colors.offset (i as isize)).green = ((g * gmult) >> 16) as c_ushort;
	  (*colors.offset (i as isize)).blue = ((b * bmult) >> 16) as c_ushort;
	  i += 1;
	}

      return;
    }

  if !(*dpyinfo).color_cells.is_null ()
    {
      let mut i: c_int;
      i = 0;
      while i < ncolors
	{
	  let pixel: c_ulong = (*colors.offset (i as isize)).pixel;
	  eassert! (pixel < (*dpyinfo).ncolor_cells as c_ulong);
	  eassert! ((*(*dpyinfo).color_cells.offset (pixel as isize)).pixel == pixel);
	  *colors.offset (i as isize) = *(*dpyinfo).color_cells.offset (pixel as isize);
	  i += 1;
	}
      return;
    }

  XQueryColors (FRAME_X_DISPLAY! (f), FRAME_X_COLORMAP! (f), colors, ncolors);
}

/* C 9202-9236: `x_query_frame_background_color`.  C's `#ifdef USE_CAIRO`/`#else` pair is the
   port's two arms, both carried (P-51): the cairo one extends the background from
   `f->alpha_background`, the other calls the non-cairo premultiplier.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_query_frame_background_color (f: *mut frame, bgcolor: *mut XColor)
{
  let mut background: c_ulong = FRAME_BACKGROUND_PIXEL (f);
  #[cfg (not (feature = "use_cairo"))]
  let mut bg: XColor = std::mem::zeroed ();

  if (*FRAME_DISPLAY_INFO! (f)).alpha_bits != 0
    {
      #[cfg (feature = "use_cairo")]
      {
	background = (background & !(*FRAME_DISPLAY_INFO! (f)).alpha_mask);
	background |= ((((*f).alpha_background * 0xffff) as c_ulong
			>> (16 - (*FRAME_DISPLAY_INFO! (f)).alpha_bits))
		       << (*FRAME_DISPLAY_INFO! (f)).alpha_offset);
      }

      #[cfg (not (feature = "use_cairo"))]
      {
	if (*FRAME_DISPLAY_INFO! (f)).alpha_bits != 0
	    && (*f).alpha_background < 1.0
	  {
	    bg.pixel = background;
	    x_query_colors (f, &raw mut bg, 1);

	    background
	      = x_premultiply_pixel (FRAME_DISPLAY_INFO! (f),
				     &raw mut bg, (*f).alpha_background);
	  }
      }
    }

  (*bgcolor).pixel = background;

  x_query_colors (f, bgcolor, 1);
}

/* C 9238-9251: `x_hash_string_ignore_case`.  C's `3323198485ul` seed and its `i *= 0x5bd1e995`
   are `unsigned int` arithmetic (`c_uint`'s wrapping multiply, P-15's pair for `*=`), and
   `c_tolower` is `lib/include/c-ctype.rs`'s; C's `for` is the `while` with the increment at the
   body's end (its `++string` runs where C puts it).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_hash_string_ignore_case (string: *const c_char) -> c_uint
{
  let mut string: *const c_char = string;
  let mut i: c_uint;

  i = 3323198485_u32;
  while *string != 0
    {
      i ^= c_tolower (*string as c_int) as c_uint;
      i = i.wrapping_mul (0x5bd1e995_u32);
      i ^= i >> 15;
      string = string.offset (1);
    }
  i
}

/* C 9253-9370: `x_parse_color`.  `parse_color_spec`'s three out-parameters take the `c_ushort`
   locals C declares, `xstrcasecmp` is `include/dispextern.rs` 2607's, and C's two
   `x_color_cache_bucket_size > 0 ? … : 128` expressions are the port's `if`/`else`; `dpyinfo`'s
   `color_name_cache_entry.valid` is a `bool_bf` the mirror keeps as a comment (the ledger's
   gap list), so C's spelling is the one the port writes.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_parse_color (f: *mut frame, color_name: *const c_char,
			     color: *mut XColor) -> Status
{
  use crate::x11_constants::{None, XA_CARDINAL};

  let mut r: c_ushort;
  let mut g: c_ushort;
  let mut b: c_ushort;
  let dpy: *mut Display;
  let cmap: Colormap;
  let dpyinfo: *mut x_display_info;
  let mut cache_entry: *mut color_name_cache_entry;
  let mut last: *mut color_name_cache_entry;
  let mut next: *mut color_name_cache_entry;
  let mut color_entry: *mut color_name_cache_entry;
  let mut hash: c_uint;
  let mut idx: c_uint;
  let mut rc: c_int;
  let mut i: c_int;

  /* Don't pass #RGB strings directly to XParseColor, because that
     follows the X convention of zero-extending each channel
     value: #f00 means #f00000.  We want the convention of scaling
     channel values, so #f00 means #ff0000, just as it does for
     HTML, SVG, and CSS.  */
  if parse_color_spec (color_name, &raw mut r, &raw mut g, &raw mut b)
    {
      (*color).red = r;
      (*color).green = g;
      (*color).blue = b;

      return 1;
    }

  /* Some X servers send BadValue on empty color names.  */
  if strlen (color_name) == 0
    { return 0; }

  cmap = FRAME_X_COLORMAP! (f);
  dpy = FRAME_X_DISPLAY! (f);
  dpyinfo = FRAME_DISPLAY_INFO! (f);

  hash = x_hash_string_ignore_case (color_name);
  idx = hash % (*dpyinfo).color_names_size as c_uint;

  last = std::ptr::null_mut ();

  cache_entry = *(*dpyinfo).color_names.offset (idx as isize);
  while !cache_entry.is_null ()
    {
      if xstrcasecmp ((*cache_entry).name, color_name) == 0
	{
	  /* Move recently used entries to the start of the color
	     cache.  */

	  if !last.is_null ()
	    {
	      (*last).next = (*cache_entry).next;
	      (*cache_entry).next = *(*dpyinfo).color_names.offset (idx as isize);

	      *(*dpyinfo).color_names.offset (idx as isize) = cache_entry;
	    }

	  if (*cache_entry).valid
	    { *color = (*cache_entry).rgb; }

	  return (*cache_entry).valid as Status;
	}

      last = cache_entry;
      cache_entry = (*cache_entry).next;
    }

  block_input ();
  rc = XParseColor (dpy, cmap, color_name, color);
  unblock_input ();

  cache_entry = xzalloc (std::mem::size_of_val (&*cache_entry)) as *mut color_name_cache_entry;
  *(*dpyinfo).color_names_length.offset (idx as isize) += 1;

  if rc != 0
    { (*cache_entry).rgb = *color; }

  (*cache_entry).valid = rc != 0;
  (*cache_entry).name = xstrdup (color_name);
  (*cache_entry).next = *(*dpyinfo).color_names.offset (idx as isize);

  *(*dpyinfo).color_names.offset (idx as isize) = cache_entry;

  /* Don't let the color cache become too big.  */
  if *(*dpyinfo).color_names_length.offset (idx as isize)
      > (if x_color_cache_bucket_size > 0 { x_color_cache_bucket_size } else { 128 })
    {
      i = 0;

      last = *(*dpyinfo).color_names.offset (idx as isize);
      while !last.is_null ()
	{
	  i += 1;
	  if i == (if x_color_cache_bucket_size > 0 { x_color_cache_bucket_size as c_int } else { 128 })
	    {
	      next = (*last).next;
	      (*last).next = std::ptr::null_mut ();

	      color_entry = next;
	      while !color_entry.is_null ()
		{
		  last = (*color_entry).next;

		  xfree ((*color_entry).name as *mut c_void);
		  xfree (color_entry as *mut c_void);

		  *(*dpyinfo).color_names_length.offset (idx as isize) -= 1;
		  color_entry = last;
		}

	      return rc;
	    }
	  last = (*last).next;
	}
    }

  return rc;
}

/* ---- C 9372-9566: the nearest-color allocator, the frame entry point and the reference-count
   copy.  */

/* C 9373-9512: `x_alloc_nearest_color_1`.  C's `start:` label with its backward `goto start` is
   the port's labelled `loop` whose `continue 'start` is the jump and whose trailing `break` the
   fall-through (PORTING.md 2.2); the `#ifdef DEBUG_X_COLORS` pair is carried under that feature
   (`debug_x_colors`), and `register_color` is xfaces.c 422's (its prototype lives inside that
   file's own `#ifdef DEBUG_X_COLORS`), so it is named and not declared.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_alloc_nearest_color_1 (dpy: *mut Display, cmap: Colormap, color: *mut XColor) -> bool
{
  use crate::x11_constants::DirectColor;

  let dpyinfo: *mut x_display_info = x_dpyinfo (dpy);
  let mut rc: bool = XAllocColor (dpy, cmap, color) != 0;

  if (*dpyinfo).visual_info.class == DirectColor
    { return rc; }

  if !rc
    {
      /* If we got to this point, the colormap is full, so we're going
	 to try and get the next closest color.  The algorithm used is
	 a least-squares matching, which is what X uses for closest
	 color matching with StaticColor visuals.  */

      let mut cells: *const XColor;
      let mut no_cells: c_int;
      let mut nearest: c_int;
      let mut nearest_delta: c_long;
      let mut trial_delta: c_long;
      let mut x: c_int;
      let mut status: Status;
      let mut retry: bool = false;
      let mut ncolor_cells: c_int;
      let mut i: c_int;
      let mut temp_allocated: bool;
      let mut temp: XColor = std::mem::zeroed ();

      'start: loop
	{
	  cells = x_color_cells (dpy, &raw mut no_cells);
	  temp_allocated = false;

	  nearest = 0;
	  /* I'm assuming CSE so I'm not going to condense this. */
	  nearest_delta = (((((*color).red >> 8) as c_int - ((*cells.offset (0)).red >> 8) as c_int)
			    * (((*color).red >> 8) as c_int - ((*cells.offset (0)).red >> 8) as c_int))
			   + ((((*color).green >> 8) as c_int
			       - ((*cells.offset (0)).green >> 8) as c_int)
			      * (((*color).green >> 8) as c_int
				 - ((*cells.offset (0)).green >> 8) as c_int))
			   + ((((*color).blue >> 8) as c_int
			       - ((*cells.offset (0)).blue >> 8) as c_int)
			      * (((*color).blue >> 8) as c_int
				 - ((*cells.offset (0)).blue >> 8) as c_int))) as c_long;
	  x = 1;
	  while x < no_cells
	    {
	      trial_delta = (((((*color).red >> 8) as c_int
			       - ((*cells.offset (x as isize)).red >> 8) as c_int)
			      * (((*color).red >> 8) as c_int
				 - ((*cells.offset (x as isize)).red >> 8) as c_int))
			     + ((((*color).green >> 8) as c_int
				 - ((*cells.offset (x as isize)).green >> 8) as c_int)
				* (((*color).green >> 8) as c_int
				   - ((*cells.offset (x as isize)).green >> 8) as c_int))
			     + ((((*color).blue >> 8) as c_int
				 - ((*cells.offset (x as isize)).blue >> 8) as c_int)
				* (((*color).blue >> 8) as c_int
				   - ((*cells.offset (x as isize)).blue >> 8) as c_int))) as c_long;
	      if trial_delta < nearest_delta
		{
		  /* We didn't decide to use this color, so free it.  */
		  if temp_allocated
		    {
		      XFreeColors (dpy, cmap, &raw mut temp.pixel, 1, 0);
		      temp_allocated = false;
		    }

		  temp.red = (*cells.offset (x as isize)).red;
		  temp.green = (*cells.offset (x as isize)).green;
		  temp.blue = (*cells.offset (x as isize)).blue;
		  status = XAllocColor (dpy, cmap, &raw mut temp);

		  if status != 0
		    {
		      temp_allocated = true;
		      nearest = x;
		      nearest_delta = trial_delta;
		    }
		}
	      x += 1;
	    }
	  (*color).red = (*cells.offset (nearest as isize)).red;
	  (*color).green = (*cells.offset (nearest as isize)).green;
	  (*color).blue = (*cells.offset (nearest as isize)).blue;

	  if !temp_allocated
	    { status = XAllocColor (dpy, cmap, color); }
	  else
	    {
	      *color = temp;
	      status = 1;
	    }

	  if status == 0 && !retry
	    {
	      /* Our private cache of color cells is probably out of date.
		 Refresh it here, and try to allocate the nearest color
		 from the new colormap.  */

	      retry = true;
	      xfree ((*dpyinfo).color_cells as *mut c_void);

	      ncolor_cells = (*dpyinfo).visual_info.colormap_size;

	      (*dpyinfo).color_cells = xnmalloc (ncolor_cells as isize,
						 std::mem::size_of_val (&*(*dpyinfo).color_cells) as isize)
		as *mut XColor;
	      (*dpyinfo).ncolor_cells = ncolor_cells;

	      i = 0;
	      while i < ncolor_cells
		{
		  (*((*dpyinfo).color_cells).offset (i as isize)).pixel = i as c_ulong;
		  i += 1;
		}

	      XQueryColors (dpy, (*dpyinfo).cmap,
			    (*dpyinfo).color_cells, ncolor_cells);

	      continue 'start;
	    }

	  rc = status != 0;
	  break 'start;
	}
    }
  else
    {
      /* If allocation succeeded, and the allocated pixel color is not
         equal to a cached pixel color recorded earlier, there was a
         change in the colormap, so clear the color cache.  */
      let dpyinfo: *mut x_display_info = x_dpyinfo (dpy);

      if !(*dpyinfo).color_cells.is_null ()
	{
	  let cached_color: *mut XColor
	    = (*dpyinfo).color_cells.offset ((*color).pixel as isize);
	  if (*cached_color).red != (*color).red
	      || (*cached_color).blue != (*color).blue
	      || (*cached_color).green != (*color).green
	    {
	      xfree ((*dpyinfo).color_cells as *mut c_void);
	      (*dpyinfo).color_cells = std::ptr::null_mut ();
	      (*dpyinfo).ncolor_cells = 0;
	    }
	}
    }

  #[cfg (feature = "debug_x_colors")]
  {
    if rc
      { register_color ((*color).pixel); }
  }

  rc
}

/* C 9514-9537: `x_alloc_nearest_color`.  C's definition has no `static`, so the port keeps it
   exported (PORTING.md 0.2); C's `?:`-free body and its `return true` are C's own.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_alloc_nearest_color (f: *mut frame, cmap: Colormap, color: *mut XColor) -> bool
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);

  gamma_correct (f, color);

  if (*dpyinfo).red_bits > 0
    {
      (*color).pixel = x_make_truecolor_pixel (dpyinfo,
					       (*color).red as c_int,
					       (*color).green as c_int,
					       (*color).blue as c_int);
      return true;
    }

  x_alloc_nearest_color_1 (FRAME_X_DISPLAY! (f), cmap, color)
}

/* C 9539-9566: `x_copy_color`.  C's definition has no `static` either (PORTING.md 0.2); the
   `#ifdef DEBUG_X_COLORS` pair is carried under its feature.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_copy_color (f: *mut frame, pixel: c_ulong) -> c_ulong
{
  let mut color: XColor = std::mem::zeroed ();

  /* If display has an immutable color map, freeing colors is not
     necessary and some servers don't allow it.  Since we won't free a
     color once we've allocated it, we don't need to re-allocate it to
     maintain the server's reference count.  */
  if !x_mutable_colormap (FRAME_X_VISUAL_INFO! (f))
    { return pixel; }

  color.pixel = pixel;
  block_input ();
  /* The color could still be found in the color_cells array.  */
  x_query_colors (f, &raw mut color, 1);
  XAllocColor (FRAME_X_DISPLAY! (f), FRAME_X_COLORMAP! (f), &raw mut color);
  unblock_input ();
  #[cfg (feature = "debug_x_colors")]
  { register_color (pixel); }
  color.pixel
}

/* ---- C 9568-9785: the highlight-limit constant, the lighter-color allocator, the relief pair
   and C 9744's three non-cairo helpers.  */

/* C 9568-9580: the comment (C's prose, kept above) and `#define
   HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 48000` - a value, so the port writes the constant the
   extractor reads it as (the `OPAQUE` shape).  */
#[cfg (feature = "have_x_windows")]
pub const HIGHLIGHT_COLOR_DARK_BOOST_LIMIT: c_int = 48000;

/* C 9583-9657: `x_alloc_lighter_color`.  C's `min`/`max` are lisp.h's `EMACS_INT` pair (so each
   call's two `int` operands and its result are spelled out), `x_free_colors` is the use-site
   declaration above, and C's `double dimness = 1 - (double) bright / LIMIT` keeps C's two
   conversions.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_alloc_lighter_color (f: *mut frame, display: *mut Display, cmap: Colormap,
				 pixel: *mut c_ulong, factor: c_double,
				 delta: c_int) -> bool
{
  let mut color: XColor = std::mem::zeroed ();
  let mut new: XColor = std::mem::zeroed ();
  let mut bright: c_long;
  let mut success_p: bool;

  /* Get RGB color values.  */
  color.pixel = *pixel;
  x_query_colors (f, &raw mut color, 1);

  /* Change RGB values by specified FACTOR.  Avoid overflow!  */
  eassert! (factor >= 0.0);
  new.red = min (0xffff, (factor * color.red as c_double) as isize) as c_ushort;
  new.green = min (0xffff, (factor * color.green as c_double) as isize) as c_ushort;
  new.blue = min (0xffff, (factor * color.blue as c_double) as isize) as c_ushort;

  /* Calculate brightness of COLOR.  */
  bright = ((2 * color.red as c_int + 3 * color.green as c_int
	     + color.blue as c_int) / 6) as c_long;

  /* We only boost colors that are darker than
     HIGHLIGHT_COLOR_DARK_BOOST_LIMIT.  */
  if bright < HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as c_long
    /* Make an additive adjustment to NEW, because it's dark enough so
       that scaling by FACTOR alone isn't enough.  */
    {
      /* How far below the limit this color is (0 - 1, 1 being darker).  */
      let dimness: c_double = 1.0 - bright as c_double
	/ HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as c_double;
      /* The additive adjustment.  */
      let min_delta: c_int = (delta as c_double * dimness * factor / 2.0) as c_int;

      if factor < 1.0
	{
	  new.red = max (0, new.red as isize - min_delta as isize) as c_ushort;
	  new.green = max (0, new.green as isize - min_delta as isize) as c_ushort;
	  new.blue = max (0, new.blue as isize - min_delta as isize) as c_ushort;
	}
      else
	{
	  new.red = min (0xffff, min_delta as isize + new.red as isize) as c_ushort;
	  new.green = min (0xffff, min_delta as isize + new.green as isize) as c_ushort;
	  new.blue = min (0xffff, min_delta as isize + new.blue as isize) as c_ushort;
	}
    }

  /* Try to allocate the color.  */
  success_p = x_alloc_nearest_color (f, cmap, &raw mut new);
  if success_p
    {
      if new.pixel == *pixel
	{
	  /* If we end up with the same color as before, try adding
	     delta to the RGB values.  */
	  x_free_colors (f, &raw mut new.pixel, 1);

	  new.red = min (0xffff, delta as isize + color.red as isize) as c_ushort;
	  new.green = min (0xffff, delta as isize + color.green as isize) as c_ushort;
	  new.blue = min (0xffff, delta as isize + color.blue as isize) as c_ushort;
	  success_p = x_alloc_nearest_color (f, cmap, &raw mut new);
	}
      else
	{ success_p = true; }
      *pixel = new.pixel;
    }

  success_p
}

/* C 9659-9707: `x_setup_relief_color`.  C's `relief->pixel != -1` compares the mirror's
   `unsigned long` member with -1 (the `pixel` field is signed in C's intent), and the two
   `XCreateGC`/`XChangeGC` calls take `display`/`mask`/`xgcv` as C writes them.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_setup_relief_color (f: *mut frame, relief: *mut relief, factor: c_double,
				delta: c_int, default_pixel: c_ulong)
{
  use crate::x11_constants::GCForeground;

  let mut xgcv: XGCValues = std::mem::zeroed ();
  let di: *mut x_output = (*f).output_data.x as *mut x_output;
  let mut mask: c_ulong = (GCForeground | GCLineWidth | GCGraphicsExposures) as c_ulong;
  let mut pixel: c_ulong;
  let background: c_ulong = (*di).relief_background;
  let cmap: Colormap = FRAME_X_COLORMAP! (f);
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);

  xgcv.graphics_exposures = false as Bool;
  xgcv.line_width = 1;

  /* Free previously allocated color.  The color cell will be reused
     when it has been freed as many times as it was allocated, so this
     doesn't affect faces using the same colors.  */
  if (*relief).gc != 0 as GC && (*relief).pixel != (-1 as c_long) as c_ulong
    {
      x_free_colors (f, &raw mut (*relief).pixel, 1);
      (*relief).pixel = (-1 as c_long) as c_ulong;
    }

  /* Allocate new color.  */
  xgcv.foreground = default_pixel;
  pixel = background;
  if (*dpyinfo).n_planes != 1
      && x_alloc_lighter_color (f, dpy, cmap, &raw mut pixel, factor, delta)
    { xgcv.foreground = pixel; (*relief).pixel = pixel; }

  if (*relief).gc == 0 as GC
    {
      xgcv.stipple = (*dpyinfo).gray;
      mask |= GCStipple as c_ulong;
      (*relief).gc = XCreateGC (dpy, FRAME_X_DRAWABLE! (f), mask,
				&raw mut xgcv as *mut c_void);
    }
  else
    { XChangeGC (dpy, (*relief).gc, mask, &raw mut xgcv as *mut c_void); }
}

/* C 9709-9742: `x_setup_relief_colors`.  `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT`
   are dispextern.h's macros (this graph's mirror keeps their `_SHIFT`/`_MASK` constants but not
   the two readers), and `WHITE_PIX_DEFAULT`/`BLACK_PIX_DEFAULT` are `include/xterm.rs` 104's
   pair.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_setup_relief_colors (s: *mut glyph_string)
{
  let di: *mut x_output = (*(*s).f).output_data.x as *mut x_output;
  let mut color: c_ulong;

  if face_bits! (use_box_color_for_shadows_p, (*((*s).face as *mut face)))
    { color = (*((*s).face as *mut face)).box_color; }
  else if glyph_bits! (type, (*(*s).first_glyph)) == IMAGE_GLYPH
	   && !(*((*s).img)).pixmap.is_null ()
	   && !IMAGE_BACKGROUND_TRANSPARENT ((*s).img, (*s).f, std::ptr::null_mut ())
    { color = IMAGE_BACKGROUND ((*s).img, (*s).f, std::ptr::null_mut ()); }
  else
    {
      let mut xgcv: XGCValues = std::mem::zeroed ();

      /* Get the background color of the face.  */
      XGetGCValues (FRAME_X_DISPLAY! ((*s).f), (*s).gc as GC, GCBackground as c_ulong,
		    &raw mut xgcv);
      color = xgcv.background;
    }

  if (*di).white_relief.gc == 0 as GC
      || color != (*di).relief_background
    {
      (*di).relief_background = color;
      x_setup_relief_color ((*s).f, &raw mut (*di).white_relief, 1.2, 0x8000,
			    WHITE_PIX_DEFAULT! ((*s).f));
      x_setup_relief_color ((*s).f, &raw mut (*di).black_relief, 0.6, 0x4000,
			    BLACK_PIX_DEFAULT! ((*s).f));
    }
}

/* C 9744-9785: `#ifndef USE_CAIRO` - `x_fill_triangle`, `x_make_point` and `x_inside_rect_p`.
   `XPoint`, `XFillPolygon`, `Convex` and `CoordModeOrigin` are Xlib's (no mirror here), and
   C's `for` loop over `rects` keeps its index.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe fn x_fill_triangle (f: *mut frame, gc: GC, point1: XPoint,
			   point2: XPoint, point3: XPoint)
{
  let mut abc: [XPoint; 3] = std::mem::zeroed ();

  abc[0] = point1;
  abc[1] = point2;
  abc[2] = point3;

  XFillPolygon (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
		gc, abc.as_mut_ptr (), 3, Convex, CoordModeOrigin);
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe fn x_make_point (x: c_int, y: c_int) -> XPoint
{
  let mut pt: XPoint = std::mem::zeroed ();

  pt.x = x as c_short;
  pt.y = y as c_short;

  pt
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe fn x_inside_rect_p (rects: *mut XRectangle, nrects: c_int, x: c_int, y: c_int) -> bool
{
  let mut i: c_int;

  i = 0;
  while i < nrects
    {
      if x >= (*rects.offset (i as isize)).x as c_int
	  && y >= (*rects.offset (i as isize)).y as c_int
	  && x < (*rects.offset (i as isize)).x as c_int
		 + (*rects.offset (i as isize)).width as c_int
	  && y < (*rects.offset (i as isize)).y as c_int
		 + (*rects.offset (i as isize)).height as c_int
	{ return true; }
      i += 1;
    }

  false
}

/* ---- Slice 10: C 9786-11915 - the relief and box drawing, the image glyph drawing, the
   glyph-string drawer, the frame clearing, the hourglass and the visible bell, and
   `x_scroll_run`.  Slice 9 stopped at C 9785 (`x_inside_rect_p`'s closing brace).

   Every item below carries the file gate `#[cfg (feature = "have_x_windows")]` and each of C's
   own `#if` conditions its lowercase feature, the口径 slices 1-9 established (the file is not in
   the frozen build, so the arms are carried per P-51/P-86 rather than dropped).  The names this
   slice's live text calls that no mirror in this graph declares (Xlib's entry points and the
   other units' functions - `window_box`, `cursor_in_mouse_face_p`, `image_ascent`, ...) stay
   bare, the class slices 1-9 record in this ledger; the X11 constants the mirror does not carry
   (`FillSolid`, `GXcopy`, `Unsorted`, ...) likewise.  Both lists are in the ledger's Decisions.  */

/* C 9786-9988: `x_draw_relief_rect`.  C's `#ifdef USE_CAIRO` arm declares its three locals and
   calls the two helpers that arm landed with (`x_fill_trapezoid_for_relief`,
   `x_erase_corners_for_relief`); the `#else` arm declares its six.  `f->output_data.x->…` is
   written the way slice 9's `x_setup_relief_colors` writes it (`*( … as *mut x_output )`), the
   local `corners` keeps C's name (the enum type `corners` lives in the type namespace, so the
   two do not collide), and C 9860's `6, 1` are the two `double` parameters the port's
   `x_erase_corners_for_relief` takes, so each conversion C makes implicitly is written out.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_relief_rect (f: *mut frame, left_x: c_int, top_y: c_int, right_x: c_int,
			      bottom_y: c_int, hwidth: c_int, vwidth: c_int, raised_p: bool,
			      top_p: bool, bot_p: bool, left_p: bool, right_p: bool,
			      clip_rect: *mut XRectangle)
{
  #[cfg (feature = "use_cairo")]
  {
    let mut top_left_gc: GC;
    let mut bottom_right_gc: GC;
    let mut corners: c_int = 0;

    if raised_p
      {
	top_left_gc = (*( (*f).output_data.x as *mut x_output )).white_relief.gc;
	bottom_right_gc = (*( (*f).output_data.x as *mut x_output )).black_relief.gc;
      }
    else
      {
	top_left_gc = (*( (*f).output_data.x as *mut x_output )).black_relief.gc;
	bottom_right_gc = (*( (*f).output_data.x as *mut x_output )).white_relief.gc;
      }

    x_set_clip_rectangles (f, top_left_gc, clip_rect, 1);
    x_set_clip_rectangles (f, bottom_right_gc, clip_rect, 1);

    if left_p
      {
	x_fill_rectangle (f, top_left_gc, left_x, top_y,
			  vwidth, bottom_y + 1 - top_y, false);
	if top_p
	  { corners |= 1 << (corners::CORNER_TOP_LEFT as c_int); }
	if bot_p
	  { corners |= 1 << (corners::CORNER_BOTTOM_LEFT as c_int); }
      }
    if right_p
      {
	x_fill_rectangle (f, bottom_right_gc, right_x + 1 - vwidth, top_y,
			  vwidth, bottom_y + 1 - top_y, false);
	if top_p
	  { corners |= 1 << (corners::CORNER_TOP_RIGHT as c_int); }
	if bot_p
	  { corners |= 1 << (corners::CORNER_BOTTOM_RIGHT as c_int); }
      }
    if top_p
      {
	if !right_p
	  {
	    x_fill_rectangle (f, top_left_gc, left_x, top_y,
			      right_x + 1 - left_x, hwidth, false);
	  }
	else
	  {
	    x_fill_trapezoid_for_relief (f, top_left_gc, left_x, top_y,
					 right_x + 1 - left_x, hwidth, 1);
	  }
      }
    if bot_p
      {
	if !left_p
	  {
	    x_fill_rectangle (f, bottom_right_gc, left_x, bottom_y + 1 - hwidth,
			      right_x + 1 - left_x, hwidth, false);
	  }
	else
	  {
	    x_fill_trapezoid_for_relief (f, bottom_right_gc,
					 left_x, bottom_y + 1 - hwidth,
					 right_x + 1 - left_x, hwidth, 0);
	  }
      }
    if left_p && vwidth > 1
      {
	x_fill_rectangle (f, bottom_right_gc, left_x, top_y,
			  1, bottom_y + 1 - top_y, false);
      }
    if top_p && hwidth > 1
      {
	x_fill_rectangle (f, bottom_right_gc, left_x, top_y,
			  right_x + 1 - left_x, 1, false);
      }
    if corners != 0
      {
	XSetBackground (FRAME_X_DISPLAY! (f), top_left_gc,
			FRAME_BACKGROUND_PIXEL (f));
	x_erase_corners_for_relief (f, top_left_gc, left_x, top_y,
				    right_x - left_x + 1, bottom_y - top_y + 1,
				    6 as c_double, 1 as c_double, corners);
      }

    x_reset_clip_rectangles (f, top_left_gc);
    x_reset_clip_rectangles (f, bottom_right_gc);
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    let mut gc: GC;
    let mut white_gc: GC;
    let mut black_gc: GC;
    let mut normal_gc: GC;
    let mut drawable: Drawable;
    let mut dpy: *mut Display;

    /* This code is more complicated than it has to be, because of two
       minor hacks to make the boxes look nicer: (i) if width > 1, draw
       the outermost line using the black relief.  (ii) Omit the four
       corner pixels.  */

    white_gc = (*( (*f).output_data.x as *mut x_output )).white_relief.gc;
    black_gc = (*( (*f).output_data.x as *mut x_output )).black_relief.gc;
    normal_gc = (*( (*f).output_data.x as *mut x_output )).normal_gc;

    drawable = FRAME_X_DRAWABLE! (f);
    dpy = FRAME_X_DISPLAY! (f);

    x_set_clip_rectangles (f, white_gc, clip_rect, 1);
    x_set_clip_rectangles (f, black_gc, clip_rect, 1);

    if raised_p
      { gc = white_gc; }
    else
      { gc = black_gc; }

    /* Draw lines.  */

    if top_p
      {
	x_fill_rectangle (f, gc, left_x, top_y,
			  right_x - left_x + 1, hwidth, false);
      }

    if left_p
      {
	x_fill_rectangle (f, gc, left_x, top_y, vwidth,
			  bottom_y - top_y + 1, false);
      }

    if raised_p
      { gc = black_gc; }
    else
      { gc = white_gc; }

    if bot_p
      {
	x_fill_rectangle (f, gc, left_x, bottom_y - hwidth + 1,
			  right_x - left_x + 1, hwidth, false);
      }

    if right_p
      {
	x_fill_rectangle (f, gc, right_x - vwidth + 1, top_y,
			  vwidth, bottom_y - top_y + 1, false);
      }

    /* Draw corners.  */

    if bot_p && left_p
      {
	x_fill_triangle (f, if raised_p { white_gc } else { black_gc },
			 x_make_point (left_x, bottom_y - hwidth),
			 x_make_point (left_x + vwidth, bottom_y - hwidth),
			 x_make_point (left_x, bottom_y));
      }

    if top_p && right_p
      {
	x_fill_triangle (f, if raised_p { white_gc } else { black_gc },
			 x_make_point (right_x - vwidth, top_y),
			 x_make_point (right_x, top_y),
			 x_make_point (right_x - vwidth, top_y + hwidth));
      }

    /* Draw outer line.  */

    if top_p && left_p && bot_p && right_p
	&& hwidth > 1 && vwidth > 1
      {
	x_draw_rectangle (f, black_gc, left_x, top_y,
			  right_x - left_x, bottom_y - top_y);
      }
    else
      {
	if top_p && hwidth > 1
	  {
	    XDrawLine (dpy, drawable, black_gc, left_x, top_y,
		       right_x + 1, top_y);
	  }

	if bot_p && hwidth > 1
	  {
	    XDrawLine (dpy, drawable, black_gc, left_x, bottom_y,
		       right_x + 1, bottom_y);
	  }

	if left_p && vwidth > 1
	  {
	    XDrawLine (dpy, drawable, black_gc, left_x, top_y,
		       left_x, bottom_y + 1);
	  }

	if right_p && vwidth > 1
	  {
	    XDrawLine (dpy, drawable, black_gc, right_x, top_y,
		       right_x, bottom_y + 1);
	  }
      }

    /* Erase corners.  */

    if hwidth > 1 && vwidth > 1
      {
	if left_p && top_p && x_inside_rect_p (clip_rect, 1, left_x, top_y)
	  {
	    /* This should respect `alpha-background' since it's being
	       cleared with the background color of the frame.  */
	    x_clear_point (f, normal_gc, left_x, top_y, true);
	  }

	if left_p && bot_p && x_inside_rect_p (clip_rect, 1, left_x, bottom_y)
	  { x_clear_point (f, normal_gc, left_x, bottom_y, true); }

	if right_p && top_p && x_inside_rect_p (clip_rect, 1, right_x, top_y)
	  { x_clear_point (f, normal_gc, right_x, top_y, true); }

	if right_p && bot_p && x_inside_rect_p (clip_rect, 1, right_x, bottom_y)
	  { x_clear_point (f, normal_gc, right_x, bottom_y, true); }
      }

    x_reset_clip_rectangles (f, white_gc);
    x_reset_clip_rectangles (f, black_gc);
  }
}

/* C 9990-10034: `x_draw_box_rect`.  C's `XGCValues xgcv` is filled by `XGetGCValues` before it
   is read, so the port zero-initialises it (PORTING.md 1.5); `s->gc` is the mirror's
   `*mut c_void` member and every use casts it to `GC` (the `w32term.rs` 1869 shape).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_box_rect (s: *mut glyph_string, left_x: c_int, top_y: c_int, right_x: c_int,
			   bottom_y: c_int, hwidth: c_int, vwidth: c_int, left_p: bool,
			   right_p: bool, clip_rect: *mut XRectangle)
{
  use crate::x11_constants::GCForeground;

  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
  let mut xgcv: XGCValues = std::mem::zeroed ();

  XGetGCValues (display, (*s).gc as GC, GCForeground, &raw mut xgcv);
  XSetForeground (display, (*s).gc as GC, (*((*s).face as *mut face)).box_color);
  x_set_clip_rectangles ((*s).f, (*s).gc as GC, clip_rect, 1);

  /* Top.  */
  x_fill_rectangle ((*s).f, (*s).gc as GC,
		    left_x, top_y, right_x - left_x + 1, hwidth,
		    false);

  /* Left.  */
  if left_p
    {
      x_fill_rectangle ((*s).f, (*s).gc as GC,
			left_x, top_y, vwidth, bottom_y - top_y + 1,
			false);
    }

  /* Bottom.  */
  x_fill_rectangle ((*s).f, (*s).gc as GC,
		    left_x, bottom_y - hwidth + 1, right_x - left_x + 1, hwidth,
		    false);

  /* Right.  */
  if right_p
    {
      x_fill_rectangle ((*s).f, (*s).gc as GC,
			right_x - vwidth + 1, top_y, vwidth, bottom_y - top_y + 1,
			false);
    }

  XSetForeground (display, (*s).gc as GC, xgcv.foreground);
  x_reset_clip_rectangles ((*s).f, (*s).gc as GC);
}

/* C 10036-10103: `x_draw_glyph_string_box`.  The right-box glyph is looked up the way C's `for`
   does it (`last_glyph = g++` after the empty body, so the port keeps C's two steps); the
   glyph-row members `full_width_p` go through the mirror's `glyph_row_bits!`
   (`include/dispextern.rs` 1107) and the window's `pseudo_window_p` through the `window_bits!`
   this unit's earlier text already calls (the mirror does not carry it - the ledger's class);
   `WINDOW_RIGHT_EDGE_X` is the mirror's macro.  `min` is lisp.h's `EMACS_INT` pair, so both
   operands widen and the result narrows (the `x_alloc_lighter_color` shape), and C's ternary is
   the port's `if`/`else` expression.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_glyph_string_box (s: *mut glyph_string)
{
  let mut hwidth: c_int;
  let mut vwidth: c_int;
  let mut left_x: c_int;
  let mut right_x: c_int;
  let mut top_y: c_int;
  let mut bottom_y: c_int;
  let mut last_x: c_int;
  let mut raised_p: bool;
  let mut left_p: bool;
  let mut right_p: bool;
  let mut last_glyph: *mut glyph;
  let mut clip_rect: XRectangle = std::mem::zeroed ();
  let face: *mut face = (*s).face as *mut face;

  last_x = if glyph_row_bits! (full_width_p, (*s).row)
	      && !window_bits! (pseudo_window_p, (*s).w)
    { WINDOW_RIGHT_EDGE_X! ((*s).w) }
  else
    { window_box_right ((*s).w, (*s).area) };

  /* The glyph that may have a right box line.  For static
     compositions and images, the right-box flag is on the first glyph
     of the glyph string; for other types it's on the last glyph.  */
  if !(*s).cmp.is_null () || !(*s).img.is_null ()
    { last_glyph = (*s).first_glyph; }
  else if glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH
	   && glyph_u_cmp_bits! (automatic, (*s).first_glyph)
    {
      /* For automatic compositions, we need to look up the last glyph
	 in the composition.  */
      let end: *mut glyph = (*((*s).row)).glyphs[(*s).area as usize]
	.offset ((*((*s).row)).used[(*s).area as usize] as isize);
      let mut g: *mut glyph = (*s).first_glyph;

      last_glyph = g;
      g = g.offset (1);
      while g < end && glyph_u_cmp_bits! (automatic, g)
	    && glyph_u_cmp_bits! (id, g) == (*s).cmp_id
	    && (*g).slice.cmp.to < (*s).cmp_to
	{
	  last_glyph = g;
	  g = g.offset (1);
	}
    }
  else
    { last_glyph = (*s).first_glyph.offset ((*s).nchars as isize - 1); }

  vwidth = eabs! ((*face).box_vertical_line_width);
  hwidth = eabs! ((*face).box_horizontal_line_width);
  raised_p = face_bits! (box, face) == FACE_RAISED_BOX;
  left_x = (*s).x;
  right_x = if glyph_row_bits! (full_width_p, (*s).row)
	       && glyph_string_bits! (extends_to_end_of_line_p, (*s))
    { last_x - 1 }
  else
    { min (last_x as isize, ((*s).x + (*s).background_width) as isize) as c_int - 1 };
  top_y = (*s).y;
  bottom_y = top_y + (*s).height - 1;

  left_p = glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    || ((*s).hl == DRAW_MOUSE_FACE
	&& ((*s).prev.is_null ()
	    || (*((*s).prev)).hl != (*s).hl));
  right_p = glyph_bits! (right_box_line_p, (*last_glyph))
    || ((*s).hl == DRAW_MOUSE_FACE
	&& ((*s).next.is_null ()
	    || (*((*s).next)).hl != (*s).hl));

  get_glyph_string_clip_rect (s, (&raw mut clip_rect) as *mut c_void);

  if face_bits! (box, face) == FACE_SIMPLE_BOX
    {
      x_draw_box_rect (s, left_x, top_y, right_x, bottom_y, hwidth,
		       vwidth, left_p, right_p, &raw mut clip_rect);
    }
  else
    {
      x_setup_relief_colors (s);
      x_draw_relief_rect ((*s).f, left_x, top_y, right_x, bottom_y, hwidth,
			  vwidth, raised_p, true, true, left_p, right_p,
			  &raw mut clip_rect);
    }
}

/* C 10105-10160: `#ifndef USE_CAIRO` - `x_composite_image`.  C's `#ifdef HAVE_XRENDER` adds the
   `Picture destination` parameter *and* the two locals, so the port carries one definition per
   arm (PORTING.md 2.1; `extract-abi.py`'s `bodies_all` compares the arm the configuration
   selects).  `s->img->picture`/`mask_picture` are the image mirror's `*mut c_void` members (the
   `picture` pair is gated `have_xrender`), so each is unwrapped before use, and `destination ==
   None` is C's pointer compare with the mirror's `None` constant (the `x_free_colors` sites'
   `None as Picture` spelling).  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "have_xrender"))]
unsafe fn x_composite_image (s: *mut glyph_string, dest: Pixmap, destination: Picture,
			     srcX: c_int, srcY: c_int, dstX: c_int, dstY: c_int,
			     width: c_int, height: c_int)
{
  use crate::x11_constants::None;

  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

  let mut default_format: *mut XRenderPictFormat = std::ptr::null_mut ();
  let mut attr: XRenderPictureAttributes = std::mem::zeroed ();

  if !(*((*s).img)).picture.is_null () && !FRAME_X_PICTURE_FORMAT! ((*s).f).is_null ()
    {
      if destination == None as Picture
	{
	  /* The destination picture was not specified.  This means we
	     have to create a picture representing the */
	  default_format = FRAME_X_PICTURE_FORMAT! ((*s).f);
	  destination = XRenderCreatePicture (display, dest,
					      default_format, 0, &raw mut attr);

	  XRenderComposite (display, if !(*((*s).img)).mask_picture.is_null ()
				      { PictOpOver } else { PictOpSrc },
			    (*((*s).img)).picture, (*((*s).img)).mask_picture,
			    destination, srcX, srcY, srcX, srcY,
			    dstX, dstY, width, height);

	  XRenderFreePicture (display, destination);
	}
      else
	{
	  XRenderComposite (display, if !(*((*s).img)).mask_picture.is_null ()
				      { PictOpOver } else { PictOpSrc },
			    (*((*s).img)).picture, (*((*s).img)).mask_picture,
			    destination, srcX, srcY, srcX, srcY,
			    dstX, dstY, width, height);
	}

      return;
    }

  XCopyArea (display, (*((*s).img)).pixmap,
	     dest, (*s).gc as GC,
	     srcX, srcY,
	     width, height, dstX, dstY);
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo"),
	    not (feature = "have_xrender")))]
unsafe fn x_composite_image (s: *mut glyph_string, dest: Pixmap,
			     srcX: c_int, srcY: c_int, dstX: c_int, dstY: c_int,
			     width: c_int, height: c_int)
{
  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

  XCopyArea (display, (*((*s).img)).pixmap,
	     dest, (*s).gc as GC,
	     srcX, srcY,
	     width, height, dstX, dstY);
}

/* C 10162-10283: `x_draw_image_foreground`.  `image_ascent` is dispextern.h's (this unit's
   slice-9 text declares nothing for it; the `w32term.rs` 3125-3128 use-site form is the
   shape and the ledger owes the mirror row), `max` is lisp.h's `EMACS_INT` pair, and the
   slice coordinates go through the mirror's `glyph_slice_bits!` (`glyph_slice` stores them as
   four 16-bit members).  The `#ifndef USE_CAIRO` arm sets the GC's clip mask and then composes
   through `x_composite_image`; C's `s->img->pixmap` null test is the pointer test its type
   asks for.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_image_foreground (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let mut x: c_int = (*s).x;
  let mut y: c_int = (*s).ybase - image_ascent ((*s).img, face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
      && glyph_slice_bits! (x, (*s).slice) == 0
    { x += max ((*face).box_vertical_line_width as isize, 0) as c_int; }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0
    { x += (*((*s).img)).hmargin; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y += (*((*s).img)).vmargin; }

  #[cfg (feature = "use_cairo")]
  {
    if !(*((*s).img)).cr_data.is_null ()
      {
	x_set_glyph_string_clipping (s);
	x_cr_draw_image ((*s).f, (*s).gc as GC,
			 (*((*s).img)).cr_data as *mut cairo_pattern_t,
			 glyph_slice_bits! (x, (*s).slice) as c_int,
			 glyph_slice_bits! (y, (*s).slice) as c_int,
			 glyph_slice_bits! (width, (*s).slice) as c_int,
			 glyph_slice_bits! (height, (*s).slice) as c_int,
			 x, y, true);
	if (*((*s).img)).mask.is_null ()
	  {
	    /* When the image has a mask, we can expect that at
	       least part of a mouse highlight or a block cursor will
	       be visible.  If the image doesn't have a mask, make
	       a block cursor visible by drawing a rectangle around
	       the image.  I believe it's looking better if we do
	       nothing here for mouse-face.  */
	    if (*s).hl == DRAW_CURSOR
	      {
		let relief: c_int = eabs! ((*((*s).img)).relief);
		x_draw_rectangle ((*s).f, (*s).gc as GC, x - relief, y - relief,
				  glyph_slice_bits! (width, (*s).slice) as c_int + relief * 2 - 1,
				  glyph_slice_bits! (height, (*s).slice) as c_int + relief * 2 - 1);
	      }
	  }
      }
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    if !(*((*s).img)).pixmap.is_null ()
      {
	if !(*((*s).img)).mask.is_null ()
	  {
	    /* We can't set both a clip mask and use XSetClipRectangles
	       because the latter also sets a clip mask.  We also can't
	       trust on the shape extension to be available
	       (XShapeCombineRegion).  So, compute the rectangle to draw
	       manually.  */
	    let mask: c_ulong = (GCClipMask | GCClipXOrigin | GCClipYOrigin
				 | GCFunction) as c_ulong;
	    let mut xgcv: XGCValues = std::mem::zeroed ();
	    let mut clip_rect: XRectangle = std::mem::zeroed ();
	    let mut image_rect: XRectangle = std::mem::zeroed ();
	    let mut r: XRectangle = std::mem::zeroed ();

	    xgcv.clip_mask = (*((*s).img)).mask as Pixmap;
	    xgcv.clip_x_origin = x;
	    xgcv.clip_y_origin = y;
	    xgcv.function = GXcopy;
	    XChangeGC (FRAME_X_DISPLAY! ((*s).f), (*s).gc as GC, mask,
		       &raw mut xgcv as *mut c_void);

	    get_glyph_string_clip_rect (s, (&raw mut clip_rect) as *mut c_void);
	    image_rect.x = x as c_short;
	    image_rect.y = y as c_short;
	    image_rect.width = glyph_slice_bits! (width, (*s).slice) as c_ushort;
	    image_rect.height = glyph_slice_bits! (height, (*s).slice) as c_ushort;
	    /* C 10248-10252's `#ifdef HAVE_XRENDER` adds `FRAME_X_PICTURE (s->f)` to the
	       call, so the port carries one arm per configuration (the `ftfont.rs` 2552
	       shape); both arms keep C's argument list.  */
	    #[cfg (feature = "have_xrender")]
	    {
	      if gui_intersect_rectangles (&clip_rect, &image_rect, &mut r)
		{
		  x_composite_image (s, FRAME_X_DRAWABLE! ((*s).f),
				     FRAME_X_PICTURE! ((*s).f),
				     glyph_slice_bits! (x, (*s).slice) as c_int + r.x as c_int - x,
				     glyph_slice_bits! (y, (*s).slice) as c_int + r.y as c_int - y,
				     r.x as c_int, r.y as c_int,
				     r.width as c_int, r.height as c_int);
		}
	    }
	    #[cfg (not (feature = "have_xrender"))]
	    {
	      if gui_intersect_rectangles (&clip_rect, &image_rect, &mut r)
		{
		  x_composite_image (s, FRAME_X_DRAWABLE! ((*s).f),
				     glyph_slice_bits! (x, (*s).slice) as c_int + r.x as c_int - x,
				     glyph_slice_bits! (y, (*s).slice) as c_int + r.y as c_int - y,
				     r.x as c_int, r.y as c_int,
				     r.width as c_int, r.height as c_int);
		}
	    }
	  }
	else
	  {
	    let mut clip_rect: XRectangle = std::mem::zeroed ();
	    let mut image_rect: XRectangle = std::mem::zeroed ();
	    let mut r: XRectangle = std::mem::zeroed ();

	    get_glyph_string_clip_rect (s, (&raw mut clip_rect) as *mut c_void);
	    image_rect.x = x as c_short;
	    image_rect.y = y as c_short;
	    image_rect.width = glyph_slice_bits! (width, (*s).slice) as c_ushort;
	    image_rect.height = glyph_slice_bits! (height, (*s).slice) as c_ushort;
	    /* C 10248-10252's `#ifdef HAVE_XRENDER` adds `FRAME_X_PICTURE (s->f)` to the
	       call, so the port carries one arm per configuration (the `ftfont.rs` 2552
	       shape); both arms keep C's argument list.  */
	    #[cfg (feature = "have_xrender")]
	    {
	      if gui_intersect_rectangles (&clip_rect, &image_rect, &mut r)
		{
		  x_composite_image (s, FRAME_X_DRAWABLE! ((*s).f),
				     FRAME_X_PICTURE! ((*s).f),
				     glyph_slice_bits! (x, (*s).slice) as c_int + r.x as c_int - x,
				     glyph_slice_bits! (y, (*s).slice) as c_int + r.y as c_int - y,
				     r.x as c_int, r.y as c_int,
				     r.width as c_int, r.height as c_int);
		}
	    }
	    #[cfg (not (feature = "have_xrender"))]
	    {
	      if gui_intersect_rectangles (&clip_rect, &image_rect, &mut r)
		{
		  x_composite_image (s, FRAME_X_DRAWABLE! ((*s).f),
				     glyph_slice_bits! (x, (*s).slice) as c_int + r.x as c_int - x,
				     glyph_slice_bits! (y, (*s).slice) as c_int + r.y as c_int - y,
				     r.x as c_int, r.y as c_int,
				     r.width as c_int, r.height as c_int);
		}
	    }

	    /* When the image has a mask, we can expect that at
	       least part of a mouse highlight or a block cursor will
	       be visible.  If the image doesn't have a mask, make
	       a block cursor visible by drawing a rectangle around
	       the image.  I believe it's looking better if we do
	       nothing here for mouse-face.  */
	    if (*s).hl == DRAW_CURSOR
	      {
		let relief: c_int = eabs! ((*((*s).img)).relief);
		x_draw_rectangle ((*s).f, (*s).gc as GC,
				  x - relief, y - relief,
				  glyph_slice_bits! (width, (*s).slice) as c_int + relief * 2 - 1,
				  glyph_slice_bits! (height, (*s).slice) as c_int + relief * 2 - 1);
	      }
	  }
      }
    else
      {
	/* Draw a rectangle if image could not be loaded.  */
	x_draw_rectangle ((*s).f, (*s).gc as GC, x, y,
			  glyph_slice_bits! (width, (*s).slice) as c_int - 1,
			  glyph_slice_bits! (height, (*s).slice) as c_int - 1);
      }
  }
}

/* C 10285-10376: `x_draw_image_relief`.  `image_ascent` is the dispextern.h prototype the
   `x_draw_image_foreground` note above records; `tab_bar_button_relief`/`tool_bar_button_relief`
   are `EMACS_INT` globals (`include/globals.rs` 7121/7124) and lisp.h's `min` is the `EMACS_INT`
   pair, so C's implicit `EMACS_INT` -> `int` conversion is written at each store.  The two
   `V*_button_margin` values are `Lisp_Object` globals and go through `CONSP`/`FIXNUMP`/`XCAR`/
   `XCDR`/`XFIXNUM`; the `Q*` names stay calls because the mirror defines them as functions
   (P-17).  C's `x -= thick + extra_x, left_p = true;` is the comma operator, so the port puts
   the two stores on their own lines (the `magic_db` row's class).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_image_relief (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let mut x1: c_int;
  let mut y1: c_int;
  let mut thick: c_int;
  let mut raised_p: bool;
  let mut top_p: bool;
  let mut bot_p: bool;
  let mut left_p: bool;
  let mut right_p: bool;
  let mut extra_x: c_int;
  let mut extra_y: c_int;
  let mut r: XRectangle = std::mem::zeroed ();
  let mut x: c_int = (*s).x;
  let mut y: c_int = (*s).ybase - image_ascent ((*s).img, face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
      && glyph_slice_bits! (x, (*s).slice) == 0
    { x += max ((*face).box_vertical_line_width as isize, 0) as c_int; }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0
    { x += (*((*s).img)).hmargin; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y += (*((*s).img)).vmargin; }

  if (*s).hl == DRAW_IMAGE_SUNKEN
      || (*s).hl == DRAW_IMAGE_RAISED
    {
      if (*face).id == TAB_BAR_FACE_ID
	{
	  thick = if tab_bar_button_relief < 0
	    { DEFAULT_TAB_BAR_BUTTON_RELIEF }
	  else
	    { min (tab_bar_button_relief, 1000000) as c_int };
	}
      else
	{
	  thick = if tool_bar_button_relief < 0
	    { DEFAULT_TOOL_BAR_BUTTON_RELIEF }
	  else
	    { min (tool_bar_button_relief, 1000000) as c_int };
	}
      raised_p = (*s).hl == DRAW_IMAGE_RAISED;
    }
  else
    {
      thick = eabs! ((*((*s).img)).relief);
      raised_p = (*((*s).img)).relief > 0;
    }

  x1 = x + glyph_slice_bits! (width, (*s).slice) as c_int - 1;
  y1 = y + glyph_slice_bits! (height, (*s).slice) as c_int - 1;

  extra_x = 0;
  extra_y = 0;
  if (*face).id == TAB_BAR_FACE_ID
    {
      if CONSP (Vtab_bar_button_margin)
	  && FIXNUMP (XCAR (Vtab_bar_button_margin))
	  && FIXNUMP (XCDR (Vtab_bar_button_margin))
	{
	  extra_x = XFIXNUM (XCAR (Vtab_bar_button_margin)) as c_int - thick;
	  extra_y = XFIXNUM (XCDR (Vtab_bar_button_margin)) as c_int - thick;
	}
      else if FIXNUMP (Vtab_bar_button_margin)
	{
	  extra_y = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;
	  extra_x = extra_y;
	}
    }

  if (*face).id == TOOL_BAR_FACE_ID
    {
      if CONSP (Vtool_bar_button_margin)
	  && FIXNUMP (XCAR (Vtool_bar_button_margin))
	  && FIXNUMP (XCDR (Vtool_bar_button_margin))
	{
	  extra_x = XFIXNUM (XCAR (Vtool_bar_button_margin)) as c_int;
	  extra_y = XFIXNUM (XCDR (Vtool_bar_button_margin)) as c_int;
	}
      else if FIXNUMP (Vtool_bar_button_margin)
	{
	  extra_y = XFIXNUM (Vtool_bar_button_margin) as c_int;
	  extra_x = extra_y;
	}
    }

  top_p = false;
  bot_p = false;
  left_p = false;
  right_p = false;

  if glyph_slice_bits! (x, (*s).slice) == 0
    { x -= thick + extra_x; left_p = true; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y -= thick + extra_y; top_p = true; }
  if glyph_slice_bits! (x, (*s).slice) as c_int + glyph_slice_bits! (width, (*s).slice) as c_int
       == (*((*s).img)).width
    { x1 += thick + extra_x; right_p = true; }
  if glyph_slice_bits! (y, (*s).slice) as c_int + glyph_slice_bits! (height, (*s).slice) as c_int
       == (*((*s).img)).height
    { y1 += thick + extra_y; bot_p = true; }

  x_setup_relief_colors (s);
  get_glyph_string_clip_rect (s, (&raw mut r) as *mut c_void);
  x_draw_relief_rect ((*s).f, x, y, x1, y1, thick, thick, raised_p,
		      top_p, bot_p, left_p, right_p, &raw mut r);
}

/* C 10378-10458: `#ifndef USE_CAIRO` - `x_draw_image_foreground_1`.  `pixmap` is the caller's
   temporary; the `#ifdef HAVE_XRENDER` adds `None` to the `x_composite_image` call, so that one
   call is carried per arm (the `ftfont.rs` 2552 shape).  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe fn x_draw_image_foreground_1 (s: *mut glyph_string, pixmap: Pixmap)
{
  use crate::x11_constants::None;

  let face: *mut face = (*s).face as *mut face;
  let mut x: c_int = 0;
  let mut y: c_int = (*s).ybase - (*s).y - image_ascent ((*s).img, face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
      && glyph_slice_bits! (x, (*s).slice) == 0
    { x += max ((*face).box_vertical_line_width as isize, 0) as c_int; }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0
    { x += (*((*s).img)).hmargin; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y += (*((*s).img)).vmargin; }

  if !(*((*s).img)).pixmap.is_null ()
    {
      let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

      if !(*((*s).img)).mask.is_null ()
	{
	  /* We can't set both a clip mask and use XSetClipRectangles
	     because the latter also sets a clip mask.  We also can't
	     trust on the shape extension to be available
	     (XShapeCombineRegion).  So, compute the rectangle to draw
	     manually.  */
	  let mask: c_ulong = (GCClipMask | GCClipXOrigin | GCClipYOrigin
			       | GCFunction) as c_ulong;
	  let mut xgcv: XGCValues = std::mem::zeroed ();

	  xgcv.clip_mask = (*((*s).img)).mask as Pixmap;
	  xgcv.clip_x_origin = x - glyph_slice_bits! (x, (*s).slice) as c_int;
	  xgcv.clip_y_origin = y - glyph_slice_bits! (y, (*s).slice) as c_int;
	  xgcv.function = GXcopy;
	  XChangeGC (display, (*s).gc as GC, mask, &raw mut xgcv as *mut c_void);

	  #[cfg (feature = "have_xrender")]
	  {
	    x_composite_image (s, pixmap,
			       None as Picture,
			       glyph_slice_bits! (x, (*s).slice) as c_int,
			       glyph_slice_bits! (y, (*s).slice) as c_int, x, y,
			       glyph_slice_bits! (width, (*s).slice) as c_int,
			       glyph_slice_bits! (height, (*s).slice) as c_int);
	  }
	  #[cfg (not (feature = "have_xrender"))]
	  {
	    x_composite_image (s, pixmap,
			       glyph_slice_bits! (x, (*s).slice) as c_int,
			       glyph_slice_bits! (y, (*s).slice) as c_int, x, y,
			       glyph_slice_bits! (width, (*s).slice) as c_int,
			       glyph_slice_bits! (height, (*s).slice) as c_int);
	  }
	  XSetClipMask (display, (*s).gc as GC, None as Pixmap);
	}
      else
	{
	  XCopyArea (display, (*((*s).img)).pixmap, pixmap, (*s).gc as GC,
		     glyph_slice_bits! (x, (*s).slice) as c_int,
		     glyph_slice_bits! (y, (*s).slice) as c_int,
		     glyph_slice_bits! (width, (*s).slice) as c_int,
		     glyph_slice_bits! (height, (*s).slice) as c_int, x, y);

	  /* When the image has a mask, we can expect that at
	     least part of a mouse highlight or a block cursor will
	     be visible.  If the image doesn't have a mask, make
	     a block cursor visible by drawing a rectangle around
	     the image.  I believe it's looking better if we do
	     nothing here for mouse-face.  */
	  if (*s).hl == DRAW_CURSOR
	    {
	      let r: c_int = eabs! ((*((*s).img)).relief);
	      x_draw_rectangle ((*s).f, (*s).gc as GC, x - r, y - r,
				glyph_slice_bits! (width, (*s).slice) as c_int + r * 2 - 1,
				glyph_slice_bits! (height, (*s).slice) as c_int + r * 2 - 1);
	    }
	}
    }
  else
    {
      /* Draw a rectangle if image could not be loaded.  */
      x_draw_rectangle ((*s).f, (*s).gc as GC, x, y,
			glyph_slice_bits! (width, (*s).slice) as c_int - 1,
			glyph_slice_bits! (height, (*s).slice) as c_int - 1);
    }
}

/* C 10460-10478: `x_draw_glyph_string_bg_rect`.  The two `XSetFillStyle` calls are Xlib's (no
   mirror row; the ledger's class).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_glyph_string_bg_rect (s: *mut glyph_string, x: c_int, y: c_int,
				       w: c_int, h: c_int)
{
  if glyph_string_bits! (stippled_p, (*s))
    {
      let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);

      /* Fill background with a stipple pattern.  */
      XSetFillStyle (display, (*s).gc as GC, FillOpaqueStippled);
      x_fill_rectangle ((*s).f, (*s).gc as GC, x, y, w, h, true);
      XSetFillStyle (display, (*s).gc as GC, FillSolid);
    }
  else
    { x_clear_glyph_string_rect (s, x, y, w, h); }
}

/* C 10480-10630: `x_draw_image_glyph_string`.  C's `#ifndef USE_CAIRO` block declares the two
   locals and the whole `s->img->mask` arm, so the port carries the mask arm under
   `not (use_cairo)` and the shared `else` arm under both (its own local declarations in each
   copy); the inner `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 || RENDER_MINOR >= 2)` is
   `all (feature = "have_xrender", feature = "have_xrender_0_2")`, whose `else` C writes outside
   the `#if` and the port therefore also duplicates.  `s->stippled_p`/`background_filled_p` are
   bitfields (the `glyph_string_bits!` setters) and `s->row->stipple_p` likewise
   (`glyph_row_bits!`).  The `memset` is this unit's own line 8560 shape.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_image_glyph_string (s: *mut glyph_string)
{
  use crate::x11_constants::{GCBackground, GCForeground, None};

  let face: *mut face = (*s).face as *mut face;
  let mut box_line_hwidth: c_int = max ((*face).box_vertical_line_width as isize, 0) as c_int;
  let mut box_line_vwidth: c_int = max ((*face).box_horizontal_line_width as isize, 0) as c_int;
  let mut height: c_int;
  #[cfg (not (feature = "use_cairo"))]
  let mut display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
  #[cfg (not (feature = "use_cairo"))]
  let mut pixmap: Pixmap = None as Pixmap;

  height = (*s).height;
  if glyph_slice_bits! (y, (*s).slice) == 0
    { height -= box_line_vwidth; }
  if glyph_slice_bits! (y, (*s).slice) as c_int + glyph_slice_bits! (height, (*s).slice) as c_int
       >= (*((*s).img)).height
    { height -= box_line_vwidth; }

  /* Fill background with face under the image.  Do it only if row is
     taller than image or if image has a clip mask to reduce
     flickering.  */
  glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
  if height > glyph_slice_bits! (height, (*s).slice) as c_int
      || (*((*s).img)).hmargin != 0
      || (*((*s).img)).vmargin != 0
      || !(*((*s).img)).mask.is_null ()
      || (*((*s).img)).pixmap.is_null ()
      || (*s).width != (*s).background_width
    {
      if glyph_string_bits! (stippled_p, (*s))
	{ glyph_row_bits! (stipple_p, (*s).row, true); }

      #[cfg (not (feature = "use_cairo"))]
      {
	if !(*((*s).img)).mask.is_null ()
	  {
	    /* Create a pixmap as large as the glyph string.  Fill it
	       with the background color.  Copy the image to it, using
	       its mask.  Copy the temporary pixmap to the display.  */
	    let depth: c_int = (*FRAME_DISPLAY_INFO! ((*s).f)).n_planes;

	    /* Create a pixmap as large as the glyph string.  */
	    pixmap = XCreatePixmap (display, FRAME_X_DRAWABLE! ((*s).f),
				    (*s).background_width,
				    (*s).height, depth);

	    /* Don't clip in the following because we're working on the
	       pixmap.  */
	    XSetClipMask (display, (*s).gc as GC, None as Pixmap);

	    /* Fill the pixmap with the background color/stipple.  */
	    if glyph_string_bits! (stippled_p, (*s))
	      {
		/* Fill background with a stipple pattern.  */
		XSetFillStyle (display, (*s).gc as GC, FillOpaqueStippled);
		XSetTSOrigin (display, (*s).gc as GC, - (*s).x, - (*s).y);
		XFillRectangle (display, pixmap, (*s).gc as GC,
				0, 0, (*s).background_width, (*s).height);
		XSetFillStyle (display, (*s).gc as GC, FillSolid);
		XSetTSOrigin (display, (*s).gc as GC, 0, 0);
	      }
	    else
	      {
		let mut xgcv: XGCValues = std::mem::zeroed ();

		#[cfg (all (feature = "have_xrender", feature = "have_xrender_0_2"))]
		{
		  if (*FRAME_DISPLAY_INFO! ((*s).f)).alpha_bits != 0
		      && (*(*s).f).alpha_background != 1.0
		      && FRAME_CHECK_XR_VERSION! ((*s).f, 0, 2)
		      && !FRAME_X_PICTURE_FORMAT! ((*s).f).is_null ()
		    {
		      let mut xc: XRenderColor = std::mem::zeroed ();
		      let mut attrs: XRenderPictureAttributes = std::mem::zeroed ();
		      let mut pict: Picture;
		      memset (&raw mut attrs as *mut c_void, 0, std::mem::size_of_val (&attrs));

		      pict = XRenderCreatePicture (display, pixmap,
						   FRAME_X_PICTURE_FORMAT! ((*s).f),
						   0, &raw mut attrs);
		      x_xrender_color_from_gc_background ((*s).f, (*s).gc as GC, &raw mut xc, true);
		      XRenderFillRectangle (FRAME_X_DISPLAY! ((*s).f), PictOpSrc, pict,
					    &raw mut xc, 0, 0, (*s).background_width, (*s).height);
		      XRenderFreePicture (display, pict);
		    }
		  else
		    {
		      XGetGCValues (display, (*s).gc as GC, GCForeground | GCBackground,
				    &raw mut xgcv);
		      XSetForeground (display, (*s).gc as GC, xgcv.background);
		      XFillRectangle (display, pixmap, (*s).gc as GC,
				      0, 0, (*s).background_width, (*s).height);
		      XSetForeground (display, (*s).gc as GC, xgcv.foreground);
		    }
		}
		#[cfg (not (all (feature = "have_xrender", feature = "have_xrender_0_2")))]
		{
		  XGetGCValues (display, (*s).gc as GC, GCForeground | GCBackground,
				&raw mut xgcv);
		  XSetForeground (display, (*s).gc as GC, xgcv.background);
		  XFillRectangle (display, pixmap, (*s).gc as GC,
				  0, 0, (*s).background_width, (*s).height);
		  XSetForeground (display, (*s).gc as GC, xgcv.foreground);
		}
	      }
	  }
	else
	  {
	    let mut x: c_int = (*s).x;
	    let mut y: c_int = (*s).y;
	    let mut width: c_int = (*s).background_width;

	    if glyph_bits! (left_box_line_p, (*(*s).first_glyph))
		&& glyph_slice_bits! (x, (*s).slice) == 0
	      {
		x += box_line_hwidth;
		width -= box_line_hwidth;
	      }

	    if glyph_slice_bits! (y, (*s).slice) == 0
	      { y += box_line_vwidth; }

	    x_draw_glyph_string_bg_rect (s, x, y, width, height);
	  }
      }

      #[cfg (feature = "use_cairo")]
      {
	let mut x: c_int = (*s).x;
	let mut y: c_int = (*s).y;
	let mut width: c_int = (*s).background_width;

	if glyph_bits! (left_box_line_p, (*(*s).first_glyph))
	    && glyph_slice_bits! (x, (*s).slice) == 0
	  {
	    x += box_line_hwidth;
	    width -= box_line_hwidth;
	  }

	if glyph_slice_bits! (y, (*s).slice) == 0
	  { y += box_line_vwidth; }

	x_draw_glyph_string_bg_rect (s, x, y, width, height);
      }

      glyph_string_bits! (background_filled_p, (*s), true);
    }

  /* Draw the foreground.  */
  #[cfg (not (feature = "use_cairo"))]
  {
    if pixmap != None as Pixmap
      {
	x_draw_image_foreground_1 (s, pixmap);
	x_set_glyph_string_clipping (s);
	XCopyArea (display, pixmap, FRAME_X_DRAWABLE! ((*s).f), (*s).gc as GC,
		   0, 0, (*s).background_width, (*s).height, (*s).x, (*s).y);
	XFreePixmap (display, pixmap);
      }
    else
      { x_draw_image_foreground (s); }
  }

  #[cfg (feature = "use_cairo")]
  { x_draw_image_foreground (s); }

  /* If we must draw a relief around the image, do it.  */
  if (*((*s).img)).relief != 0
      || (*s).hl == DRAW_IMAGE_RAISED
      || (*s).hl == DRAW_IMAGE_SUNKEN
    { x_draw_image_relief (s); }
}

/* C 10632-10744: `x_draw_stretch_glyph_string`.  `x_stretch_cursor_p` is the `bool` global
   (`include/globals.rs` 7200), the three window-box readers and `cursor_in_mouse_face_p` stay
   bare (this graph's use-site declarations live in other units - the ledger's class), the row
   flags go through `glyph_row_bits!` and the string's through `glyph_string_bits!`, and the two
   `s->face->gc` reads cast the mirror's untyped `*mut c_void` to `GC`.  `eassert` is the
   port's macro (P-78).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_stretch_glyph_string (s: *mut glyph_string)
{
  use crate::x11_constants::{GCBackground, GCForeground};

  let face: *mut face = (*s).face as *mut face;

  eassert! (glyph_bits! (type, (*(*s).first_glyph)) == STRETCH_GLYPH);

  if (*s).hl == DRAW_CURSOR
      && !x_stretch_cursor_p
    {
      /* If `x-stretch-cursor' is nil, don't draw a block cursor as
	 wide as the stretch glyph.  */
      let mut width: c_int;
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;

      if !glyph_row_bits! (reversed_p, (*s).row)
	{
	  let left_x: c_int = window_box_left_offset ((*s).w, TEXT_AREA);

	  if x < left_x
	    {
	      background_width -= left_x - x;
	      x = left_x;
	    }
	}
      else
	{
	  /* In R2L rows, draw the cursor on the right edge of the
	     stretch glyph.  */
	  let right_x: c_int = window_box_right ((*s).w, TEXT_AREA);

	  if x + background_width > right_x
	    { background_width -= x - right_x; }
	  x += background_width;
	}
      width = min (FRAME_COLUMN_WIDTH ((*s).f) as isize, background_width as isize) as c_int;
      if glyph_row_bits! (reversed_p, (*s).row)
	{ x -= width; }

      /* Draw cursor.  */
      x_draw_glyph_string_bg_rect (s, x, (*s).y, width, (*s).height);

      /* Clear rest using the GC of the original non-cursor face.  */
      if width < background_width
	{
	  let y: c_int = (*s).y;
	  let mut w: c_int = background_width - width;
	  let h: c_int = (*s).height;
	  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
	  let mut r: XRectangle = std::mem::zeroed ();
	  let mut gc: GC;

	  if !glyph_row_bits! (reversed_p, (*s).row)
	    { x += width; }
	  else
	    { x = (*s).x; }
	  if glyph_row_bits! (mouse_face_p, (*s).row)
	      && cursor_in_mouse_face_p ((*s).w)
	    {
	      x_set_mouse_face_gc (s);
	      gc = (*s).gc as GC;
	    }
	  else
	    { gc = (*face).gc as GC; }

	  get_glyph_string_clip_rect (s, (&raw mut r) as *mut c_void);
	  x_set_clip_rectangles ((*s).f, gc, &raw mut r, 1);

	  if (*face).stipple != 0
	    {
	      /* Fill background with a stipple pattern.  */
	      XSetFillStyle (display, gc, FillOpaqueStippled);
	      x_fill_rectangle ((*s).f, gc, x, y, w, h, true);
	      XSetFillStyle (display, gc, FillSolid);

	      glyph_row_bits! (stipple_p, (*s).row, true);
	    }
	  else
	    {
	      let mut xgcv: XGCValues = std::mem::zeroed ();
	      XGetGCValues (display, gc, GCForeground | GCBackground, &raw mut xgcv);
	      XSetForeground (display, gc, xgcv.background);
	      x_fill_rectangle ((*s).f, gc, x, y, w, h, true);
	      XSetForeground (display, gc, xgcv.foreground);
	    }

	  x_reset_clip_rectangles ((*s).f, gc);
	}
    }
  else if !glyph_string_bits! (background_filled_p, (*s))
    {
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;
      let text_left_x: c_int = window_box_left ((*s).w, TEXT_AREA);

      /* Don't draw into left fringe or scrollbar area except for
         header line and mode line.  */
      if (*s).area == TEXT_AREA
	  && x < text_left_x && !glyph_row_bits! (mode_line_p, (*s).row)
	{
	  background_width -= text_left_x - x;
	  x = text_left_x;
	}

      if !glyph_row_bits! (stipple_p, (*s).row)
	{ glyph_row_bits! (stipple_p, (*s).row, glyph_string_bits! (stippled_p, (*s))); }

      if background_width > 0
	{
	  x_draw_glyph_string_bg_rect (s, x, (*s).y,
				       background_width, (*s).height);
	}
    }

  glyph_string_bits! (background_filled_p, (*s), true);
}

/* C 10745-10761: `x_get_scale_factor`.  `floor` is `<math.h>`'s (the unit's `math.rs`), the
   two `double` comparisons widen the `int` the way C's usual arithmetic conversions do, and a
   null DPYINFO skips both (C's `if (dpyinfo)`).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_get_scale_factor (dpyinfo: *mut x_display_info, scale_x: *mut c_int,
			      scale_y: *mut c_int)
{
  let base_res: c_int = 96;

  *scale_x = 1;
  *scale_y = 1;

  if !dpyinfo.is_null ()
    {
      if (*dpyinfo).resx > base_res as c_double
	{ *scale_x = floor ((*dpyinfo).resx / base_res as c_double) as c_int; }
      if (*dpyinfo).resy > base_res as c_double
	{ *scale_y = floor ((*dpyinfo).resy / base_res as c_double) as c_int; }
    }
}

/* C 10763-10843: `x_draw_underwave`.  C's declaration block mixes initialised and uninitialised
   locals, so the port zero-initialises the ones C leaves alone (PORTING.md 1.5) and keeps the
   two initialised writers as they stand.  The non-cairo arm's `XSetLineAttributes`,
   `XDrawLine`, `XSetClipRectangles` and `Unsorted` are X11 names with no mirror row (the
   ledger's class), `(&raw mut (*s).clip) as *mut XRectangle` is this unit's own cast for the
   `NativeRectangle clip[2]` member (line 9381), and `INT_MAX` is `limits.in.rs` 43.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_underwave (s: *mut glyph_string, decoration_width: c_int)
{
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! ((*s).f);
  /* Adjust for scale/HiDPI.  */
  let mut scale_x: c_int = 0;
  let mut scale_y: c_int = 0;

  x_get_scale_factor (dpyinfo, &raw mut scale_x, &raw mut scale_y);

  let wave_height: c_int = 3 * scale_y;
  let wave_length: c_int = 2 * scale_x;

  #[cfg (feature = "use_cairo")]
  {
    x_draw_horizontal_wave ((*s).f, (*s).gc as GC, (*s).x, (*s).ybase - wave_height + 3,
			    decoration_width, wave_height, wave_length);
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    let mut display: *mut Display;
    let mut dx: c_int;
    let mut dy: c_int;
    let mut x0: c_int;
    let mut y0: c_int;
    let mut width: c_int;
    let mut x1: c_int;
    let mut y1: c_int;
    let mut x2: c_int;
    let mut y2: c_int;
    let mut xmax: c_int;
    let thickness: c_int = scale_y;
    let mut odd: bool;
    let mut wave_clip: XRectangle = std::mem::zeroed ();
    let mut string_clip: XRectangle = std::mem::zeroed ();
    let mut final_clip: XRectangle = std::mem::zeroed ();

    dx = wave_length;
    dy = wave_height - 1;
    x0 = (*s).x;
    y0 = (*s).ybase + wave_height / 2 - scale_y;
    width = decoration_width;
    xmax = x0 + width;

    /* Find and set clipping rectangle */

    wave_clip.x = x0 as c_short;
    wave_clip.y = y0 as c_short;
    wave_clip.width = width as c_ushort;
    wave_clip.height = wave_height as c_ushort;
    get_glyph_string_clip_rect (s, (&raw mut string_clip) as *mut c_void);

    if !gui_intersect_rectangles (&wave_clip, &string_clip, &mut final_clip)
      { return; }

    display = (*dpyinfo).display;
    XSetClipRectangles (display, (*s).gc as GC, 0, 0, &raw mut final_clip, 1, Unsorted);

    /* Draw the waves */

    x1 = x0 - (x0 % dx);
    x2 = x1 + dx;
    odd = ((x1 / dx) & 1) != 0;
    y1 = y0;
    y2 = y0;

    if odd
      { y1 += dy; }
    else
      { y2 += dy; }

    if INT_MAX - dx < xmax
      { emacs_abort (); }

    while x1 <= xmax
      {
	XSetLineAttributes (display, (*s).gc as GC, thickness, LineSolid, CapButt,
			    JoinRound);
	XDrawLine (display, FRAME_X_DRAWABLE! ((*s).f), (*s).gc as GC, x1, y1, x2, y2);
	x1 = x2;
	y1 = y2;
	x2 += dx;
	y2 = y0 + odd as c_int * dy;
	odd = !odd;
      }

    /* Restore previous clipping rectangle(s) */
    XSetClipRectangles (display, (*s).gc as GC, 0, 0,
			(&raw mut (*s).clip) as *mut XRectangle, (*s).num_clips, Unsorted);
  }
}

/* C 10845-10893: `x_draw_dash`.  C's `char segment` is the port's `c_char` and `&segment` its
   `&raw mut` (the `XSetDashes` argument is `char *`); C's `XGCValues gcv` is written field by
   field and passed whole, so it is zero-initialised first (PORTING.md 1.5).  The two
   `XChangeGC`/`XSetDashes`/`XDrawLine` calls are X11's, and the cairo arm's `cairo_*` names live
   in `include/gtk.rs` (the ledger's class).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_dash (f: *mut frame, s: *mut glyph_string, width: c_int,
		       mut segment: c_char, offset: c_int, thickness: c_int)
{
  #[cfg (not (feature = "use_cairo"))]
  {
    let mut gcv: XGCValues = std::mem::zeroed ();
    let mut y_center: c_int;

    /* Configure the GC, the dash pattern and a suitable offset.  */
    let gc: GC = (*s).gc as GC;
    let display: *mut Display = FRAME_X_DISPLAY! (f);

    gcv.line_style = LineOnOffDash;
    gcv.line_width = thickness;
    XChangeGC (display, (*s).gc as GC, GCLineStyle | GCLineWidth,
	       &raw mut gcv as *mut c_void);
    XSetDashes (display, (*s).gc as GC, (*s).x, &raw mut segment as *mut c_char, 1);

    /* Offset the origin of the line by half the line width. */
    y_center = (*s).ybase + offset + thickness / 2;
    XDrawLine (display, FRAME_X_DRAWABLE! (f), gc,
	       (*s).x, y_center, (*s).x + width, y_center);

    /* Restore the initial line style.  */
    gcv.line_style = LineSolid;
    gcv.line_width = 1;
    XChangeGC (display, (*s).gc as GC, GCLineStyle | GCLineWidth,
	       &raw mut gcv as *mut c_void);
  }

  #[cfg (feature = "use_cairo")]
  {
    let mut y_center: c_double;
    let mut cr_segment: c_double;

    let cr: *mut cairo_t = x_begin_cr_clip (f, (*s).gc as GC);
    cr_segment = segment as c_double;
    y_center = (*s).ybase as c_double + offset as c_double + (thickness / 2.0) as c_double;

    x_set_cr_source_with_gc_foreground (f, (*s).gc as GC, false);
    cairo_set_dash (cr, &raw mut cr_segment, 1, (*s).x as c_double);
    cairo_set_line_width (cr, thickness as c_double);
    cairo_move_to (cr, (*s).x as c_double, y_center);
    cairo_line_to (cr, ((*s).x + width) as c_double, y_center);
    cairo_stroke (cr);
    x_end_cr_clip (f);
  }
}

/* C 10895-10935: `x_fill_underline`.  C's `switch` is the port's `match` and the `FALLTHROUGH`
   from the DOTS arm into the DASHES one is the entry stage the two arms share (PORTING.md 2.2;
   the `macro-body-notes.tsv` row the `--check` gate asks for is appended by this slice).  C's
   `char x_segment` holds `min (segment, CHAR_MAX)`, so the port keeps the same narrowing and
   `CHAR_MAX` is `limits.in.rs`'s.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_fill_underline (f: *mut frame, s: *mut glyph_string,
			    style: face_underline_type, position: c_int,
			    decoration_width: c_int, thickness: c_int)
{
  let mut segment: c_int;
  let mut x_segment: c_char;

  segment = thickness * 3;

  match style
    {
      /* FACE_UNDERLINE_DOUBLE_LINE is treated identically to SINGLE, as
	 the second line will be filled by another invocation of this
	 function.  */
      FACE_UNDERLINE_SINGLE | FACE_UNDERLINE_DOUBLE_LINE =>
	{
	  x_fill_rectangle (f, (*s).gc as GC, (*s).x, (*s).ybase + position,
			    decoration_width, thickness, false);
	}

      FACE_UNDERLINE_DOTS | FACE_UNDERLINE_DASHES =>
	{
	  if style == FACE_UNDERLINE_DOTS
	    { segment = thickness; }
	  x_segment = min (segment as isize, CHAR_MAX as isize) as c_char;
	  x_draw_dash (f, s, decoration_width, x_segment, position,
		       thickness);
	}

      FACE_NO_UNDERLINE | FACE_UNDERLINE_WAVE | _ =>
	{ emacs_abort (); }
    }
}

/* C 10937-11304: `x_draw_glyph_string`.  The three walks C writes as `for` loops keep their
   bodies and the port's `while` (the `x_get_scroll_valuator_delta` shape); `switch
   (s->first_glyph->type)` is the port's `match` on the mirror's `glyph_bits!` value with
   `emacs_abort` for `default:` (the ledger's `macro-body-notes.tsv` row the `--check` gate asks
   for), `s->first_glyph->u.cmp.automatic` is `glyph_u_cmp_bits!`, and every `bool_bf` member
   (`for_overlaps`, `background_filled_p`, the face's `underline*`/`overline*`/`strike_through*`
   family) goes through its accessor.  The underline arithmetic keeps C's `unsigned long`
   local and therefore C's conversions and wraps (the `haikuterm.rs` 1400-1520 reading of the
   same code); `position.wrapping_sub (thickness).wrapping_sub (1)` is C 11170's
   `position = position - thickness - 1;`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_draw_glyph_string (s: *mut glyph_string)
{
  use crate::x11_constants::GCForeground;

  let face: *mut face = (*s).face as *mut face;
  let mut relief_drawn_p: bool = false;

  /* If S draws into the background of its successors, draw the
     background of the successors first so that S can draw into it.
     This makes S->next use XDrawString instead of XDrawImageString.  */
  if !(*s).next.is_null () && (*s).right_overhang != 0
      && glyph_string_bits! (for_overlaps, (*s)) == 0
    {
      let mut width: c_int;
      let mut next: *mut glyph_string;

      width = 0;
      next = (*s).next;
      while !next.is_null () && width < (*s).right_overhang
	{
	  if glyph_bits! (type, (*(*next).first_glyph)) != IMAGE_GLYPH
	    {
	      x_set_glyph_string_gc (next);
	      x_set_glyph_string_clipping (next);
	      if glyph_bits! (type, (*(*next).first_glyph)) == STRETCH_GLYPH
		{ x_draw_stretch_glyph_string (next); }
	      else
		{ x_draw_glyph_string_background (next, true); }
	      (*next).num_clips = 0;
	    }
	  width += (*next).width;
	  next = (*next).next;
	}
    }

  /* Set up S->gc, set clipping and draw S.  */
  x_set_glyph_string_gc (s);

  /* Draw relief (if any) in advance for char/composition so that the
     glyph string can be drawn over it.  */
  if glyph_string_bits! (for_overlaps, (*s)) == 0
      && face_bits! (box, face) != FACE_NO_BOX
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)

    {
      x_set_glyph_string_clipping (s);
      x_draw_glyph_string_background (s, true);
      x_draw_glyph_string_box (s);
      x_set_glyph_string_clipping (s);
      relief_drawn_p = true;
    }
  else if (*s).clip_head.is_null () /* draw_glyphs didn't specify a clip mask. */
	   && (*s).clip_tail.is_null ()
	   && ((!(*s).prev.is_null () && (*((*s).prev)).hl != (*s).hl
		&& (*s).left_overhang != 0)
	       || (!(*s).next.is_null () && (*((*s).next)).hl != (*s).hl
		   && (*s).right_overhang != 0))
    /* We must clip just this glyph.  left_overhang part has already
       drawn when s->prev was drawn, and right_overhang part will be
       drawn later when s->next is drawn. */
    { x_set_glyph_string_clipping_exactly (s, s); }
  else
    { x_set_glyph_string_clipping (s); }

  match glyph_bits! (type, (*(*s).first_glyph))
    {
      IMAGE_GLYPH =>
	{ x_draw_image_glyph_string (s); }

      XWIDGET_GLYPH =>
	{ x_draw_xwidget_glyph_string (s as *mut c_void); }

      STRETCH_GLYPH =>
	{ x_draw_stretch_glyph_string (s); }

      CHAR_GLYPH =>
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0
	    { glyph_string_bits! (background_filled_p, (*s), true); }
	  else
	    { x_draw_glyph_string_background (s, false); }
	  x_draw_glyph_string_foreground (s);
	}

      COMPOSITE_GLYPH =>
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0
	      || ((*s).cmp_from > 0
		  && !glyph_u_cmp_bits! (automatic, (*s).first_glyph))
	    { glyph_string_bits! (background_filled_p, (*s), true); }
	  else
	    { x_draw_glyph_string_background (s, true); }
	  x_draw_composite_glyph_string_foreground (s);
	}

      GLYPHLESS_GLYPH =>
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0
	    { glyph_string_bits! (background_filled_p, (*s), true); }
	  else
	    { x_draw_glyph_string_background (s, true); }
	  x_draw_glyphless_glyph_string_foreground (s);
	}

      _ =>
	{ emacs_abort (); }
    }

  if glyph_string_bits! (for_overlaps, (*s)) == 0
    {
      let mut area_x: c_int;
      let mut area_y: c_int;
      let mut area_width: c_int;
      let mut area_height: c_int;
      let mut area_max_x: c_int;
      let mut decoration_width: c_int;

      /* Prevent the underline from overwriting surrounding areas
	 and the fringe.  */
      window_box ((*s).w, (*s).area, &raw mut area_x, &raw mut area_y,
		  &raw mut area_width, &raw mut area_height);
      area_max_x = area_x + area_width - 1;

      decoration_width = (*s).width;
      if !glyph_row_bits! (mode_line_p, (*s).row)
	  && !glyph_row_bits! (tab_line_p, (*s).row)
	  && area_max_x < ((*s).x + decoration_width - 1)
	{ decoration_width -= ((*s).x + decoration_width - 1) - area_max_x; }

      /* Draw relief if not yet drawn.  */
      if !relief_drawn_p && face_bits! (box, face) != FACE_NO_BOX
	{ x_draw_glyph_string_box (s); }

      /* Draw underline.  */
      if face_bits! (underline, face) != 0
	{
	  if face_bits! (underline, face) == FACE_UNDERLINE_WAVE
	    {
	      if face_bits! (underline_defaulted_p, face)
		{ x_draw_underwave (s, decoration_width); }
	      else
		{
		  let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
		  let mut xgcv: XGCValues = std::mem::zeroed ();
		  XGetGCValues (display, (*s).gc as GC, GCForeground, &raw mut xgcv);
		  XSetForeground (display, (*s).gc as GC, (*face).underline_color);
		  x_draw_underwave (s, decoration_width);
		  XSetForeground (display, (*s).gc as GC, xgcv.foreground);
		}
	    }
	  else if face_bits! (underline, face) >= FACE_UNDERLINE_SINGLE
	    {
	      let mut thickness: c_ulong;
	      let mut position: c_ulong;

	      if !(*s).prev.is_null ()
		  && (face_bits! (underline, (*((*s).prev)).face as *mut face)
		      != FACE_UNDERLINE_WAVE
		      && face_bits! (underline, (*((*s).prev)).face as *mut face)
			 >= FACE_UNDERLINE_SINGLE)
		  && (face_bits! (underline_at_descent_line_p,
				  (*((*s).prev)).face as *mut face)
		      == face_bits! (underline_at_descent_line_p, face))
		  && ((*( (*((*s).prev)).face as *mut face )).underline_pixels_above_descent_line
		      == (*face).underline_pixels_above_descent_line)
		{
		  /* We use the same underline style as the previous one.  */
		  thickness = (*(*s).prev).underline_thickness as c_ulong;
		  position = (*(*s).prev).underline_position as c_ulong;
		}
	      else
		{
		  let font: *mut font = font_for_underline_metrics (s);
		  let mut minimum_offset: c_ulong;
		  let mut underline_at_descent_line: bool;
		  let mut use_underline_position_properties: bool;
		  let mut val: Lisp_Object = (WINDOW_BUFFER_LOCAL_VALUE
					      (Qunderline_minimum_offset (), (*s).w));

		  if FIXNUMP (val)
		    { minimum_offset = max (0, XFIXNUM (val)) as c_ulong; }
		  else
		    { minimum_offset = 1; }

		  val = (WINDOW_BUFFER_LOCAL_VALUE
			 (Qx_underline_at_descent_line (), (*s).w));
		  underline_at_descent_line
		    = (! (NILP (val) || BASE_EQ (val, Qunbound ()))
		       || face_bits! (underline_at_descent_line_p, face));

		  val = (WINDOW_BUFFER_LOCAL_VALUE
			 (Qx_use_underline_position_properties (), (*s).w));
		  use_underline_position_properties
		    = ! (NILP (val) || BASE_EQ (val, Qunbound ()));

		  /* Get the underline thickness.  Default is 1 pixel.  */
		  if !font.is_null () && (*font).underline_thickness > 0
		    { thickness = (*font).underline_thickness as c_ulong; }
		  else
		    { thickness = 1; }
		  if underline_at_descent_line
		    {
		      position = (((*s).height) as c_ulong).wrapping_sub (thickness)
				 .wrapping_sub (((*s).ybase - (*s).y) as c_ulong)
				 .wrapping_sub ((*face).underline_pixels_above_descent_line
						as c_ulong);
		    }
		  else
		    {
		      /* Get the underline position.  This is the
			 recommended vertical offset in pixels from
			 the baseline to the top of the underline.
			 This is a signed value according to the
			 specs, and its default is

			 ROUND ((maximum descent) / 2), with
			 ROUND(x) = floor (x + 0.5)  */

		      if use_underline_position_properties
			  && !font.is_null () && (*font).underline_position >= 0
			{ position = (*font).underline_position as c_ulong; }
		      else if !font.is_null ()
			{ position = (((*font).descent + 1) / 2) as c_ulong; }
		      else
			{ position = minimum_offset; }
		    }

		  /* Ignore minimum_offset if the amount of pixels was
		     explicitly specified.  */
		  if (*face).underline_pixels_above_descent_line == 0
		    { position = max (position as isize, minimum_offset as isize) as c_ulong; }
		}
	      /* Check the sanity of thickness and position.  We should
		 avoid drawing underline out of the current line area.  */
	      if (((*s).y + (*s).height) as c_ulong)
		  <= ((*s).ybase as c_ulong).wrapping_add (position)
		{ position = (((*s).height - 1) - ((*s).ybase - (*s).y)) as c_ulong; }
	      if (((*s).y + (*s).height) as c_ulong)
		  < ((*s).ybase as c_ulong).wrapping_add (position).wrapping_add (thickness)
		{
		  thickness = (((*s).y + (*s).height) as c_ulong)
			      .wrapping_sub ((*s).ybase as c_ulong + position);
		}
	      (*s).underline_thickness = thickness as c_int;
	      (*s).underline_position = position as c_int;

	      {
		let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
		let mut xgcv: XGCValues = std::mem::zeroed ();

		if !face_bits! (underline_defaulted_p, face)
		  {
		    XGetGCValues (display, (*s).gc as GC, GCForeground, &raw mut xgcv);
		    XSetForeground (display, (*s).gc as GC, (*face).underline_color);
		  }

		x_fill_underline ((*s).f, s, face_bits! (underline, face),
				  position as c_int, decoration_width,
				  thickness as c_int);

		/* Place a second underline above the first if this was
		   requested in the face specification.  */

		if face_bits! (underline, face) == FACE_UNDERLINE_DOUBLE_LINE
		  {
		    /* Compute the position of the second underline.  */
		    position = position.wrapping_sub (thickness).wrapping_sub (1);
		    x_fill_underline ((*s).f, s, face_bits! (underline, face),
				      position as c_int, decoration_width,
				      thickness as c_int);
		  }

		if !face_bits! (underline_defaulted_p, face)
		  { XSetForeground (display, (*s).gc as GC, xgcv.foreground); }
	      }
	    }
	}
      /* Draw overline.  */
      if face_bits! (overline_p, face)
	{
	  let mut dy: c_ulong = 0;
	  let h: c_ulong = 1;

	  if face_bits! (overline_color_defaulted_p, face)
	    {
	      x_fill_rectangle ((*s).f, (*s).gc as GC, (*s).x,
				((*s).y as c_ulong + dy) as c_int,
				decoration_width, h as c_int, false);
	    }
	  else
	    {
	      let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
	      let mut xgcv: XGCValues = std::mem::zeroed ();
	      XGetGCValues (display, (*s).gc as GC, GCForeground, &raw mut xgcv);
	      XSetForeground (display, (*s).gc as GC, (*face).overline_color);
	      x_fill_rectangle ((*s).f, (*s).gc as GC, (*s).x,
				((*s).y as c_ulong + dy) as c_int,
				decoration_width, h as c_int, false);
	      XSetForeground (display, (*s).gc as GC, xgcv.foreground);
	    }
	}

      /* Draw strike-through.  */
      if face_bits! (strike_through_p, face)
	{
	  /* Y-coordinate and height of the glyph string's first
	     glyph.  We cannot use s->y and s->height because those
	     could be larger if there are taller display elements
	     (e.g., characters displayed with a larger font) in the
	     same glyph row.  */
	  let glyph_y: c_int = (*s).ybase - (*((*s).first_glyph)).ascent as c_int;
	  let glyph_height: c_int = (*((*s).first_glyph)).ascent as c_int
				    + (*((*s).first_glyph)).descent as c_int;
	  /* Strike-through width and offset from the glyph string's
	     top edge.  */
	  let h: c_ulong = 1;
	  let dy: c_ulong = ((glyph_height as c_ulong).wrapping_sub (h)) / 2;

	  if face_bits! (strike_through_color_defaulted_p, face)
	    {
	      x_fill_rectangle ((*s).f, (*s).gc as GC, (*s).x,
				((glyph_y as c_ulong + dy) as c_int),
				(*s).width, h as c_int, false);
	    }
	  else
	    {
	      let display: *mut Display = FRAME_X_DISPLAY! ((*s).f);
	      let mut xgcv: XGCValues = std::mem::zeroed ();
	      XGetGCValues (display, (*s).gc as GC, GCForeground, &raw mut xgcv);
	      XSetForeground (display, (*s).gc as GC, (*face).strike_through_color);
	      x_fill_rectangle ((*s).f, (*s).gc as GC, (*s).x,
				((glyph_y as c_ulong + dy) as c_int),
				decoration_width, h as c_int, false);
	      XSetForeground (display, (*s).gc as GC, xgcv.foreground);
	    }
	}

      if !(*s).prev.is_null ()
	{
	  let mut prev: *mut glyph_string;

	  prev = (*s).prev;
	  while !prev.is_null ()
	    {
	      if (*prev).hl != (*s).hl
		  && (*prev).x + (*prev).width + (*prev).right_overhang > (*s).x
		{
		  /* As prev was drawn while clipped to its own area, we
		     must draw the right_overhang part using s->hl now.  */
		  let save: draw_glyphs_face = (*prev).hl;

		  (*prev).hl = (*s).hl;
		  x_set_glyph_string_gc (prev);
		  x_set_glyph_string_clipping_exactly (s, prev);
		  if glyph_bits! (type, (*(*prev).first_glyph)) == CHAR_GLYPH
		    { x_draw_glyph_string_foreground (prev); }
		  else
		    { x_draw_composite_glyph_string_foreground (prev); }
		  x_reset_clip_rectangles ((*prev).f, (*prev).gc as GC);
		  (*prev).hl = save;
		  (*prev).num_clips = 0;
		}
	      prev = (*prev).prev;
	    }
	}

      if !(*s).next.is_null ()
	{
	  let mut next: *mut glyph_string;

	  next = (*s).next;
	  while !next.is_null ()
	    {
	      if (*next).hl != (*s).hl
		  && (*next).x - (*next).left_overhang < (*s).x + (*s).width
		{
		  /* As next will be drawn while clipped to its own area,
		     we must draw the left_overhang part using s->hl now.  */
		  let save: draw_glyphs_face = (*next).hl;

		  (*next).hl = (*s).hl;
		  x_set_glyph_string_gc (next);
		  x_set_glyph_string_clipping_exactly (s, next);
		  if glyph_bits! (type, (*(*next).first_glyph)) == CHAR_GLYPH
		    { x_draw_glyph_string_foreground (next); }
		  else
		    { x_draw_composite_glyph_string_foreground (next); }
		  x_reset_clip_rectangles ((*next).f, (*next).gc as GC);
		  (*next).hl = save;
		  (*next).num_clips = 0;
		  (*next).clip_head = (*s).next;
		}
	      next = (*next).next;
	    }
	}
    }

  /* Reset clipping.  */
  x_reset_clip_rectangles ((*s).f, (*s).gc as GC);
  (*s).num_clips = 0;

  /* Set the stippled flag that tells redisplay whether or not a
     stipple was actually draw.  */

  if glyph_bits! (type, (*(*s).first_glyph)) != STRETCH_GLYPH
      && glyph_bits! (type, (*(*s).first_glyph)) != IMAGE_GLYPH
      && !glyph_row_bits! (stipple_p, (*s).row)
    { glyph_row_bits! (stipple_p, (*s).row, glyph_string_bits! (stippled_p, (*s))); }
}

/* C 11306-11318: `x_shift_glyphs_for_insert`.  C's comment sits inside the body and the port
   keeps it there.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_shift_glyphs_for_insert (f: *mut frame, x: c_int, y: c_int, width: c_int,
				     height: c_int, shift_by: c_int)
{
/* Never called on a GUI frame, see
   https://lists.gnu.org/r/emacs-devel/2015-05/msg00456.html
*/
  XCopyArea (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f), FRAME_X_DRAWABLE! (f),
	     (*( (*f).output_data.x as *mut x_output )).normal_gc,
	     x, y, width, height,
	     x + shift_by, y);
}

/* C 11320-11327: `x_delete_glyphs`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_delete_glyphs (f: *mut frame, n: c_int)
{
  emacs_abort ();
}

/* C 11329-11340: `#if defined USE_GTK || !defined USE_CAIRO` - `x_clear_area1`.  */
#[cfg (all (feature = "have_x_windows",
	    any (feature = "use_gtk", not (feature = "use_cairo"))))]
unsafe fn x_clear_area1 (dpy: *mut Display, window: Window,
			 x: c_int, y: c_int, width: c_int, height: c_int, exposures: c_int)
{
  eassert! (width > 0 && height > 0);
  XClearArea (dpy, window, x, y, width, height, exposures);
}

/* C 11342-11398: `x_clear_area` (C's definition is not `static`, so the unit exports it under
   C's name - the `x_dpyinfo` shape).  C's two `#ifndef USE_GTK` guards leave the
   `x_clear_area1` arm out of a GTK build and the `#ifdef HAVE_XDBE` term out of a non-Xdbe one,
   so the port computes C's condition in three cfg'd spellings and keeps one copy of the shared
   body; the `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 || (RENDER_MINOR >= 2))` block and
   its `else` are the two arms of one `if`, and the else exists in both configurations.  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe fn x_clear_area (f: *mut frame, x: c_int, y: c_int, width: c_int, height: c_int)
{
  #[cfg (feature = "use_cairo")]
  {
    let cr: *mut cairo_t;

    eassert! (width > 0 && height > 0);

    cr = x_begin_cr_clip (f, std::ptr::null_mut ());
    x_set_cr_source_with_gc_background (f, (*( (*f).output_data.x as *mut x_output )).normal_gc,
					true);
    cairo_rectangle (cr, x as c_double, y as c_double,
		     width as c_double, height as c_double);
    cairo_fill (cr);
    x_end_cr_clip (f);
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    #[cfg (all (not (feature = "use_gtk"), feature = "have_xdbe"))]
    let clear_p: bool = (*f).alpha_background != 1.0
      || FRAME_X_DOUBLE_BUFFERED_P! (f);
    #[cfg (all (not (feature = "use_gtk"), not (feature = "have_xdbe")))]
    let clear_p: bool = (*f).alpha_background != 1.0;
    #[cfg (feature = "use_gtk")]
    let clear_p: bool = true;

    if clear_p
      {
	#[cfg (all (feature = "have_xrender", feature = "have_xrender_0_2"))]
	{
	  use crate::x11_constants::None;

	  x_xr_ensure_picture (f);
	  if (*FRAME_DISPLAY_INFO! (f)).alpha_bits != 0
	      && FRAME_X_PICTURE! (f) != None as Picture
	      && (*f).alpha_background != 1.0
	      && FRAME_CHECK_XR_VERSION! (f, 0, 2)
	    {
	      let mut xc: XRenderColor = std::mem::zeroed ();
	      let gc: GC = (*( (*f).output_data.x as *mut x_output )).normal_gc;

	      x_xr_apply_ext_clip (f, gc);
	      x_xrender_color_from_gc_background (f, gc, &raw mut xc, true);
	      XRenderFillRectangle (FRAME_X_DISPLAY! (f),
				    PictOpSrc, FRAME_X_PICTURE! (f),
				    &raw mut xc, x, y, width, height);
	      x_xr_reset_ext_clip (f);
	      x_mark_frame_dirty (f);
	    }
	  else
	    {
	      XFillRectangle (FRAME_X_DISPLAY! (f),
			      FRAME_X_DRAWABLE! (f),
			      (*( (*f).output_data.x as *mut x_output )).reverse_gc,
			      x, y, width, height);
	    }
	}
	#[cfg (not (all (feature = "have_xrender", feature = "have_xrender_0_2")))]
	{
	  XFillRectangle (FRAME_X_DISPLAY! (f),
			  FRAME_X_DRAWABLE! (f),
			  (*( (*f).output_data.x as *mut x_output )).reverse_gc,
			  x, y, width, height);
	}
      }
    else
      {
	#[cfg (not (feature = "use_gtk"))]
	{
	  x_clear_area1 (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
			 x, y, width, height, False);
	}
      }
  }
}

/* C 11400-11419: `x_clear_frame`.  `font_drop_xrender_surfaces` is font.h's prototype with no
   mirror row, declared here the way this unit's `x_drop_xrender_surfaces` declares it (P-55);
   `x_scroll_bar_clear` is this file's own later slice, so it stays bare.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_clear_frame (f: *mut frame)
{
  unsafe extern "C" {
    /// C: font.h 932: `extern void font_drop_xrender_surfaces (struct frame *f);`
    pub fn font_drop_xrender_surfaces (f: *mut frame);
  }

  /* Clearing the frame will erase any cursor, so mark them all as no
     longer visible.  */
  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));

  block_input ();
  font_drop_xrender_surfaces (f);
  x_clear_window (f);

  /* We have to clear the scroll bars.  If we have changed colors or
     something like that, then they should be notified.  */
  x_scroll_bar_clear (f);

  unblock_input ();
}

/* C 11421-11444: `x_send_hourglass_message`.  `msg.xclient`'s members are the opaque-`XEvent`
   class this ledger records (`include/xlib.rs` 1181 keeps 33 members owed), `ClientMessage` and
   `NoEventMask` are `x11_constants` rows, `False` is Xlib's with no row, and C's
   `memset (&msg, 0, sizeof msg)` is this unit's `memset` shape (line 8147).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_send_hourglass_message (f: *mut frame, hourglass_enabled: bool)
{
  use crate::x11_constants::{ClientMessage, NoEventMask};

  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut msg: XEvent = std::mem::zeroed ();

  memset (&raw mut msg as *mut c_void, 0, std::mem::size_of_val (&msg));

  msg.xclient.r#type = ClientMessage;
  msg.xclient.message_type
    = (*dpyinfo).Xatom_EMACS_TMP;
  msg.xclient.format = 8;
  msg.xclient.window = FRAME_X_WINDOW! (f);
  msg.xclient.data.b[0] = if hourglass_enabled { 1 } else { 0 };

  XSendEvent ((*dpyinfo).display, FRAME_X_WINDOW! (f),
	      False, NoEventMask, &raw mut msg);
}

/* C 11446-11526: `x_show_hourglass`.  C's `#ifdef USE_X_TOOLKIT if (x->widget) #else if
   (FRAME_OUTER_WINDOW (f)) #endif` and the two `#ifdef USE_GTK parent = …` sites are conditions
   without a common spelling, so the port keeps one variable per arm (the `ftfont.rs` 2552
   shape); the `USE_XCB`/non-`USE_XCB` blocks are C's own two alternative bodies.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_show_hourglass (f: *mut frame)
{
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);

  if !dpy.is_null ()
    {
      let x: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;

      /* If the hourglass window is mapped inside a popup menu, input
	 could be lost if the menu is popped down and the grab is
	 relinquished, but the hourglass window is still up.  Just
	 avoid displaying the hourglass at all while popups are
	 active.  */

      if popup_activated ()
	{ return; }

      x_send_hourglass_message (f, true);

      #[cfg (feature = "use_x_toolkit")]
      let outer_p: bool = !(*x).widget.is_null ();
      #[cfg (not (feature = "use_x_toolkit"))]
      let outer_p: bool = FRAME_OUTER_WINDOW! (f) != 0;

      if outer_p
       {
         if (*x).hourglass_window == 0
           {
	     #[cfg (not (feature = "use_xcb"))]
	     {
	       let mask: c_ulong = CWCursor;
	       let mut attrs: XSetWindowAttributes = std::mem::zeroed ();
	       #[cfg (feature = "use_gtk")]
	       let parent: Window = FRAME_X_WINDOW! (f);
	       #[cfg (not (feature = "use_gtk"))]
	       let parent: Window = FRAME_OUTER_WINDOW! (f);

	       attrs.cursor = (*x).hourglass_cursor;

	       (*x).hourglass_window = XCreateWindow
		 (dpy, parent, 0, 0, 32000, 32000, 0, 0,
		  InputOnly, CopyFromParent, mask, &raw mut attrs);
	     }
	     #[cfg (feature = "use_xcb")]
	     {
	       let cursor: uint32_t = (*x).hourglass_cursor as uint32_t;
	       #[cfg (feature = "use_gtk")]
	       let parent: xcb_window_t = FRAME_X_WINDOW! (f) as xcb_window_t;
	       #[cfg (not (feature = "use_gtk"))]
	       let parent: xcb_window_t = FRAME_OUTER_WINDOW! (f) as xcb_window_t;

	       (*x).hourglass_window
		 = xcb_generate_id ((*FRAME_DISPLAY_INFO! (f)).xcb_connection) as Window;

	       xcb_create_window ((*FRAME_DISPLAY_INFO! (f)).xcb_connection,
				  XCB_COPY_FROM_PARENT,
				  (*x).hourglass_window as xcb_window_t,
				  parent, 0, 0, FRAME_PIXEL_WIDTH (f),
				  FRAME_PIXEL_HEIGHT (f), 0,
				  XCB_WINDOW_CLASS_INPUT_ONLY,
				  XCB_COPY_FROM_PARENT, XCB_CW_CURSOR,
				  &raw const cursor);
	     }
           }

	 #[cfg (not (feature = "use_xcb"))]
	 {
	   XMapRaised (dpy, (*x).hourglass_window);
	   /* Ensure that the spinning hourglass is shown.  */
	   flush_frame! (f);
	 }
	 #[cfg (feature = "use_xcb")]
	 {
	   let value: uint32_t = XCB_STACK_MODE_ABOVE;

	   xcb_configure_window ((*FRAME_DISPLAY_INFO! (f)).xcb_connection,
				 (*x).hourglass_window as xcb_window_t,
				 XCB_CONFIG_WINDOW_STACK_MODE, &raw const value);
	   xcb_map_window ((*FRAME_DISPLAY_INFO! (f)).xcb_connection,
			   (*x).hourglass_window as xcb_window_t);
	   xcb_flush ((*FRAME_DISPLAY_INFO! (f)).xcb_connection);
	 }
       }
    }
}

/* C 11528-11546: `x_hide_hourglass`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_hide_hourglass (f: *mut frame)
{
  let x: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;

  /* Watch out for newly created frames.  */
  if (*x).hourglass_window != 0
    {
      #[cfg (not (feature = "use_xcb"))]
      { XUnmapWindow (FRAME_X_DISPLAY! (f), (*x).hourglass_window); }
      #[cfg (feature = "use_xcb")]
      {
	xcb_unmap_window ((*FRAME_DISPLAY_INFO! (f)).xcb_connection,
			  (*x).hourglass_window as xcb_window_t);
      }
      x_send_hourglass_message (f, false);
    }
}

/* C 11548-11652: `XTflash`.  C's two `#if`-less halves are written twice (C repeats them); the
   `fd_set`/`FD_ZERO`/`FD_SET`/`pselect` names are the ones the unit's `x_sync_wait_for_frame_…`
   already uses, `detect_input_pending` is lisp.h's prototype with no mirror row, and
   `ConnectionNumber`/`XCreateGC`/`XFillRectangle`/`XFreeGC` are Xlib's (the ledger's class).  */
#[cfg (feature = "have_x_windows")]
unsafe fn XTflash (f: *mut frame)
{
  let mut gc: GC;
  let mut values: XGCValues = std::mem::zeroed ();
  let mut fds: fd_set = std::mem::zeroed ();
  let fd: c_int;
  let mut rc: c_int;

  block_input ();
  values.function = GXinvert;
  gc = XCreateGC (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
		  GCFunction, &raw mut values as *mut c_void);

  /* Get the height not including a menu bar widget.  */
  let height: c_int = FRAME_PIXEL_HEIGHT (f);
  /* Height of each line to flash.  */
  let flash_height: c_int = FRAME_LINE_HEIGHT (f);
  /* These will be the left and right margins of the rectangles.  */
  let flash_left: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);
  let flash_right: c_int = FRAME_PIXEL_WIDTH (f) - FRAME_INTERNAL_BORDER_WIDTH (f);
  let width: c_int = flash_right - flash_left;

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT (f)
    {
      XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), gc,
		      flash_left,
		      (FRAME_INTERNAL_BORDER_WIDTH (f)
		       + FRAME_TOP_MARGIN_HEIGHT (f)),
		      width, flash_height);
      XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), gc,
		      flash_left,
		      (height - flash_height
		       - FRAME_INTERNAL_BORDER_WIDTH (f)
		       - FRAME_BOTTOM_MARGIN_HEIGHT (f)),
		      width, flash_height);

    }
  else
    /* If it is short, flash it all.  */
    {
      XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), gc,
		      flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),
		      width, height - 2 * FRAME_INTERNAL_BORDER_WIDTH (f));
    }

  x_flush (f);

  let delay: timespec = make_timespec (0, 150 * 1000 * 1000);
  let wakeup: timespec = timespec_add (current_timespec (), delay);
  fd = ConnectionNumber (FRAME_X_DISPLAY! (f));

  /* Keep waiting until past the time wakeup or any input gets
     available.  */
  while !detect_input_pending ()
    {
      let current: timespec = current_timespec ();
      let mut timeout: timespec;

      /* Break if result would not be positive.  */
      if timespec_cmp (wakeup, current) <= 0
	{ break; }

      /* How long `select' should wait.  */
      timeout = make_timespec (0, 10 * 1000 * 1000);

      /* Wait for some input to become available on the X
	 connection.  */
      FD_ZERO (&raw mut fds);
      FD_SET (fd, &raw mut fds);

      /* Try to wait that long--but we might wake up sooner.  */
      rc = pselect (fd + 1, &raw mut fds, std::ptr::null_mut (), std::ptr::null_mut (),
		    &raw mut timeout, std::ptr::null_mut ());

      /* Some input is available, exit the visible bell.  */
      if rc >= 0 && FD_ISSET (fd, &fds)
	{ break; }
    }

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT (f)
    {
      XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), gc,
		      flash_left,
		      (FRAME_INTERNAL_BORDER_WIDTH (f)
		       + FRAME_TOP_MARGIN_HEIGHT (f)),
		      width, flash_height);
      XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), gc,
		      flash_left,
		      (height - flash_height
		       - FRAME_INTERNAL_BORDER_WIDTH (f)
		       - FRAME_BOTTOM_MARGIN_HEIGHT (f)),
		      width, flash_height);
    }
  else
    /* If it is short, flash it all.  */
    {
      XFillRectangle (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), gc,
		      flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),
		      width, height - 2 * FRAME_INTERNAL_BORDER_WIDTH (f));
    }

  XFreeGC (FRAME_X_DISPLAY! (f), gc);
  x_flush (f);

  unblock_input ();
}

/* C 11654-11686: `XTring_bell`.  `visible_bell` is the `bool` global (`include/globals.rs`
   7145), `x_ignore_errors_for_next_request`/`x_stop_ignoring_errors` are this file's own later
   slices, and the `HAVE_XKB` arm's `None` is the `x11_constants` row.  */
#[cfg (feature = "have_x_windows")]
unsafe fn XTring_bell (f: *mut frame)
{
  let dpy: *mut Display = FRAME_X_DISPLAY! (f);

  if dpy.is_null ()
    { return; }

  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);

  if visible_bell
    { XTflash (f); }
  else
    {
      /* When Emacs is untrusted, Bell requests sometimes generate
	 Access errors.  This is not in the security extension
	 specification but seems to be a bug in the X consortium XKB
	 implementation.  */

      block_input ();
      x_ignore_errors_for_next_request (dpyinfo, 0);
      #[cfg (feature = "have_xkb")]
      {
	use crate::x11_constants::None;

	XkbBell (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), 0, None);
      }
      #[cfg (not (feature = "have_xkb"))]
      { XBell (FRAME_X_DISPLAY! (f), 0); }
      XFlush (FRAME_X_DISPLAY! (f));
      x_stop_ignoring_errors (dpyinfo);
      unblock_input ();
    }
}

/* Perform an insert-lines or delete-lines operation, inserting N
   lines or deleting -N lines at vertical position VPOS.  */

/* C 11694-11699: `x_ins_del_lines`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_ins_del_lines (f: *mut frame, vpos: c_int, n: c_int)
{
  emacs_abort ();
}

/* C 11701-11915: `x_scroll_run`.  The window/frame box comes from `window_box` and the mirror's
   `WINDOW_TO_FRAME_PIXEL_Y!`; the two `HAVE_XWIDGETS`/`USE_CAIRO_XCB_SURFACE` blocks and the
   cairo block's three arms are C's own, and the two `else`/`else if` sites whose conditions are
   configuration-dependent keep C's chain by nesting the arm the configuration selects.  The
   `run` fields are the mirror's `struct run` (`include/dispextern.rs` 1981), `gui_clear_cursor`
   is dispextern.h's prototype with no mirror row, and `XCopyArea`/`XUnmapWindow`/
   `XMoveResizeWindow`/`XFree`/`XQueryTree`/`XFlushGC` are Xlib's (the ledger's class).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_scroll_run (w: *mut window, run: *mut run)
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut x: c_int;
  let mut y: c_int;
  let mut width: c_int;
  let mut height: c_int;
  let mut from_y: c_int;
  let mut to_y: c_int;
  let mut bottom_y: c_int;

  /* Get frame-relative bounding box of the text display area of W,
     without mode lines.  Include in this box the left and right
     fringe of W.  */
  window_box (w, ANY_AREA, &raw mut x, &raw mut y, &raw mut width, &raw mut height);

  from_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).current_y);
  to_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).desired_y);
  bottom_y = y + height;

  if to_y < from_y
    {
      /* Scrolling up.  Make sure we don't copy part of the mode
	 line at the bottom.  */
      if from_y + (*run).height > bottom_y
	{ height = bottom_y - from_y; }
      else
	{ height = (*run).height; }

      /* Don't draw over the display parts above if destination is off
         the top of the current window.  */
      if to_y < y
	{
	  let d: c_int = y - to_y;
	  height -= d;
	  to_y += d;
	  from_y += d;
	}
    }
  else
    {
      /* Scrolling down.  Make sure we don't copy over the mode line.
	 at the bottom.  */
      if to_y + (*run).height > bottom_y
	{ height = bottom_y - to_y; }
      else
	{ height = (*run).height; }
    }

  block_input ();

  /* Cursor off.  Will be switched on again in gui_update_window_end.  */
  gui_clear_cursor (w);

  #[cfg (feature = "have_xwidgets")]
  {
    /* "Copy" xwidget windows in the area that will be scrolled.  */
    let dpy: *mut Display = FRAME_X_DISPLAY! (f);
    let window: Window = FRAME_X_WINDOW! (f);

    let mut root: Window;
    let mut parent: Window;
    let mut children: *mut Window;
    let mut nchildren: c_uint;

    if XQueryTree (dpy, window, &raw mut root, &raw mut parent, &raw mut children,
		   &raw mut nchildren) != 0
      {
	/* Now find xwidget views situated between from_y and to_y, and
	   attached to w.  */
	let mut i: c_uint = 0;
	while i < nchildren
	  {
	    let child: Window = *children.offset (i as isize);
	    let view: *mut xwidget_view = xwidget_view_from_window (child);

	    if !view.is_null () && !(*view).hidden
	      {
		let window_y: c_int = (*view).y + (*view).clip_top;
		let window_height: c_int = (*view).clip_bottom - (*view).clip_top;

		let mut r1: Emacs_Rectangle = std::mem::zeroed ();
		let mut r2: Emacs_Rectangle = std::mem::zeroed ();
		let mut result: Emacs_Rectangle = std::mem::zeroed ();
		r1.x = (*w).pixel_left as c_short;
		r1.y = from_y as c_short;
		r1.width = (*w).pixel_width as c_ushort;
		r1.height = height as c_ushort;
		r2 = r1;
		r2.y = window_y as c_short;
		r2.height = window_height as c_ushort;

		/* The window is offscreen, just unmap it.  */
		if window_height == 0
		  {
		    (*view).hidden = true;
		    XUnmapWindow (dpy, child);
		    i += 1;
		    continue;
		  }

		let intersects_p: bool =
		  gui_intersect_rectangles (&r1, &r2, &mut result);

		if XWINDOW ((*view).w) == w && intersects_p
		  {
		    /* C's own `int y = view->y + (to_y - from_y);` shadows the box's `y` for
		       the rest of this block (Rust shadows it the same way).  */
		    let mut y: c_int = (*view).y + (to_y - from_y);
		    let mut text_area_x: c_int;
		    let mut text_area_y: c_int;
		    let mut text_area_width: c_int;
		    let mut text_area_height: c_int;
		    let mut clip_top: c_int;
		    let mut clip_bottom: c_int;

		    window_box (w, (*view).area, &raw mut text_area_x, &raw mut text_area_y,
				&raw mut text_area_width, &raw mut text_area_height);

		    (*view).y = y;

		    clip_top = 0;
		    clip_bottom = (*XXWIDGET ((*view).model)).height;

		    if y < text_area_y
		      { clip_top = text_area_y - y; }

		    if (y + clip_bottom) > (text_area_y + text_area_height)
		      {
			clip_bottom -= (y + clip_bottom) - (text_area_y + text_area_height);
		      }

		    (*view).clip_top = clip_top;
		    (*view).clip_bottom = clip_bottom;

		    /* This means the view has moved offscreen.  Unmap
		       it and hide it here.  */
		    if ((*view).clip_bottom - (*view).clip_top) <= 0
		      {
			(*view).hidden = true;
			XUnmapWindow (dpy, child);
		      }
		    else
		      {
			XMoveResizeWindow (dpy, child, (*view).x + (*view).clip_left,
					   (*view).y + (*view).clip_top,
					   (*view).clip_right - (*view).clip_left,
					   (*view).clip_bottom - (*view).clip_top);
			cairo_xlib_surface_set_size ((*view).cr_surface,
						     (*view).clip_right - (*view).clip_left,
						     (*view).clip_bottom - (*view).clip_top);
		      }
		    xwidget_expose (view);
		  }
	      }
	    i += 1;
	  }
	XFree (children as *mut c_void);
      }
  }

  #[cfg (all (feature = "use_xcb", feature = "use_cairo_xcb"))]
  {
    /* Some of the following code depends on `normal_gc' being
       up-to-date on the X server, but doesn't call a routine that will
       flush it first.  So do this ourselves instead.  */
    XFlushGC (FRAME_X_DISPLAY! (f),
	      (*( (*f).output_data.x as *mut x_output )).normal_gc);
  }

  #[cfg (feature = "use_cairo")]
  {
    if !FRAME_CR_CONTEXT! (f).is_null ()
      {
	let surface: *mut cairo_surface_t = cairo_get_target (FRAME_CR_CONTEXT! (f));
	if cairo_surface_get_type (surface) == CAIRO_SURFACE_TYPE_XLIB
	  {
	    eassert! (cairo_xlib_surface_get_display (surface)
		      == FRAME_X_DISPLAY! (f));
	    eassert! (cairo_xlib_surface_get_drawable (surface)
		      == FRAME_X_RAW_DRAWABLE! (f));
	    cairo_surface_flush (surface);
	    XCopyArea (FRAME_X_DISPLAY! (f),
		       FRAME_X_DRAWABLE! (f), FRAME_X_DRAWABLE! (f),
		       (*( (*f).output_data.x as *mut x_output )).normal_gc,
		       x, from_y,
		       width, height,
		       x, to_y);
	    cairo_surface_mark_dirty_rectangle (surface, x, to_y, width, height);
	  }
	else
	  {
	    #[cfg (all (feature = "use_xcb", feature = "use_cairo_xcb"))]
	    {
	      if cairo_surface_get_type (surface) == CAIRO_SURFACE_TYPE_XCB
		{
		  cairo_surface_flush (surface);
		  xcb_copy_area ((*FRAME_DISPLAY_INFO! (f)).xcb_connection,
				 FRAME_X_DRAWABLE! (f) as xcb_drawable_t,
				 FRAME_X_DRAWABLE! (f) as xcb_drawable_t,
				 XGContextFromGC ((*( (*f).output_data.x as *mut x_output )).normal_gc) as xcb_gcontext_t,
				 x, from_y, x, to_y, width, height);
		  cairo_surface_mark_dirty_rectangle (surface, x, to_y, width, height);
		}
	      else
	    {
	      let s: *mut cairo_surface_t
		= cairo_surface_create_similar (surface,
						cairo_surface_get_content (surface),
						width, height);
	      let mut cr: *mut cairo_t = cairo_create (s);
	      cairo_set_source_surface (cr, surface, -x as c_double, -from_y as c_double);
	      cairo_paint (cr);
	      cairo_destroy (cr);

	      cr = FRAME_CR_CONTEXT! (f);
	      cairo_save (cr);
	      cairo_set_source_surface (cr, s, x as c_double, to_y as c_double);
	      cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);
	      cairo_rectangle (cr, x as c_double, to_y as c_double,
			       width as c_double, height as c_double);
	      cairo_fill (cr);
	      cairo_restore (cr);
	      cairo_surface_destroy (s);
	    }
	    }
	    #[cfg (not (all (feature = "use_xcb", feature = "use_cairo_xcb")))]
	    {
	      let s: *mut cairo_surface_t
		= cairo_surface_create_similar (surface,
						cairo_surface_get_content (surface),
						width, height);
	      let mut cr: *mut cairo_t = cairo_create (s);
	      cairo_set_source_surface (cr, surface, -x as c_double, -from_y as c_double);
	      cairo_paint (cr);
	      cairo_destroy (cr);

	      cr = FRAME_CR_CONTEXT! (f);
	      cairo_save (cr);
	      cairo_set_source_surface (cr, s, x as c_double, to_y as c_double);
	      cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);
	      cairo_rectangle (cr, x as c_double, to_y as c_double,
			       width as c_double, height as c_double);
	      cairo_fill (cr);
	      cairo_restore (cr);
	      cairo_surface_destroy (s);
	    }
	  }
      }
    else
      {
	XCopyArea (FRAME_X_DISPLAY! (f),
		   FRAME_X_DRAWABLE! (f), FRAME_X_DRAWABLE! (f),
		   (*( (*f).output_data.x as *mut x_output )).normal_gc,
		   x, from_y,
		   width, height,
		   x, to_y);
      }
  }

  #[cfg (not (feature = "use_cairo"))]
  {
    XCopyArea (FRAME_X_DISPLAY! (f),
	       FRAME_X_DRAWABLE! (f), FRAME_X_DRAWABLE! (f),
	       (*( (*f).output_data.x as *mut x_output )).normal_gc,
	       x, from_y,
	       width, height,
	       x, to_y);
  }

  unblock_input ();
}
/* Create invisible cursor on the X display referred by DPYINFO.  */
/// C 12067-12095: `static Cursor make_invisible_cursor (struct x_display_info *dpyinfo)` - `static`
/// in C with one call site (C 12110), so a plain `unsafe fn`.  `no_data` is C's **function-`static`**
/// one-byte array and stays in the function where C puts it (PORTING.md 1.3); `XColor col` is C's
/// uninitialised local handed to `XCreatePixmapCursor` by address, so the port zeroes it
/// (Deviation 12) and C 12084's chained `col.red = col.green = col.blue = 0` is written out in C's
/// order; `None` is `include/xlib.rs`'s `x11_constants::None` (X.h 115) and this slice's three new
/// rows are its `DoRed`/`DoGreen`/`DoBlue` (X.h 622-624, measured values 1/2/4) with
/// `XCreatePixmapCursor` (Xlib.h 1554-1562) beside the mirror's other bitmap entries.  C's `!rc`
/// on `x_had_errors_p`'s `int` return is the port's `== 0`, and `col.flags` is XColor's `char`
/// field, so C's implicit narrowing of the flag expression is written out (`as c_char`).
/// X11/X.h 622-624: `#define DoRed (1<<0)` / `#define DoGreen (1<<1)` / `#define DoBlue (1<<2)` -
/// the three bits `XColor.flags` carries, which C 12085's `DoRed | DoGreen | DoBlue` sets.  They
/// are declared here rather than in `include/xlib.rs`'s `x11_constants` because this slice's write
/// scope is this unit and the mirror round that moves them (with the row below) is the xlib
/// ledger's to land (P-88 1's use-site rule, the `src/xfns.rs` 1155-1161 Xlib rows' shape; the
/// unit's own `#![allow (non_upper_case_globals, …)]` at line 638 covers the three names).
#[cfg (feature = "have_x_windows")]
pub const DoRed: c_int = 1;
#[cfg (feature = "have_x_windows")]
pub const DoGreen: c_int = 2;
#[cfg (feature = "have_x_windows")]
pub const DoBlue: c_int = 4;

#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1554-1562: `extern Cursor XCreatePixmapCursor (Display *, Pixmap, Pixmap,
  /// XColor *, XColor *, unsigned int, unsigned int);` - C 12086's call (the `src/xfns.rs`
  /// 1155-1161 Xlib rows' use-site shape).
  pub fn XCreatePixmapCursor (display: *mut Display, source: Pixmap, mask: Pixmap,
                              foreground_color: *mut XColor, background_color: *mut XColor,
                              x: c_uint, y: c_uint) -> Cursor;
}

#[cfg (feature = "have_x_windows")]
unsafe fn make_invisible_cursor (dpyinfo: *mut x_display_info) -> Cursor
{
  use x11_constants::None;

  let dpy: *mut Display = (*dpyinfo).display;                           // C 12070
  static no_data: [c_char; 1] = [0];                                    // C 12071
  let pix: Pixmap;                                                      // C 12072
  let mut col: XColor = core::mem::zeroed ();                           // C 12073
  let mut c: Cursor;                                                    // C 12074

  c = None as Cursor;                                                   // C 12076

  x_catch_errors (dpy);                                                 // C 12078
  pix = XCreateBitmapFromData (dpy, (*dpyinfo).root_window,             // C 12079
			       no_data.as_ptr (), 1, 1);
  if !x_had_errors_p (dpy) && pix != None as Pixmap                 // C 12080
    {
      let pixc: Cursor;                                                 // C 12082
      col.pixel = 0;                                                    // C 12083
      col.red = 0;                                                      // C 12084
      col.green = 0;
      col.blue = 0;
      col.flags = (DoRed | DoGreen | DoBlue) as c_char;                 // C 12085
      pixc = XCreatePixmapCursor (dpy, pix, pix, &raw mut col,          // C 12086
				  &raw mut col, 0, 0);
      if !x_had_errors_p (dpy) && pixc != None as Cursor            // C 12087
	{ c = pixc; }                                                   // C 12088
      XFreePixmap (dpy, pix);                                           // C 12089
    }

  x_uncatch_errors ();                                                  // C 12092

  return c;                                                             // C 12094
}
/* C 26217-26239's `struct x_error_message_stack`.  */
/// C 26217-26239: `struct x_error_message_stack` - this file's own record.  `handler`'s type is
/// xterm.h 1748-1749's `x_special_error_handler`, declared at this use site (P-88 1); its
/// function-pointer position takes the mirror's `Option<unsafe extern "C" fn (…)>` shape
/// (PORTING.md 1.4).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct x_error_message_stack
{
  pub string: *mut c_char,                       // C 26221: `char *string;`
  pub dpy: *mut Display,                         // C 26224: `Display *dpy;`
  pub handler: x_special_error_handler,          // C 26227: `x_special_error_handler handler;`
  pub handler_data: *mut c_void,                 // C 26230: `void *handler_data;`
  pub prev: *mut x_error_message_stack,          // C 26233: `struct x_error_message_stack *prev;`
  pub first_request: c_ulong,                    // C 26238: `unsigned long first_request;`
}

/// C: xterm.h 1748-1749: `typedef void (*x_special_error_handler) (Display *, XErrorEvent *,
/// char *, void *);`
#[cfg (feature = "have_x_windows")]
pub type x_special_error_handler
  = Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent, *mut c_char, *mut c_void)>;

/* Stack of X error message handlers.  Whenever an error is generated
   on a display, look in this stack for an appropriate error handler,
   set its `string' to the error message and call its `handler' with
   `handler_data'.  If no handler applies to the error, don't catch
   it, and let it crash Emacs instead.

   This used to be a pointer to a string in which any error would be
   placed before 2006.  */
/// C 26249: `static struct x_error_message_stack *x_error_message;`
#[cfg (feature = "have_x_windows")]
static mut x_error_message: *mut x_error_message_stack = std::ptr::null_mut ();

/// C 26252: `int x_error_message_count;` - a **non-`static`** C definition, so it is exported
/// (PORTING.md 0.2); `include/xterm.rs` 1644 carries the header's own declaration for the units
/// that read it (the `x_display_list` shape this unit already uses at its 774-775).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub static mut x_error_message_count: c_int = 0;

/* Compare various request serials while handling wraparound.  Treat a
   difference of more than X_ULONG_MAX / 2 as wraparound.

   Note that these functions truncate serials to 32 bits before
   comparison.  */
/// C 26260-26267: `static bool x_is_serial_more_than (unsigned int a, unsigned int b)` - C's
/// `b - a` is `unsigned int` arithmetic that wraps and then widens to `unsigned long` for the
/// comparison, so the port writes the wrap out (`wrapping_sub`) and the widening (`as c_ulong`);
/// C's callers hand these functions `unsigned long` serials and the parameters truncate them to
/// 32 bits (C 26257-26258 says so), which the call sites write out (`as c_uint`).
#[cfg (feature = "have_x_windows")]
unsafe fn x_is_serial_more_than (a: c_uint, b: c_uint) -> bool
{
  if a > b                                                              // C 26263
    { return true; }                                                    // C 26264

  return b.wrapping_sub (a) as c_ulong > X_ULONG_MAX / 2;               // C 26266
}

/// C 26269-26276: `static bool x_is_serial_more_than_or_equal_to (unsigned int a, unsigned int b)`
/// (the `>=` twin of the above, same wrap and widening).
#[cfg (feature = "have_x_windows")]
unsafe fn x_is_serial_more_than_or_equal_to (a: c_uint, b: c_uint) -> bool
{
  if a >= b                                                             // C 26272
    { return true; }                                                    // C 26273

  return b.wrapping_sub (a) as c_ulong > X_ULONG_MAX / 2;               // C 26275
}

/// C 26278-26285: `static bool x_is_serial_less_than (unsigned int a, unsigned int b)`.
#[cfg (feature = "have_x_windows")]
unsafe fn x_is_serial_less_than (a: c_uint, b: c_uint) -> bool
{
  if a < b                                                              // C 26281
    { return true; }                                                    // C 26282

  return a.wrapping_sub (b) as c_ulong > X_ULONG_MAX / 2;               // C 26284
}

/// C 26287-26294: `static bool x_is_serial_less_than_or_equal_to (unsigned int a, unsigned int b)`.
#[cfg (feature = "have_x_windows")]
unsafe fn x_is_serial_less_than_or_equal_to (a: c_uint, b: c_uint) -> bool
{
  if a <= b                                                             // C 26290
    { return true; }                                                    // C 26291

  return a.wrapping_sub (b) as c_ulong > X_ULONG_MAX / 2;               // C 26293
}

/// C 26296-26314: `static struct x_error_message_stack *x_find_error_handler (Display *dpy,
/// XErrorEvent *event)` - C's `while (stack)` is the port's null test and C's `NULL` its
/// `null_mut ()`.
#[cfg (feature = "have_x_windows")]
unsafe fn x_find_error_handler (dpy: *mut Display, event: *mut XErrorEvent)
  -> *mut x_error_message_stack
{
  let mut stack: *mut x_error_message_stack;                            // C 26299

  stack = x_error_message;                                              // C 26301

  while !stack.is_null ()                                               // C 26303
    {
      if x_is_serial_more_than_or_equal_to ((*event).serial as c_uint,  // C 26305
					    (*stack).first_request as c_uint)  // C 26306
	  && dpy == (*stack).dpy                                        // C 26307
	{ return stack; }                                               // C 26308

      stack = (*stack).prev;                                            // C 26310
    }

  return std::ptr::null_mut ();                                         // C 26313
}

/// C 26317-26323: `void x_unwind_errors_to (int depth)` - a **non-`static`** C definition
/// (`extern "C"` + `#[unsafe(no_mangle)]`, PORTING.md 0.2).  C 26322's `x_uncatch_errors` is
/// C 26568's own definition, which this unit has not reached yet - it stands as one `E0425` row
/// until that slice lands (P-91 1's not-yet-ported-definition class).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_unwind_errors_to (depth: c_int)
{
  while x_error_message_count > depth                                   // C 26319
    /* This is safe to call because we check whether or not
       x_error_message->dpy is still alive before calling XSync.  */
    { x_uncatch_errors (); }                                            // C 26322
}

/* An X error handler which stores the error message in the first
   applicable handler in the x_error_message stack.  This is called
   from *x_error_handler if an x_catch_errors for DISPLAY is in
   effect.  */

/// C 26330-26347: `static void x_error_catcher (Display *display, XErrorEvent *event,
/// struct x_error_message_stack *stack)` - the address C 27006 hands to `XSetErrorHandler`, so
/// `unsafe extern "C" fn` (PORTING.md 1.4).  `buf` is C's uninitialised local filled by
/// `XGetErrorText`, so the port zeroes it (Deviation 12); `event->error_code` is `unsigned char`
/// and `XGetErrorText`'s second parameter is an `int`, so C's widening is written out; and C's
/// `if (stack->handler)` plus call is the port's `is_some`/`unwrap` (the `xterm.rs`
/// `surface_set_size_func` shape C 7760 uses on the pgtk side).
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_error_catcher (display: *mut Display, event: *mut XErrorEvent,
				      stack: *mut x_error_message_stack)
{
  let mut buf: [c_char; X_ERROR_MESSAGE_SIZE as usize] =
    [0; X_ERROR_MESSAGE_SIZE as usize];                                 // C 26334: `char buf[…];`

  XGetErrorText (display, (*event).error_code as c_int,                  // C 26336
		 buf.as_mut_ptr (), X_ERROR_MESSAGE_SIZE);              // C 26337

  if !(*stack).string.is_null ()                                        // C 26339
    { xfree ((*stack).string as *mut c_void); }                         // C 26340

  (*stack).string = xstrdup (buf.as_ptr ());                            // C 26342

  if (*stack).handler.is_some ()                                        // C 26344
    {
      ((*stack).handler.unwrap ()) (display, event, (*stack).string,    // C 26345
				   (*stack).handler_data);              // C 26346
    }
}
/* Begin trapping X errors for display DPY.

   After calling this function, X protocol errors generated on DPY no
   longer cause Emacs to exit; instead, they are recorded in an error
   handler pushed onto the stack `x_error_message'.

   Calling x_check_errors signals an Emacs error if an X error has
   occurred since the last call to x_catch_errors or x_check_errors.

   Calling x_uncatch_errors resumes the normal error handling,
   skipping an XSync if the last request made is known to have been
   processed.  Calling x_uncatch_errors_after_check is similar, but
   always skips an XSync to the server, and should be used only
   immediately after x_had_errors_p or x_check_errors, or when it is
   known that no requests have been made since the last x_catch_errors
   call for DPY.

   There is no need to use this mechanism for ignoring errors from
   single asynchronous requests, such as sending a ClientMessage to a
   window that might no longer exist.  Use
   x_ignore_errors_for_next_request (paired with
   x_stop_ignoring_errors) instead.  */

/// C 26372-26387: `void x_catch_errors_with_handler (Display *dpy, x_special_error_handler
/// handler, void *handler_data)` - a **non-`static`** C definition (`extern "C"` +
/// `#[unsafe(no_mangle)]`, PORTING.md 0.2).  `xzalloc`'s `*mut c_void` needs C's own cast back to
/// the record, and C 26383 advances the display's request serial through the mirror's
/// `XNextRequest` (the exported procedure `NextRequest` degrades to, that mirror's Slice-13 note).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_catch_errors_with_handler (dpy: *mut Display,
						      handler: x_special_error_handler,
						      handler_data: *mut c_void)
{
  let data: *mut x_error_message_stack;                                 // C 26376

  data = xzalloc (core::mem::size_of::<x_error_message_stack> ()) as *mut x_error_message_stack;  // C 26378
  (*data).dpy = dpy;                                                    // C 26379
  (*data).handler = handler;                                            // C 26380
  (*data).handler_data = handler_data;                                  // C 26381
  (*data).prev = x_error_message;                                       // C 26382
  (*data).first_request = XNextRequest (dpy);                           // C 26383
  x_error_message = data;                                               // C 26384

  x_error_message_count += 1;                                           // C 26386
}

/// C 26389-26393: `void x_catch_errors (Display *dpy)` - a **non-`static`** C definition; C's
/// `NULL, NULL` arguments are the port's `None` and `null_mut ()`.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_catch_errors (dpy: *mut Display)
{
  x_catch_errors_with_handler (dpy, None, std::ptr::null_mut ());       // C 26392
}

/* Return if errors for REQUEST should be ignored even if there is no
   error handler applied.  */
/// C 26397-26416: `static struct x_failable_request *x_request_can_fail (struct x_display_info
/// *dpyinfo, unsigned long request)` - C's `for` over a pointer range is the port's
/// init/`while`/step trio (the file's spelling), its `!failable_requests->end` is the port's
/// `== 0`, and the serial arguments truncate as the comparators' parameters do (`as c_uint`,
/// C 26257-26258's rule).
#[cfg (feature = "have_x_windows")]
unsafe fn x_request_can_fail (dpyinfo: *mut x_display_info, request: c_ulong)
  -> *mut x_failable_request
{
  let mut failable_requests: *mut x_failable_request;                   // C 26401

  failable_requests = (*dpyinfo).failable_requests.as_mut_ptr ();       // C 26403
  while failable_requests < (*dpyinfo).next_failable_request             // C 26404
    {
      if x_is_serial_more_than_or_equal_to (request as c_uint,          // C 26407
					    (*failable_requests).start as c_uint)
	  && ((*failable_requests).end == 0                             // C 26409
	      || x_is_serial_less_than_or_equal_to (request as c_uint,  // C 26410
						    (*failable_requests).end as c_uint))
	{ return failable_requests; }                                   // C 26412

      failable_requests = failable_requests.offset (1);                 // C 26405
    }

  return std::ptr::null_mut ();                                         // C 26415
}

/* Remove outdated request serials from
   dpyinfo->failable_requests.  */
/// C 26420-26443: `static void x_clean_failable_requests (struct x_display_info *dpyinfo)` -
/// C's two-pointer `for` is the port's trio, `LastKnownRequestProcessed` is the mirror's macro
/// over the exported procedure, `memmove` is libc's own binding (PORTING.md P-93's rule) and
/// C 26441-26442's pointer arithmetic is written out with `offset`.
#[cfg (feature = "have_x_windows")]
unsafe fn x_clean_failable_requests (dpyinfo: *mut x_display_info)
{
  let mut first: *mut x_failable_request;                               // C 26423
  let last: *mut x_failable_request;

  last = (*dpyinfo).next_failable_request;                              // C 26425

  first = (*dpyinfo).failable_requests.as_mut_ptr ();                   // C 26427
  while first < last                                                   // C 26427
    {
      if x_is_serial_more_than ((*first).start as c_uint,               // C 26429
				LastKnownRequestProcessed! ((*dpyinfo).display) as c_uint)  // C 26430
	  || (*first).end == 0                                          // C 26431
	  || x_is_serial_more_than ((*first).end as c_uint,             // C 26432
				    LastKnownRequestProcessed! ((*dpyinfo).display) as c_uint)  // C 26433
	{ break; }                                                      // C 26434

      first = first.offset (1);                                         // C 26427
    }

  if first != last                                                      // C 26437
    {
      memmove ((*dpyinfo).failable_requests.as_mut_ptr () as *mut c_void,  // C 26438
	       first as *const c_void,
	       core::mem::size_of::<x_failable_request> () * last.offset_from (first) as usize);  // C 26439
    }

  (*dpyinfo).next_failable_request                                      // C 26441-26442
    = (*dpyinfo).failable_requests.as_mut_ptr ().offset (last.offset_from (first));
}

/* Protect a section of X requests.

   Ignore errors generated by X requests made from now until
   `x_stop_ignoring_errors'.  Each call must be paired with a call to
   `x_stop_ignoring_errors', and recursive calls inside the protected
   section are not allowed.

   The advantage over x_catch_errors followed by
   x_uncatch_errors_after_check is that this function does not sync to
   catch errors if requests were made.  It should be used instead of
   those two functions for catching errors around requests that do not
   require a reply.

   If SELECTION_SERIAL is an arbitrary number greater than zero,
   x_select_handle_selection_error is called with the specified number
   after any errors within the protected section are received to
   delete the selection request that encountered errors.  */

/// C 26463-26517: `void x_ignore_errors_for_next_request (struct x_display_info *dpyinfo,
/// unsigned int selection_serial)` - a **non-`static`** C definition, with C 26474-26482's
/// `HAVE_GTK3` arm (`have_gtk3`) and C 26489/26510's two `emacs_abort ()` guards.  C 26486's
/// `(dpyinfo->next_failable_request - 1)->end` is the port's `offset (-1)` plus the field read
/// (the dereference written out), and C 26492's `N_FAILABLE_REQUESTS` comes from the record's
/// mirror.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_ignore_errors_for_next_request (dpyinfo: *mut x_display_info,
							   selection_serial: c_uint)
{
  let mut request: *mut x_failable_request;                             // C 26467
  let max: *mut x_failable_request;
  let next_request: c_ulong;

  /* This code is not reentrant, so be sure nothing calls it
     recursively in response to input.  */
  block_input ();                                                       // C 26472

  #[cfg (feature = "have_gtk3")]
  {
    /* GTK 3 tends to override our own error handler inside certain
       callbacks, which this can be called from.  Instead of trying to
       restore our own, add a trap for the following requests with
       GDK as well.  */
    let gdpy: *mut GdkDisplay = gdk_x11_lookup_xdisplay ((*dpyinfo).display);  // C 26479
    if !gdpy.is_null ()                                                 // C 26480
      { gdk_x11_display_error_trap_push (gdpy); }                       // C 26481
  }

  if (*dpyinfo).next_failable_request                                   // C 26484-26485
       != (*dpyinfo).failable_requests.as_mut_ptr ()
      && (*(*dpyinfo).next_failable_request.offset (-1)).end == 0       // C 26486
    /* A new sequence should never be started before an old one
       finishes.  Use `x_catch_errors' to nest error handlers.  */
    { emacs_abort (); }                                                 // C 26489

  request = (*dpyinfo).next_failable_request;                           // C 26491
  max = (*dpyinfo).failable_requests.as_mut_ptr ().offset (N_FAILABLE_REQUESTS as isize);  // C 26492
  next_request = XNextRequest ((*dpyinfo).display);                     // C 26493

  if request >= max                                                     // C 26495
    {
      /* There is no point in making this extra sync if all requests
	 are known to have been fully processed.  */
      if LastKnownRequestProcessed! ((*dpyinfo).display)                // C 26499
	   != next_request.wrapping_sub (1)                             // C 26500
	{ XSync ((*dpyinfo).display, 0); }                          // C 26501

      x_clean_failable_requests (dpyinfo);                              // C 26503
      request = (*dpyinfo).next_failable_request;                       // C 26504
    }

  if request >= max                                                     // C 26507
    /* A request should always be made immediately after calling this
       function.  */
    { emacs_abort (); }                                                 // C 26510

  (*request).start = next_request;                                      // C 26512
  (*request).end = 0;                                                   // C 26513
  (*request).selection_serial = selection_serial;                       // C 26514

  (*dpyinfo).next_failable_request = request.offset (1);                // C 26516
}

/// C 26519-26541: `void x_stop_ignoring_errors (struct x_display_info *dpyinfo)` - a
/// **non-`static`** C definition with its own C 26533-26538 `HAVE_GTK3` arm.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_stop_ignoring_errors (dpyinfo: *mut x_display_info)
{
  let range: *mut x_failable_request;                                   // C 26522

  range = (*dpyinfo).next_failable_request.offset (-1);                 // C 26524
  (*range).end = XNextRequest ((*dpyinfo).display).wrapping_sub (1);    // C 26525

  /* Abort if no request was made since
     `x_ignore_errors_for_next_request'.  */

  if x_is_serial_less_than ((*range).end as c_uint,                     // C 26530
			    (*range).start as c_uint)
    { emacs_abort (); }                                                 // C 26531

  #[cfg (feature = "have_gtk3")]
  {
    let gdpy: *mut GdkDisplay = gdk_x11_lookup_xdisplay ((*dpyinfo).display);  // C 26534

    if !gdpy.is_null ()                                                 // C 26536
      { gdk_x11_display_error_trap_pop_ignored (gdpy); }                // C 26537
  }

  unblock_input ();                                                     // C 26540
}

/* Undo the last x_catch_errors call.
   DPY should be the display that was passed to x_catch_errors.

   This version should be used only if the immediately preceding
   X-protocol-related thing was x_check_errors or x_had_error_p, both
   of which issue XSync calls, so we don't need to re-sync here.  */

/// C 26550-26563: `void x_uncatch_errors_after_check (void)` - a **non-`static`** C definition.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_uncatch_errors_after_check ()
{
  let tmp: *mut x_error_message_stack;                                  // C 26553

  block_input ();                                                       // C 26555
  tmp = x_error_message;                                                // C 26556
  x_error_message = (*x_error_message).prev;                            // C 26557
  x_error_message_count -= 1;                                           // C 26558
  if !(*tmp).string.is_null ()                                          // C 26559
    { xfree ((*tmp).string as *mut c_void); }                           // C 26560
  xfree (tmp as *mut c_void);                                           // C 26561
  unblock_input ();                                                     // C 26562
}

/* Undo the last x_catch_errors call.  */

/// C 26568-26607: `void x_uncatch_errors (void)` - a **non-`static`** C definition; C 26577's
/// `== NULL` is the port's `is_null ()` and C 26586's `dpyinfo != 0` its inverse, with the
/// `XSync`/`NextRequest`/`XNextRequest` rows the mirror now carries.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_uncatch_errors ()
{
  let tmp: *mut x_error_message_stack;                                  // C 26570
  let dpyinfo: *mut x_display_info;

  /* In rare situations when running Emacs run in daemon mode,
     shutting down an emacsclient via delete-frame can cause
     x_uncatch_errors to be called when x_error_message is set to
     NULL.  */
  if x_error_message.is_null ()                                         // C 26577
    { return; }                                                         // C 26578

  block_input ();                                                       // C 26580

  dpyinfo = x_display_info_for_display ((*x_error_message).dpy);         // C 26582

  /* The display may have been closed before this function is called.
     Check if it is still open before calling XSync.  */
  if !dpyinfo.is_null ()                                                // C 26586
      /* There is no point in making this extra sync if all requests
	 are known to have been fully processed.  */
      && LastKnownRequestProcessed! ((*x_error_message).dpy)            // C 26589
	   != XNextRequest ((*x_error_message).dpy).wrapping_sub (1)     // C 26590
      /* Likewise if no request was made since the trap was
	 installed.  */
      && NextRequest! ((*x_error_message).dpy)                          // C 26593
	   > (*x_error_message).first_request                            // C 26594
    {
      XSync ((*x_error_message).dpy, 0);                            // C 26596
      x_clean_failable_requests (dpyinfo);                              // C 26597
    }

  tmp = x_error_message;                                                // C 26600
  x_error_message = (*x_error_message).prev;                            // C 26601
  x_error_message_count -= 1;                                           // C 26602
  if !(*tmp).string.is_null ()                                          // C 26603
    { xfree ((*tmp).string as *mut c_void); }                           // C 26604
  xfree (tmp as *mut c_void);                                           // C 26605
  unblock_input ();                                                     // C 26606
}

/* If any X protocol errors have arrived since the last call to
   x_catch_errors or x_check_errors, signal an Emacs error using
   sprintf (a buffer, FORMAT, the x error message text) as the text.  */

/// C 26613-26645: `void x_check_errors (Display *dpy, const char *format)` - a **non-`static`**
/// C definition.  C 26640's `alloca` is the port's stack pool (`sa_stack_alloc`, PORTING.md
/// P-05) and C 26641's `strcpy` libc's own binding (P-93's rule); `format` is a C string the
/// caller passes, so it stays `*const c_char`.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_check_errors (dpy: *mut Display, format: *const c_char)
{
  let dpyinfo: *mut x_display_info;                                     // C 26616
  let string: *mut c_char;

  /* This shouldn't happen, since x_check_errors should be called
     immediately inside an x_catch_errors block.  */
  if dpy != (*x_error_message).dpy                                      // C 26621
    { emacs_abort (); }                                                 // C 26622

  /* There is no point in making this extra sync if all requests
     are known to have been fully processed.  */
  if LastKnownRequestProcessed! (dpy)                                   // C 26626
       != XNextRequest (dpy).wrapping_sub (1)                           // C 26627
      && NextRequest! (dpy)                                             // C 26628
	   > (*x_error_message).first_request                           // C 26629
    { XSync (dpy, 0); }                                             // C 26630

  dpyinfo = x_display_info_for_display (dpy);                           // C 26632

  /* Clean the array of failable requests, since a sync happened.  */
  if !dpyinfo.is_null ()                                                // C 26635
    { x_clean_failable_requests (dpyinfo); }                            // C 26636

  if !(*x_error_message).string.is_null ()                              // C 26638
    {
      string = sa_stack_alloc (strlen ((*x_error_message).string) + 1) as *mut c_char;  // C 26640
      strcpy (string, (*x_error_message).string);                       // C 26641

      error (format, string);                                           // C 26643
    }
}

/* Nonzero if any X protocol errors were generated since the last call
   to x_catch_errors on DPY.  */

/// C 26650-26674: `bool x_had_errors_p (Display *dpy)` - a **non-`static`** C definition; C
/// 26673's `!!x_error_message->string` is the port's null test.
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_had_errors_p (dpy: *mut Display) -> bool
{
  let dpyinfo: *mut x_display_info;                                     // C 26653

  /* This shouldn't happen, since x_check_errors should be called
     immediately inside an x_catch_errors block.  */
  if dpy != (*x_error_message).dpy                                      // C 26657
    { emacs_abort (); }                                                 // C 26658

  /* Make sure to catch any errors incurred so far.  */
  if LastKnownRequestProcessed! (dpy)                                   // C 26661
       != XNextRequest (dpy).wrapping_sub (1)                           // C 26662
      && NextRequest! (dpy)                                             // C 26663
	   > (*x_error_message).first_request                           // C 26664
    { XSync (dpy, 0); }                                             // C 26665

  dpyinfo = x_display_info_for_display (dpy);                           // C 26667

  /* Clean the array of failable requests, since a sync happened.  */
  if !dpyinfo.is_null ()                                                // C 26670
    { x_clean_failable_requests (dpyinfo); }                            // C 26671

  return !(*x_error_message).string.is_null ();                         // C 26673
}

/* Forget about any errors we have had, since we did x_catch_errors on
   DPY.  */

/// C 26679-26689: `void x_clear_errors (Display *dpy)` - a **non-`static`** C definition; C
/// 26688's `NULL` store is the port's `null_mut ()` and `xfree` takes C's `void *` (P-93 3).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_clear_errors (dpy: *mut Display)
{
  /* This shouldn't happen, since x_check_errors should be called
     immediately inside an x_catch_errors block.  */
  if dpy != (*x_error_message).dpy                                      // C 26684
    { emacs_abort (); }                                                 // C 26685

  xfree ((*x_error_message).string as *mut c_void);                     // C 26687
  (*x_error_message).string = std::ptr::null_mut ();                    // C 26688
}

/* C 26691-26702: `#if false` - dead text in every configuration.  C keeps `x_fully_uncatch_errors`
   and the comment above it inside the false branch ("See comment in unwind_to_catch why calling
   this is a bad idea.  --lorentey"), so the port carries the block as the comment it is (the
   C 5127-5156 `#if false` shape this file's gate table records).

     #if false
           /* See comment in unwind_to_catch why calling this is a bad
            * idea.  --lorentey   */
     /* Close off all unclosed x_catch_errors calls.  */

     void
     x_fully_uncatch_errors (void)
     {
       while (x_error_message)
         x_uncatch_errors ();
     }
     #endif
*/
/* C 26704-26713: a second `#if false` pair (`x_wire_count` and `x_trace_wire`), carried as the
   comment it is (the C 5127-5156/26691-26702 shape this file's gate table records).

     #if false
     static unsigned int x_wire_count;

     static int
     x_trace_wire (Display *dpy)
     {
       fprintf (stderr, "Lib call: %u\n", ++x_wire_count);
       return 0;
     }
     #endif
*/

/************************************************************************
			  Handling X errors
 ************************************************************************/

/* Error message passed to x_connection_closed.  */

/// C 26722: `static char *error_msg;` - C leaves it uninitialised (a file-scope pointer), so the
/// port's zero value is the null pointer.
#[cfg (feature = "have_x_windows")]
static mut error_msg: *mut c_char = std::ptr::null_mut ();

/* Try to find a frame in Vframe_list, and make it the selected frame.
   `delete_frame' sometimes misses the initial frame for an unknown
   reason when Emacs is running as a background daemon.  */

/// C 26728-26738: `static void x_try_restore_frame (void)` - C's `FOR_EACH_FRAME` body is the
/// mirror's three-argument macro (this unit's C 10073 shape) and `do_switch_frame` comes from
/// `include/frame.rs`'s rows.
#[cfg (feature = "have_x_windows")]
unsafe fn x_try_restore_frame ()
{
  let mut tail: Lisp_Object;                                            // C 26731
  let mut frame: Lisp_Object;

  FOR_EACH_FRAME! (tail, frame, {                                       // C 26733
    if !NILP (do_switch_frame (frame, 0, 1, Qnil ()))                   // C 26735
      { return; }                                                       // C 26736
  });
}

/* Handle the loss of connection to display DPY.  ERROR_MESSAGE is
   the text of an error message that lead to the connection loss.  */

/* C 26743-26992: `static void x_connection_closed (Display *dpy, const char *error_message, bool
   ioerror)` - the connection-loss teardown, this file's second-longest function.  Four shapes
   worth naming: `error_msg` is C's file-scope pointer handed to the **stack pool**
   (`sa_stack_alloc`, P-05 - C's `alloca` here outlives the frame only because the function never
   returns, C 26991's `error` being a non-local exit); `current_display` is C's own
   **function-`static`** and stays in the function (PORTING.md 1.3); C 26863/26887's two labels
   with backward `goto`s are the port's labelled `loop`s with `continue 'label` (the C 1960
   `'retry_drag_window` shape this unit already uses); and `dpyinfo->failable_requests` is the
   record's **array**, so C's pointer decay is written `.as_mut_ptr ()` (the Slice-13 reading's
   rule).  `shut_down_emacs` (C 26911) and `clear_waiting_for_input` (C 26985) have no row in this
   unit's include graph and are declared at the use site below (P-88 1).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: dispnew.h 3327: `extern void clear_waiting_for_input (void);` - C 26985's call.
  pub fn clear_waiting_for_input ();
  /// C: lisp.h 5119: `extern Lisp_Object do_switch_frame (Lisp_Object, int, int, Lisp_Object);` -
  /// C 26735's call (`src/frame.rs` defines it; no mirror row is in this unit's graph).
  pub fn do_switch_frame (frame: Lisp_Object, track: c_int, for_deletion: c_int,
                          norecord: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1509: `extern Lisp_Object delete_frame (Lisp_Object, Lisp_Object);` - C 26879 and
  /// C 26895's calls (`src/frame.rs` defines it; the `src/pgtkterm.rs` use-site row is its twin).
  pub fn delete_frame (frame: Lisp_Object, force: Lisp_Object) -> Lisp_Object;
  /// C: Xlib.h 1857-1858: `extern XIOErrorHandler XSetIOErrorHandler (XIOErrorHandler);` - C 26783
  /// and C 26826's calls; the parameter is this unit's own `Emacs_XIOErrorHandler` type (unit 699,
  /// the earlier slices' binding of Xlib.h 1853's typedef, which `src/xterm.rs` 1789 already uses).
  pub fn XSetIOErrorHandler (handler: Emacs_XIOErrorHandler) -> Emacs_XIOErrorHandler;
}

#[cfg (feature = "have_x_windows")]
unsafe fn x_connection_closed (dpy: *mut Display, error_message: *const c_char, ioerror: bool)
{
  use x11_constants::None;

  let mut dpyinfo: *mut x_display_info;                                 // C 26746
  let mut frame: Lisp_Object;                                           // C 26747
  let mut tail: Lisp_Object;
  let idx: specpdl_ref = SPECPDL_INDEX ();                              // C 26748
  let io_error_handler: Emacs_XIOErrorHandler;                          // C 26749
  let mut dmsg: xm_drop_start_message = core::mem::zeroed ();            // C 26750
  let f: *mut frame;                                                    // C 26751
  let mut minibuf_frame: Lisp_Object;                                   // C 26752
  let tmp: Lisp_Object;
  let mut failable: *mut x_failable_request;                            // C 26753
  let mut stack: *mut x_error_message_stack;                            // C 26754
  static mut current_display: *mut Display = std::ptr::null_mut ();     // C 26755

  /* Prevent recursive calls of this function for the same display.
     This is because destroying a frame might still cause an IO error
     in some cases.  (bug#56528) */
  if current_display == dpy                                             // C 26760
    { return; }                                                         // C 26761

  current_display = dpy;                                                // C 26763

  dpyinfo = x_display_info_for_display (dpy);                           // C 26765
  error_msg = sa_stack_alloc (strlen (error_message) + 1) as *mut c_char;  // C 26766
  strcpy (error_msg, error_message);                                    // C 26767

  /* Inhibit redisplay while frames are being deleted. */
  specbind (Qinhibit_redisplay (), Qt ());                              // C 26770

  /* If drag-and-drop is in progress, cancel drag-and-drop.  If DND
     frame's display is DPY, don't reset event masks or try to send
     responses to other programs because the display is going
     away.  */

  if x_dnd_in_progress || x_dnd_waiting_for_finish                       // C 26777
    {
      if !ioerror                                                        // C 26779
	{
	  /* Handle display disconnect errors here because this function
	     is not reentrant at this particular spot.  */
	  io_error_handler = XSetIOErrorHandler (Some (x_dnd_io_error_handler));  // C 26783

	  if sigsetjmp (&raw mut x_dnd_disconnect_handler, 1) != 0           // C 26785
	      && x_dnd_in_progress                                          // C 26786
	      && dpy == (if x_dnd_waiting_for_finish                        // C 26787
			   { x_dnd_finish_display as *mut Display }
			 else
			   { FRAME_X_DISPLAY! (x_dnd_frame) })              // C 26789
	    {
	      /* Clean up drag and drop if the drag frame's display isn't
		 the one being disconnected.  */
	      f = x_dnd_frame;                                              // C 26793

	      if x_dnd_last_seen_window != None as Window                   // C 26795
		  && x_dnd_last_protocol_version != -1                     // C 26796
		{
		  x_dnd_send_leave (x_dnd_frame,                            // C 26797
				    x_dnd_last_seen_window,
				    x_dnd_last_seen_toplevel);              // C 26799
		}
	      else if x_dnd_last_seen_window != None as Window              // C 26800
		       && XM_DRAG_STYLE_IS_DROP_ONLY! (x_dnd_last_motif_style) == false  // C 26801
		       && x_dnd_last_motif_style != xm_drag_style::XM_DRAG_STYLE_NONE as c_int  // C 26802
		       && x_dnd_motif_setup_p                               // C 26803
		{
		  dmsg.reason = XM_DRAG_REASON! (xm_drag_originator::XM_DRAG_ORIGINATOR_INITIATOR
						 as c_int,
						 xm_drag_reason::XM_DRAG_REASON_DROP_START
						 as c_int) as u8;       // C 26805
		  dmsg.byte_order = XM_BYTE_ORDER_CUR_FIRST as u8;          // C 26807
		  dmsg.timestamp = (*FRAME_DISPLAY_INFO! (f)).last_user_time as u32;  // C 26808
		  dmsg.side_effects                                         // C 26809
		    = XM_DRAG_SIDE_EFFECT! (xm_side_effect_from_action (FRAME_DISPLAY_INFO! (f),
								       x_dnd_wanted_action),  // C 26811
					   xm_drop_site_status::XM_DROP_SITE_VALID as c_int,
					   x_dnd_motif_operations as c_int,
					   xm_drag_action::XM_DROP_ACTION_DROP_CANCEL
					   as c_int) as u16;                // C 26812-26813
		  dmsg.x = 0;                                               // C 26814
		  dmsg.y = 0;                                               // C 26815
		  dmsg.index_atom = x_dnd_motif_atom as u32;                // C 26816
		  dmsg.source_window = FRAME_X_WINDOW! (f) as u32;          // C 26817

		  x_dnd_send_xm_leave_for_drop (FRAME_DISPLAY_INFO! (f), f,  // C 26819
						x_dnd_last_seen_window, 0);     // C 26820
		  xm_send_drop_message (FRAME_DISPLAY_INFO! (f), FRAME_X_WINDOW! (f),  // C 26821
					x_dnd_last_seen_window, &raw mut dmsg);  // C 26822
		}
	    }

	  XSetIOErrorHandler (io_error_handler);                            // C 26826
	}

      dpyinfo = x_display_info_for_display (dpy);                       // C 26829

      x_dnd_last_seen_window = None as Window;                          // C 26831
      x_dnd_last_seen_toplevel = None as Window;                        // C 26832
      x_dnd_in_progress = false;                                        // C 26833
      x_dnd_waiting_for_finish = false;                                 // C 26834

      if x_dnd_use_toplevels                                           // C 26836
	{ x_dnd_free_toplevels (!ioerror); }                            // C 26837

      x_dnd_return_frame_object = std::ptr::null_mut ();                // C 26839
      x_dnd_movement_frame = std::ptr::null_mut ();                     // C 26840
      x_dnd_wheel_frame = std::ptr::null_mut ();                        // C 26841
      x_dnd_frame = std::ptr::null_mut ();                              // C 26842
    }

  if !dpyinfo.is_null ()                                                // C 26845
    {
      /* Protect display from being closed when we delete the last
         frame on it. */
      (*dpyinfo).reference_count += 1;                                  // C 26849
      (*(*dpyinfo).terminal).reference_count += 1;                       // C 26850
      if ioerror                                                        // C 26851
	{ (*dpyinfo).display = std::ptr::null_mut (); }                 // C 26852
    }

  /* delete_frame can still try to read async input (even though we
     tell pass `noelisp'), because looking up the `delete-before'
     parameter calls Fassq which then calls maybe_quit.  So block
     input while deleting frames.  */
  block_input ();                                                       // C 26859

  /* First delete frames whose mini-buffers are on frames
     that are on the dead display.  */
  'delete_more_minibuffer_frames: loop                                  // C 26863
    {
      FOR_EACH_FRAME! (tail, frame, {                                   // C 26864
	/* Tooltip frames don't have these, so avoid crashing.  */

	if FRAME_TOOLTIP_P (XFRAME (frame))                              // C 26868
	  { continue; }                                                 // C 26869

	minibuf_frame                                                   // C 26871
	  = WINDOW_FRAME! (XWINDOW (FRAME_MINIBUF_WINDOW (XFRAME (frame))));  // C 26872

	if FRAME_X_P (XFRAME (frame))                                   // C 26874
	    && FRAME_X_P (XFRAME (minibuf_frame))                       // C 26875
	    && !EQ (frame, minibuf_frame)                                // C 26876
	    && FRAME_DISPLAY_INFO! (XFRAME (minibuf_frame)) == dpyinfo  // C 26877
	  {
	    delete_frame (frame, Qnoelisp ());                          // C 26879
	    continue 'delete_more_minibuffer_frames;                    // C 26880
	  }
      });
      break;
    }

  /* Now delete all remaining frames on the dead display.
     We are now sure none of these is used as the mini-buffer
     for another frame that we need to delete.  */
  'delete_more_frames: loop                                             // C 26887
    {
      FOR_EACH_FRAME! (tail, frame, {                                   // C 26888
	if FRAME_X_P (XFRAME (frame))                                   // C 26889
	    && FRAME_DISPLAY_INFO! (XFRAME (frame)) == dpyinfo          // C 26890
	  {
	    /* Set this to t so that delete_frame won't get confused
	       trying to find a replacement.  */
	    kset_default_minibuffer_frame (FRAME_KBOARD (XFRAME (frame)) as *mut kboard,
					     Qt ());  // C 26894
	    delete_frame (frame, Qnoelisp ());                          // C 26895
	    continue 'delete_more_frames;                               // C 26896
	  }
      });
      break;
    }

  /* If DPYINFO is null, this means we didn't open the display in the
     first place, so don't try to close it.  */
  if !dpyinfo.is_null ()                                                // C 26901
    {
      /* We can not call XtCloseDisplay here because it calls XSync.
         XSync inside the error handler apparently hangs Emacs.  On
         current Xt versions, this isn't needed either.  */
      #[cfg (feature = "use_gtk")]
      {
	/* A long-standing GTK bug prevents proper disconnect handling
	   <https://gitlab.gnome.org/GNOME/gtk/issues/221>.  Once,
	   the resulting Glib error message loop filled a user's disk.
	   To avoid this, kill Emacs unconditionally on disconnect.  */
	shut_down_emacs (0, Qnil ());                                   // C 26911
	fprintf (stderr, c"%s\nWhen compiled with GTK, Emacs cannot recover from X disconnects.\nThis is a GTK bug: https://gitlab.gnome.org/GNOME/gtk/issues/221\nFor details, see etc/PROBLEMS.\n".as_ptr (),  // C 26912-26915
		 error_msg);                                            // C 26916
	emacs_abort ();                                                 // C 26917
      }

      /* Indicate that this display is dead.  */
      (*dpyinfo).display = std::ptr::null_mut ();                       // C 26921

      (*dpyinfo).reference_count -= 1;                                  // C 26923
      (*(*dpyinfo).terminal).reference_count -= 1;                      // C 26924
      if (*dpyinfo).reference_count != 0                                // C 26925
	/* We have just closed all frames on this display. */
	{ emacs_abort (); }                                             // C 26927

      /* This was the last terminal remaining, so print the error
	 message and associated error handlers and kill Emacs.  */
      if (*dpyinfo).terminal == terminal_list                           // C 26931
	  && (*terminal_list).next_terminal.is_null ()                  // C 26932
	{
	  fprintf (stderr, c"%s\n".as_ptr (), error_msg);                // C 26934

	  if !ioerror && !dpyinfo.is_null ()                            // C 26936
	    {
	      /* Dump the list of error handlers for debugging
		 purposes if the list exists.  */

	      if (*dpyinfo).failable_requests.as_mut_ptr ()             // C 26941
		   != (*dpyinfo).next_failable_request                  // C 26942
		  || !x_error_message.is_null ()
		{ fprintf (stderr, c"X error handlers currently installed:\n".as_ptr ()); }  // C 26943

	      failable = (*dpyinfo).failable_requests.as_mut_ptr ();     // C 26945
	      while failable < (*dpyinfo).next_failable_request          // C 26946
		{
		  if (*failable).end != 0                                // C 26949
		    { fprintf (stderr, c"Ignoring errors between %lu to %lu\n".as_ptr (),  // C 26950
			       (*failable).start, (*failable).end); }    // C 26951
		  else
		    { fprintf (stderr, c"Ignoring errors from %lu onwards\n".as_ptr (),  // C 26953
			       (*failable).start); }                     // C 26954
		  failable = failable.offset (1);                        // C 26947
		}

	      stack = x_error_message;                                   // C 26957
	      while !stack.is_null ()                                    // C 26957
		{
		  fprintf (stderr, c"Trapping errors from %lu\n".as_ptr (),  // C 26958
			   (*stack).first_request);                      // C 26959
		  stack = (*stack).prev;                                 // C 26957
		}
	    }
	}

      XSETTERMINAL! (tmp, (*dpyinfo).terminal);                          // C 26963
      Fdelete_terminal (tmp, Qnoelisp ());                               // C 26964
    }

  /* The initial "daemon" frame is sometimes not selected by
     `delete_frame' when Emacs is a background daemon.  */
  if NILP (selected_frame) || !FRAME_LIVE_P (XFRAME (selected_frame))  // C 26969
    { x_try_restore_frame (); }                                         // C 26970

  unblock_input ();                                                     // C 26972

  /* Sometimes another terminal is still alive, but deleting this
     terminal caused all frames to vanish.  In that case, simply kill
     Emacs, since the next redisplay will abort as there is no more
     selected frame.  (bug#56528) */
  if terminal_list.is_null () || NILP (selected_frame)                  // C 26978-26979
      || !FRAME_LIVE_P (XFRAME (selected_frame))
    { Fkill_emacs (make_fixnum (70), Qnil ()); }                        // C 26980

  totally_unblock_input ();                                             // C 26982

  unbind_to (idx, Qnil ());                                             // C 26984
  clear_waiting_for_input ();                                           // C 26985

  /* Here, we absolutely have to use a non-local exit (e.g. signal, throw,
     longjmp), because returning from this function would get us back into
     Xlib's code which will directly call `exit'.  */
  current_display = std::ptr::null_mut ();                              // C 26990
  error (c"%s".as_ptr (), error_msg);                                   // C 26991
}
/* C 26994: `static void x_error_quitter (Display *, XErrorEvent *);` - C's prototype for the
   definition below; Rust has no prototypes and the definition keeps C's position (PORTING.md
   0.1-12), so the line is this comment.  */

/* This is the first-level handler for X protocol errors.
   It calls x_error_quitter or x_error_catcher.  */

/* C 26999-27073: `static int x_error_handler (Display *display, XErrorEvent *event)` - a `static`
   whose **address** goes to `XSetErrorHandler` (C 30791/32358), so it is an `unsafe extern "C" fn`
   (PORTING.md 1.4).  The `USE_GTK && HAVE_GTK3` arm's two error codes and its request-code test are
   the measured constants declared at this use site (X.h 353/358/373 and Xproto.h 2063, all four
   measured on this machine's xorgproto 2025.1); `event->error_code`/`request_code`/`minor_code` are
   XErrorEvent's `unsigned char` fields, so the comparisons write C's promotion out; C 27027's
   `memmove (&dpyinfo->failable_requests, …)` and C 27029's pointer arithmetic keep the array decay
   slice 13's reading established; and C 27027-27029 and C 27038's `x_handle_selection_error` are the
   use-site rows below (xterm.h 1924, `src/xselect.rs` 1465 defines it).  */
#[cfg (feature = "have_x_windows")]
pub const BadWindow: c_int = 3;                                         // C: X.h 353 (measured)
#[cfg (feature = "have_x_windows")]
pub const BadMatch: c_int = 8;                                          // C: X.h 358 (measured)
#[cfg (feature = "have_x_windows")]
pub const BadName: c_int = 15;                                          // C: X.h 373 (measured)
#[cfg (feature = "have_x_windows")]
pub const X_SetInputFocus: c_int = 42;                                  // C: Xproto.h 2063 (measured)

#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: xterm.h 1924: `extern void x_handle_selection_error (unsigned int, XErrorEvent *);` -
  /// C 27038's call.
  pub fn x_handle_selection_error (serial: c_uint, error: *mut XErrorEvent);
  /// C: Xlib.h 1737: `extern char *XDisplayString (Display *);` - the procedure C's `DisplayString`
  /// macro (which reads `_XPrivDisplay`) degrades to, the C 2070/unit 1803 spelling.
  pub fn XDisplayString (display: *mut Display) -> *mut c_char;
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_error_handler (display: *mut Display, event: *mut XErrorEvent) -> c_int
{
  let stack: *mut x_error_message_stack;                                // C 27002
  let dpyinfo: *mut x_display_info;                                     // C 27003
  let fail: *mut x_failable_request;                                    // C 27004
  let last: *mut x_failable_request;

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
    if ((*event).error_code == BadMatch as c_uchar                         // C 27007
	|| (*event).error_code == BadWindow as c_uchar)                    // C 27008
	&& (*event).request_code == X_SetInputFocus as c_uchar             // C 27009
      { return 0; }                                                     // C 27010
  }

  dpyinfo = x_display_info_for_display (display);                       // C 27013

  if !dpyinfo.is_null ()                                                // C 27015
    {
      fail = x_request_can_fail (dpyinfo, (*event).serial);              // C 27017

      if !fail.is_null ()                                                // C 27019
	{
	  /* Now that this request sequence has been fully handled,
	     remove it from the list of requests that can fail.  */

	  if (*event).serial == (*fail).end                                 // C 27024
	    {
	      last = (*dpyinfo).next_failable_request;                      // C 27026
	      memmove ((*dpyinfo).failable_requests.as_mut_ptr () as *mut c_void,  // C 27027
		       fail as *const c_void,
		       core::mem::size_of::<x_failable_request> () * last.offset_from (fail) as usize);  // C 27028
	      (*dpyinfo).next_failable_request                              // C 27029-27030
		= (*dpyinfo).failable_requests.as_mut_ptr ().offset (last.offset_from (fail));
	    }

	  /* If a selection transfer is the cause of this error,
	     remove the selection transfer now.  */

	  if (*fail).selection_serial != 0                                  // C 27036
	    {
	      x_handle_selection_error ((*fail).selection_serial,            // C 27038
					event);                             // C 27039

	      /* Clear selection_serial to prevent
		 x_handle_selection_error from being called again if
		 any more requests within the protected section cause
		 errors to be reported.  */
	      (*fail).selection_serial = 0;                                 // C 27045
	    }

	  return 0;                                                         // C 27048
	}
    }

  /* If we try to ungrab or grab a device that doesn't exist anymore
     (that happens a lot in xmenu.c), just ignore the error.  */

  #[cfg (feature = "have_xinput2")]
  {
    /* Handle errors from some specific XI2 requests here to avoid a
       sync in handle_one_xevent.  */
    if !dpyinfo.is_null ()                                              // C 27058
	&& (*dpyinfo).supports_xi2                                      // C 27058
	&& (*event).request_code as c_int == (*dpyinfo).xi2_opcode       // C 27059
	&& ((*event).minor_code == X_XIGrabDevice! () as c_uchar             // C 27060
	    || (*event).minor_code == X_XIUngrabDevice! () as c_uchar        // C 27061
	    || (*event).minor_code == X_XIAllowEvents! () as c_uchar)        // C 27062
      { return 0; }                                                     // C 27063
  }

  stack = x_find_error_handler (display, event);                        // C 27066

  if !stack.is_null ()                                                  // C 27068
    { x_error_catcher (display, event, stack); }                        // C 27069
  else
    { x_error_quitter (display, event); }                               // C 27071
  return 0;                                                             // C 27072
}

/* This is the usual handler for X protocol errors.
   It kills all frames on the display that we got the error for.
   If that was the only one, it prints an error message and kills Emacs.  */

/* .gdbinit puts a breakpoint here, so make sure it is not inlined.  */

/// C 27081-27107: `static void NO_INLINE x_error_quitter (Display *display, XErrorEvent *event)` -
/// a `static` that C only *calls* (C 27071), so a plain `unsafe fn`; C's `NO_INLINE` is an
/// optimization attribute with no Rust counterpart, spelled `#[inline(never)]` (the
/// `include/conf_post.rs` 204-208 reading of the name).  `buf1`'s size is C's own constant
/// expression (`800 + INT_STRLEN_BOUND (int) + INT_STRLEN_BOUND (unsigned long) +
/// INT_STRLEN_BOUND (XID) + INT_STRLEN_BOUND (int)`), **measured** on this machine as **858**
/// (`10 + 19 + 19 + 10`); `sizeof (buf)` is `core::mem::size_of_val` (unit 8124's note) and C's
/// adjacent string literals become one `c"…"` (PORTING.md 0.1-6).  C's bug address is this
/// repository's own now: the literal carries the value `PACKAGE_BUGREPORT` holds
/// (`src/emacs.rs` 360, PORTING.md 0.1-19's ninth half).
#[cfg (feature = "have_x_windows")]
#[inline(never)]
unsafe fn x_error_quitter (display: *mut Display, event: *mut XErrorEvent)
{
  let mut buf: [c_char; 256] = [0; 256];                                // C 27084: `char buf[256]`
  let mut buf1: [c_char; 858] = [0; 858];                               // C 27084-27087

  /* Ignore BadName errors.  They can happen because of fonts
     or colors that are not defined.  */

  if (*event).error_code == BadName as c_uchar                          // C 27092
    { return; }                                                         // C 27093

  /* Note that there is no real way portable across R3/R4 to get the
     original error handler.  */

  XGetErrorText (display, (*event).error_code as c_int, buf.as_mut_ptr (),  // C 27098
		 core::mem::size_of_val (&buf) as c_int);
  sprintf (buf1.as_mut_ptr (), c"X protocol error: %s on protocol request %d\nSerial no: %lu\nFailing resource ID (if any): 0x%lx\nMinor code: %d\nThis is a bug!  Please report this to https://github.com/liuchong/femacs/issues!\n".as_ptr (),  // C 27099-27103
	   buf.as_mut_ptr (), (*event).request_code as c_int, (*event).serial,
	   (*event).resourceid, (*event).minor_code as c_int);           // C 27104
  x_connection_closed (display, buf1.as_mut_ptr (), false);             // C 27106
}


/* This is the handler for X IO errors, always.
   It kills all frames on the display that we lost touch with.
   If that was the only one, it prints an error message and kills Emacs.  */

/// C 27114-27126: `static int NO_INLINE x_io_error_quitter (Display *display)` - a `static` whose
/// **address** goes to `XSetIOErrorHandler` (C 30792/32359), so an `unsafe extern "C" fn`
/// (PORTING.md 1.4) with C's `NO_INLINE` spelled `#[inline(never)]`.  C's `DisplayString` macro
/// dereferences Xlib's private record and degrades to `XDisplayString` (the C 2070/unit 1803
/// spelling); `fmt` is C's **function-`static`** array and stays in the function (PORTING.md 1.3),
/// with `sizeof fmt - sizeof "%s"` written `size_of_val (&fmt) - 3` (`"%s"` plus its NUL).
#[cfg (feature = "have_x_windows")]
#[inline(never)]
unsafe extern "C" fn x_io_error_quitter (display: *mut Display) -> c_int
{
  let server: *const c_char = XDisplayString (display);                 // C 27117
  static fmt: &core::ffi::CStr = c"Connection lost to X server '%s'";   // C 27118
  USE_SAFE_ALLOCA! (sa_avail, sa_count);                                // C 27119
  let buf: *mut c_char                                                  // C 27120
    = SAFE_ALLOCA! (sa_avail, 30 + strlen (server) + 1) as *mut c_char;  // C's `sizeof fmt - sizeof "%s"` is 30
  sprintf (buf, fmt.as_ptr (), server);                                 // C 27121 (fmt is a `&CStr`)
  x_connection_closed (display, buf, true);                             // C 27122
  SAFE_FREE! (sa_count);                                                // C 27123

  return 0;                                                             // C 27125
}

/* Changing the font of the frame.  */

/* Give frame F the font FONT-OBJECT as its default font.  The return
   value is FONT-OBJECT.  FONTSET is an ID of the fontset for the
   frame.  If it is negative, generate a new fontset from
   FONT-OBJECT.  */

/// C: Xlib.h 1090: `typedef struct _XOC *XOC, *XFontSet;` - `struct x_output`'s `xic_xfs` row is
/// declared with it, so the name has to exist wherever that mirror's `have_x_i18n` arm is on.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XFontSet = *mut c_void;
/// C: Xlib.h 1141: `typedef struct _XIM *XIM;` - `struct x_display_info`'s `xim` row.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIM = *mut c_void;
/// C: Xlib.h 1142: `typedef struct _XIC *XIC;` - `struct x_output`'s `xic` row.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIC = *mut c_void;
/// C: Xlib.h 1162: `typedef unsigned long XIMStyle;` - `struct x_output`'s `xic_style` row, and
/// the type C 27184's mask is taken in.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub type XIMStyle = c_ulong;
/// C: Xlib.h 1164-1167: `typedef struct { unsigned short count_styles; XIMStyle
/// *supported_styles; } XIMStyles;` - measured `sizeof 16 align 8`, offsets `count_styles 0`
/// `supported_styles 8` (the `src/xfns.rs` 3860 row's measurement).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XIMStyles
{
  pub count_styles: c_ushort,   // C: unsigned short count_styles;
  pub supported_styles: *mut XIMStyle,   // C: XIMStyle *supported_styles;
}
/// C: Xlib.h 1169-1176: the preedit/status masks the mirror's five `STYLE_*` rows OR together.
/// C's `#define`s are `long` literals and `XIMStyle` is `unsigned long`; the only uses are those
/// `STYLE_*` constants and C 27184's test, both `XIMStyle`, so the port writes the conversion at
/// the use site here (the `src/xfns.rs` 4112-4135 spelling, which types them `XIMStyle`).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditArea: XIMStyle = 0x0001;                            // C: Xlib.h 1169
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditCallbacks: XIMStyle = 0x0002;                       // C: Xlib.h 1170
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditPosition: XIMStyle = 0x0004;                        // C: Xlib.h 1171
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMPreeditNothing: XIMStyle = 0x0008;                         // C: Xlib.h 1172
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStatusArea: XIMStyle = 0x0100;                             // C: Xlib.h 1174
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
pub const XIMStatusNothing: XIMStyle = 0x0400;                          // C: Xlib.h 1176

/// C 27136-27194: `static Lisp_Object x_new_font (struct frame *f, Lisp_Object font_object, int
/// fontset)` - a `static` whose **address** goes to `terminal->set_new_font_hook` (C 32271), so
/// the ABI is `extern "C"` (PORTING.md 1.4, the `pgtk_new_font` shape).  The frame.h accessors C
/// *writes* need the write arm the mirror does not carry (`include/frame.rs` has the getters),
/// so the five two-arm macros below are this unit's, the `src/pgtkterm.rs` 312-336 shape; the
/// functions C calls from other units are declared at the use site (P-72 rule 4) with the
/// prototypes the reference's own headers carry (`fontset.h` 39/44, `dispextern.h` 3631,
/// `window.h` 930 and `xfns.c` 3814's non-static `xic_set_xfontset`); `FRAME_XIC`/`FRAME_XIC_STYLE`
/// are `include/xterm.rs`'s macros; and C 27184's two XIM style bits are measured Xlib constants
/// (Xlib.h 1171/1174, `XIMPreeditPosition` 4 and `XIMStatusArea` 256).
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_COLUMN_WIDTH {
  ($f:expr) => { FRAME_COLUMN_WIDTH ($f) };
  ($f:expr, $v:expr) => { (*$f).column_width = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_LINE_HEIGHT {
  ($f:expr) => { FRAME_LINE_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).line_height = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr) => { FRAME_MENU_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).menu_bar_height = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
#[cfg (feature = "have_x_windows")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_COLS ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: dispextern.h 3631: `extern void get_font_ascent_descent (struct font *, int *, int *);`
  pub fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int, descent: *mut c_int);
  /// C: window.h 930: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
  /// Lisp_Object);` - C 27178's call.
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int, inhibit: c_int,
			    pretend: bool, parameter: Lisp_Object);
  /// C: xfns.c 3814: `void xic_set_xfontset (struct frame *f, const char *base_fontname);` - the
  /// `HAVE_X_I18N` arm's call (a non-static definition in this tree's `src/xfns.rs`).
  pub fn xic_set_xfontset (f: *mut frame, base_fontname: *const c_char);
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_new_font (f: *mut frame, font_object: Lisp_Object,
				 mut fontset: c_int) -> Lisp_Object
{
  let font: *mut font = XFONT_OBJECT (font_object);                     // C 27139
  let unit: c_int;                                                      // C 27140
  let mut font_ascent: c_int = 0;
  let mut font_descent: c_int = 0;

  if fontset < 0                                                       // C 27142
    { fontset = fontset_from_font (font_object); }
  FRAME_FONTSET! (f) = fontset;                                         // C 27143
  if FRAME_FONT! (f) == font                                           // C 27144
    /* This font is already set in frame F.  There's nothing more to
       do.  */
    { return font_object; }                                            // C 27147

  FRAME_FONT! (f) = font;                                              // C 27149
  /* C's member lives inside font.h's `#ifdef HAVE_WINDOW_SYSTEM` (font.h 325-403, the field
     at 368), so the write carries the macro the mirror's own row carries (the
     `src/xfns.rs` 1595-1597 shape).  */
  #[cfg (feature = "have_window_system")]
  {
    FRAME_BASELINE_OFFSET! (f) = (*font).baseline_offset;               // C 27150
  }
  FRAME_COLUMN_WIDTH! (f, (*font).average_width);                       // C 27151
  get_font_ascent_descent (font, &raw mut font_ascent, &raw mut font_descent);  // C 27152
  FRAME_LINE_HEIGHT! (f, font_ascent + font_descent);                   // C 27153

  #[cfg (not (feature = "use_x_toolkit"))]
  {
    FRAME_MENU_BAR_HEIGHT! (f, FRAME_MENU_BAR_LINES (f) * FRAME_LINE_HEIGHT (f));  // C 27156
  }
  /* We could use a more elaborate calculation here.  */
  FRAME_TAB_BAR_HEIGHT! (f, FRAME_TAB_BAR_LINES (f) * FRAME_LINE_HEIGHT (f));     // C 27160

  /* Compute character columns occupied by scrollbar.

     Don't do things differently for non-toolkit scrollbars
     (Bug#17163).  */
  unit = FRAME_COLUMN_WIDTH (f);                                        // C 27166
  if FRAME_CONFIG_SCROLL_BAR_WIDTH (f) > 0                              // C 27167
    {
      FRAME_CONFIG_SCROLL_BAR_COLS! (f,                                   // C 27168-27169
	(FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + unit - 1) / unit);
    }
  else
    {
      FRAME_CONFIG_SCROLL_BAR_COLS! (f, (14 + unit - 1) / unit);          // C 27171
    }

  /* Don't change the size of a tip frame; there's no point in doing it
     because it's done in Fx_show_tip, and it leads to problems because
     the tip frame has no widget.  */
  if FRAME_X_WINDOW! (f) != 0 && !FRAME_TOOLTIP_P (f)                   // C 27177
    {
      adjust_frame_size                                                   // C 27178-27180
	(f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
	 FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 3, false, Qfont ());
    }

  #[cfg (feature = "have_x_i18n")]
  {
    if !FRAME_XIC! (f).is_null ()                                       // C 27183
      && (FRAME_XIC_STYLE! (f) & (XIMPreeditPosition | XIMStatusArea)) != 0  // C 27184
      {
	block_input ();                                                 // C 27186
	xic_set_xfontset (f, SSDATA (fontset_ascii (fontset)));         // C 27187
	unblock_input ();                                               // C 27188
      }
  }

  return font_object;                                                   // C 27192
}

/***********************************************************************
			   X Input Methods
***********************************************************************/

/* C 27200-27210: `#ifdef HAVE_X_I18N` / `#ifdef HAVE_X11R6` / `#ifdef HAVE_X11R6_XIM`.
   `HAVE_X_I18N` is the port's `have_x_i18n`; the other two are `undef` in the frozen table
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1686-1687), so everything between C 27210 and
   C 27282 is dead text there, and the arms are the features C tests.  */

/* HAVE_X11R6 means Xlib conforms to the R6 specification or later.
   HAVE_X11R6_XIM, OTOH, means that Emacs should try to use R6-style
   callback driven input method initialization.  They are separate
   because Sun apparently ships buggy Xlib with some versions of
   Solaris... */

/* If preedit text is set on F, cancel preedit, free the text, and
   generate the appropriate events to cancel the preedit display.

   This is mainly useful when the connection to the IM server is
   dropped during preconversion.  */

/// C 27218-27243: `static void x_maybe_clear_preedit (struct frame *f)` - a `static` whose
/// callers are in this file, so a plain `unsafe fn`.  C 27235's `kbd_buffer_store_event` is
/// `include/keyboard.rs`'s declaration, `PREEDIT_TEXT_EVENT` is `include/termhooks.rs`'s
/// constant written through this unit's `input_event_bits!` (the C 1416-1417 spelling), and
/// `ie` is zeroed because C hands an uninitialised `struct input_event` to `EVENT_INIT`
/// (Deviation 12).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
unsafe fn x_maybe_clear_preedit (f: *mut frame)
{
  let output: *mut x_output;                                            // C 27221
  let mut ie: input_event = core::mem::zeroed ();                       // C 27222 (Deviation 12)

  output = FRAME_X_OUTPUT! (f) as *mut x_output;                        // C 27224

  if (*output).preedit_chars.is_null ()                                 // C 27226
    { return; }                                                         // C 27227

  EVENT_INIT (&raw mut ie);                                             // C 27229
  input_event_bits! (kind, &raw mut ie, PREEDIT_TEXT_EVENT);            // C 27230
  ie.arg = Qnil ();                                                     // C 27231
  XSETFRAME! (ie.frame_or_window, f);                                   // C 27232
  XSETINT! (ie.x, 0);                                                   // C 27233
  XSETINT! (ie.y, 0);                                                   // C 27234
  kbd_buffer_store_event (&raw mut ie as *mut c_void);                  // C 27235

  xfree ((*output).preedit_chars as *mut c_void);                       // C 27237

  (*output).preedit_size = 0;                                           // C 27239
  (*output).preedit_active = false;                                     // C 27240
  (*output).preedit_chars = std::ptr::null_mut ();                      // C 27241
  (*output).preedit_caret = 0;                                          // C 27242
}

/* XIM destroy callback function, which is called whenever the
   connection to input method XIM dies.  CLIENT_DATA contains a
   pointer to the x_display_info structure corresponding to XIM.  */

/* C: Xlib.h 2526-2528: `extern int XFree (void *);` - this unit's X text frees Xlib memory in
   sixteen places, so the declaration stands for every X arm (P-72 rule 4); the comment is not a
   `///` one because a doc comment on the block below is an `unused doc comment` warning.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XFree (data: *mut c_void) -> c_int;
}
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
unsafe extern "C" {
  /// C: xfns.c 3023-3025: `void xic_free_xfontset (struct frame *f);` - no `static`, and
  /// `src/xfns.rs` 4592 defines it.
  pub fn xic_free_xfontset (f: *mut frame);
}

/// C 27249-27280: `static void xim_destroy_callback (XIM xim, XPointer client_data, XPointer
/// call_data)` - a `static` whose **address** goes to `XSetIMValues`' `XNDestroyCallback` (this
/// file's `xim_open_dpy`, C 27330), so the ABI is `extern "C"` (PORTING.md 1.4) and the three
/// `XPointer`/`XIM` parameters are Xlib's untyped pointers (`include/termhooks.rs`'s `XPointer`
/// spelling, `lib-src/emacsclient.rs` 106's).  `FRAME_XIC! (f) = NULL` writes the value the
/// mirror's macro names; C 27264's `xic_free_xfontset` is `src/xfns.rs`'s exported function
/// (xfns.c 3025's `xic_free_xfontset (struct frame *f)` has no `static`), and C 27277's XFree is
/// Xlib's (Xlib.h 2526-2528), so both are declared at this use site (P-72 rule 4).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
unsafe extern "C" fn xim_destroy_callback (xim: XIM, client_data: XPointer,
					   call_data: XPointer)
{
  let dpyinfo: *mut x_display_info = client_data as *mut x_display_info;  // C 27252
  let mut frame: Lisp_Object;                                           // C 27253
  let mut tail: Lisp_Object;

  block_input ();                                                       // C 27255

  /* No need to call XDestroyIC.. */
  FOR_EACH_FRAME! (tail, frame, {                                       // C 27258
    let f: *mut frame = XFRAME (frame);                                 // C 27260
    if FRAME_X_P (f) && FRAME_DISPLAY_INFO! (f) == dpyinfo              // C 27261
      {
	FRAME_XIC! (f) = std::ptr::null_mut ();                         // C 27263
	xic_free_xfontset (f);                                          // C 27264

	/* Free the preedit text if necessary.  */
	x_maybe_clear_preedit (f);                                      // C 27267
      }
  });

  /* No need to call XCloseIM.  */
  (*dpyinfo).xim = std::ptr::null_mut ();                               // C 27272

  /* Also free IM values; those are allocated separately upon
     XGetIMValues.  */
  if !(*dpyinfo).xim_styles.is_null ()                                  // C 27276
    { XFree ((*dpyinfo).xim_styles as *mut c_void); }                   // C 27277
  (*dpyinfo).xim_styles = std::ptr::null_mut ();                        // C 27278
  unblock_input ();                                                     // C 27279
}

/* Open the connection to the XIM server on display DPYINFO.
   RESOURCE_NAME is the resource name Emacs uses.  */

/// C: Xlib.h 79: `#define Status int` - `XCloseIM`'s return (`src/xmenu.rs` 477's spelling).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim"))]
pub type Status = c_int;
/// C: Xlib.h 1144-1148: `typedef void (*XIMProc)(XIM, XPointer, XPointer);` - `XIMCallback`'s
/// second field.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim"))]
pub type XIMProc = Option<unsafe extern "C" fn (xim: XIM, client_data: XPointer,
						call_data: XPointer)>;
/// C: Xlib.h 1230-1234: `typedef struct { XPointer client_data; XIMProc callback; }
/// XIMCallback;` - measured `sizeof 16 align 8`, offsets `client_data 0 callback 8` (the
/// `src/xfns.rs` 3883-3895 reading of the same record).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XIMCallback
{
  pub client_data: XPointer,   // C: XPointer client_data;
  pub callback: XIMProc,   // C: XIMProc callback;
}
/* C: Xlib.h 1180: `#define XNQueryInputStyle "queryInputStyle"` / Xlib.h 1187:
   `#define XNDestroyCallback "destroyCallback"` - both are passed as `char *`, so they take the
   function spelling the `src/xfns.rs` 4171 / 4177 pair of the same macro family uses.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim"))]
pub fn XNQueryInputStyle () -> *mut c_char { c"queryInputStyle".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim"))]
pub fn XNDestroyCallback () -> *mut c_char { c"destroyCallback".as_ptr () as *mut c_char }

/* C: the five Xlib entry points C 27300-27329 calls (Xlib.h 3836-3861).  None of them is in
   `include/xlib.rs` yet, so each stands at this use site with the header line it comes from
   (P-72 rule 4); `XGetIMValues`/`XSetIMValues` are variadic, the `src/xfns.rs` 4208-4209 shape.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim"))]
unsafe extern "C" {
  pub fn XOpenIM (display: *mut Display, db: XrmDatabase, res_name: *mut c_char,
		  res_class: *mut c_char) -> XIM;
  pub fn XCloseIM (xim: XIM) -> Status;
  pub fn XGetIMValues (xim: XIM, ...) -> *mut c_char;
  pub fn XSetIMValues (xim: XIM, ...) -> *mut c_char;
  pub fn XLocaleOfIM (xim: XIM) -> *mut c_char;
}

/// C 27289-27341: `static void xim_open_dpy (struct x_display_info *dpyinfo, char
/// *resource_name)` - a `static` whose callers are this file's (C 27364 and C 27429), so a plain
/// `unsafe fn`; it sits inside C 27200's `#ifdef HAVE_X_I18N`.  `HAVE_XIM` is `undef` in the
/// frozen table (`darwin-arm64.macros.tsv` 1700), so C's own body there is the two lines around
/// it - `dpyinfo->xim = NULL;` - and both arms are carried (P-86 ①): the `have_xim` arm and the
/// `#else` text C spells.  `emacs_class` is this unit's `static mut` array (C 858-861,
/// `have_x_i18n`), `safe_calln!` is lisp.h's macro, and `xim_destroy_callback` is C 27324's
/// address (slice 17's `HAVE_X11R6_XIM` gate).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn xim_open_dpy (dpyinfo: *mut x_display_info, resource_name: *mut c_char)
{
  #[cfg (feature = "have_xim")]
  {
    let xim: XIM;                                                       // C 27293
    let locale: *const c_char;                                          // C 27294

    if (*dpyinfo).use_xim                                               // C 27296
      {
	if !(*dpyinfo).xim.is_null ()                                   // C 27298
	  {
	    XCloseIM ((*dpyinfo).xim);                                  // C 27300

	    /* Free values left over from the last time the IM
	       connection was established.  */

	    if !(*dpyinfo).xim_styles.is_null ()                        // C 27305
	      { XFree ((*dpyinfo).xim_styles as *mut c_void); }         // C 27306
	    (*dpyinfo).xim_styles = std::ptr::null_mut ();              // C 27307
	  }

	xim = XOpenIM ((*dpyinfo).display, (*dpyinfo).rdb, resource_name,  // C 27310
		       emacs_class.as_mut_ptr ());                          // C 27311
	(*dpyinfo).xim = xim;                                           // C 27312

	if !xim.is_null ()                                              // C 27314
	  {
	    #[cfg (all (feature = "have_x11r6", feature = "have_x11r6_xim"))]
	    let mut destroy: XIMCallback;                               // C 27317

	    /* Get supported styles and XIM values.  */
	    XGetIMValues (xim, XNQueryInputStyle (),                    // C 27321
			  &raw mut (*dpyinfo).xim_styles,
			  std::ptr::null_mut::<c_void> ());

	    #[cfg (all (feature = "have_x11r6", feature = "have_x11r6_xim"))]
	    {
	      destroy.callback = Some (xim_destroy_callback);           // C 27324
	      destroy.client_data = dpyinfo as XPointer;                // C 27325
	      XSetIMValues (xim, XNDestroyCallback (),                  // C 27326
			    &raw mut destroy, std::ptr::null_mut::<c_void> ());
	    }

	    locale = XLocaleOfIM (xim);                                 // C 27329

	    /* Now try to determine the coding system that should be
	       used.  locale is in Host Portable Character Encoding, and
	       as such can be passed to build_string as is.  */
	    (*dpyinfo).xim_coding = safe_calln! (Vx_input_coding_function,  // C 27334-27335
						 build_string (locale));
	  }
      }
    else
      { (*dpyinfo).xim = std::ptr::null_mut (); }                       // C 27340
  }
  #[cfg (not (feature = "have_xim"))]
  {
    (*dpyinfo).xim = std::ptr::null_mut ();                             // C 27340 (C's `#else` arm)
  }
}

/* XIM instantiate callback function, which is called whenever an XIM
   server is available.  DISPLAY is the display of the XIM.
   CLIENT_DATA contains a pointer to an xim_inst_t structure created
   when the callback was registered.  */

/* C: Xlib.h 1154-1158: `typedef void (*XIDProc)(Display *, XPointer, XPointer);` - the type
   `XRegisterIMInstantiateCallback` takes C 27425's callback as.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
pub type XIDProc = Option<unsafe extern "C" fn (display: *mut Display, client_data: XPointer,
						call_data: XPointer)>;
/* C: Xlib.h 3941-3948: `extern Bool XRegisterIMInstantiateCallback (Display *, struct
   _XrmHashBucketRec *, char *, char *, XIDProc, XPointer);` - C 27421's call.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
unsafe extern "C" {
  pub fn XRegisterIMInstantiateCallback (display: *mut Display, rdb: XrmDatabase,
					 res_name: *mut c_char, res_class: *mut c_char,
					 callback: XIDProc, client_data: XPointer) -> Bool;
}
/* C: xfns.c 3090/3251/3302 - the three entry points C 27381-27385 calls.  None is `static`, and
   `src/xfns.rs` 4701/4878/4943 defines them, so each stands here at the use site (P-72 rule 4).  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
unsafe extern "C" {
  pub fn create_frame_xic (f: *mut frame);
  pub fn xic_set_preeditarea (w: *mut window, x: c_int, y: c_int);
  pub fn xic_set_statusarea (f: *mut frame);
}

/// C 27351-27391: `static void xim_instantiate_callback (Display *display, XPointer client_data,
/// XPointer call_data)` - a `static` whose **address** goes to
/// `XRegisterIMInstantiateCallback` (C 27425), so the ABI is `extern "C"` (PORTING.md 1.4).
/// `xim_inst` is the mirror's record (C 158-166, the same `HAVE_X11R6_XIM` gate),
/// `x_dnd_in_progress` is this unit's `static mut`, and the three XIC entry points are declared
/// just above.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_x11r6",
            feature = "have_x11r6_xim"))]
unsafe extern "C" fn xim_instantiate_callback (display: *mut Display, client_data: XPointer,
					       call_data: XPointer)
{
  let xim_inst: *mut xim_inst_t = client_data as *mut xim_inst_t;       // C 27354
  let dpyinfo: *mut x_display_info = (*xim_inst).dpyinfo;               // C 27355

  if x_dnd_in_progress                                                 // C 27357
    { return; }                                                        // C 27358

  /* We don't support multiple XIM connections. */
  if !(*dpyinfo).xim.is_null ()                                        // C 27361
    { return; }                                                        // C 27362

  xim_open_dpy (dpyinfo, (*xim_inst).resource_name);                    // C 27364

  /* Create XIC for the existing frames on the same display, as long
     as they have no XIC.  */
  if !(*dpyinfo).xim.is_null () && (*dpyinfo).reference_count > 0       // C 27368
    {
      let mut tail: Lisp_Object;                                        // C 27370
      let mut frame: Lisp_Object;

      block_input ();                                                   // C 27372
      FOR_EACH_FRAME! (tail, frame, {                                   // C 27373
	let f: *mut frame = XFRAME (frame);                             // C 27375

	if FRAME_X_P (f)                                                // C 27377
	  && FRAME_DISPLAY_INFO! (f) == (*xim_inst).dpyinfo             // C 27378
	  {
	    if FRAME_XIC! (f).is_null ()                                // C 27379
	      {
		create_frame_xic (f);                                   // C 27381
		if (FRAME_XIC_STYLE! (f) & XIMStatusArea) != 0          // C 27382
		  { xic_set_statusarea (f); }                           // C 27383
		let w: *mut window = XWINDOW ((*f).selected_window);    // C 27384
		xic_set_preeditarea (w, (*w).cursor.x, (*w).cursor.y);  // C 27385
	      }
	  }
      });

      unblock_input ();                                                 // C 27389
    }
}

/* Open a connection to the XIM server on display DPYINFO.
   RESOURCE_NAME is the resource name for Emacs.  On X11R5, open the
   connection only at the first time.  On X11R6, open the connection
   in the XIM instantiate callback function.  */

/// C 27401-27433: `static void xim_initialize (struct x_display_info *dpyinfo, char
/// *resource_name)` - the `#ifdef HAVE_XIM` arm and its `#else` (C 27428-27430, the
/// `xim_open_dpy` call), inside C 27200's `HAVE_X_I18N`; C 27404's write is outside the guards.
/// `xmalloc`/`xstrdup` are `include/lisp.rs`'s, and the callback's `(void *) xim_inst` is C's own
/// cast of the record pointer to the `XPointer` the prototype takes.
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn xim_initialize (dpyinfo: *mut x_display_info, resource_name: *mut c_char)
{
  (*dpyinfo).xim = std::ptr::null_mut ();                               // C 27404

  #[cfg (feature = "have_xim")]
  {
    if (*dpyinfo).use_xim                                              // C 27406
      {
	#[cfg (all (feature = "have_x11r6", feature = "have_x11r6_xim"))]
	{
	  let xim_inst: *mut xim_inst_t                                    // C 27409
	    = xmalloc (core::mem::size_of::<xim_inst_t> ()) as *mut xim_inst_t;

	  (*dpyinfo).xim_callback_data = xim_inst;                         // C 27412
	  (*xim_inst).dpyinfo = dpyinfo;                                   // C 27413
	  (*xim_inst).resource_name = xstrdup (resource_name);             // C 27414

	  /* The last argument is XPointer in XFree86 but (XPointer *) on
	     Tru64, at least, but the configure test doesn't work because
	     xim_instantiate_callback can either be XIMProc or XIDProc, so
	     just cast to void *.  */

	  let ret: Bool                                                 // C 27410/27421-27426
	    = XRegisterIMInstantiateCallback ((*dpyinfo).display,
					      (*dpyinfo).rdb,
					      (*xim_inst).resource_name,
					      emacs_class.as_mut_ptr (),
					      Some (xim_instantiate_callback),
					      xim_inst as XPointer);  // C's `(void *) xim_inst`
	  eassert! (ret);                                                  // C 27427
	}
	#[cfg (not (all (feature = "have_x11r6", feature = "have_x11r6_xim")))]
	{
	  xim_open_dpy (dpyinfo, resource_name);                           // C 27429
	}
      }
  }
}

/* Close the connection to the XIM server on display DPYINFO.
   Unregister any IM instantiation callback previously installed,
   close the connection to the IM server if possible, and free any
   retrieved IM values.  */

/* C: Xlib.h 3950-3957: `extern Bool XUnregisterIMInstantiateCallback (Display *, struct
   _XrmHashBucketRec *, char *, char *, XIDProc, XPointer);` - C 27459's call.  */
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n", feature = "have_xim",
            feature = "have_x11r6", feature = "have_x11r6_xim"))]
unsafe extern "C" {
  pub fn XUnregisterIMInstantiateCallback (display: *mut Display, rdb: XrmDatabase,
					   res_name: *mut c_char, res_class: *mut c_char,
					   callback: XIDProc, client_data: XPointer) -> Bool;
}

/// C 27441-27487: `static void xim_close_dpy (struct x_display_info *dpyinfo)` - a `static` whose
/// callers are this file's, so a plain `unsafe fn`; it sits inside C 27200's `#ifdef HAVE_X_I18N`,
/// whose `#endif` is C 27489's (that line's own comment says "not HAVE_X11R6_XIM", which no `#if`
/// in the region opens - C's comment is stale, and the guard it really closes is `HAVE_X_I18N`,
/// the one this function's gate carries).  `HAVE_XIM` and `HAVE_X11R6_XIM` are both `undef` in the
/// frozen table, so the whole body is dead there and the port carries C's own structure: the
/// `have_xim` arm, its `have_x11r6_xim` sub-arm, and nothing in the `#else` positions - which is
/// what C writes (`#endif /* HAVE_XIM */` with no `#else`).
#[cfg (all (feature = "have_x_windows", feature = "have_x_i18n"))]
unsafe fn xim_close_dpy (dpyinfo: *mut x_display_info)
{
  #[cfg (feature = "have_xim")]
  {
    #[cfg (all (feature = "have_x11r6", feature = "have_x11r6_xim"))]
    {
      let xim_inst: *mut xim_inst_t;                                    // C 27446
      /* C 27447's `Bool rc;` is declared here and assigned at C 27459; the port initialises it
	 where it is assigned, because `eassert!` - its only reader - compiles only under
	 `enable_checking` (the `src/pdumper.rs` 1027-1029 shape).  */

      /* If dpyinfo->xim_callback_data is not set, then IM support wasn't
	 initialized, which can happen if Xlib doesn't understand the C
	 locale being used.  */

      if !(*dpyinfo).xim_callback_data.is_null ()                       // C 27453
	{
	  xim_inst = (*dpyinfo).xim_callback_data;                      // C 27455

	  if !(*dpyinfo).display.is_null ()                             // C 27457
	    {
	      let rc: Bool                                             // C 27459-27464
		= XUnregisterIMInstantiateCallback ((*dpyinfo).display,
						    (*dpyinfo).rdb,
						    (*xim_inst).resource_name,
						    emacs_class.as_mut_ptr (),
						    Some (xim_instantiate_callback),
						    xim_inst as XPointer);
	      eassert! (rc);                                            // C 27465
	    }

	  xfree ((*xim_inst).resource_name as *mut c_void);             // C 27468
	  xfree (xim_inst as *mut c_void);                              // C 27469
	}
    }

    /* Now close the connection to the input method server.  This may
       access the display connection, and isn't safe if the display has
       already been closed.  */

    if !(*dpyinfo).display.is_null () && !(*dpyinfo).xim.is_null ()     // C 27477
      { XCloseIM ((*dpyinfo).xim); }                                    // C 27478
    (*dpyinfo).xim = std::ptr::null_mut ();                             // C 27479

    /* Free the list of XIM styles retrieved.  */

    if !(*dpyinfo).xim_styles.is_null ()                                // C 27483
      { XFree ((*dpyinfo).xim_styles as *mut c_void); }                 // C 27484
    (*dpyinfo).xim_styles = std::ptr::null_mut ();                      // C 27485
  }
}

/* Calculate the absolute position in frame F
   from its current recorded position values and gravity.  */

/// C: Xutil.h 74-75: `#define XNegative 0x0010` / `#define YNegative 0x0020` - the two
/// `size_hint_flags` bits this function tests and clears (measured on this machine).
#[cfg (feature = "have_x_windows")]
pub const XNegative: c_int = 0x0010;
#[cfg (feature = "have_x_windows")]
pub const YNegative: c_int = 0x0020;
/* C: xterm.h 1863-1864: `extern int x_display_pixel_height (struct x_display_info *);` /
   `extern int x_display_pixel_width (struct x_display_info *);` - both are defined in this file
   (C 18482/18491, still unchecked in the ledger's list), so the port declares them at this use
   site until those definitions land (P-72 rule 4).  */
/* C 27531/27573 call `x_display_pixel_width`/`x_display_pixel_height`, which this file defines
   at C 18482/18491 (still unticked below) and xterm.h 1863-1864 declares for *every* X build.
   `include/gtk.rs` is where the port has the pair, and its rows sit inside that mirror's
   `use_gtk` block (`#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]` with a
   `not (have_pgtk)` inner gate), so the non-GTK arms have no name to call and the two `E0425`
   rows stand as this unit's owed record until either the definitions land or that mirror's gate
   matches C's.  The unit does not declare them itself: the mirror owns the name (P-26/P-90 ⑦,
   the `check-mirror-dupes` reading this slice produced).  */
/* C: X11/Intrinsic.h: `typedef struct _WidgetRec *Widget;` / `extern void XtVaGetValues (Widget,
   ...);` / `#define XtNheight "height"` - C 27553's call, in the `USE_X_TOOLKIT && USE_MOTIF`
arm.  libXt's headers are not on this machine, so the citations are the header's own names and
   the `src/xfns.rs` 1203/1389 notes' rule applies; `Widget` is the mirror's field type.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_motif"))]
pub type Widget = *mut c_void;
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_motif"))]
pub fn XtNheight () -> *mut c_char { c"height".as_ptr () as *mut c_char }
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_motif"))]
unsafe extern "C" {
  pub fn XtVaGetValues (widget: Widget, ...);
}

/// C 27496-27581: `static void x_calc_absolute_position (struct frame *f)` - a `static` whose
/// callers are this file's, so a plain `unsafe fn`.  `flags` and `p` keep C's own declarations;
/// `has_been_visible` is a `bool_bf` member of `struct x_output` that `include/xterm.rs` carries
/// as a comment rather than as storage (its own heading), so the port spells C's member name and
/// the two `E0609 no field` rows this produces are this unit's owed record (the
/// `src/xfns.rs` 988-999 reading of the same group); `Fx_frame_edges`/`Fnth`/`Qouter_edges` are
/// `include/globals.rs`'s, and `make_fixnum`/`XFIXNUM` are the mirror's.
#[cfg (feature = "have_x_windows")]
unsafe fn x_calc_absolute_position (f: *mut frame)
{
  let flags: c_int = (*f).size_hint_flags;                              // C 27499
  let p: *mut frame = FRAME_PARENT_FRAME (f);                           // C 27500

  /* We have nothing to do if the current position
     is already for the top-left corner.  */
  if !((flags & XNegative) != 0 || (flags & YNegative) != 0)            // C 27504
    { return; }                                                         // C 27505

  /* Treat negative positions as relative to the leftmost bottommost
     position that fits on the screen.  */
  if (flags & XNegative) != 0                                           // C 27509
    {
      let mut width: c_int = FRAME_PIXEL_WIDTH (f);                     // C 27511

      /* A frame that has been visible at least once should have outer
	 edges.  */
      if (*FRAME_X_OUTPUT! (f)).has_been_visible && p.is_null ()        // C 27515
	{
	  let mut frame: Lisp_Object;                                   // C 27517
	  let mut edges: Lisp_Object = Qnil ();                         // C 27518

	  XSETFRAME! (frame, f);                                        // C 27520
	  edges = Fx_frame_edges (frame, Qouter_edges ());              // C 27521
	  if !NILP (edges)                                              // C 27522
	    {
	      width = (XFIXNUM (Fnth (make_fixnum (2), edges))          // C 27523-27524
		       - XFIXNUM (Fnth (make_fixnum (0), edges))) as c_int;  // C's narrowing to `int`
	    }
	}

      if !p.is_null ()                                                  // C 27527
	{
	  (*f).left_pos = FRAME_PIXEL_WIDTH (p) - width - 2 * (*f).border_width   // C 27528-27529
			  + (*f).left_pos;
	}
      else
	{
	  (*f).left_pos = x_display_pixel_width (FRAME_DISPLAY_INFO! (f))  // C 27531-27532
			  - width + (*f).left_pos;
	}
    }

  if (flags & YNegative) != 0                                           // C 27536
    {
      let mut height: c_int = FRAME_PIXEL_HEIGHT (f);                   // C 27538

      #[cfg (all (feature = "use_x_toolkit", feature = "use_motif"))]
      {
	/* Something is fishy here.  When using Motif, starting Emacs with
	   `-g -0-0', the frame appears too low by a few pixels.  ...  */
	XtVaGetValues ((*FRAME_X_OUTPUT! (f)).column_widget, XtNheight (),  // C 27553
		       &raw mut height, std::ptr::null_mut::<c_void> ());
      }

      if (*FRAME_X_OUTPUT! (f)).has_been_visible && p.is_null ()        // C 27556
	{
	  let mut frame: Lisp_Object;                                   // C 27558
	  let mut edges: Lisp_Object = Qnil ();                         // C 27559

	  XSETFRAME! (frame, f);                                        // C 27561
	  if NILP (edges)                                               // C 27562
	    { edges = Fx_frame_edges (frame, Qouter_edges ()); }        // C 27563
	  if !NILP (edges)                                              // C 27564
	    {
	      height = (XFIXNUM (Fnth (make_fixnum (3), edges))         // C 27565-27566
			- XFIXNUM (Fnth (make_fixnum (1), edges))) as c_int;  // C's narrowing to `int`
	    }
	}

      if !p.is_null ()                                                  // C 27569
	{
	  (*f).top_pos = FRAME_PIXEL_HEIGHT (p) - height - 2 * (*f).border_width  // C 27570-27571
			 + (*f).top_pos;
	}
      else
	{
	  (*f).top_pos = x_display_pixel_height (FRAME_DISPLAY_INFO! (f))  // C 27573-27574
			 - height + (*f).top_pos;
	}
    }

  /* The left_pos and top_pos
     are now relative to the top and left screen edges,
     so the flags should correspond.  */
  (*f).size_hint_flags &= !(XNegative | YNegative);                     // C 27580
}
