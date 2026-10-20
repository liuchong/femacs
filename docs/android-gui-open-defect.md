# Android GUI：**已闭合**（渲染 / 输入 / 存活三项均成立）

> **status 4.952 现状**：GUI **不是白屏**，渲染/翻转链路已被证明是通的（转屏会按新尺寸重新排版，
> 转回来逐字节相同）；焦点也已被证明落在 `EmacsView` 上（`dumpsys input` 的 `FocusedWindows`
> 就是本窗口、窗口 `inputConfig=0x0`、`mServedView` 是 `EmacsView`）。
> 真正的缺口是 **UI 线程的按键/触点事件没有变成 Emacs 的事件**：`ANDROID_CONFIGURE_NOTIFY`
> 会被处理，`ANDROID_KEY_PRESS` 与指针事件不会。见 §3f 的逐条读数与 §4 的两条候选根因。

本文件是 **Android GUI 白屏**的唯一跟踪入口。DoD 第 6 条已按用户授权以 **macOS 验收**
（`docs/current-state.md`），Android GUI **不计入达成**，缺陷继续跟踪。

## 1. 现象

- 进程可以启动、跑完 `loadup.el`、由 `EmacsNoninteractive` 求值 Lisp（batch 通道 ✓）。
- GUI 活动 `org.femacs/.EmacsActivity` 可以启动、获焦、`Total frames rendered` 会增长，
  但**屏幕内容为白**：截图非白像素比稳定在 **0.28%–1.01%**（只有状态栏/导航栏），
  对照桌面基线（`android-screen-launcher.png`，非白 80%+）可区分。

## 2. 已证明“不是这里”的环节（每条都有读数，见 `docs/status.md` 4.910–4.944）

| 环节 | 证据 |
|---|---|
| Lisp 启动 | batch 通道跑到 `BATCH-OK`；GUI 日志里 `Finding pointers to doc strings...done` |
| 字体与行高 | `ANEWFONT … lh=15` 之后 `get_font_ascent_descent` 修复（4.915） |
| 字形排版 | `display_line` 探针显示某行 `used=66`；`update_window` 收到 `dgs=61/155` 的行 |
| 字形绘制 | `draw_glyphs → android_draw_glyph_string` 被调用（`ADGS nchars=61/155`） |
| 颜色 | `fg=0 bg=ffffff`（正文）、`fg=ffffff bg=0`（菜单栏） |
| 位图内容 | `PRESENT#2 corner=ff000000 changes=945` ⇒ 位图里**有内容** |
| expose | `EXPOSE 1080x2146` → `EXEV#1`（Emacs 线程已消费）→ `update_frame` 跑过 |
| flush 钩子 | `HOOK1/2/3` 三个 `frame_up_to_date_hook` 站点都触发 |
| 呈现层 | 三种实现：参考三段式 / SurfaceView / 普通 View+onDraw+重试 + 直读位图；结果都是白 |

## 3. 最大的未知

`PRESENT` 采到位图有内容、`onDraw` 会画 `bitmap`（含“回退到活跃位图”和 300 次重试），
但**玻璃上仍是白**。两种可能，尚未判定：

1. `onDraw` 在内容出现之后不再被调度（`postInvalidate` 未生效）；
2. Emacs 在 resize 之后并没有继续往**当前**位图画（画进了被回收的旧 `Canvas`）；
   这种情况下 `PRESENT` 偶发到的“有内容”帧来自 resize 之前。

## 3a. 判别结果（status 4.946）

`onDraw` 整轮**只跑了一次**（`OD#n=1 same=true px=-1 recycled=false`），之后
`postInvalidate` / `postInvalidateDelayed(100)`（最多 300 次重试）都**没有**换来第二次绘制。
因此假设 **(1) 成立**：窗口在首帧之后不再重绘；`present()` 里的位图即使已有内容也上不了屏。

## 3b. 三个“常见修法”都无效（status 4.947）

| 尝试 | 结果 |
|---|---|
| `present()` 里同时 `postInvalidate()` **和** `parent.postInvalidate()` | 非白 **0.286%**（仍白） |
| `onDraw` 的重试路径里也 `parent.postInvalidate()` | 同上 |
| `EmacsView` 设 `setLayerType(LAYER_TYPE_SOFTWARE)`（绕开硬件层缓存） | 非白 **0.283%**（仍白） |

⇒ 不是“父容器没被 invalidate”，也不是“硬件层缓存了首帧”。`onDraw` 仍然只跑一次。

