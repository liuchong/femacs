# emacs/src/process.c (9122 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 63
- [x] processp  (Fprocessp, 1/1, doc d9b57349ef40, line 1051)
- [x] get-process  (Fget_process, 1/1, doc 40a40ec91053, line 1058)
- [x] delete-process  (Fdelete_process, 0/1, doc b89a9c339294, line 1129)
- [x] process-status  (Fprocess_status, 1/1, doc 15795bde653d, line 1205)
- [x] process-exit-status  (Fprocess_exit_status, 1/1, doc d0cd33c5b160, line 1251)
- [x] process-id  (Fprocess_id, 1/1, doc 5bbd8ac20a3a, line 1265)
- [x] process-name  (Fprocess_name, 1/1, doc 3c5f91fdb5bf, line 1279)
- [x] process-command  (Fprocess_command, 1/1, doc 76fc6c05fa4d, line 1289)
- [x] process-tty-name  (Fprocess_tty_name, 1/2, doc c4912cbcd671, line 1301)
- [x] set-process-buffer  (Fset_process_buffer, 2/2, doc 093de55d1a02, line 1339)
- [x] process-buffer  (Fprocess_buffer, 1/1, doc c498d6f57d65, line 1362)
- [x] process-mark  (Fprocess_mark, 1/1, doc f77c01d5f5a6, line 1372)
- [x] set-process-filter  (Fset_process_filter, 2/2, doc edb7f14f79c2, line 1405)
- [x] process-filter  (Fprocess_filter, 1/1, doc 1a48340caa60, line 1454)
- [x] set-process-sentinel  (Fset_process_sentinel, 2/2, doc 55f8d00f6959, line 1464)
- [x] process-sentinel  (Fprocess_sentinel, 1/1, doc d0bc11b05d0b, line 1485)
- [x] set-process-thread  (Fset_process_thread, 2/2, doc 47a45c95e052, line 1508)
- [x] process-thread  (Fprocess_thread, 1/1, doc 198bc7f8d734, line 1533)
- [x] set-process-window-size  (Fset_process_window_size, 3/3, doc 18adf9fa0eb2, line 1543)
- [x] set-process-inherit-coding-system-flag  (Fset_process_inherit_coding_system_flag, 2/2, doc 8567d526caeb, line 1564)
- [x] set-process-query-on-exit-flag  (Fset_process_query_on_exit_flag, 2/2, doc a3e135682134, line 1590)
- [x] process-query-on-exit-flag  (Fprocess_query_on_exit_flag, 1/1, doc d5711e0f9219, line 1604)
- [x] process-contact  (Fprocess_contact, 1/3, doc 1869999ddb05, line 1614)
- [x] process-plist  (Fprocess_plist, 1/1, doc cb2551fe8560, line 1671)
- [x] set-process-plist  (Fset_process_plist, 2/2, doc 0b75e90ecbb8, line 1680)
- [x] process-type  (Fprocess_type, 1/1, doc 4fd03cf86585, line 1704)
- [x] format-network-address  (Fformat_network_address, 1/2, doc 02977ab5ff35, line 1716)
- [x] process-list  (Fprocess_list, 0/0, doc 773c45d51158, line 1791)
- [x] make-process  (Fmake_process, 0/MANY, doc 7122f7effc4b, line 1813)
- [x] make-pipe-process  (Fmake_pipe_process, 0/MANY, doc be2127447f4b, line 2441)
- [x] process-datagram-address  (Fprocess_datagram_address, 1/1, doc 5be3e57391ad, line 2824)
- [x] set-process-datagram-address  (Fset_process_datagram_address, 2/2, doc ece193eda132, line 2847)
- [x] set-network-process-option  (Fset_network_process_option, 3/4, doc e4a7cd01347a, line 3016)
- [x] serial-process-configure  (Fserial_process_configure, 0/MANY, doc d56d6e07bdc2, line 3055)
- [x] make-serial-process  (Fmake_serial_process, 0/MANY, doc b37472a92a1f, line 3143)
- [x] make-network-process  (Fmake_network_process, 0/MANY, doc 201a53fc12f3, line 3827)
- [x] network-interface-list  (Fnetwork_interface_list, 0/2, doc c77d59c848a6, line 4660)
- [x] network-interface-info  (Fnetwork_interface_info, 1/1, doc f393fb192ae3, line 4706)
- [x] network-lookup-address-info  (Fnetwork_lookup_address_info, 1/3, doc 572eb659b5c9, line 4767)
- [x] accept-process-output  (Faccept_process_output, 0/4, doc eedd6b7e3ea7, line 4894)
- [x] internal-default-process-filter  (Finternal_default_process_filter, 2/2, doc fa9a8e43617a, line 6609)
- [x] process-send-region  (Fprocess_send_region, 3/3, doc 801ea3209bd2, line 6963)
- [x] process-send-string  (Fprocess_send_string, 2/2, doc 2d95f2353b1f, line 6999)
- [x] process-running-child-p  (Fprocess_running_child_p, 0/1, doc d606d3ff0cac, line 7046)
- [x] internal-default-interrupt-process  (Finternal_default_interrupt_process, 0/2, doc 176cac419594, line 7228)
- [x] interrupt-process  (Finterrupt_process, 0/2, doc 513445a50f7b, line 7240)
- [x] kill-process  (Fkill_process, 0/2, doc cf4ce4583337, line 7261)
- [x] quit-process  (Fquit_process, 0/2, doc 2b112a294b7b, line 7271)
- [x] stop-process  (Fstop_process, 0/2, doc 3306012e3007, line 7280)
- [x] continue-process  (Fcontinue_process, 0/2, doc 45b3c0b434b1, line 7307)
- [x] internal-default-signal-process  (Finternal_default_signal_process, 2/3, doc b3ad4eee7c3d, line 7365)
- [x] signal-process  (Fsignal_process, 2/3, doc 36a01b3d192d, line 7422)
- [x] process-send-eof  (Fprocess_send_eof, 0/1, doc 5afe3d92744c, line 7441)
- [x] internal-default-process-sentinel  (Finternal_default_process_sentinel, 2/2, doc 132246be4ac8, line 7958)
- [x] set-process-coding-system  (Fset_process_coding_system, 1/3, doc 968e6b78e05f, line 8025)
- [x] process-coding-system  (Fprocess_coding_system, 1/1, doc b40ccafa6b2d, line 8052)
- [x] get-buffer-process  (Fget_buffer_process, 1/1, doc 946ad028ca88, line 8422)
- [x] process-inherit-coding-system-flag  (Fprocess_inherit_coding_system_flag, 1/1, doc 6935e6691982, line 8443)
- [x] waiting-for-user-input-p  (Fwaiting_for_user_input_p, 0/0, doc 0e714931b5cd, line 8484)
- [x] list-system-processes  (Flist_system_processes, 0/0, doc b29777fe1fe5, line 8524)
- [x] process-attributes  (Fprocess_attributes, 1/1, doc d6225a857242, line 8542)
- [x] num-processors  (Fnum_processors, 0/1, doc 5ecd35391881, line 8610)
- [x] signal-names  (Fsignal_names, 0/0, doc bdcf1797462f, line 8628)

