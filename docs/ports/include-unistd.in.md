# emacs/lib/unistd.in.h (2447 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [ ] rpl_environ  (line 623)
- [ ] getpagesize  (line 1464)

## Macros defined — 91
- [ ] CLSIZE  (line 1449, body 356a192b7913)
- [ ] F_OK  (line 275, body b6589fc6ab0d)
- [ ] GNULIB_defined_getpagesize_function  (line 1468, body 356a192b7913)
- [ ] GNULIB_defined_isatty  (line 1627, body 356a192b7913)
- [ ] R_OK  (line 278, body 1b6453892473)
- [ ] STDERR_FILENO  (line 270, body da4b9237bacc)
- [ ] STDIN_FILENO  (line 264, body b6589fc6ab0d)
- [ ] STDOUT_FILENO  (line 267, body 356a192b7913)
- [ ] W_OK  (line 277, body da4b9237bacc)
- [ ] X_OK  (line 276, body 356a192b7913)
- [ ] _  (line 70, body e6fccbcff661)
- [ ] _GL_GETPAGESIZE_INLINE  (line 183, body a960df3ab827)
- [ ] _GL_INCLUDING_UNISTD_H  (line 38, body da39a3ee5e6b)
- [ ] _GL_INCLUDING_WINSOCK2_H  (line 64, body da39a3ee5e6b)
- [ ] _GL_UNISTD_INLINE  (line 180, body a960df3ab827)
- [ ] _GL_USE_CRT_EXTERNS  (line 602, body da39a3ee5e6b)
- [ ] __need_system_stdlib_h  (line 122, body da39a3ee5e6b)
- [ ] _gl_getpagesize  (line 1454, body 0687a32f08c0)
- [ ] accept  (line 196, body 23c177e695b8)
- [ ] access  (line 319, body 17982a145cd2)
- [ ] bind  (line 198, body b0f7368fca0d)
- [ ] chdir  (line 352, body 79abe245b19f)
- [ ] chown  (line 371, body e3f5fbbd2acf)
- [ ] close  (line 428, body c02131dc7c32)
- [ ] connect  (line 194, body 793ce4533509)
- [ ] copy_file_range  (line 442, body b504d0fb1101)
- [ ] dup  (line 501, body 5d156978e3d3)
- [ ] dup2  (line 545, body c56b37f413ae)
- [ ] dup3  (line 567, body 439bb087081f)
- [ ] endusershell  (line 1587, body ed98cf7f3617)
- [ ] environ  (line 628, body ea5bd3da5c28)
- [ ] execl  (line 680, body de5ab1699f62)
- [ ] execle  (line 714, body 2f0a51d09b83)
- [ ] execlp  (line 749, body 692eddfca573)
- [ ] execv  (line 785, body e92ccd7575c5)
- [ ] execve  (line 823, body 67f6ec2d3508)
- [ ] execvp  (line 860, body a9e6a75c7101)
- [ ] execvpe  (line 906, body c7202b1c5c17)
- [ ] faccessat  (line 930, body 04d86cdbbec6)
- [ ] fchdir  (line 966, body 30f5e41fe02d)
- [ ] fchownat  (line 997, body a05b62331e2c)
- [ ] fdatasync  (line 1030, body 6b019effb942)
- [ ] ftruncate  (line 1077, body eab53e29722c)
- [ ] getcwd  (line 1144, body cb36405f62bb)
- [ ] getdomainname  (line 1170, body c4caa990bcc9)
- [ ] getdtablesize  (line 1203, body 9b44990f54e9)
- [ ] getentropy  (line 1229, body 1081c88ad146)
- [ ] getgroups  (line 1263, body 0d005ea85175)
- [ ] gethostname  (line 1315, body e2293a7f1ec7)
- [ ] getlogin  (line 1338, body 35a7ed6ad47b)
- [ ] getlogin_r  (line 1376, body b55b1da99c69)
- [ ] getpagesize  (line 1460, body 7b32ea787d8a)
- [ ] getpass  (line 1496, body 9967dc3b2df2)
- [ ] getpeername  (line 200, body 8d67ebf56174)
- [ ] getpid  (line 1524, body 440456939c68)
- [ ] getsockname  (line 202, body 73cbc23b0678)
- [ ] getsockopt  (line 204, body 70c568bd4a93)
- [ ] getusershell  (line 1540, body 2d9450e94bc5)
- [ ] isatty  (line 1652, body a9e4af0e4432)
- [ ] lchown  (line 1671, body 86569a5deb92)
- [ ] link  (line 1699, body 3647cab9631c)
- [ ] linkat  (line 1727, body 77655abdf480)
- [ ] listen  (line 206, body 6c83886edf9e)
- [ ] lseek  (line 1791, body cda812659b9a)
- [ ] pipe2  (line 1831, body c72d8338a0f0)
- [ ] pread  (line 1861, body 81eac5d2f767)
- [ ] pwrite  (line 1897, body 0d615522ea8d)
- [ ] read  (line 1954, body be410059a97a)
- [ ] readlink  (line 1972, body d5cffda4ed69)
- [ ] readlinkat  (line 2004, body a943e94ff93c)
- [ ] recv  (line 208, body 322654f6db7f)
- [ ] recvfrom  (line 212, body 33ed9871c4fb)
- [ ] rmdir  (line 2065, body f69c092dbdd7)
- [ ] select  (line 253, body e78cf58530e6)
- [ ] send  (line 210, body de503e7a67a6)
- [ ] sendto  (line 214, body 3931182241d0)
- [ ] sethostname  (line 2087, body 1b665da62c69)
- [ ] setsockopt  (line 216, body b316418cd81c)
- [ ] setusershell  (line 1563, body 74ee1e9e8975)
- [ ] shutdown  (line 218, body 7b25afff2732)
- [ ] sleep  (line 2125, body f9035b10855f)
- [ ] socket  (line 192, body e3118d6fa129)
- [ ] swab  (line 2151, body a8dae1fb2b6a)
- [ ] symlink  (line 2173, body 9fff43eb0890)
- [ ] symlinkat  (line 2202, body c5b3add47263)
- [ ] truncate  (line 2237, body a37dbe9ea626)
- [ ] ttyname_r  (line 2266, body 8a6f13c19b28)
- [ ] unlink  (line 2323, body 706a5717bd3c)
- [ ] unlinkat  (line 2337, body a1d58370447f)
- [ ] usleep  (line 2366, body 9730fc484983)
- [ ] write  (line 2420, body 72e37d6c70ea)

## Types and their fields — 0

## File-scope objects — 1
- [ ] extern char environ  (line 612)

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/unistd.in.h` @ `fd7afced403`; port `femacs/lib/include/unistd.in.rs`.

Deliberately partial: the mirror carries `ssize_t` (measured: `long`) and the entries the
ported units name so far (`readlink`, `symlink`, the `*at` and file-descriptor family, the id
accessors, `sleep`/`usleep`, `sysconf`, the `SEEK_*` values).  Every `REPLACE_*` macro is zero
in the frozen configuration, so the aliases are the system entries.
**`chmod` is declared once, in `sys_stat.in.rs`**: C declares it in both <unistd.h> and
<sys/stat.h>, and Rust has one namespace per module, so the port keeps one declaration and
records the other here (the unchecked box below).  The unchecked boxes are the rest of the
2447-line reference header.
