package main

import (
	"github.com/myl7/bsize"
	"log"
)

func main() {
	num, unit, err := bsize.Parse("10M", bsize.BiStrategyCheckBi)
	if err != nil {
		log.Fatalln(err)
	}
	log.Printf("num = %d, uint = %d", num, unit)
}
