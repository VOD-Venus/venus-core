# Venus core

[Venus](https://github.com/VOD-Venus/venus) core library. Spawn v2ray core as a child process and controls its configs.

## Design

After process spawnned we will create a `mpsc::channel` to pipe `stdio` into the channel.

## Example

```rust
let mut venus = venus.lock()?;
venus
    .config
    .reload_rua()
    .with_context(|| "reading venus configuration failed")?;
venus
    .config
    .reload_core()
    .with_context(|| "reading core configuration failed")?;
venus
    .config
    .write_core()
    .with_context(|| "write core configuration failed")?;
venus.spawn_core().with_context(|| "staring core failed")?;
let child_rx = venus
    .child_rx
    .take()
    .ok_or(anyhow!("get child rx failed"))?;
thread::spawn(move || {
    let core_span = span!(Level::INFO, "core").entered();
    while let Ok(msg) = child_rx.recv() {
        info!("{msg}");
    }
    core_span.exit();
});
```
