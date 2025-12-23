#!/usr/bin/env bash
# Check that tech debt (sw-checklist FAIL/WARN counts) doesn't increase.
# This script enforces the ratchet: counts must go down, never up.
#
# RULES:
# 1. Total FAIL/WARN counts cannot increase
# 2. Per-component counts cannot increase
# 3. New components MUST start at 0 FAIL, 0 WARN (no exceptions)

set -e

# Baseline counts - update these ONLY when counts decrease
# Format: component:fail_max:warn_max
# New components MUST be added with 0:0
BASELINES="
frontend:6:18
cli:3:7
rest:0:0
crud:0:2
agent:0:2
server:0:1
shared:0:0
nodes:2:0
project:1:0
ffmpeg:1:4
"

# TOTAL baseline - this is the key metric that cannot increase
# This prevents shuffling debt to new components
TOTAL_FAIL_MAX=13
TOTAL_WARN_MAX=35

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Run sw-checklist with verbose output, strip ANSI codes
OUTPUT=$(cd "$ROOT_DIR" && sw-checklist -v 2>&1 | sed 's/\x1b\[[0-9;]*m//g')

# Count totals using tr to handle multi-line safely
TOTAL_FAIL=$(echo "$OUTPUT" | grep -c "\[FAIL\]" 2>/dev/null || echo "0")
TOTAL_WARN=$(echo "$OUTPUT" | grep -c "\[WARN\]" 2>/dev/null || echo "0")

echo "Tech Debt Ratchet Check"
echo "======================="
echo ""
echo "TOTAL COUNTS (cannot increase):"
printf "  Failures: %d (max %d)\n" "$TOTAL_FAIL" "$TOTAL_FAIL_MAX"
printf "  Warnings: %d (max %d)\n" "$TOTAL_WARN" "$TOTAL_WARN_MAX"

FAILED=0

# Check total counts first - this is the primary gate
if [[ $TOTAL_FAIL -gt $TOTAL_FAIL_MAX ]]; then
    echo ""
    echo "ERROR: Total FAIL count increased from $TOTAL_FAIL_MAX to $TOTAL_FAIL!"
    FAILED=1
fi
if [[ $TOTAL_WARN -gt $TOTAL_WARN_MAX ]]; then
    echo ""
    echo "ERROR: Total WARN count increased from $TOTAL_WARN_MAX to $TOTAL_WARN!"
    FAILED=1
fi

echo ""
printf "%-12s %8s %8s %8s %8s\n" "Component" "Fail" "Max" "Warn" "Max"
echo "-------------------------------------------"

for baseline in $BASELINES; do
    if [[ -z "$baseline" ]]; then continue; fi

    component=$(echo "$baseline" | cut -d: -f1)
    fail_max=$(echo "$baseline" | cut -d: -f2)
    warn_max=$(echo "$baseline" | cut -d: -f3)

    # Count current fails and warns for this component (handle 0 case)
    fail_cur=$(echo "$OUTPUT" | grep -c "\[FAIL\].*\[yt-rs-$component\]" 2>/dev/null || true)
    fail_cur=${fail_cur:-0}
    warn_cur=$(echo "$OUTPUT" | grep -c "\[WARN\].*\[yt-rs-$component\]" 2>/dev/null || true)
    warn_cur=${warn_cur:-0}

    status=""
    if [[ $fail_cur -gt $fail_max ]]; then
        status=" FAIL!"
        FAILED=1
    elif [[ $warn_cur -gt $warn_max ]]; then
        status=" WARN!"
        FAILED=1
    elif [[ $fail_cur -lt $fail_max ]] || [[ $warn_cur -lt $warn_max ]]; then
        status=" (improved!)"
    fi

    printf "%-12s %8d %8d %8d %8d%s\n" "$component" "$fail_cur" "$fail_max" "$warn_cur" "$warn_max" "$status"
done

echo "-------------------------------------------"
printf "%-12s %8d %8d %8d %8d\n" "TOTAL" "$TOTAL_FAIL" "$TOTAL_FAIL_MAX" "$TOTAL_WARN" "$TOTAL_WARN_MAX"

echo ""
if [[ $FAILED -eq 1 ]]; then
    echo "ERROR: Tech debt increased! Refactor to reduce counts."
    echo ""
    echo "To fix:"
    echo "1. Refactor code to reduce FAIL/WARN counts"
    echo "2. Only after reducing, update baselines in this script"
    echo "3. New components MUST start with 0:0 baseline"
    exit 1
else
    if [[ $TOTAL_FAIL -lt $TOTAL_FAIL_MAX ]] || [[ $TOTAL_WARN -lt $TOTAL_WARN_MAX ]]; then
        echo "IMPROVED! Update the baseline values in this script."
        echo "New TOTAL_FAIL_MAX=$TOTAL_FAIL"
        echo "New TOTAL_WARN_MAX=$TOTAL_WARN"
    fi
    echo "OK: Tech debt is within baseline or improved."
    exit 0
fi
