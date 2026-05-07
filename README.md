# cachyos-tune

System tuning CLI for CachyOS on AMD APU with five hardware-aware profiles.

## Features

- Five tuning profiles: `default`, `ml-inference`, `gaming`, `battery`, `compile`
- Tunes CPU governor, swappiness, dirty ratios, THP, ZRAM, GPU power, I/O scheduler, TCP congestion
- Diff view shows exactly what a profile would change before applying
- Status view compares current system state against all profiles at once
- Snapshot and restore — save current settings, roll back later
- Built-in quick benchmarks (CPU, memory, sequential read)

## Install

```
cargo build --release
```

Binary lands at `target/release/cachyos-tune`.

## Usage

Show current system tuning vs all profiles:

```
cachyos-tune status
```

Preview what a profile would change:

```
cachyos-tune diff ml-inference
```

Apply a profile (requires sudo):

```
sudo cachyos-tune apply gaming
```

Save current settings for later rollback:

```
cachyos-tune save
```

Restore from snapshot (requires sudo):

```
sudo cachyos-tune restore
```

Run quick benchmarks:

```
cachyos-tune bench
```

## Profiles

| Profile        | Governor     | Swappiness | THP     | GPU Power | I/O Sched |
|----------------|-------------|------------|---------|-----------|-----------|
| `default`      | schedutil   | 60         | madvise | auto      | none      |
| `ml-inference` | performance | (tuned)    | always  | (tuned)   | (tuned)   |
| `gaming`       | performance | (tuned)    | (tuned) | (tuned)   | (tuned)   |
| `battery`      | powersave   | (tuned)    | (tuned) | low       | (tuned)   |
| `compile`      | performance | (tuned)    | (tuned) | (tuned)   | (tuned)   |

---

Built with Rust + clap + comfy-table.
