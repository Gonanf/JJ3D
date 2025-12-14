import sys
import sdl2
import sdl2.ext

HEIGHT = 200
WIDTH = 320


def lineaRecta(pixels, y, x):
    if x >= HEIGHT:
        return
    for p in range(HEIGHT - x):
        pixels[y][p] = 255


def main():
    sdl2.ext.init()
    window = sdl2.ext.Window("PySDL2 Test", size=(WIDTH, HEIGHT))
    window.show()

    sur = window.get_surface()
    pixels = sdl2.ext.pixelaccess.pixels2d(sur)

    lineaRecta(pixels, 100, 0)

    # surface = sdl2.ext.SoftwareSpriteRenderSystem(window)
    # sdl2.ext.pixelaccess.pixels2d(surface)[100][100] = 255

    sdl2.ext.TestEventProcessor().run(window)

    sdl2.ext.quit()
    return 0


if __name__ == "__main__":
    sys.exit(main())
