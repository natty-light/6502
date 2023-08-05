package main

import "os"

const romLen = 32768

var rom []byte

func main() {
	rom = make([]byte, romLen)
	for i := range rom {
		rom[i] = 0xea
	}

	file, err := os.Create("rom.bin")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	if _, err = file.Write(rom); err != nil {
		panic(err)
	}
}
