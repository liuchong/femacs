#include <config.h>
#include <stdio.h>
#include "lisp.h"
#include "buffer.h"
/* The C side of spikes/write-probe.sh: the places C's buffer.h macros denote, and the
   bitfields the mirror packs by mask.  current_buffer is thread-local state, so the
   probe supplies the storage P-19 records.  */
static struct thread_state TS;
struct thread_state *current_thread = &TS;
static struct buffer B;
static struct buffer_text T;
static unsigned char ARR[64];
static struct itree_node N;
int main (void) {
  current_thread->m_current_buffer = &B;
  B.text = &T; T.beg = ARR;
  GPT = 7; ZV += 3; ZV_BYTE = 9; GAP_SIZE = 4; MODIFF = 5;
  modiff_count *m = &MODIFF; *m += 1;
  SAVE_MODIFF = MODIFF; CHARS_MODIFF = 2; BEGV = 2; Z = ZV; Z_BYTE = ZV_BYTE;
  BEG_UNCHANGED = 1; END_UNCHANGED = 3;
  FETCH_BYTE (1) = 0x41;
  printf ("places %td %td %td %td %td %td %td %td %td %td %td %td %d %d\n",
          GPT, ZV, ZV_BYTE, GAP_SIZE, MODIFF, SAVE_MODIFF, CHARS_MODIFF, BEGV, Z, Z_BYTE,
          BEG_UNCHANGED + END_UNCHANGED, BUF_GPT (current_buffer), FETCH_BYTE (1),
          (int) BYTE_POS_ADDR (1)[0]);
  B.prevent_redisplay_optimizations_p = 1; B.clip_changed = 1;
  B.inhibit_buffer_hooks = 1; B.long_line_optimizations_p = 1;
  T.inhibit_shrinking = 1; T.redisplay = 1;
  printf ("bits %u %u\n", (unsigned) *(unsigned char *) ((char *) &B + 960),
          (unsigned) *(unsigned char *) ((char *) &T + 136));
  B.prevent_redisplay_optimizations_p = 0; B.clip_changed = 0;
  B.inhibit_buffer_hooks = 0; B.long_line_optimizations_p = 0;
  T.inhibit_shrinking = 0; T.redisplay = 0;
  printf ("cleared %u %u\n", (unsigned) *(unsigned char *) ((char *) &B + 960),
          (unsigned) *(unsigned char *) ((char *) &T + 136));
  N.red = 1; N.rear_advance = 1; N.front_advance = 1;
  printf ("itree %u\n", (unsigned) *(unsigned char *) ((char *) &N + 72));
  N.red = 0; N.rear_advance = 0; N.front_advance = 0;
  return 0;
}
