# ===== termstats file logging  =====
export TERMSTATS_SESSION_ID="$(uuidgen 2>/dev/null || cat /proc/sys/kernel/random/uuid)"
export TERMSTATS_LOGFILE="$HOME/.local/share/termstat/termstat.log"

termstats_debug() {
    export TERMSTATS_LAST_CMD="$BASH_COMMAND"
    export TERMSTATS_CMD_START_MS=$(date +%s%3N)
}
trap 'termstats_debug' DEBUG

termstats_prompt_command() {
    local end_ms=$(date +%s%3N)
    local start_ms="${TERMSTATS_CMD_START_MS:-$end_ms}"
    local duration=$((end_ms - start_ms))

    local hist="$(history 1)"
    local cmd="${hist#*  }"
    local status=$?

    local sanitized_cmd="$cmd"
    local blacklisted_keywords=("pass" "secret" "token" "key" "aws" "gpg" "pgp")
    for kw in "${blacklisted_keywords[@]}"; do
        if [[ "$cmd" =~ $kw ]]; then
            sanitized_cmd="${cmd%% *}"
            break
        fi
    done

    printf '{"ts":%s,"user":"%s","session":"%s","shell":"bash","cmd":"%s","cwd":"%s","exit":%d,"dur":%d}\n' \
        "$(date +%s%3N)" \
        "$USER" \
        "$TERMSTATS_SESSION_ID" \
        "$sanitized_cmd" \
        "$PWD" \
        "$status" \
        "$duration" >> "$TERMSTATS_LOGFILE"
}
PROMPT_COMMAND="termstats_prompt_command; $PROMPT_COMMAND"
# ===== end termstats file logging =====
