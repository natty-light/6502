package main

import (
	"encoding/binary"
	"os"
)

const romLen = 32768

var rom []byte
var program []byte = []byte{
	0xa9, 0xff, // lda #$ff
	0x8d, 0x02, 0x60, // sta #6002
	0xa9, 0x55, // lda #$55
	0x8d, 0x00, 0x60, // sta #6000
	0xa9, 0xaa, //lda #$aa
	0x8d, 0x00, 0x60, // sta #6000
	0x4c, 0x05, 0x80, // jmp #8005
}

func main() {
	rom = make([]byte, romLen-len(program))
	for i := range rom {
		rom[i] = 0xea
	}
	rom = append(program, rom...)
	rom[0x7ffc] = 0x00
	rom[0x7ffd] = 0x80
	file, err := os.Create("rom.bin")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	if err = binary.Write(file, binary.LittleEndian, rom); err != nil {
		panic(err)
	}
}