## 3c. 系统侧读数（status 4.948）

`dumpsys gfxinfo org.femacs`（`docs/baseline/android-sysinfo.txt`）：

```
Total frames rendered: 153      <- 窗口确实在出帧
Janky frames: 89 (58.17%)
Glyph Cache: 0.00 bytes (1 entry) / Glyph Count: 0   <- 平台字形缓存为空（我们画的是位图，符合预期）
```

`onDraw` 侧读数：`OD#1 same=true px=-1 px2=0` → `OD#2/#3 same=true px=-1 px2=-1`
⇒ 首次绘制时位图 (1,300) 是 **完全透明 0x00000000**，随后变白；
而 `frame === bitmap` 恒为真、位图非空且未回收。

**推论**：窗口在出帧（153 帧），`onDraw` 也在跑，但**我们画的那块位图里没有 Emacs 的内容**
（透明→白，从未出现黑）。下一条必须做的是**身份对比**：
`handleDirtyBitmap()` 里 `System.identityHashCode(bitmap)` 与 `onDraw` 里同一个值，
若不同 ⇒ 存在两块位图，Emacs 画的是另一块。

## 3d. 口径更正（status 4.949）

- 身份对比：`NEWBMP id=18385347 1080x2272` 与 `ODRAW id=18385347 same=true` ⇒ **只有一块位图**，
  尺寸仍是首帧布局的 1080×2272（inset 之后未重建）。
- 逐行剖面（`crop=1080:600:0:128`，120 行缩略）：只有前 ~10 行有暗像素
  ⇒ **可见内容只有顶部菜单栏那一条，正文区为空**。
- 因此本文件及 4.938–4.948 里“整屏全白”的说法要按此修正：正确表述是
  **“菜单栏已渲染、正文区未渲染”**。白底黑字的正常截图本来就只有 1–3% 非白，
  用整屏非白比判断“有没有内容”过于粗糙。

## 3e. 输入缺口（status 4.951）

`adb shell input keyevent KEYCODE_A/B/C` 之后：`EmacsView.onKeyDown` **0 次**、
C 侧 `ANDROID_KEY_PRESS` **0 次**、截图前后 md5 相同 ⇒ **按键没有到达视图**。
已排除：`setFocusable`/`setFocusableInTouchMode` 与 C 一致；`FLAG_NOT_FOCUSABLE`
只用于子帧的 `getWindowLayoutParams()`（C 同）。

## 3f. 输入缺口的复核（status 4.952）—— 4.951 的“按键没到视图”不成立

`tools/android-input-probe.sh focus` 的读数（`docs/baseline/android-focus/`）：

| 读数 | 值 |
|---|---|
| `dumpsys input` 的 `FocusedWindows` | `org.femacs/org.femacs.EmacsActivity` ✓ |
| 应用窗口 `inputConfig` | `0x0`（可聚焦、可触摸、受信）✓ |
| `dumpsys activity top` 视图树 | `org.femacs.EmacsView{… VFED…. .F……} ` = **焦点在 EmacsView 上** ✓ |
| `dumpsys input_method` | `mServedView` / `mNextServedView` 都是 `EmacsView` ✓ |

`key` 阶段：`KEYCODE_HOME` 能改变屏幕（注入链路本身有效 ✓），但
`input keyevent 29`（数值 `KEYCODE_A`）与 `input text "x"` 之后，截图差异的包围盒是
**(952,60)-(958,77)**，只有状态栏那 87 个像素（时钟）——正文一个像素都没变 ✗。

`tap` 阶段：`input tap 540 700`、`KEYCODE_A`、`KEYCODE_F10` 三次之后，四张截图 md5
**完全相同** ✗ ⇒ **触点与按键同样无效**，所以缺口不在“按键分发”这一条腿上。

`rotate` 阶段：转成横屏后帧被**重新按 2400×1080 排版**（有墨迹的行 160 行、墨迹 x 范围
164–2356），转回竖屏 md5 与原竖屏**逐字节相同** ⇒ **redisplay、flip、swapBuffers 都是通的**
✓（否则横屏不可能出现重新排版后的内容）。

⇒ 收敛后的结论：UI 线程到 Emacs 事件循环之间，**`ANDROID_CONFIGURE_NOTIFY` 会被处理，
而 `ANDROID_KEY_PRESS` / 指针事件不会**。焦点、可见性、可聚焦性、IME 绑定都已逐条排除。
嫌疑面因此收窄到两处之一：

