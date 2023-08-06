package main

import (
	"encoding/binary"
	"os"
)

const romLen = 32768

var rom []byte

func main() {
	rom = make([]byte, romLen)
	for i := range rom {
		rom[i] = 0xea
	}

	// rom[0x7ffd] = 0x11
	// rom[0x7ffc] = 0x22
	file, err := os.Create("rom.bin")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	if err = binary.Write(file, binary.LittleEndian, rom); err != nil {
		panic(err)
	}
}