## Functions defined — 125
- [x] would_block  (line 186, static)
- [x] close_on_exec  (line 197, static)
- [x] accept4  (line 208, static)
- [x] process_socket  (line 214, static)
- [x] pset_buffer  (line 344, static)
- [x] pset_command  (line 349, static)
- [x] pset_decode_coding_system  (line 354, static)
- [x] pset_decoding_buf  (line 359, static)
- [x] pset_encode_coding_system  (line 364, static)
- [x] pset_encoding_buf  (line 369, static)
- [x] pset_filter  (line 374, static)
- [x] pset_log  (line 379, static)
- [x] pset_mark  (line 384, static)
- [x] pset_thread  (line 389, static)
- [x] pset_name  (line 394, static)
- [x] pset_plist  (line 399, static)
- [x] pset_sentinel  (line 404, static)
- [x] pset_tty_name  (line 409, static)
- [x] pset_type  (line 414, static)
- [x] pset_write_queue  (line 419, static)
- [x] pset_stderrproc  (line 424, static)
- [x] make_lisp_proc  (line 431, static)
- [x] clear_fd_callback_data  (line 467, static)
- [x] inrange_fd  (line 482, static)
- [x] inrange_pipe  (line 494, static)
- [x] add_read_fd  (line 518)
- [x] add_non_keyboard_read_fd  (line 528)
- [x] add_process_read_fd  (line 535, static)
- [x] delete_read_fd  (line 551)
- [x] add_write_fd  (line 560)
- [x] add_non_blocking_write_fd  (line 572, static)
- [x] recompute_max_desc  (line 584, static)
- [x] delete_write_fd  (line 603)
- [x] compute_input_wait_mask  (line 622, static)
- [x] compute_non_process_wait_mask  (line 645, static)
- [x] compute_non_keyboard_wait_mask  (line 669, static)
- [x] compute_write_mask  (line 693, static)
- [x] clear_waiting_thread_info  (line 716, static)
- [x] kbd_is_ours  (line 731, static)
- [x] update_status  (line 751, static)
- [x] status_convert  (line 762, static)
- [x] connecting_status  (line 778, static)
- [x] decode_status  (line 787, static)
- [x] status_message  (line 814, static)
- [x] allocate_pty  (line 883, static)
- [x] allocate_process  (line 949, static)
- [x] make_process  (line 956, static)
- [x] remove_process  (line 1004, static)
- [x] update_processes_for_thread_death  (line 1015)
- [x] free_dns_request  (line 1039, static)
- [x] get_process  (line 1074, static)
- [x] record_deleted_pid  (line 1121)
- [x] update_process_mark  (line 1329, static)
- [x] set_process_filter_masks  (line 1382, static)
- [x] is_pty_from_symbol  (line 1393, static)
- [x] set_proc_thread  (line 1496, static)
- [x] get_required_string_keyword_param  (line 1800, static)
- [x] start_process_unwind  (line 2147, static)
- [x] close_process_fd  (line 2156, static)
- [x] dissociate_controlling_tty  (line 2167)
- [x] create_process  (line 2206, static)
- [x] create_pty  (line 2388, static)
- [x] conv_sockaddr_to_lisp  (line 2626)
- [x] conv_addrinfo_to_lisp  (line 2697, static)
- [x] get_lisp_to_sockaddr_size  (line 2708, static)
- [x] conv_lisp_to_sockaddr  (line 2756, static)
- [x] set_socket_option  (line 2929, static)
- [x] set_network_socket_coding_system  (line 3329, static)
- [x] finish_after_tls_connection  (line 3417, static)
- [x] connect_network_socket  (line 3459, static)
- [x] network_interface_list  (line 4353, static)
- [x] network_interface_info  (line 4514, static)
- [x] network_lookup_address_info_1  (line 4729, static)
- [x] deactivate_process  (line 4841, static)
- [x] server_accept_connection  (line 5001, static)
- [x] check_for_dns  (line 5207, static)
- [x] wait_for_socket_fds  (line 5253, static)
- [x] wait_while_connecting  (line 5264, static)
- [x] wait_for_tls_negotiation  (line 5274, static)
- [x] wait_reading_process_output_unwind  (line 5287, static)
- [x] wait_reading_process_output_1  (line 5295, static)
- [x] android_select_wrapper  (line 5306, static)
- [x] wait_reading_process_output  (line 5358)
- [x] read_process_output_call  (line 6222, static)
- [x] read_process_output_error_handler  (line 6228, static)
- [x] read_process_output  (line 6259, static)
- [x] read_process_output_before_insert  (line 6371, static)
- [x] read_process_output_after_insert  (line 6401, static)
- [x] read_process_output_set_last_coding_system  (line 6440, static)
- [x] read_and_insert_process_output  (line 6482, static)
- [x] read_and_dispose_of_process_output  (line 6541, static)
- [x] write_queue_push  (line 6671, static)
- [x] write_queue_pop  (line 6701, static)
- [x] send_process  (line 6734, static)
- [x] emacs_get_tty_pgrp  (line 7023, static)
- [x] process_send_signal  (line 7093, static)
- [x] abbr_to_signal  (line 7347, static)
- [x] child_signal_init  (line 7582)
- [x] child_signal_read  (line 7614, static)
- [x] child_signal_notify  (line 7628, static)
- [x] dummy_handler  (line 7667, static)
- [x] handle_child_signal  (line 7702, static)
- [x] deliver_child_signal  (line 7789, static)
- [x] exec_sentinel_error_handler  (line 7796, static)
- [x] exec_sentinel  (line 7811, static)
- [x] status_notify  (line 7884, static)
- [x] add_gpm_wait_descriptor  (line 8068)
- [x] delete_gpm_wait_descriptor  (line 8074)
- [x] keyboard_bit_set  (line 8087, static)
- [x] update_processes_for_thread_death  (line 8107)
- [x] wait_reading_process_output  (line 8146)
- [x] add_timer_wait_descriptor  (line 8337)
- [x] remove_slash_colon  (line 8350)
- [x] add_keyboard_wait_descriptor  (line 8362)
- [x] delete_keyboard_wait_descriptor  (line 8376)
- [x] setup_process_coding_systems  (line 8391)
- [x] kill_buffer_processes  (line 8466)
- [x] hold_keyboard_input  (line 8500)
- [x] unhold_keyboard_input  (line 8508)
- [x] kbd_on_hold_p  (line 8516)
- [x] catch_child_signal  (line 8656)
- [x] restore_nofile_limit  (line 8678)
- [x] open_channel_for_module  (line 8687)
- [x] init_process_emacs  (line 8706)
- [x] syms_of_process  (line 8813)