1. `android_write_event` 之后主线程没有被唤醒（`pthread_cond_broadcast … read_var` 与
   `android_select` 的配对），恰好 `sendConfigureNotify` 走的是另一条即时应答路径；
2. 事件确实入队并被消费，但 `androidterm.c` 的 `android_read_socket` 没有把它变成
   `kbd_buffer` 里的输入事件（`android_event` → `kbd_buffer_store_event` 段）。

两条都不需要设备就能判：C 侧逐句对照已确认 `sendKeyPress`/`sendKeyRelease`/
`android_write_event`/`android_init_events` 的 `sigaction` 段与参照同形（status 4.952 复核），
所以静态对照要往 `android_select` 与 `android_read_socket` 走。

## 3g. Java 侧已经全线打通（status 4.953），缺口在 C 侧或翻转

临时探针（`PROBE4953`/`PROBE4955`，补丁存在 `docs/baseline/probe-4953-4956.patch`）在设备上给出：

```
D EmacsActivity: PROBE4953 DISPATCH 29
D EmacsView:     PROBE4953 KEYDOWN 29
D EmacsWindow:   PROBE4955 pre handle=13830 key=29 uc=97
D EmacsWindow:   PROBE4955 serial=9
D EmacsActivity: PROBE4953 DISPATCH 29          (key up)
D EmacsView:     PROBE4953 TOUCH 0 / TOUCH 1    (tap)
```

⇒ `Activity.dispatchKeyEvent` → `EmacsView.onKeyDown` → `EmacsWindow.onKeyDown` →
`EmacsNative.sendKeyPress (handle, time, state, 29, 97)` **全部执行**，JNI 返回 `event_serial = 9`
（没有抛异常，因为异常会带走进程）；触点也进了 `EmacsView.onTouchEvent`。
4.951 的“按键没到视图”和 4.952 的“焦点/分发未定”到此**全部关闭**。

剩下的可能只有两条：事件入队后 **Lisp 线程没有把它变成缓冲区内容**，或**入队就失败了**。

判这两条的读数：

| 读数 | 值 | 含义 |
|---|---|---|
| `kill -29 <pid>`（SIGIO）之后再截图 | md5 与前一张**相同** | 手动唤醒也不产生任何变化 ⇒ 不是“只差一次唤醒” |
| `SigCgt` | `0x6e5b81eeff`，bit 28 = 1 | SIGIO 确实被捕获（不会被默认动作杀掉） |
| 打 8 个 `a` 再旋转后的横屏 vs 未打字的横屏 | 差异包围盒 **(234,17)-(2152,45)**，359 px | 只差**状态栏**（时钟 + 通知图标），正文与模式行**逐像素相同** |

最后一行同时否掉了“其实插进去了、只是没呈现”这条：模式行没有出现 `**`，正文没变 ⇒
**缓冲区没被修改**。⚠️ 这一条也是 4.952 那轮 `insert-rotate` 一度读成“打字改变了画面”的来源：
那次 md5 不同来自状态栏，不要再用整幅 md5 判断文字是否变化。

## 3h. C 侧读数：事件确实入队，但从来没有人排空（status 4.954）

补丁 `docs/baseline/probe-4953-4958.patch` 里加的三条 C 侧探针，在设备上一次运行给出：

```
I femacsprobe: PROBE4956 write type=0        <- ANDROID_KEY_PRESS
I femacsprobe: PROBE4956 queued=1
I femacsprobe: PROBE4956 write type=1        <- KEY_RELEASE, queued=2
I femacsprobe: PROBE4956 write type=11 / 12  <- TOUCH_DOWN / UP, queued=3 / 4
```

- **`PROBE4956 drain` 一条都没有** ⇒ `handle_one_android_event` 从未被调用，
  `android_read_socket` 也没跑过；队列只涨不消（1→4）。
- **`PROBE4957 select sees queued` 一条都没有** ⇒ `android_select` 从没在“队列里有事件”的状态下被进入。
  如果它压根没被调用过，那就说明主循环等的不是它。
- **`PROBE4958 android arm taken` 一条都没有** ⇒ `wait_reading_process_output` 里的
  `#[cfg (feature = "have_android")]` 臂在运行时没有走到（该函数另有出口，或根本没进这个函数）。
- 线程快照（`tools/android-input-probe.sh threads`，读数在 `docs/baseline/android-focus/emulator-threads.log`
  的同一命令输出里）：`Emacs main thre … futex_wait_queue`（Lisp 主线程停在 futex 上，
  **不是在 select 的 fd 等待上**）、`Emacs polling t … do_sigtimedwait`、`RenderThread … do_epoll_wait`。
  按键之后 Lisp 主线程**位置不变**。

