# Crab Chess

Yet another chess game implementation, focusing on Rust and Web technologies.

Featuring:
* Sycamore (https://github.com/sycamore-rs/sycamore)
* Storybook (https://storybook.js.org/)
* Bevy (https://bevyengine.org/)
* [TODO] Tauri (https://tauri.app/)
* [TODO] Capacitor (https://capacitorjs.com/)
* [TODO] Hyper (https://hyper.rs/)
* [TODO] Hyper-Tungstenite (https://docs.rs/hyper-tungstenite/latest/hyper_tungstenite/)


## Running

### Storybook

Run trunk
```bash
cd storybook
trunk build # or trunk watch
```

Then run storybook in another terminal
```bash
cd storybook
pnpm run storybook
```

## Issues

[ ] Need to find out how to load several Bevy instances (winit event loops) at the same time
