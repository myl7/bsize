/*
MIT License

Copyright (c) 2021 myl7

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */


#ifndef BSIZE
#define BSIZE

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "bsize_utils.h"

#ifdef __cplusplus
namespace bsize {
#endif // __cplusplus

typedef enum BsizeBiStrategy {
  kCheckBi = 1,
  kAlwaysTen = 2,
  kAlwaysBi = 3,
  kRevertBi = 4,
} BsizeBiStrategy;

typedef enum BsizeUnit {
  kBit = 1,
  kByte = 2,
  kNone = 3,
} BsizeUnit;

typedef struct BsizeRes {
  int error;
  uint64_t num;
  enum BsizeUnit unit;
} BsizeRes;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct BsizeRes BsizeParse(const char *p, enum BsizeBiStrategy bi_strategy);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#ifdef __cplusplus
} // namespace bsize
#endif // __cplusplus

#endif /* BSIZE */
