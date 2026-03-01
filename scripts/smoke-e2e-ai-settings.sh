#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_PATH="$ROOT_DIR/src-tauri/target/debug/bundle/macos/Texere.app"
APP_DATA_DIR="$HOME/Library/Application Support/com.mendlex.texere"
SETTINGS_PATH="$APP_DATA_DIR/settings.json"
BACKUP_PATH="$APP_DATA_DIR/settings.json.ai-smoke-backup"

if [ ! -d "$APP_PATH" ]; then
  echo "Missing app bundle: $APP_PATH"
  exit 1
fi

mkdir -p "$APP_DATA_DIR"

restore_settings() {
  if [ -f "$BACKUP_PATH" ]; then
    mv "$BACKUP_PATH" "$SETTINGS_PATH"
  else
    rm -f "$SETTINGS_PATH"
  fi
}

cleanup_app() {
  pkill -f "/MacOS/app" 2>/dev/null || true
}

trap 'restore_settings; cleanup_app' EXIT

if [ -f "$SETTINGS_PATH" ]; then
  cp "$SETTINGS_PATH" "$BACKUP_PATH"
fi

cat > "$SETTINGS_PATH" <<'JSON'
{
  "hotkeys": {
    "summon": "CommandOrControl+Shift+Space",
    "copyAndDismiss": "CommandOrControl+Enter"
  },
  "vim": {
    "enabled": true
  },
  "ai": {
    "apiKey": "",
    "model": "gpt-4.1-mini",
    "baseUrl": "https://api.openai-proxy.test/v1",
    "systemPrompt": "   "
  },
  "appearance": {
    "bgColor": "#1e1e2e",
    "opacity": 0.95,
    "mode": "auto",
    "style": "tokyo-night"
  },
  "autoPaste": false
}
JSON

cleanup_app
sleep 1

open "$APP_PATH"
sleep 4

osascript <<'APPLESCRIPT' >/tmp/texere-ai-smoke-settings-open.txt
tell application "System Events"
    set targetProc to missing value
    if exists process "Texere" then
        set targetProc to process "Texere"
    else if exists process "app" then
        set targetProc to process "app"
    end if

    if targetProc is missing value then
        return
    end if

    tell targetProc
        set mb2 to menu bar 2
        set tItems to every menu bar item of mb2
        repeat with t in tItems
            try
                click t
                delay 0.4
                set mNames to name of every menu item of menu 1 of t
                repeat with mn in mNames
                    if (mn as text) contains "Settings" then
                        click menu item (mn as text) of menu 1 of t
                        return
                    end if
                end repeat
                key code 53
            end try
        end repeat
    end tell
end tell
APPLESCRIPT

sleep 2

WINDOWS_AFTER_SETTINGS=$(osascript <<'APPLESCRIPT'
tell application "System Events"
    if exists process "Texere" then
        tell process "Texere"
            return (name of every window) as text
        end tell
    else if exists process "app" then
        tell process "app"
            return (name of every window) as text
        end tell
    end if
    return "NO_PROCESS"
end tell
APPLESCRIPT
)

if [[ "$WINDOWS_AFTER_SETTINGS" != *"Settings"* ]]; then
  echo "AI settings smoke failed: settings window not found"
  echo "Window names: $WINDOWS_AFTER_SETTINGS"
  exit 1
fi

AI_MODEL_EXPECTED="gpt-4.1-mini"
AI_BASE_URL_EXPECTED="https://api.openai-proxy.test/v1"
AI_PROMPT_PHRASE="You are a text polishing assistant"

python3 <<PY
import json
import sys
import time
from pathlib import Path

settings_path = Path("$SETTINGS_PATH")
if not settings_path.exists():
    sys.exit("AI settings smoke failed: settings file missing")

last_model = None
last_base_url = None
last_system_prompt = ""

for _ in range(16):
    with settings_path.open() as f:
        data = json.load(f)

    ai = data.get("ai", {})
    last_model = ai.get("model")
    last_base_url = ai.get("baseUrl")
    last_system_prompt = ai.get("systemPrompt", "")

    if (
        last_model == "$AI_MODEL_EXPECTED"
        and last_base_url == "$AI_BASE_URL_EXPECTED"
        and last_system_prompt.strip()
        and "$AI_PROMPT_PHRASE" in last_system_prompt
    ):
        print("AI settings E2E: PASSED")
        break

    time.sleep(0.5)
else:
    if last_model != "$AI_MODEL_EXPECTED":
        sys.exit(f"AI settings smoke failed: ai.model expected '$AI_MODEL_EXPECTED', got '{last_model}'")
    if last_base_url != "$AI_BASE_URL_EXPECTED":
        sys.exit(f"AI settings smoke failed: ai.baseUrl expected '$AI_BASE_URL_EXPECTED', got '{last_base_url}'")
    if not last_system_prompt.strip():
        sys.exit("AI settings smoke failed: ai.systemPrompt empty after fallback")
    sys.exit("AI settings smoke failed: ai.systemPrompt did not fallback to default prompt")
PY

echo "AI settings smoke passed"
echo "Windows after settings: $WINDOWS_AFTER_SETTINGS"
