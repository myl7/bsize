#ifdef __cplusplus
namespace bsize {

extern "C" {

#include <stdint.h

struct BsizeRes;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
extern struct BsizeRes BsizeParse(const char *p, bool ignore_bi);
#pragma clang diagnostic pop

} // extern "C"

#include <string>

static inline BsizeRes BsizeParseString(std::string s, bool ignore_bi) {
  return BsizeParse(s.c_str(), ignore_bi)
}

} // namespace bsize
#endif // __cplusplus
