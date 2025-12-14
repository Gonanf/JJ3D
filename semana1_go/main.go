package main

import (
	"fmt"

	"github.com/veandco/go-sdl2/sdl"
)

func main() {
	fmt.Println("initing")
	sdl.Init(sdl.INIT_EVERYTHING)
	defer sdl.Quit()
	window, err := sdl.CreateWindow("windo", sdl.WINDOWPOS_UNDEFINED, sdl.WINDOWPOS_UNDEFINED, 200, 200, sdl.WINDOW_SHOWN)
	if err != nil {
		fmt.Println(err)
	}
	defer window.Destroy()

	surface, err := window.GetSurface()
	if err != nil {
		fmt.Println(err)
	}

	surface.FillRect(nil, 0)
	surface.Lock()
	window.UpdateSurface()

	running := true
	for x := 0; x < (200*4)-1; x += 4 {
		for y := 0; y < (200*4)-1; y += 4 {
			surface.Pixels()[(y*200)+x] = 255
			surface.Pixels()[(y*200)+(x+1)] = 255
			surface.Pixels()[(y*200)+(x+2)] = 255
		}
		window.UpdateSurface()
		sdl.Delay(50)
	}
	fmt.Println("Done")
	for running {
		for event := sdl.PollEvent(); event != nil; event = sdl.PollEvent() {
			switch event.(type) {
			case *sdl.QuitEvent:
				println("Quit")
				running = false
				break
			}
		}
	}
}
