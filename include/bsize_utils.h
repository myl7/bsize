#ifdef __cplusplus
namespace bsize {

extern "C" {

enum BsizeBiStrategy;

struct BsizeRes;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
extern struct BsizeRes BsizeParse(const char *p, enum BsizeBiStrategy bi_strategy);
#pragma clang diagnostic pop

} // extern "C"

#include <string>

static inline BsizeRes BsizeParseString(std::string s, BsizeBiStrategy bi_strategy) {
  return BsizeParse(s.c_str(), ignore_bi)
}

} // namespace bsize
#endif // __cplusplus
