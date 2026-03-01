#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_PATH="$ROOT_DIR/src-tauri/target/debug/bundle/macos/Texere.app"
APP_DATA_DIR="$HOME/Library/Application Support/com.mendlex.texere"
SETTINGS_PATH="$APP_DATA_DIR/settings.json"
BACKUP_PATH="$APP_DATA_DIR/settings.json.smoke-backup"

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

trap restore_settings EXIT

if [ -f "$SETTINGS_PATH" ]; then
  cp "$SETTINGS_PATH" "$BACKUP_PATH"
fi

cat > "$SETTINGS_PATH" <<'JSON'
{
  "hotkeys": {
    "summon": "CommandOrControl+Shift+Enter",
    "copyAndDismiss": "CommandOrControl+Enter"
  },
  "vim": {
    "enabled": false
  },
  "ai": {
    "apiKey": "",
    "model": "gpt-4.1-mini",
    "baseUrl": "https://api.openai-proxy.test/v1",
    "systemPrompt": "   "
  },
  "appearance": {
    "bgColor": "#ff0000",
    "opacity": 0.85,
    "theme": "dark"
  },
  "autoPaste": true
}
JSON

pkill -f "/MacOS/app" 2>/dev/null || true
sleep 1

open "$APP_PATH"
sleep 5

if ! osascript <<'APPLESCRIPT' >/tmp/texere-smoke-permission.txt 2>/tmp/texere-smoke-permission.err
tell application "System Events"
    key code 36 using {shift down, command down}
end tell
APPLESCRIPT
then
  echo "Smoke failed: cannot send hotkey via System Events"
  cat /tmp/texere-smoke-permission.err
  echo "Grant Accessibility permission to Terminal in macOS Settings > Privacy & Security > Accessibility"
  exit 2
fi

WINDOWS_NEW_HOTKEY="NO_PROCESS"
for _ in 1 2 3 4 5; do
  osascript <<'APPLESCRIPT' >/tmp/texere-smoke-summon.txt
tell application "System Events"
    key code 36 using {shift down, command down}
