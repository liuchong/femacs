# Android GUI：白屏缺陷（独立跟踪项）

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

## 4. 下一步（按代价排序）

1. **一条读数即可判定**：在 `EmacsView.onDraw` 里同时记录
   `frame === bitmap`、`bitmap.getPixel(1,20)`、以及 `canvas.isHardwareAccelerated`；
   若 `onDraw` 的次数在 `PRESENT` 之后不再增长 ⇒ 是 (1)；若在增长但像素仍白 ⇒ 是 (2)。
2. 若 (2)：在 `handleDirtyBitmap()` 之后把新 `canvas` **回写**给 Emacs（C 侧无需改动，
   `getCanvas` 每次重新取 `canvas` 字段），并确认没有旧 `Canvas` 被缓存。
3. 若 (1)：把 `present()` 改成 `postInvalidateOnAnimation()` 并在 UI 线程上 `invalidate()`。
4. 设备环境要求：`-gpu guest`（swiftshader 会引入额外噪声）、至少 8 GiB 空闲磁盘
   （低于 ~3 GiB 时模拟器会拒启动，见 `docs/status.md` 4.925/4.933）。

## 5. 复现命令

```
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- bash /tmp/crash-hunt.sh
# 截图 → docs/baseline/android-shot-12s.png；非白比：
ffmpeg -v quiet -i docs/baseline/android-shot-12s.png -f rawvideo -pix_fmt rgb24 - | python3 -c "…"
```
