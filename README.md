# Mandelbrot Set Explorer

A real-time Mandelbrot set visualizer written in Rust. Click and drag to zoom into the fractal, right-click to reset.

![Mandelbrot Set](https://github.com/user-attachments/assets/2ca98948-b791-4142-a4db-0229455552d9)

## What is this?

This is a simple interactive viewer for the Mandelbrot set --one of the most famous fractals in mathematics--. The visualization uses smooth coloring based on the escape time algorithm, which creates those beautiful color gradients you see around the edges.

I built this to learn more about Rust and to have something pretty to look at while procrastinating. It's surprisingly satisfying to zoom into different parts of the fractal and see the patterns repeat infinitely. (not infinitely for sure because of the finite precision of floating point numbers)

## How to use

**Desktop:**
- Click and drag to select an area to zoom into
- Right-click to reset back to the full view
- Close the window when you're done exploring

**Web:**
- Same controls work in the browser version

## Running it

You'll need Rust installed. If you don't have it, grab it from [rustup.rs](https://rustup.rs/).

```bash
cargo run --release
```

### Web version

To build for the web:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Then open `index.html` in a browser. You might need to serve it with a local server if you hit CORS issues:

```bash
python3 -m http.server 8000
```

or in a more rusty way 

```bash
cargo install basic-http-server
basic-http-server .
```

## Dependencies

- **macroquad** - for graphics and windowing (works natively and in WASM)
- **num-complex** - complex number math

## How it works

The code checks each pixel to see if it's in the Mandelbrot set by repeatedly applying the formula `z = z² + c` and seeing if it "escapes" to infinity. Points that escape quickly get bright colors, points that escape slowly get darker colors, and points that never escape are black (those are actually in the set).

I'm using HSV color space and smooth coloring to avoid the banding you often see in simpler implementations. The color calculation uses logarithms to interpolate between iteration counts, which is why the gradients look smooth instead of having discrete bands.

## Known issues

- Zooming in too far eventually hits floating point precision limits and things get blocky
- The iteration count is hardcoded at 255, so deep zooms might show false "interior" regions
- No way to adjust colors or iteration depth without editing the code
- Performance could be better with parallelization or GPU compute