end tell
APPLESCRIPT

  sleep 1

  WINDOWS_NEW_HOTKEY=$(osascript <<'APPLESCRIPT'
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

  if [ "$WINDOWS_NEW_HOTKEY" != "NO_PROCESS" ] && [ -n "$WINDOWS_NEW_HOTKEY" ]; then
    break
  fi

  sleep 1
done

if [ "$WINDOWS_NEW_HOTKEY" = "NO_PROCESS" ] || [ -z "$WINDOWS_NEW_HOTKEY" ]; then
  echo "Smoke failed: new summon hotkey did not open a window"
  exit 1
fi

osascript <<'APPLESCRIPT' >/tmp/texere-smoke-old-hotkey.txt
tell application "System Events"
    key code 49 using {shift down, command down}
end tell
APPLESCRIPT

sleep 2

WINDOWS_AFTER_OLD_HOTKEY=$(osascript <<'APPLESCRIPT'
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

if [ "$WINDOWS_AFTER_OLD_HOTKEY" != "$WINDOWS_NEW_HOTKEY" ]; then
  echo "Smoke failed: old default hotkey still summons window"
  echo "Before old hotkey: $WINDOWS_NEW_HOTKEY"
  echo "After old hotkey:  $WINDOWS_AFTER_OLD_HOTKEY"
  exit 1
fi

osascript <<'APPLESCRIPT' >/tmp/texere-smoke-settings-open.txt
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
  echo "Smoke failed: settings window not found"
  echo "Window names: $WINDOWS_AFTER_SETTINGS"
  exit 1
fi

AI_MODEL_EXPECTED="gpt-4.1-mini"
AI_BASE_URL_EXPECTED="https://api.openai-proxy.test/v1"
AI_PROMPT_PHRASE="You are a text polishing assistant"

python3 <<PY
import json
import sys
from pathlib import Path

settings_path = Path("$SETTINGS_PATH")
if not settings_path.exists():
    sys.exit("Smoke failed: settings file missing for AI assertions")

with settings_path.open() as f:
    data = json.load(f)
ai = data.get("ai", {})

if ai.get("model") != "$AI_MODEL_EXPECTED":
    sys.exit("Smoke failed: ai.model expected '{AI_MODEL_EXPECTED}', got '{ai.get('model')}'")

if ai.get("baseUrl") != "$AI_BASE_URL_EXPECTED":
    sys.exit("Smoke failed: ai.baseUrl expected '{AI_BASE_URL_EXPECTED}', got '{ai.get('baseUrl')}'")

system_prompt = ai.get("systemPrompt", "")
if not system_prompt.strip():
    sys.exit("Smoke failed: ai.systemPrompt empty after fallback")

if "$AI_PROMPT_PHRASE" not in system_prompt:
    sys.exit("Smoke failed: ai.systemPrompt missing fallback phrase '{AI_PROMPT_PHRASE}'")

print("AI settings persistence verified")
PY

# ──────────────────────────────────────────────────────
# Auto-paste E2E test
# ──────────────────────────────────────────────────────

# Close the settings window first (press Cmd+W)
osascript <<'APPLESCRIPT'
tell application "System Events"
    keystroke "w" using command down
end tell
APPLESCRIPT
sleep 1

# Open TextEdit as the target paste destination
open -a TextEdit
sleep 2

# Create a new empty document in TextEdit
osascript <<'APPLESCRIPT'
tell application "TextEdit"
    activate
    make new document
end tell
APPLESCRIPT
sleep 1

# Summon Texere with the configured hotkey (Cmd+Shift+Enter)
osascript <<'APPLESCRIPT'
tell application "System Events"
    key code 36 using {shift down, command down}
end tell
APPLESCRIPT
sleep 2

# Type test content into the Texere editor
PASTE_TEST_CONTENT="smoke-auto-paste-test-$(date +%s)"
osascript -e "tell application \"System Events\" to keystroke \"${PASTE_TEST_CONTENT}\""
sleep 1

# Trigger copy+dismiss with Cmd+Enter (should auto-paste to TextEdit)
osascript <<'APPLESCRIPT'
tell application "System Events"
    key code 36 using command down
end tell
APPLESCRIPT

# Wait for auto-paste sequence: 300ms thread delay + 300ms AppleScript delay + margin
sleep 2

FRONTMOST_PROCESS=""
for _ in 1 2 3 4 5; do
  FRONTMOST_PROCESS=$(osascript <<'APPLESCRIPT'
tell application "System Events"
    return name of first process whose frontmost is true
end tell
APPLESCRIPT
)

  if [ "$FRONTMOST_PROCESS" = "TextEdit" ]; then
    break
  fi

  sleep 0.2
done

if [ "$FRONTMOST_PROCESS" != "TextEdit" ]; then
  echo "Auto-paste E2E: FAILED (focus stolen)"
  echo "Expected frontmost process: TextEdit"
  echo "Got: $FRONTMOST_PROCESS"
  exit 1
fi

# Read the content from the frontmost TextEdit document
TEXTEDIT_CONTENT=$(osascript <<'APPLESCRIPT'
tell application "TextEdit"
    set docText to text of document 1
    return docText
end tell
APPLESCRIPT
)

# Close TextEdit without saving
osascript <<'APPLESCRIPT'
tell application "TextEdit"
    close every document saving no
    quit
end tell
APPLESCRIPT

# Verify the pasted content matches
if [[ "$TEXTEDIT_CONTENT" == *"$PASTE_TEST_CONTENT"* ]]; then
  echo "Auto-paste E2E: PASSED"
else
  echo "Auto-paste E2E: FAILED"
  echo "Expected content containing: $PASTE_TEST_CONTENT"
  echo "Got: $TEXTEDIT_CONTENT"
  exit 1
fi

echo "Smoke passed"
echo "Windows after new hotkey: $WINDOWS_NEW_HOTKEY"
echo "Windows after old hotkey check: $WINDOWS_AFTER_OLD_HOTKEY"
echo "Windows after settings: $WINDOWS_AFTER_SETTINGS"
echo "Auto-paste content verified: $PASTE_TEST_CONTENT"
