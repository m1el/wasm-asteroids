# SVG Asteroids

This is a demo of an Asteroids clone written in Rust, rendering to an SVG canvas.

[Live demo](https://m1el.github.io/wasm-asteroids/demo/demo.html)

## Architecture

Most communication with the web page is going through an event loop.

Event loop usage looks like this:

```rust
    EventLoop::new(Box::new(move |event, event_loop| {
        // process event
    }))
```

Events are:

```rust
    enum Event {
        Destroyed,
        AnimationFrame,
        MouseMove { x: u32, y: u32 },
        KeyDown { code: u32, chr: Option<char>, flags: u32 },
        KeyUp { code: u32, chr: Option<char>, flags: u32 },
    }
```

Event Loop from Rust side is defined in [src/eventloop.rs](src/eventloop.rs).
It's technically possible to have multiple event loops running on the same page.
Event Loops on Rust side need to be explicitly disposed, in order for the JS side to dispose of event loop correctly.

The JS implementation of event loop is located in [demo/demo.js](demo/demo.js) automatically subscribes to Mouse and Keyboard events on window object and provides an interface for `requestAnimationFrame`.

## Game logic

Game logic is mostly implemented in [src/game.rs](src/game.rs), and it's very messy.
I don't have a lot of experience making games, and I think I learned some lessons from making this little demo, which I'll take into account.

For now, I'm happy with the result, and I do not plan to refactor this game in the near future.

## Rendering

Rendering is done using by setting the path `d` attribute:

```javascript
    svg_set_path: (ptr, len) => window.path.setAttributeNS(null, 'd', getStr(Module, ptr, len)),
```

Rust generates path points in [src/render\_path.js](src/render_path.rs), which contains paths for digits, ship, etc.

## LICENSE

The MIT License