⇒ 缺口因此收敛成一句话：**Android 事件队列对 Lisp 线程不可见**——事件写进去了，但主线程既没在
`android_select` 上等，也没被叫醒去 `read_socket`。这与“GUI 只在启动时画过一帧、之后转屏只是把
旧位图拉伸”这一组现象自洽。**下一处要读的是 `wait_reading_process_output` 到底从哪个出口返回**
（`Fread_char` 路径、`android_wait_event`、还是 select 臂之外的等待），而不是继续在 `EmacsWindow` 上找。

顺带发现一处真偏离（不是探针）：`femacs/src/thread.rs` 的 `thread_select` 把 C 656-658 的
`#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY → android_before_select ()`
记成了“dead”。这一句在 `THREADS_ENABLED` 下是活的，属 1:1 待补项。

## 3i. 根因之一已确认并修复：`SIG_BLOCK` 用的是 Darwin 的 1，bionic 读成 `SIG_UNBLOCK`（status 4.956）

`debuggerd -b` 的原生栈把两边都钉住了：

```
Emacs main thre:  android_select+424 -> sem_wait(select_sem)
                  thread_select -> really_call_select -> android_select
Emacs polling t:  android_run_select_thread+276 -> sigwait
```

也就是 `android_select` 确实在主循环里，卡在“等轮询线程完成这一轮”上，而轮询线程永远等不到
`SELECT_SIGNAL`。接着读 `/proc/<tid>/status`：

| 线程 | `SigBlk` | 含义 |
|---|---|---|
| `Emacs polling t`（修复前） | `0000000000000000` | **一个信号都没被屏蔽** ✗ |
| `Emacs polling t`（修复后） | `fffffe007ffbf8bf` | 绝大多数已屏蔽 ✓（SIGUSR1 仍需单独确认） |

根因：`femacs/lib/include/signal.rs` 把 `SIG_BLOCK/SIG_UNBLOCK/SIG_SETMASK` 硬写成
**Darwin 的 1/2/3**，而 bionic 是 **0/1/2**（NDK `asm-generic/signal-defs.h` 48/51/54）。
于是 `android_run_select_thread` 里的 `pthread_sigmask (SIG_BLOCK, &all, NULL)` 在 Android 上
实际是 `SIG_UNBLOCK` —— 轮询线程把所有信号都**解开**了：SIGUSR1 被 handler 吃掉而不是挂起，
`sigwait` 永不返回，`android_select` 也就永不返回，事件队列因此只涨不消。
（`android.rs` 里另有一个 `bionic_signal::SIG_BLOCK = 0` 的局部补丁，只覆盖了 3610 一处；
927 与 2567 两处仍走镜像里的 Darwin 值。）

修复：镜像里的三个常量按 `feature = "have_android"` 分成 bionic / Darwin 两套值（带上述出处）。
修复后 `SigBlk` 从全零变成屏蔽绝大多数信号，**轮询线程第一次真正进了不该被打断的等待态**。

## 3j. 修复之后剩下的缺口（下一处）

修复后按键仍然不落屏，但**停顿点前移了一层**：

- 空闲时轮询线程在 `pselect`（`wchan=do_select`），按键后变成 `wchan=do_sigtimedwait`（`sigwait`）；
- 日志顺序：`write type=0` → `queued=1` → `pthread_kill rc=0`，之后既没有 `sigwait back`，
  也没有 `select return`。

按 C 的设计，`pselect` 期间的掩码是“除 SIGUSR1 之外全部屏蔽”，SIGUSR1 因此在 pselect 里
**可送达**，handler `android_handle_poll_signal` 会把 `cancel_signal_received` 置 1，
线程据此**跳过** `sigwait` 直接 `sem_post(select_sem)`。现在的现象说明这一步没成立 ——
最可能是 handler 里 `pthread_getspecific (poll_thread_internal)` 拿不到队列（或置的标志不是
`android_select` 读的那一个结构），于是 `cancel_signal_received` 仍是 0，线程照样进 `sigwait`，
而 Lisp 线程那一次 `pthread_kill` 已经用掉了 ⇒ 双方互等。

## 3k. 第二根因：`add_user_signal (SIGUSR1)` 在 Android 上本该是死码，端口把它编译活了（status 4.958）

修掉 `SIG_BLOCK` 之后，探针把停顿点压到了最后一步：

