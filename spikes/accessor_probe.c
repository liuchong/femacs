#include <config.h>
#include <stdio.h>
#include "lisp.h"
#include "buffer.h"
/* The C side of spikes/accessor-probe.sh: the buffer.h accessors that need no runtime
   beyond a buffer whose text lives in a static array.  current_buffer is thread-local
   state (P-19), and marker.c's two conversion functions are stubbed to the identity so
   CHAR_POS_ADDR / CHAR_TO_BYTE / BYTE_TO_CHAR are reachable too.  */
static struct thread_state TS;
struct thread_state *current_thread = &TS;
static struct buffer B;
static struct buffer_text T;
static unsigned char ARR[64];
ptrdiff_t buf_charpos_to_bytepos (struct buffer *b, ptrdiff_t c) { return c; }
ptrdiff_t buf_bytepos_to_charpos (struct buffer *b, ptrdiff_t c) { return c; }
int main (void) {
  current_thread->m_current_buffer = &B;
  B.text = &T; T.beg = ARR;
  for (int i = 0; i < 64; i++) ARR[i] = (unsigned char) (0x40 + i);
  B.begv = 2; B.pt = 3; B.zv = 5; B.begv_byte = 2; B.pt_byte = 3; B.zv_byte = 5;
  T.gpt = 4; T.gpt_byte = 4; T.z = 6; T.z_byte = 6; T.gap_size = 3;
  printf ("pos %td %td %td %td %td %td %td %td %td %td %td %td %td %td %td %td\n",
          BEGV, PT, GPT, GPT_BYTE, ZV, ZV_BYTE, Z, Z_BYTE, GAP_SIZE,
          BUF_BEG (current_buffer), BUF_BEG_BYTE (current_buffer),
          BUF_PT (current_buffer), BUF_PT_BYTE (current_buffer),
          BUF_ZV (current_buffer), BUF_ZV_BYTE (current_buffer), BUF_COMPACT (current_buffer));
  for (int n = 1; n <= 6; n++)
    printf ("addr %d %td %d %td %td %td %u %u %d %td\n", n,
            (ptrdiff_t) (BYTE_POS_ADDR (n) - BEG_ADDR), (int) BYTE_POS_ADDR (n)[0],
            PTR_BYTE_POS (BYTE_POS_ADDR (n)), CHAR_POS_ADDR (n) - BEG_ADDR,
            BUF_PTR_BYTE_POS (current_buffer, BYTE_POS_ADDR (n)),
            (unsigned) FETCH_BYTE (n), (unsigned) BUF_FETCH_BYTE (current_buffer, n),
            (int) PTR_BYTE_POS (BUF_BYTE_ADDRESS (current_buffer, n)), CHAR_TO_BYTE (n));
  printf ("gap %td %td %td\n",
          BUF_GPT_ADDR (current_buffer) - BEG_ADDR,
          BUF_Z_ADDR (current_buffer) - BEG_ADDR,
          BUF_GAP_END_ADDR (current_buffer) - BEG_ADDR);
  T.modiff = 10; T.unchanged_modified = 10;
  T.overlay_modiff = 20; T.overlay_unchanged_modified = 20;
  T.beg_unchanged = 0; T.end_unchanged = 0;
  BUF_COMPUTE_UNCHANGED (current_buffer, 2, 2);
  printf ("unch eq %td %td\n", BUF_BEG_UNCHANGED (current_buffer), BUF_END_UNCHANGED (current_buffer));
  T.overlay_unchanged_modified = 21;
  T.beg_unchanged = 0; T.end_unchanged = 0;
  BUF_COMPUTE_UNCHANGED (current_buffer, 2, 2);
  printf ("unch ne %td %td\n", BUF_BEG_UNCHANGED (current_buffer), BUF_END_UNCHANGED (current_buffer));
  T.beg_unchanged = 5; T.end_unchanged = 9;
  BUF_COMPUTE_UNCHANGED (current_buffer, 3, 4);
  printf ("unch min %td %td\n", BUF_BEG_UNCHANGED (current_buffer), BUF_END_UNCHANGED (current_buffer));
  return 0;
}
