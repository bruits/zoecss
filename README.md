# ZoeCSS

> Nothing in life is to be feared, it is only to be understood. Now is the time to understand more, so that we may fear less.

An on-demand Atomic CSS engine (work in progress), with a strong focus on performance and extensibility.

## Crates

ZoeCSS is a monorepo that contains the following crates (Rust packages):

| Name             | Description                                     | Registry | README                                      |
| ---------------- | ----------------------------------------------- | -------- | ------------------------------------------- |
| `zoecss`         | CLI — scan, extract, cache, and output          | *WIP*    | [README](./crates/zoecss/README.md)         |
| `zoecss-core`    | Core CSS generation engine (match tokens → CSS) | *WIP*    | [README](./crates/zoecss-core/README.md)    |
| `zoecss-config`  | Configuration model, merging, and compilation   | *WIP*    | [README](./crates/zoecss-config/README.md)  |
| `zoecss-presets` | Composable configuration presets                | *WIP*    | [README](./crates/zoecss-presets/README.md) |

## Acknowledgements

ZoeCSS is deeply inspired by the work of [Anthony Fu](https://antfu.me/posts/reimagine-atomic-css) on [UnoCSS](https://github.com/unocss/unocss), please check it out!

The name « Zoe » is an homage to [Zoé](https://en.wikipedia.org/wiki/Zo%C3%A9_(reactor)), the first french nuclear reactor, and to the [Joliot-Curie family](https://en.wikipedia.org/wiki/Curie_family), pioneers of nuclear science and chemistry.

ZoeCSS is an open-source project born from [Bruits](https://bruits.org/), a Rust-focused collective 💛