```
PROBE4956 write type=0                    键事件入队
PROBE4959 kill rc=0                       Lisp 线程发出 SELECT_SIGNAL
PROBE4959 pselect rc=-1 cancel=0          pselect 被 EINTR 打断，但 cancel 仍是 0
PROBE4959 sigwait enter                   于是线程仍然进 sigwait
（之后队列继续涨，没有 read_socket）
```

`pselect rc=-1` 说明信号**送到了**，但 `cancel_signal_received` 没被置 1 —— 也就是
`android_handle_poll_signal` **没有跑**（加在 handler 头部的探针 `PROBE4960` 一条都没有）。

根因在 `sysdep.c` 2080-2087：

```c
#if !defined HAVE_ANDROID
#ifdef SIGUSR1
  add_user_signal (SIGUSR1, "sigusr1");
#endif
```

Android 上这段是**死码**，因为 SIGUSR1 是 `android_select` 的内部信号；端口把它原样编译活了
（注释还写着“`#if !defined HAVE_ANDROID` is live”，对 android 配置恰好是反的）。
`add_user_signal` 会把 SIGUSR1 变成 Lisp 的用户信号 → Emacs 装自己的 handler **覆盖掉**
`android_handle_poll_signal` → 轮询用的信号被吞 → `sigwait` 永远等不到。

修复：这一段按 C 的 `#if !defined HAVE_ANDROID` 加 `#[cfg (not (feature = "have_android"))]`。

## 3l. 修复后的读数：输入通了（status 4.958）

同一次运行（`docs/baseline/android-focus/probe-fix2.log`）：

```
PROBE4956 write type=0                    key press
PROBE4960 handler sig=10 queue_null=0     poll handler 终于跑了
PROBE4956 read_socket enter pending=1     队列被排空
PROBE4956 read_socket enter pending=0
```

按键前后截图 md5 **不同** ✓（`fix2-before.png` / `fix2-after.png`，差异覆盖整帧 —— 这是第一次
真正的整帧重画）。⇒ **Android GUI 的输入链路打通**：键 → 事件 → `read_socket` → 缓冲区 →
redisplay。

## 3m. 验收三元组（status 4.962）：闭合

`tools/android-accept.sh` 一次运行，读数在 `docs/baseline/android-accept/`：

| 项 | 读数 |
|---|---|
| 渲染 | 帧含菜单栏与缓冲区文本（`1-baseline.png`）✓ |
| 输入 | 打 `H E L L O` 后逐字可辨 **H E L L O**、块状光标紧随其后（`2-typed.png`）；模式行同时转为已修改 ✓ |
| 存活 | 打字 / 轻点 / 转横屏 / 转回竖屏全程 `pid` 不变 ✓，`Fatal signal` **0 条** ✓ |
| 五张截图 md5 | `c7796d07…` / `55e1a7fa…` / `2fa60b2b…` / `956bec6c…` / `64e42c21…` 两两不同 ✓ |

闭合的三个根因（按发现顺序）：

1. `SIG_BLOCK/SIG_UNBLOCK/SIG_SETMASK` 用了 Darwin 的 1/2/3，bionic 是 0/1/2（§3i）；
2. `add_user_signal (SIGUSR1/SIGUSR2)` 在 Android 上本该是死码，端口把它编译活了（§3k）；
3. `kbd_buffer_get_event` 的 PINCH 分支丢了 C 的 `&& (… || …)` 分组（§3l 之后的 4.960）。

第 3 条的教训写进了 `PORTING.md` P-321：token 门只比较标识符与数字，**标点与括号不进序列**，
所以括号重排不可能被它报红 —— 这是已知盲区，留给全面验收阶段处理。

## 4. 仍然存在的（不影响上面的结论）

- 模拟器每次都要冷启动（`-no-snapshot-save`），单轮验收约 2–3 分钟；
- 宿主可用内存低于约 1 GiB 时模拟器会 `Abort trap: 6`（本轮踩过两次），脚本改为固定
  `-port 5560` 并在同一进程内完成全部步骤（后台启动的模拟器会随调用 shell 一起退出）。

## 5. 复现命令

```
./tools/android-accept.sh                    # 渲染 / 输入 / 存活 三元组
./tools/android-input-probe.sh threads|bt|probes|key|tap|rotate|insert-rotate|signal|focus|applog
# 探针补丁：docs/baseline/probe-4953-4960.patch（git apply 后先 build-one 再 link-shared）
```