## Macros defined — 24
- [x] ADD_SUBFEATURE  (line 9086, body 2e2bd63316fd)
- [x] AF_LOCAL  (line 63, body be7a7252b53c)
- [x] ASYNC_RETRY_NSEC  (line 140, body b1a65867cc11)
- [x] DATAGRAM_CHAN_P  (line 327, body 129fbdc13396)
- [x] DATAGRAM_CONN_P  (line 333, body f28321d56c3c)
- [x] DATAGRAM_SOCKETS  (line 242, body da39a3ee5e6b)
- [x] FOR_EACH_PROCESS  (line 339, body 53e714fec50f)
- [x] HAVE_LOCAL_SOCKETS  (line 66, body da39a3ee5e6b)
- [x] HAVE_SEQPACKET  (line 248, body da39a3ee5e6b)
- [x] HOST_FORMAT_IN  (line 5037, body 00b3625c4904)
- [x] HOST_FORMAT_IN6  (line 5038, body 1771307ac86b)
- [x] NETCONN1_P  (line 223, body c0c1e4f4f22b)
- [x] NETCONN_P  (line 222, body ad200f9ad4f8)
- [x] PIPECONN1_P  (line 227, body 395ac6bb2172)
- [x] PIPECONN_P  (line 226, body 52eefd129784)
- [x] READ_OUTPUT_DELAY_INCREMENT  (line 251, body 2a70a022aed6)
- [x] READ_OUTPUT_DELAY_MAX  (line 252, body bc86f9851482)
- [x] READ_OUTPUT_DELAY_MAX_MAX  (line 253, body e4b356ff45b9)
- [x] SERIALCONN1_P  (line 225, body d62a92371803)
- [x] SERIALCONN_P  (line 224, body 54a5cb8f7865)
- [x] SOCK_CLOEXEC  (line 177, body b6589fc6ab0d)
- [x] SOCK_NONBLOCK  (line 180, body b6589fc6ab0d)
- [x] accept4  (line 205, body 866d2d7f7d3d)
- [x] socket  (line 219, body ffcd27d38cd4)

## Types and their fields — 3
- [x] ifflag_def: int flag_bit, const char* flag_sym
- [x] req: struct gaicb gaicb, struct addrinfo hints, char str
- [x] u_sockaddr: struct sockaddr sa, struct sockaddr_in in, struct sockaddr_in6 in6, struct sockaddr_un un

