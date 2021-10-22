package bsize

// #cgo CPPFLAGS: -I${SRCDIR}/include
// #cgo LDFLAGS: -L${SRCDIR}/lib -lbsize -Wl,--no-as-needed -ldl -lm
// #include "bsize.h"
import "C"
import (
	"errors"
	"strconv"
)

const (
	BiStrategyCheckBi   = 1
	BiStrategyAlwaysTen = 2
	BiStrategyAlwaysBi  = 3
	BiStrategyRevertBi  = 4
)

const (
	UnitBit  = 1
	UnitByte = 2
	UnitNone = 3
)

func Parse(s string, biStrategy int) (uint64, int, error) {
	res := C.BsizeParse(C.CString(s), uint32(biStrategy))
	if res.error != 0 {
		return 0, 0, errors.New(strconv.Itoa(int(res.error)))
	}
	return uint64(res.num), int(res.unit), nil
}
