#!/bin/bash
# Check for new crash logs and analyze them

REPO_DIR="/home/node/.openclaw/workspace/dwarf-the-world"
cd "$REPO_DIR"

# Save current crash log count
CRASH_COUNT_BEFORE=$(ls -t crash-logs/*.log 2>/dev/null | head -1 | xargs basename 2>/dev/null)

# Pull latest changes
git fetch origin main 2>/dev/null
LOCAL=$(git rev-parse @)
REMOTE=$(git rev-parse origin/main)

if [ "$LOCAL" != "$REMOTE" ]; then
    echo "New commits available, pulling..."
    git stash 2>/dev/null
    git pull --force origin main 2>/dev/null
    git stash pop 2>/dev/null
fi

# Check for new crash logs
LATEST_CRASH=$(ls -t crash-logs/*.log 2>/dev/null | head -1)
if [ -n "$LATEST_CRASH" ] && [ "$LATEST_CRASH" != "$CRASH_COUNT_BEFORE" ]; then
    echo "=== NEW CRASH DETECTED ==="
    echo "Crash file: $LATEST_CRASH"
    echo "=== CONTENT ==="
    cat "$LATEST_CRASH"
    echo "=== END CRASH ==="
    
    # Create a marker file to signal we have a new crash
    echo "$LATEST_CRASH" > /tmp/latest_crash_marker
    
    # Signal OpenClaw to check
    touch /tmp/crash_detected
fi