## File-scope objects — 20
- [x] static struct rlimit nofile_limit  (line 55, static)
- [x] static bool kbd_is_on_hold  (line 157, static)
- [x] bool inhibit_sentinels  (line 161)
- [x] static EMACS_INT process_tick  (line 230, static)
- [x] static EMACS_INT update_tick  (line 232, static)
- [x] static int process_output_delay_count  (line 258, static)
- [x] static bool process_output_skip  (line 262, static)
- [x] static int num_pending_connects  (line 280, static)
- [x] static int max_desc  (line 283, static)
- [x] static int external_sock_fd  (line 289, static)
- [x] static int child_signal_read_fd  (line 292, static)
- [x] static int child_signal_write_fd  (line 296, static)
- [x] static Lisp_Object chan_process  (line 303, static)
- [x] static Lisp_Object Vprocess_alist  (line 307, static)
- [x] static int proc_buffered_char  (line 315, static)
- [x] static struct coding_system proc_decode_coding_system  (line 318, static)
- [x] static struct coding_system proc_encode_coding_system  (line 319, static)
- [x] static Lisp_Object deleted_pid_list  (line 1118, static)
- [x] static EMACS_INT connect_counter  (line 4998, static)
- [x] bool glib_installs_sigchld_handler  (line 7674)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/process.c` @ `fd7afced403`; port `femacs/src/process.rs`.  **WIP: this port is a
faithful prefix, C 1-2624 of 9,123 lines** (through `Fmake_pipe_process`).  The 123 unchecked
rows above are the remainder (C 2626-9123: the socket layer, `make-network-process`,
`accept-process-output`, `wait_reading_process_output`, the read/write/signal machinery and
`syms_of_process`).  The unit therefore **does not compile yet**: Rust has no forward
declarations, and six callees C defines later in the file (`deactivate_process` C 4841,
`wait_for_socket_fds` C 5253, `child_signal_init` C 7582, `status_notify` C 7884,
`remove_slash_colon` C 8350, `setup_process_coding_systems` C 8391) have no definition.
Every other error the compiler reports has been cleared: `xtask build-tu src/process.c` reaches
`error[E0425]: cannot find function ...` for exactly those six names and nothing else.  The
unit is listed here with unchecked boxes, so `tools/extract-abi.py --all` counts it as *in
progress* rather than failing (measured: `checked 365 ported files: 339 done, 26 in progress,
0 failing`).

- **Dead-arm policy (§3.49/§3.51, the `sysdep.rs`/`print.rs`/`callproc.rs` precedent).**
  `python3 tools/check-cond-eval.py src/process.c` first reported
  `222 conditions, 0 unknown, 29 disagreements [NOT VERIFIED]`: every disagreement was a
  `<sys/ioctl.h>`/`<net/if.h>`/`<netinet/tcp.h>`/`<net/if_dl.h>` name the frozen table could
  not see (see the tool/table note below).  The file is **not** in
  `docs/baseline/config/cond-eval-verified.txt`, so `tools/extract-abi.py`'s
  `blind_conditions` keeps *both* arms of every `#if` live, and per the integrator's
  2026-10-03 decision the port **carries** the text of the arms the frozen build does not
  compile, each under `#[cfg (feature = "…")]` naming the C condition's non-frozen side.
  None of those features is ever set (`xtask dead-arms` only flips `config.rs` keys), so the
  compiled text is exactly what clang compiles.  The arms carried so far, in C order:
  `no_subprocesses` (C 44-46), `have_gnutls` (C 118-121, 963-967, 1920-1925),
  `have_android` (C 122-125), `have_window_system` (C 127-129), `have_glib` (C 131-136),
  `have_getaddrinfo_a` (C 138-141, 1035-1049, 1151-1170), `windowsnt` (C 143-146, 2359-2361,
  2507-2509), `no_pty_iteration`/`no_pty_name_sprintf`/`no_pty_open`/`no_pty_tty_name_sprintf`
  (C 888-906), `no_tiocnotty` (C 2171-2181), `have_pipe_sz` (C 2269-2273), `dont_reopen_pty`
  (C 2406-2411), `no_ifreq_hwaddr` (C 4528-4586, not reached yet).
  **`src/process.c` is deliberately not added to `cond-eval-verified.txt`**: after the table
  fix it *is* eligible (0 disagreements), but a verified file's checker demands only the live
  arm, while the port carries both (docs/status.md 3.51's `src/callproc.c` note is the
  precedent).

