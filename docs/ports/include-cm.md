# emacs/src/cm.h (173 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 45
- [x] AbsPosition  (line 116, body 4466e5e35851)
- [x] AbsPositionCost  (line 138, body 69fc0f0a7a41)
- [x] AutoWrap  (line 123, body 40f9ca64ed67)
- [x] BackTab  (line 111, body 2589c89548ca)
- [x] BackTabCost  (line 137, body 605a5967c476)
- [x] CR  (line 113, body 72bfeefca4b4)
- [x] CRCost  (line 134, body 5eaf960e98fe)
- [x] ColPosition  (line 117, body c622d63668f6)
- [x] ColPositionCost  (line 139, body f78cdca20a3b)
- [x] Down  (line 107, body 18302130d9ed)
- [x] DownCost  (line 130, body f76e1adb498c)
- [x] EMACS_CM_H  (line 20, body da39a3ee5e6b)
- [x] FrameCols  (line 127, body 4df7acfe0efd)
- [x] FrameRows  (line 126, body 04596f589bd1)
- [x] Home  (line 114, body 42574e8dce88)
- [x] HomeCost  (line 133, body 5ac33c419cfb)
- [x] LastLine  (line 115, body 26f55fe1dbe2)
- [x] LastLineCost  (line 135, body 56ca1d5dada2)
- [x] Left  (line 108, body 8481bb373ef0)
- [x] LeftCost  (line 131, body 34fc840f4fd4)
- [x] MagicWrap  (line 124, body 7b72a21ebab4)
- [x] MultiDown  (line 120, body 837e3cf2870b)
- [x] MultiDownCost  (line 142, body 5110a4dd21b4)
- [x] MultiLeft  (line 121, body eb67fc6b1392)
- [x] MultiLeftCost  (line 143, body f795f46fcb79)
- [x] MultiRight  (line 122, body 76fdf04b78b6)
- [x] MultiRightCost  (line 144, body 85855aae1825)
- [x] MultiUp  (line 119, body 061e1f081bac)
- [x] MultiUpCost  (line 141, body d718431bfb1c)
- [x] Right  (line 109, body 483137784b83)
- [x] RightCost  (line 132, body 4c2c48de0f02)
- [x] RowPosition  (line 118, body db23f6571dcd)
- [x] RowPositionCost  (line 140, body 112109f0af89)
- [x] Tab  (line 110, body cb8a7d5a80db)
- [x] TabCost  (line 136, body 337fcc0b1816)
- [x] TabWidth  (line 112, body ee32bc65d18f)
- [x] Up  (line 106, body 70c2a09bfb53)
- [x] UpCost  (line 129, body 85fcc057cf20)
- [x] UseTabs  (line 125, body ce0961c4241a)
- [x] cmat  (line 147, body 75df08d35237)
- [x] cmplus  (line 148, body f1a698401d83)
- [x] curX  (line 105, body a4d034e2864c)
- [x] curY  (line 104, body d0f259f96ca6)
- [x] emacs_tputs  (line 163, body c35e6a6a1416)
- [x] losecursor  (line 158, body 5eaed4358ca5)

## Types and their fields — 2
- [x] cm: int cm_curY, int cm_curX, const char* cm_up, const char* cm_down, const char* cm_left, const char* cm_right, const char* cm_home, const char* cm_cr, const char* cm_ll, const char* cm_tab, const char* cm_backtab, char* cm_abs, const char* cm_habs, const char* cm_vabs, const char* cm_multiup, const char* cm_multidown, const char* cm_multileft, const char* cm_multiright, int cm_cols, int cm_rows, int cm_tabwidth, bool_bf cm_autowrap : 1, bool_bf cm_magicwrap : 1, bool_bf cm_usetabs : 1, bool_bf cm_losewrap : 1, bool_bf cm_autolf : 1, int cc_up, int cc_down, int cc_left, int cc_right, int cc_home, int cc_cr, int cc_ll, int cc_tab, int cc_backtab, int cc_abs, int cc_habs, int cc_vabs
- [x] parmcap: int mincost, int maxcost

## File-scope objects — 2
- [x] extern int cost  (line 160)
- [x] extern struct tty_display_info current_tty  (line 165)
