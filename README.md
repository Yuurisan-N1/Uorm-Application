<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Uorm%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Mission%20%7C%20Mining%20%7C%20Daily%20Bonus%20%7C%20Referral%20%7C%20Multi-Account&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Sign-up+%26+Email+Verify+%7C+Session+Cached;Auto+Daily+Bonus+%7C+Claim+Each+Cycle;Auto+Missions+%7C+Social+%2F+Board+%2F+Bundle;Auto+Mining+%7C+Boost+%7C+Claim+Session;Proxy+Support+%7C+Multi-Account"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Uorm%20App-111111"/>
  <img alt="multi-account" src="https://img.shields.io/badge/Multi--Account-Supported-111111"/>
  <img alt="build" src="https://github.com/Yuurisan-N1/Uorm-Application/actions/workflows/build.yml/badge.svg"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Uorm Bot</b> is a mission, mining and referral automation client for the Uorm app.<br/>
  It handles email sign-up with OTP verification (done once per account), caches the session token, and on every cycle claims the daily bonus, completes social missions, works through the mission board and bundle missions, claims the mining session reward, applies a boost, and then sleeps until the next cycle, all across multiple accounts with proxy support.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Install Rust](#install-rust)
- [Installation](#installation)
- [Configuration](#configuration)
- [First-Time Setup](#first-time-setup)
- [Running the Bot](#running-the-bot)
- [Run Modes](#run-modes)
- [Download Prebuilt Binary](#download-prebuilt-binary)
- [Features](#features)
- [Troubleshooting](#troubleshooting)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Rust `1.70+` (includes `cargo`, only needed if building from source)
- Git

---

## Install Rust

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:

```bash
rustc --version
```

### Windows

Download and run the installer from https://rustup.rs, then reopen the terminal and verify:

```cmd
rustc --version
```

### Termux (Android)

```bash
pkg update && pkg install proot-distro
proot-distro install ubuntu
proot-distro login ubuntu
```

Then inside Ubuntu:

```bash
apt update && apt install -y curl git build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## Installation

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Uorm-Application.git
cd Uorm-Application
```

**Build the binary:**

```bash
cargo build --release
# or
./run.sh build
```

---

## Configuration

### 1. Accounts (data.txt)

Fill `data.txt` with one account per line in `email|password` format. Blank lines and lines starting with `#` are ignored. Keep the file private:

```
you@example.com|yourpassword
another@example.com|anotherpassword
```

```bash
chmod 600 data.txt
```

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line, in the same order as `data.txt` (optional, leave empty to run without proxy):

```
http://user:pass@host:port
socks5://user:pass@host:port
```

### 3. Bot Settings (config.json)

`sleep_seconds` controls how many seconds the bot waits between cycles. `referral_code` is applied automatically when registering new accounts. Mission toggles, daily bonus toggle, and mining toggle are also set here.

---

## First-Time Setup

Each account must confirm its email once before the bot can sign in. This only needs to be done once per account. After confirmation, the session is cached in `state/` and no code is needed again.

**Step 1 - Request the confirmation code:**

```bash
./run.sh signup you@example.com
```

The server sends a 6-digit code to the inbox.

**Step 2 - Submit the code:**

```bash
./run.sh verify you@example.com 123456
```

Replace `123456` with the actual code from the email. If the code expired, run the signup step again to get a fresh one.

After verification, check the profile looks correct before starting the loop:

```bash
./run.sh probe
```

---

## Running the Bot

Make `run.sh` executable first:

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh run        # build if needed and run in foreground
./run.sh bg         # build if needed and run in background, logs saved to uorm-bot.log
./run.sh screen     # build if needed and run in a detached screen session
./run.sh stop       # stop the background or screen session
./run.sh probe      # read profile, missions, and mining without claiming anything
./run.sh selftest   # offline checks, no network
./run.sh build      # build only, do not run
```

For background mode, tail the log anytime:

```bash
tail -f uorm-bot.log
```

For screen mode, attach anytime:

```bash
screen -r uorm-bot
```

Press `Ctrl+C` once to stop the bot cleanly when running in foreground.

### Using make (Linux / macOS)

```bash
make release    # optimized release build
make start      # build release and run immediately
make run        # build debug and run
make check      # check for errors without building
make fmt        # format the code
make clean      # remove all build artifacts
make size       # show release binary size
```

### Manual run

```bash
# Linux / macOS / Termux
./target/release/uorm-bot

# Windows
.\target\release\uorm-bot.exe
```

---

## Run Modes

| Mode | Command | Description |
|---|---|---|
| run | `./run.sh run` | Full cycle: daily bonus, missions, mining, sleep |
| bg | `./run.sh bg` | Same as run but in background with log file |
| screen | `./run.sh screen` | Same as run in a detached screen session |
| probe | `./run.sh probe` | Read profile, missions, and mining session without claiming |
| bgprobe | `./run.sh bgprobe` | Probe in background |
| signup | `./run.sh signup <email>` | Request email confirmation code |
| verify | `./run.sh verify <email> <code>` | Submit the confirmation code |
| selftest | `./run.sh selftest` | Offline checks, no network |
| stop | `./run.sh stop` | Stop background or screen session |
| build | `./run.sh build` | Build only |

---

## Download Prebuilt Binary

Prebuilt binaries are automatically compiled on every push to main via GitHub Actions across 3 platforms.

Download the latest binaries from the Actions page:
https://github.com/Yuurisan-N1/Uorm-Application/actions/workflows/build.yml

Open the latest successful run and scroll to the Artifacts section.

| Artifact | Platform |
|---|---|
| `uorm-bot-linux-x86_64` | Linux x86_64 |
| `uorm-bot-windows-x86_64` | Windows x86_64 |
| `uorm-bot-macos-aarch64` | macOS Apple Silicon |

**Linux / macOS after downloading:**

```bash
chmod +x uorm-bot-linux-x86_64
./uorm-bot-linux-x86_64
```

**Windows:**

```bash
.\uorm-bot-windows-x86_64.exe
```

Place the binary in the same folder as `data.txt`, `proxy.txt`, and `config.json` before running.

---

## Features

### Email Sign-up and OTP Verification
New accounts are registered and verified via a 6-digit email OTP. After the first successful verification the session token is cached in `state/` (mode 0600, gitignored). Tokens are never printed to the log. Delete the file for a specific account to force a fresh sign-in.

### Daily Bonus
The bot claims the daily bonus reward for each account at the start of every cycle. If already claimed, it is skipped with a note.

### Auto Missions
Three mission types are processed in order each cycle. Social missions verify social actions. Mission board missions are fetched, opened, and claimed individually. Bundle missions are claimed as a group. Each claimed mission logs its reward.

### Auto Mining
The bot reads the mining session config from the server (session length, boost multiplier, daily boost quota). If a session is complete, it claims the reward and starts a new session. If still running, the remaining time is logged. After the session, the boost is applied up to the server-configured daily quota.

### Session Cache
After each sign-in the session token is saved to `state/`. On the next run the cached token is reused and the sign-in step is skipped. If the token expires, a fresh sign-in happens automatically.

### Multi Account
All accounts in `data.txt` are processed sequentially within every cycle. Each account uses its own proxy from the pool and its own session cache file.

### Proxy Support
Proxies are loaded from `proxy.txt` and paired to accounts by index. HTTP and SOCKS5 are both supported. Proxy credentials are masked in log output. Running without proxies is fully supported.

### Auto Countdown
After all accounts complete a cycle, the bot displays a live `HH:MM:SS` countdown until the next cycle starts.

---

## Troubleshooting

| Message | Cause | Fix |
|---|---|---|
| `a wrong email and password pair` | Credentials in `data.txt` do not match the account | Check the email and password |
| `the email is not confirmed yet` | Account was never verified | Run `./run.sh verify <email> <code>` once |
| `the confirmation code expired` | The OTP timed out | Run `./run.sh signup <email>` for a fresh code |
| `the mining session has not finished yet` | Nothing to claim yet | Wait for the session to complete |
| `the server is rate limiting the account` | Too many requests | Slow down the cycle or rotate the proxy |

---

## File Structure

```text
Uorm-Application/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 3-platform matrix build
├── src/
│   ├── main.rs                          # Entry point, Ctrl+C handler, mode dispatch
│   ├── app/
│   │   ├── bootstrap.rs                 # Config load, account and proxy setup
│   │   ├── dispatch.rs                  # Mode selector
│   │   └── mod.rs
│   ├── domain/
│   │   ├── account.rs                   # Account model: email, password, device hash
│   │   ├── mission.rs                   # Mission model
│   │   ├── profile.rs                   # Profile model: name, balance
│   │   ├── session.rs                   # Session model and cache
│   │   └── mod.rs
│   ├── modes/
│   │   ├── run.rs                       # Full cycle: multi-account loop and countdown
│   │   ├── probe.rs                     # Probe: read without claiming
│   │   ├── selftest.rs                  # Offline checks
│   │   ├── signup.rs                    # Email registration
│   │   ├── verify.rs                    # OTP submission
│   │   └── mod.rs
│   ├── net/
│   │   ├── client.rs                    # HTTP client with cookie jar
│   │   ├── endpoint.rs                  # API endpoint definitions
│   │   ├── proxy.rs                     # Proxy pool loader and masker
│   │   └── mod.rs
│   ├── ops/
│   │   ├── runner.rs                    # Per-account runner
│   │   ├── auth/
│   │   │   ├── login.rs                 # Email/password sign-in
│   │   │   ├── otp.rs                   # OTP request and submit
│   │   │   ├── renew.rs                 # Token renewal
│   │   │   ├── session.rs               # Session cache read and write
│   │   │   └── mod.rs
│   │   ├── bonus/
│   │   │   ├── daily.rs                 # Daily bonus claim
│   │   │   └── mod.rs
│   │   ├── mine/
│   │   │   ├── config.rs                # Mining config fetch
│   │   │   ├── session.rs               # Mining session claim, start, boost
│   │   │   └── mod.rs
│   │   ├── mission/
│   │   │   ├── list.rs                  # Mission list fetch
│   │   │   ├── open.rs                  # Mission open
│   │   │   ├── claim.rs                 # Mission claim
│   │   │   ├── bundle.rs                # Bundle mission claim
│   │   │   ├── social.rs                # Social mission verify
│   │   │   └── mod.rs
│   │   ├── profile/
│   │   │   ├── fetch.rs                 # Profile and balance fetch
│   │   │   └── mod.rs
│   │   ├── referral/
│   │   │   ├── apply.rs                 # Referral code apply
│   │   │   └── mod.rs
│   │   └── room/
│   │       ├── chat.rs                  # Chat room interaction
│   │       └── mod.rs
│   └── support/
│       ├── banner.rs                    # ASCII banner
│       ├── cfg/                         # Config loader and schema
│       ├── consts/                      # App and network constants
│       ├── error.rs                     # Error types
│       ├── log/                         # Log init, colored format, countdown helper
│       └── mod.rs
├── assets/
│   ├── icon.ico                         # Windows binary icon
│   └── icon.png                         # App icon
├── state/                               # Session token cache per account (auto-generated, gitignored)
├── Cargo.toml                           # Project manifest and dependencies
├── Cargo.lock
├── build.rs                             # Build script (embeds icon into Windows binary)
├── Makefile                             # make targets: build, run, release, start, clean, size
├── run.sh                               # Run helper: run, bg, screen, probe, signup, verify, stop, build
├── config.json                          # Sleep duration, referral code, feature toggles
├── data.txt                             # Account credentials, email|password one per line
├── proxy.txt                            # Proxy list (optional)
├── HOWTO.txt                            # Quick start guide
└── LICENSE                              # GNU AGPL v3.0
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>