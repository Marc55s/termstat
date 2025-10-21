export TERMSTATS_SESSION_ID="$(uuidgen 2>/dev/null || cat /proc/sys/kernel/random/uuid)"
export TERMSTATS_LOGFILE="$HOME/.local/share/termstat/termstat.log"

# --- Create the log directory if it doesn't exist ---
mkdir -p "$(dirname "$TERMSTATS_LOGFILE")"

termstats_preexec() {
    echo "DEBUG: preexec is RUNNING for '$1'" >&2  # Add this line
    export TERMSTATS_LAST_CMD="$1"
    export TERMSTATS_CMD_START_MS=$(date +%s%3N)
}

termstats_precmd() {
    echo "DEBUG: precmd is RUNNING, writing log." >&2 # Add this line
    local end_ms=$(date +%s%3N)
    # ... rest of the function ...
}

# This function runs *after* the command (before the prompt)
termstats_precmd() {
    local end_ms=$(date +%s%3N)
    local start_ms="${TERMSTATS_CMD_START_MS:-$end_ms}"
    local duration=$((end_ms - start_ms))
    
    local cmd="$TERMSTATS_LAST_CMD"
    local exit_status=$? # Get exit status
    
    local sanitized_cmd="$cmd"
    local blacklisted_keywords=("pass" "secret" "token" "key" "aws" "gpg" "pgp")
    for kw in "${blacklisted_keywords[@]}"; do
        if [[ "$cmd" =~ $kw ]]; then
            sanitized_cmd="${cmd%% *}"
            break
        fi
    done

    printf '{"ts":%s,"user":"%s","session":"%s","shell":"zsh","cmd":"%s","cwd":"%s","exit":%d,"dur":%d}\n' \
        "$(date +%s%3N)" \
        "$USER" \
        "$TERMSTATS_SESSION_ID" \
        "$sanitized_cmd" \
        "$PWD" \
        "$exit_status" \
        "$duration" >> "$TERMSTATS_LOGFILE"
}

# --- Load and register the hook functions ---
autoload -U add-zsh-hook
add-zsh-hook preexec termstats_preexec
add-zsh-hook precmd termstats_precmd
