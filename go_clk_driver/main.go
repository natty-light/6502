package main

import (
	"machine"
	"time"
)

const delay = 250 * time.Millisecond

var (
	isManual          = false
	shouldToggleClock = false
)

func main() {
	clk := machine.PD5
	mBtn := machine.PD2
	cBtn := machine.PD3

	clk.Configure(machine.PinConfig{Mode: machine.PinOutput})
	mBtn.Configure(machine.PinConfig{Mode: machine.PinInput})
	cBtn.Configure(machine.PinConfig{Mode: machine.PinInput})

	for {
		if isManual {
			toggleClock(clk)
		} else if shouldToggleClock {
			toggleClock(clk)
			shouldToggleClock = false
		}
	}

}

func INT0() {
	isManual = !isManual
}

func INT1() {
	shouldToggleClock = true
}

func toggleClock(clk machine.Pin) {
	clk.High()
	time.Sleep(delay)
	clk.Low()
	time.Sleep(delay)
}
