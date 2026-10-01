#include <config.h>
#include <stdio.h>
#include <stdbool.h>
#include "character.h"
/* multibyte_length / STRING_CHAR / raw_prev_char_len are header-only, so this
   probe needs no runtime (char_string, which is not, is avoided). */
static void one (unsigned char *buf, int n) {
  printf ("%02x%02x%02x%02x%02x %d %d %d %d %d %d\n", buf[0], buf[1], buf[2], buf[3], buf[4],
          n, multibyte_length (buf, NULL, false, true), multibyte_length (buf, NULL, false, false),
          multibyte_length (buf, buf + n, true, true), STRING_CHAR (buf),
          n > 1 ? raw_prev_char_len (buf + n) : 1);
}
int main (void) {
  static const int v[] = {0x00, 0x41, 0x80, 0x88, 0x9F, 0xA0, 0xBF, 0xC0, 0xFF};
  unsigned char b[6];
  /* one-byte forms */
  for (int i = 0; i < 9; i++) { b[0] = (unsigned char) v[i]; one (b, 1); }
  /* two, three and four byte forms: sweep the continuation bytes */
  for (int a = 0xC0; a <= 0xDF; a += 0x1F)
    for (int i = 0; i < 9; i++) { b[0] = (unsigned char) a; b[1] = (unsigned char) v[i]; one (b, 2); }
  for (int a = 0xE0; a <= 0xEF; a += 0x0F)
    for (int i = 0; i < 9; i++) for (int j = 0; j < 9; j++) {
      b[0] = (unsigned char) a; b[1] = (unsigned char) v[i]; b[2] = (unsigned char) v[j]; one (b, 3);
    }
  for (int a = 0xF0; a <= 0xF7; a += 0x07)
    for (int i = 0; i < 9; i += 2) for (int j = 0; j < 9; j += 4) for (int k = 0; k < 9; k += 8) {
      b[0] = (unsigned char) a; b[1] = (unsigned char) v[i]; b[2] = (unsigned char) v[j];
      b[3] = (unsigned char) v[k]; b[4] = 0; one (b, 4);
    }
  /* the five-byte forms the precedence bug rejected */
  for (int a = 0x88; a <= 0x8F; a++)
    for (int i = 0; i < 9; i++) for (int j = 0; j < 9; j++) for (int k = 0; k < 9; k++) {
      b[0] = 0xF8; b[1] = (unsigned char) a; b[2] = (unsigned char) v[i];
      b[3] = (unsigned char) v[j]; b[4] = (unsigned char) v[k]; one (b, 5);
    }
  return 0;
}
