# Unofficial Carnegie Randomizer

Use the tool [here](https://alexkazik.github.io/random-carnegie/).

## Running it yourself

### Requirements

- https://rustup.rs/
- `rustup target add wasm32-unknown-unknown`
- https://trunkrs.dev/#install

### Running

Run this application with the trunk development server:

```bash
trunk serve --open
```

### Building

```bash
trunk build
```

If the application will not be in the domain root (e.g. `https://example.com/random-carnegie`):

```bash
trunk build --public-url /random-carnegie
```