- **Source-header macro rows (added this round).**  The 29 disagreements were the §3.35/§3.51
  class: macros from the *source* headers `<sys/ioctl.h>` (TIOCNOTTY, TIOCGPGRP, SIOCGIFADDR,
  SIOCGIFFLAGS, SIOCGIFNETMASK, SIOCGIFBRDADDR), `<netinet/tcp.h>` (TCP_NODELAY),
  `<net/if.h>` (the 16 `IFF_*` names) and `<net/if_dl.h>` (LLADDR).  24 rows were appended to
  `docs/baseline/config/source-header-macros.tsv` (each measured by including exactly that
  header after `config.h`; the five `SIOCGIF*` names come from `<sys/ioctl.h>` on this target,
  not `<sys/socket.h>`, which the generator's own warning caught) and
  `python3 tools/gen-cond-macros.py` re-ran (7,305 macros, 6,865 defined, 84 from the source
  rows).  After it: `src/process.c: 222 conditions, 0 unknown, 0 disagreements [VERIFIED]`
  and `check-cond-eval.py --verified docs/baseline/config/cond-eval-verified.txt` still ends
  `src/xwidget.h: 19 conditions, 0 unknown, 0 disagreements [VERIFIED]` (exit 0, no
  regression).  No unit text was adjusted to please the gate.

- **`DEFVAR_*` storage (§3.29).**  Not reached yet: the unit's `V*` carriers live in
  `syms_of_process` (C 8813-9123), which is part of the unchanged remainder.

- **Spelling deviations (all recorded in the port's comments).**
  1. C's `static` helpers are plain `unsafe fn`; the ones whose address C takes
     (`start_process_unwind`, `remove_process`) are `unsafe extern "C" fn` (PORTING.md 1.4).
  2. C's `# undef accept4`/`# define accept4(…)` and `# undef socket`/`# define socket(…)`
     renames are the port's `macro_rules! accept4`/`macro_rules! socket`, with the definitions
     named `process_accept4`/`process_socket` (the names C's call sites expand to).  The real
     libc `socket` is reached through the local `libc_socket` module so the unit's `socket`
     macro cannot capture it; `process_socket`'s body keeps the token `socket` for the checker.
  3. `Lisp_Process`'s two bitfield units are read and written through P-80's member-name macro
     `process_bits!` (`raw_status_new`, `alive`, `pty_in`, `pty_out`, `kill_without_query`,
     `adaptive_read_buffering`, `inherit_coding_system_flag`, `is_non_blocking_client`,
     `is_server`, `read_output_skip`), so every C field name stays in the port's text.
  4. `FOR_EACH_PROCESS` is a `macro_rules!` that takes C's body as its third argument: Rust has
     no statement-position loop macro usable as C's `for` followed by a statement (P-22), so
     the call sites read `FOR_EACH_PROCESS! (tail, proc, { … })` and the expansion
     (`for_each_alist_value!`) writes lisp.h 5908's loop with C's three names.  None of this
     file's five loops has a `continue`; one that did would need the increment repeated (the
     minibuf.rs/buffer.rs shape).
  5. `AUTO_STRING (suffix, coredump ? " (core dumped)
" : "
")` becomes
     `let suffix = if coredump { AUTO_STR! (…) } else { AUTO_STR! (…) }` (the macro must stay
     an expression, not a `let` statement, in that branch).
  6. `args2[i++] = …` becomes `*args2.offset (i) = …` (raw pointers do not index).
  7. `p->tick = ++process_tick;` keeps both names: `process_tick += 1; (*p).tick =
     process_tick;`.
  8. `sizeof (rq.ifr_flags)`-style `sizeof` folds and `INT_STRLEN_BOUND`/`PRIdMAX` are spelled
     as the target's values with a `docs/ports/macro-body-notes.tsv` row for `make_process`.
  9. `#[cfg]`-selected `macro_rules!` pairs (`PTY_ITERATION`, `PTY_OPEN`, `PTY_NAME_SPRINTF`,
     `PTY_TTY_NAME_SPRINTF`) carry the `#else` arms; a `macro_rules!` body resolves caller
     locals at its definition site (mixed-site hygiene, docs/status.md 3.69), so those macros
     take the enclosing function's locals as parameters.
  10. C's forward declarations (C 264-296) are dropped per PORTING.md 0.1-12; the definitions
      keep their C positions (which is why the unit cannot compile before the remainder lands).
  11. `#if 0` text (C 1692-1701's `Fprocess_connection`, C 5955, C 6550, C 7823, C 8791) is
      recorded here and not carried (P-08): it is dead in every configuration.

- **Use-site declarations (P-72 rule 3), grouped 1-4 in the unit.**  Mirrors do not exist for
  `<sys/socket.h>`, `<netdb.h>`, `<netinet/in.h>`, `<netinet/tcp.h>`, `<arpa/inet.h>`,
  `<sys/un.h>`, `<sys/ioctl.h>`, `<net/if.h>`, `<ifaddrs.h>`, `<net/if_dl.h>`, `<util.h>`,
  `<pty.h>`, `<sys/file.h>`, `"systty.h"`, `"termopts.h"`, `"syswait.h"`, `"gnutls.h"`,
  `"android.h"`, `"androidterm.h"`, `"xgselect.h"`.  The declared types are measured with the
  frozen configuration and pinned by `const _` assertions: `sockaddr` 16/1, `in_addr` 4,
  `in6_addr` 16, `sockaddr_in` 16/4, `sockaddr_in6` 28/4, `sockaddr_un` 106/1,
  `sockaddr_storage` 128/8, `sockaddr_dl` 20/2, `addrinfo` 48/8, `ifaddrs` 56/8, `ifreq`
  32/8 (`ifr_ifru`@16), `servent` 32/8, `linger` 8/4, `socklen_t` 4, `u_sockaddr` 108/4,
  `sockaddr_and_len` 16/8, `fd_callback_data` 40/8, `struct gaicb` (unmeasurable: the only arm
  that names it is `HAVE_GETADDRINFO_A`, glibc-only, and the frozen table records it `undef` —
  carried text only, no assertion).  `FD_SET`/`FD_ZERO` are declared here because
  `lib/include/sys_select.rs` carries only `FD_ISSET` (reported to the integrator as a mirror
  gap), together with the errno names the `errno.rs` mirror lacks (EAGAIN, EWOULDBLOCK, EMFILE)
  and the `<errno.h>`/`<sys/wait.h>` entries the unit needs by name.
- **No drifts recorded yet**: `docs/ports/line-drift-tolerance.tsv` has no `process` row
  because the port is a prefix, not a whole file; the row (basis `total`, the carried arms are
  most of the excess) is owed when the remainder lands.

## Integrator note (WIP state, 2026-10-04)

- `python3 tools/extract-abi.py --check emacs/src/process.c femacs/src/process.rs` currently
  ends `FAIL ... (182 problems)`; 166 are `missing …` for the unported remainder and the rest
  are the ledger's 123 unchecked rows.  One of them is a real *todo for the remainder*:
  C 8362/8376/8337/8343 define `add_keyboard_wait_descriptor`,
  `delete_keyboard_wait_descriptor`, `add_timer_wait_descriptor` and
  `delete_timer_wait_descriptor` (all non-`static`), and the port so far only *declares* them
  at the use site — their definitions must carry `#[unsafe(no_mangle)]` when they land, and the
  declarations must then go.
- `python3 tools/check-linkage.py` reports `src/process: the reference defines these non-static
  objects, the port exports none: ['_glib_installs_sigchld_handler']` — C 7674's definition is
  in the unported remainder (its `#if` is `HAVE_GLIB`, `undef` here; the tool asks for triage
  because the file is outside the verified list).
- `python3 tools/check-undef.py` reports 3 new references from the stale `process.o`:
  `_add_keyboard_wait_descriptor`, `_delete_keyboard_wait_descriptor`,
  `_setup_process_coding_systems` — all three are C functions *defined by this file* in the
  unported remainder (C 8362/8376/8391), referenced by the use-site declarations the ported
  prefix needs.  They resolve as the remainder lands.  (The other 16 new entries are
  `buffer.o`'s, from the concurrent `src/buffer.c` port; the 4 stale ratchet entries are not
  this unit's.)
- `python3 tools/check-globals.py` ends
  `check-globals: 262 units, 0 DEFVAR_* variables without a definition in their own unit` (this
  unit has no `DEFVAR_*` call site yet).

## Progress update (2026-10-04, second round)

The port now covers **C 1-6541** of 9,123 lines (`femacs/src/process.rs`, 6,379 lines): the whole
socket layer (`make-network-process`, `connect_network_socket`, the `getaddrinfo`/`getifaddrs`
machinery, the datagram address accessors, `set_socket_option` and the `ifflag_table`),
`accept-process-output`, `wait_reading_process_output` with its fd-mask helpers, and the
read-pipeline entry points (`read_process_output` through `read_and_insert_process_output`).
The 84 rows still unchecked are C 6560-9123: `read_and_dispose_of_process_output`, the
write/`send_process` path, the signal machinery (`process_send_signal`,
`handle_child_signal`, `exec_sentinel`, `status_notify`), the descriptor/timer/gpm helpers,
`Fprocess_attributes`/`Flist_system_processes`/`Fnum_processors`/`Fsignal_names`, and
`syms_of_process` (with its `DEFVAR_*` carriers and `defsubr`s).

`xtask build-tu src/process.c` still reaches `error[E0425]` for a fixed, shrinking set of names
C defines later in the file: `child_signal_init` (C 7582), `exec_sentinel` (7811),
`status_notify` (7884), `remove_slash_colon` (8350), `setup_process_coding_systems` (8391),
`read_and_dispose_of_process_output` (6541), `kbd_on_hold_p`/`keyboard_bit_set` (part of the
same tail) — every other compiler error is cleared.  The unit stays registered WIP, so
`extract-abi.py --all` counts it "in progress" and the tree's gates stay green.

Additional Decisions this round:

- **`coding_system` bitfield writers.**  The `include/coding.rs` mirror carries only the readers
  (`mode_of`, `src_multibyte_p`, `dst_multibyte_p`); the two members this file assigns (`mode`,
  `insert_before_markers`) are written through a unit-local `coding_bits!` macro in P-80's
  member-name form, so C's field names stay in the port's text.
- **`do_display` is `c_int`, not `bool`, in `wait_reading_process_output`.**  C's call sites pass
  the integer literals `0`/`1` for the `bool` parameter, and the per-function token check
  requires those tokens, so the port keeps `c_int` and tests `!= 0` (recorded here).
- **`process_error_pause_time`** is a `DEFVAR_INT` carrier owned by this unit (C 9004): the port
  defines `static mut process_error_pause_time: c_int = 0;` (status 3.29).  The four
  descriptor helpers (`add/delete_keyboard_wait_descriptor`,
  `add/delete_timer_wait_descriptor`) still need `#[unsafe(no_mangle)]` when they land, and
  `glib_installs_sigchld_handler` (C 7674) must export a symbol.
- **`FD_CLR`** joins `FD_SET`/`FD_ZERO` as a use-site declaration (the `sys_select.rs` mirror
  carries only `FD_ISSET`).
- **No `line-drift-tolerance.tsv` row is appended** by this round: the port is still a prefix
  (6,379 Rust vs 9,122 C), so any tolerance written now would be a claim about a file that does
  not exist yet.  For the record, the finished prefix's density is ~0.97 Rust lines per C line,
  so the completed unit should need a `total`-basis row near 0.15-0.30 (the carried arms are
  most of the excess); the row is owed at completion.

## Final state (2026-10-04, third round — the port is complete)

`femacs/src/process.rs` now covers **C 1-9,123** (9,202 Rust lines; the ledger's 235 rows are
all ticked).  The gates, each run from the repository root unless noted:

    build-tu src/process.c        -> built src/process.rs -> ../build/femacs/process.o
    extract-abi.py --check        -> OK emacs/src/process.c -> femacs/src/process.rs
    check-globals.py              -> check-globals: 265 units, 0 DEFVAR_* variables without a definition in their own unit
    check-linkage.py              -> check-linkage: 4 unit object(s) compared ..., 0 with 0 missing symbol(s)
    rustc --cfg enable_checking   -> exit 0 (from femacs/)
    check-declarations.py         -> 8 declaration sites (0 forbidden, 8 allowed literals), 0 failures
    check-verbatim.py             -> 6 trees, 4607 files, 0 failures
    check-undef.py                -> 20 NEW entries, *all* referenced by frame.o (the concurrent
                                     frame lane); **process.o has 0 new references**

Final Decisions added in this round:

- **`MANY` subrs take `isize`.**  The five `DEFUN!`s with `maxargs MANY` (`Fmake_process`,
  `Fmake_pipe_process`, `Fserial_process_configure`, `Fmake_serial_process`,
  `Fmake_network_process`) declare `nargs: isize`, which is what `DEFUN_ARGS_MANY` spells
  (`(isize, *mut Lisp_Object)`) — `ptrdiff_t` is its alias, but the gate compares the spelling.
- **Carried arms are text, not comments.**  Every non-frozen `#if` arm this file has —
  `have_gnutls` (the async DNS/TLS loop, the TLS buffer probe, the handshake limit, the
  `finish_after_tls_connection` tail, `emacs_gnutls_deinit`/`read`/`write`), `have_getaddrinfo_a`
  (`check_for_dns`, the `gai_cancel`/`gai_suspend` cancel path, the `struct req` request,
  `postpone_connection`), `have_android`/`threads_enabled` (`android_select*`),
  `have_glib`/`have_ns` (`xg_select`/`ns_select`, the Glib child-watch workaround),
  `windowsnt` (`register_child`, `register_aux_fd`, `init_winsock`, the `getpeername` slippage
  probe, `PurgeComm`), `no_subprocesses` (the whole MS-DOS `wait_reading_process_output`),
  `have_timerfd`, `have_gpm`, `broken_pty_read_after_eagain`, `tiocsigsend`, `no_sigcont`,
  `no_sigtstp`, `no_tiocnotty`, `have_pipe_sz`, `dont_reopen_pty`, `no_pty_*`, `have_so_*`,
  the five `have_iff_*`/`ns_impl_cocoa` table rows, `no_ifreq_hwaddr`,
  `have_ifreq_netmask`, `no_gai_strerror`, `msdos`, `no_getifaddrs`, `no_net_if` — is
  **transcribed as Rust under its `#[cfg (feature = "…")]`**, so the tokens are in the port
  (C's `#if` text is part of the function body the checker sees).  None of those features is
  ever set, so the compiled text is exactly what clang compiles.
- **`ADD_SUBFEATURE` takes `subfeatures` as its first argument** (mixed-site hygiene,
  status 3.69); C's `#define` sits inside `syms_of_process`, the port's `macro_rules!` must
  precede the function, and the ledger records the two-argument call shape as a deviation.
- **Docstring escapes.**  `doc_comment_value` unescapes C's `\X` to `X` for the unknown
  escapes, so C's `\(HOST SERVICE)` in a docstring is a plain `(HOST SERVICE)` in the port's
  literal (and C's `"\\\\.\\COM10"` is `"\.\COM10"`).  Three docstrings
  (`Fprocess_contact`, `Fserial_process_configure`, `Fmake_serial_process`) were regenerated
  with that rule.
- **`AF_LOCAL` keeps C's spelling through `AF_UNIX`** (C 63-64's `#define AF_LOCAL AF_UNIX`
  arm is the one cinventory digests), `DATAGRAM_CHAN_P` keeps C's `!= 0` comparison, and
  `HOST_FORMAT_IN`/`HOST_FORMAT_IN6` are string **constants** (cinventory classifies C's
  `#define` of a string as a constant, not a macro).
- **`tools/extract-abi.py` (a tool fix, not a port change).**  The Rust-side docstring lookup
  scanned only `m.end() + 4000` characters; a literal longer than that cannot close inside the
  window, so `Fmake_network_process` (8 KB doc) was reported as "no docstring found on the
  Rust side" although its digest is right.  The lookup now re-scans with a 400,000-character
  window when the first pass finds nothing, so an over-long docstring is still compared instead
  of being reported as missing.
- **`macro-body-notes.tsv`** gained 46 rows this round (43 in one block) for the token classes a
  Rust spelling cannot carry: C's `switch`/`case`/`break`/`default` (the port's `match`),
  `DECLARE_POINTER_ALIAS`, the `uint16_t`/`uint32_t`/`ssize_t` typedefs, `SSDATA`,
  `sigaction.sa_handler` (the port's union member), the `0`/`1` literals C passes for `bool`
  parameters, and the two `#if`-condition tokens (`HAVE_GETIFADDRS`,
  `HAVE_STRUCT_IFREQ_IFR_*`, `SIOCGIF*`, `WINDOWSNT`).
- **`line-drift-tolerance.tsv`** now carries the final `process` row (0.05, `total` basis,
  9,202 vs 9,122 = 0.9%), superseding the provisional 0.10 row that the unfinished prefix had
  produced.

## Final state (2026-10-04, fourth round — close-out; two defects fixed)

The tail (C 6560-9123) had already landed; this round finished the 84 rows of the second half,
ticked the ledger, and fixed **two defects that only the object-level and preprocessor evidence
exposes**:

- **`TIOCNOTTY` at C 2171-2181 was carried as dead text; it is live.**  `src/process.c`
  includes `<sys/ioctl.h>` (C 71), so the C preprocessor keeps the `#ifdef TIOCNOTTY` body:
  preprocessing the real file leaves `int ttyfd = emacs_open (dev_tty, 0x0002, 0);` and
  `ioctl (ttyfd, 0x20007471, 0)` (the macro is `_IO ('t', 113)`), and
  `check-cond-eval.py src/process.c` agrees (`clang True`).  The port had the body under
  `#[cfg (feature = "no_tiocnotty")]`, i.e. it compiled *nothing* where clang compiles the
  syscall.  Fixed: the body is now the compiled text, and the use-site constant
  `pub const TIOCNOTTY: c_int = 536900721;` was added — it had no declaration anywhere, because
  the never-set cfg never resolved the identifier.
- **`TIOCNOTTY` cannot be a `source-header-macros.tsv` row (the row added this round was
  removed again).**  With it, `src/process.c` reported 0 disagreements — but it *regressed*
  `src/term.c`, which is in `cond-eval-verified.txt`: term.c never includes `<sys/ioctl.h>`, so
  clang answers `False` for term.c's own `#ifdef TIOCNOTTY` (verified by preprocessing: only
  `block_tty_out_signal`'s prototypes survive, not the calls).  A row is global and cannot say
  "defined in process.c, undefined in term.c".  `src/process.c` is therefore left with exactly
  **1 disagreement** (`TIOCNOTTY`) — informational, since the file is *not* in
  `cond-eval-verified.txt` and `extract-abi.py` keeps both arms live for it anyway.  The other
  23 rows (TIOCGPGRP, TCP_NODELAY, 4 `SIOCGIF*`, 16 `IFF_*`, LLADDR) decide the remaining 28
  conditions and regress no listed file.
- **`inherit_process_coding_system` was defined here; C defines it in `src/coding.c`.**  The
  object comparison (`tools/check-symbols.py --dir src`, against the reference
  `build/emacs-ref/src/process.o`) reported it as *invented*: C's `DEFVAR_BOOL
  ("inherit-process-coding-system", ...)` is at `src/coding.c` 12023 and its storage lives in
  the single `globals` object, so C's `process.o` exports no such symbol.  The definition (an
  `#[unsafe(no_mangle)] pub static mut`) was removed; the file's live references (C 2068, 3319,
  3412) stay undefined references until `src/coding.c`'s unit lands and defines its carrier.
  The invented object also masked `callproc.o`'s identical reference, which is why the
  removal turns it into one new pending name.  Only `Vinternal__daemon_sockname` remains as
  this file's own `DEFVAR_LISP` carrier (status 3.29).
- **The `TIOCNOTTY` fix is +17 lines** (the carried cfg block, the constant declaration and the
  comment became the compiled body); `femacs/src/process.rs` is now **9,226** lines against C's
  9,122 (1.06 %), and the tolerance file carries a fresh `process` row.

Gates re-run this round, each from the repository root unless noted:

```text
build-tu src/process.c        -> built src/process.rs -> ../build/femacs/process.o
extract-abi.py --check        -> OK emacs/src/process.c -> femacs/src/process.rs
check-globals.py              -> check-globals: 266 units, 0 DEFVAR_* variables without a definition in their own unit
check-linkage.py              -> check-linkage: 3 unit object(s) compared ..., 0 with 0 missing symbol(s)
check-symbols.py --dir src    -> check-symbols: 64 units compared, 0 mismatched, 0 still pending
rustc --cfg enable_checking   -> exit 0 (from femacs/)
check-declarations.py         -> 8 declaration sites (0 forbidden, 8 allowed literals), 0 failures
check-verbatim.py             -> 6 trees, 4607 files, 0 failures
check-undef.py                -> 515 unit object(s), 151 reference object(s)/archive(s): 151 pending, 1 NEW
                                 (_inherit_process_coding_system, referenced by callproc.o,
                                  callproc.checking.o, process.o) — the dependency on the unported
                                  src/coding.c; the integrator adds it with `--write-pending`
grep -c '^\- \[ \]' docs/ports/process.md -> 0   (235 rows ticked)
```

- **Cross-lane finding, not this lane's to fix: `src/font.h` regresses in
  `check-cond-eval.py --verified`.**  `source-header-macros.tsv` carries the font lane's row
  `font.h\tFONT_DEBUG`; because `font.h` itself defines `FONT_DEBUG` (C 998), a global
  "defined" makes font.h's own `#ifndef FONT_DEBUG` at C 997 disagree with clang
  (`clang True, evaluator False`).  Removing the row fixes `src/font.h` but breaks
  `extract-abi.py --check emacs/src/font.c femacs/src/font.rs` (four `unexpected defsubr`), so
  the row is load-bearing for that lane and was left in place.  With the `TIOCNOTTY` row gone,
  `src/font.h` is the *only* file the `--verified` run still reports (`verified list
  regressed: src/font.h`, exit 1); it needs a font-lane decision — drop the row and take the
  recorded deviation, or drop `src/font.h` from the verified list.

## Close-out addendum (2026-10-04, fifth round — the integrator's 3.76/3.77 landed)

Two of these were this lane's, found while re-running the gates against the integrator's new
commits:

- **`tools/check-mirror-dupes.py` (`338c68c`, status 3.77) failed this unit with 26 duplicate
  declarations.**  `process.rs` repeated declarations a mirror in its own include graph already
  carries at its top level (the local copy shadows the mirror's item and turns into
  `clashing_extern_declarations`).  All 25 reported copies were deleted together with their own
  `/// C:` line, and the two `unsafe extern "C"` blocks left empty were removed:
  `pthread_sigmask`, `sigaction`, `sprintf`, `dtotimespec`, `run_all_atimers`,
  `timerfd_callback`, `turn_on_atimers`, `validate_region`, `Fcurrent_buffer`, `Fget_buffer`,
  `Fgoto_char`, `Finsert`, `Fmatch_data`, `Fprovide`, `Fset_match_data`,
  `Fstring_as_unibyte`, `Fstring_to_multibyte`, `insert_string`, `list1`, `list3`,
  `make_uninit_string`, `move_gap_both`, `staticpro`, `wrong_type_argument`, `xfree`
  (the emptied blocks were `dtotimespec`/`wrong_type_argument` and the file-tail
  `staticpro`/`list1`/`Fprovide` one).  After it: `check-mirror-dupes.py` ->
  `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)` (the one ratchet row is
  `src/insdel.rs`'s, another lane's), and the unit is **9,172** lines (0.55 % over C's 9,122).
- **The `src/font.h` regression is fixed by the integrator's `a7d3fd9`** (status 3.76,
  "check-cond-eval: hand the header under test to the evaluator").  The fourth-round section
  below wrongly calls it a cross-lane blocker: the evaluator's second pass now defers a
  `source-header-macros.tsv` row to *after* the file's own `#ifndef NAME` / `#define NAME` pair,
  so `src/font.h`'s guard at C 997 agrees with clang while `src/font.c`'s two
  `#ifdef FONT_DEBUG` conditions stay live.  The full `check-cond-eval.py --verified` run now
  ends with **0 disagreements** and no regressed file.
- **The `TIOCNOTTY` row stays out** and the port keeps the live body (fourth-round section).
  The committed `source-header-macros.tsv` carries the 23 remaining rows; `src/process.c`
  reports exactly one disagreement, which is not a gate input (the file is not in
  `cond-eval-verified.txt`).

Gates on the current tree (repository root unless noted):

```text
build-tu src/process.c        -> built src/process.rs -> ../build/femacs/process.o
extract-abi.py --check        -> OK emacs/src/process.c -> femacs/src/process.rs
check-globals.py              -> check-globals: 267 units, 0 DEFVAR_* variables without a definition in their own unit
check-mirror-dupes.py         -> 1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
check-linkage.py              -> check-linkage: 2 unit object(s) compared ..., 0 with 0 missing symbol(s)
check-symbols.py --dir src    -> check-symbols: 67 units compared, 0 mismatched, 0 still pending
rustc --cfg enable_checking   -> exit 0 (from femacs/)
check-declarations.py         -> 8 declaration sites (0 forbidden, 8 allowed literals), 0 failures
check-verbatim.py             -> 6 trees, 4607 files, 0 failures
check-cond-eval.py --verified -> 5717 conditions, 4 unknown to the evaluator, 0 disagreements
check-cond-eval.py src/process.c -> 222 conditions, 0 unknown, 1 disagreements (TIOCNOTTY, informational)
check-undef.py                -> 1 NEW: _inherit_process_coding_system (the unported src/coding.c; see above)
grep -c '^- [ ]' docs/ports/process.md -> 0   (235 rows ticked)
```

- **Integrator note (3.263): `snprintf` is declared by the mirror, not here.**  `lib/include/stdio.rs`
  gained the `<stdio.h>` entry so the `GLYPH_DEBUG` arms (which call it) compile - `xtask dead-arms`
  with the `glyph_debug` flip is what found it - and this unit's own declaration of the same name was
  what made the name ambiguous once the mirror carried it.  The declaration is gone; the mirror is
  the single declaration point (PORTING.md 0.1-3).
