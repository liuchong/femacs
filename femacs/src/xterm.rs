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

/* C 1969-1975: C declares it `_Noreturn int`, which the port writes `-> !` (PORTING.md 5.2);
   `siglongjmp` is the libc function declared above.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn xm_drag_window_io_error_handler (dpy: *mut c_void) -> !
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
  rc = x_had_errors_p ((*dpyinfo).display) == 0;
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
    { XFree (tmp_data); }

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
      XSync (temp_display, false);

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
	    { XFree (tmp_data); }

	  if x_had_errors_p ((*dpyinfo).display) != 0
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
      had_errors = x_had_errors_p ((*dpyinfo).display) != 0;
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
	{ XFree (tmp_data); }

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
	  if x_had_errors_p ((*dpyinfo).display) != 0
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

	  if x_had_errors_p ((*dpyinfo).display) == 0
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
    { XFree (tmp_data); }

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

  if x_had_errors_p ((*dpyinfo).display) != 0
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
    { XFree (tmp_data); }

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
      XFree (data);
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
	      && x_had_errors_p ((*dpyinfo).display) == 0) as c_int;

	if rc != 0
	  {
	    rc = (XTranslateCoordinates ((*dpyinfo).display, *toplevels.offset (i as isize),
					 attrs.root, -attrs.border_width,
					 -attrs.border_width, &raw mut dest_x,
					 &raw mut dest_y, &raw mut child) != 0
		  && x_had_errors_p ((*dpyinfo).display) == 0) as c_int;
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
		  && x_had_errors_p ((*dpyinfo).display) == 0
		  && !wmstate_data.is_null () && wmstate_items == 2
		  && format == 32) as c_int;
	  }

	if XGetWindowProperty ((*dpyinfo).display, *toplevels.offset (i as isize),
			       (*dpyinfo).Xatom_net_frame_extents,
			       0, 4, false, XA_CARDINAL, &raw mut r#type,
			       &raw mut format, &raw mut extent_items, &raw mut bytes_after,
			       &raw mut extent_data) == Success
	    && x_had_errors_p ((*dpyinfo).display) == 0
	    && !extent_data.is_null () && extent_items >= 4 && format == 32
	  {
	    frame_extents[0] = *(extent_data as *mut c_ulong).offset (0) as c_int;
	    frame_extents[1] = *(extent_data as *mut c_ulong).offset (1) as c_int;
	    frame_extents[2] = *(extent_data as *mut c_ulong).offset (2) as c_int;
	    frame_extents[3] = *(extent_data as *mut c_ulong).offset (3) as c_int;
	  }

	if !extent_data.is_null ()
	  { XFree (extent_data); }

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

		      XFree (rects);
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

			  XFree (rects);
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
	    XFree (wmstate_data);
	    wmstate_data = std::ptr::null_mut ();
	  }
      }
      i += 1;
    }

  #[cfg (feature = "use_xcb")]
  { SAFE_FREE! (); }

  if !data.is_null ()
    { XFree (data); }

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
#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_dnd_io_error_handler (display: *mut c_void) -> !
{
  #[cfg (feature = "use_gtk")]
  { emacs_abort () }
  #[cfg (not (feature = "use_gtk"))]
  {
    siglongjmp (&raw mut x_dnd_disconnect_handler, 1);
    /* C's arm is `_Noreturn`; `siglongjmp` never returns, so the port gives Rust's `-> !` the
       divergence it needs (the `xm_drag_window_io_error_handler` shape).  */
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
	  && x_had_errors_p ((*dpyinfo).display) == 0
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
      { XFree (data); }
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

		if x_had_errors_p ((*dpyinfo).display) == 0
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

      if x_had_errors_p ((*dpyinfo).display) != 0 || rc == 0
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

	    if x_had_errors_p ((*dpyinfo).display) == 0
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

  if x_had_errors_p ((*dpyinfo).display) == 0
      && rc == Success
      && !tmp_data.is_null ()
      && actual_type == XA_WINDOW
      && actual_format == 32
      && actual_size == 1
    {
      proxy = *(tmp_data as *mut Window);

      /* Verify the proxy window exists.  */
      XGetWindowAttributes ((*dpyinfo).display, proxy, &raw mut attrs);

      if x_had_errors_p ((*dpyinfo).display) != 0
	{ proxy = None as Window; }
    }

  if !tmp_data.is_null ()
    { XFree (tmp_data); }
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
  had_errors = x_had_errors_p ((*dpyinfo).display) != 0;
  x_uncatch_errors_after_check ();

  if had_errors || rc != Success || actual != XA_ATOM || format != 32 || n < 1
      || tmp_data.is_null ()
    {
      if !tmp_data.is_null ()
	{ XFree (tmp_data); }
      return -1;
    }

  value = *(tmp_data as *mut Atom);
  XFree (tmp_data);

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
	{ XFree (children); }
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
