#!/usr/bin/env bash
set -u

BINARY="./target/release/uorm-bot"
LOG_FILE="uorm-bot.log"
SESSION_NAME="uorm-bot"

build_release() {
    echo "Building release binary..."
    if ! cargo build --release; then
        echo "Build failed, aborting."
        exit 1
    fi
}

prepare() {
    if [ ! -f "data.txt" ]; then
        echo "data.txt not found, create it first (email|password per line)."
        exit 1
    fi
    if [ ! -x "$BINARY" ] || [ "Cargo.toml" -nt "$BINARY" ]; then
        build_release
    fi
}

mode_direct() {
    prepare
    exec "$BINARY" "${1:-run}"
}

mode_nohup() {
    prepare
    nohup "$BINARY" "${1:-run}" >> "$LOG_FILE" 2>&1 &
    echo "Started in background (PID $!)"
    echo "Logs: tail -f $LOG_FILE"
}

mode_screen() {
    if ! command -v screen >/dev/null 2>&1; then
        echo "screen not found, install with: apt install screen"
        exit 1
    fi
    prepare
    screen -dmS "$SESSION_NAME" "$BINARY" "${1:-run}"
    echo "Started in screen session '$SESSION_NAME'"
    echo "Attach : screen -r $SESSION_NAME"
    echo "Detach : Ctrl+A then D"
}

mode_stop() {
    if command -v screen >/dev/null 2>&1 && screen -ls | grep -q "$SESSION_NAME"; then
        screen -S "$SESSION_NAME" -X quit
        echo "Screen session stopped."
    fi
    if pkill -f "$BINARY" >/dev/null 2>&1; then
        echo "Process stopped."
    else
        echo "No running process found."
    fi
}

case "${1:-run}" in
    run|probe|selftest)
        mode_direct "$1"
        ;;
    bg)
        mode_nohup "run"
        ;;
    bgprobe)
        mode_nohup "probe"
        ;;
    verify)
        prepare
        shift
        exec "$BINARY" verify "$@"
        ;;
    signup)
        prepare
        shift
        exec "$BINARY" signup "$@"
        ;;
    screen)
        mode_screen "run"
        ;;
    stop)
        mode_stop
        ;;
    build)
        build_release
        ;;
    *)
        echo "Usage: ./run.sh [run|probe|selftest|signup <email>|verify <email> <code>|bg|bgprobe|screen|stop|build]"
        exit 1
        ;;
esac
