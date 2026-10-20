# Android GUI：渲染已通、输入未验收（独立跟踪项）

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

## 4. 下一步

1. **静态**（不需要设备/编译）：逐句对照 `femacs/src/android.rs` 的 `android_select`
   与 `emacs/src/android.c` 的同名函数（`read_var` 的等待条件、`num_events` 的判定、
   `select_sem` 的配对），再对照 `androidterm.rs` 的 `android_read_socket`。
2. **一次带探针的构建**（设备可用且磁盘 ≥8 GiB 时）：在 `EmacsWindow.onKeyDown` 入口、
   `android_write_event` 入口、`android_select` 返回处各放一条 `__android_log_print`，
   一次运行即可分辨上面第 1/2 条。不要为零散假设反复重编（用户要求）。
3. 设备环境要求：`-gpu guest`、数据卷**至少 8 GiB 空闲**；低于 ~3 GiB 模拟器直接拒启动
   （4.925/4.933，本轮 4.952 再次撞到：数据卷从 5.3 GiB 降到 2.0 GiB，停止设备腿）。

## 5. 复现命令

```
./tools/android-input-probe.sh focus    # 焦点/输入配置/视图树/IME
./tools/android-input-probe.sh key      # HOME 自检 + 按键 + 输入法提交
./tools/android-input-probe.sh tap      # 触点 / 按键 / F10 三连
./tools/android-input-probe.sh rotate   # 旋转重排（redisplay 是否还活）
./tools/android-input-probe.sh back     # 三连 BACK（Emacs 吞 BACK 的旁证）
./tools/android-input-probe.sh applog   # 按键前后的应用进程日志
```
