#include <stdio.h>
#include "bsize.h"

int main() {
  BsizeRes res = BsizeParse("10M", kCheckBi);
  printf("error = %d, num = %lu, unit = %d\n", res.error, res.num, (int)res.unit);
  return 0;
}
