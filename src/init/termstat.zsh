export TERMSTATS_SESSION_ID="$(uuidgen 2>/dev/null || cat /proc/sys/kernel/random/uuid)"
export TERMSTATS_LOGFILE="$HOME/.local/share/termstat/termstat.log"

mkdir -p "$(dirname "$TERMSTATS_LOGFILE")"

zmodload -i zsh/datetime # faster built in functions

_termstats_get_ms() {
  REPLY=${EPOCHREALTIME/./}
  REPLY=${REPLY:0:13}
}

termstats_preexec() {
    # echo "DEBUG: preexec is RUNNING for '$1'" >&2
    export TERMSTATS_LAST_CMD="$1"
    _termstats_get_ms
    export TERMSTATS_CMD_START_MS=$REPLY
}

_termstats_log_entry_async() {
    local end_ms=$1
    local start_ms=$2
    local cmd=$3
    local exit_status=$4
    local cwd=$5
    local user=$6
    local session=$7
    local logfile=$8

    local duration=$((end_ms - start_ms))
    
    local sanitized_cmd="$cmd"
    local blacklisted_keywords=("pass" "secret" "token" "key" "aws" "gpg" "pgp" "API" "KEY" "PASS" "TOKEN")
    for kw in "${blacklisted_keywords[@]}"; do
        if [[ "$cmd" =~ $kw ]]; then
            sanitized_cmd="${cmd%% *}" # Get just the command name
            break
        fi
    done

    jq -n -c \
        --argjson ts "$end_ms" \
        --arg     user "$user" \
        --arg     session "$session" \
        --arg     shell "zsh" \
        --arg     cmd "$sanitized_cmd" \
        --arg     cwd "$cwd" \
        --argjson exit "$exit_status" \
        --argjson dur "$duration" \
        '$ARGS.named' >> "$logfile"
}

termstats_precmd() {
    # --- FIX: Don't log if no command was run (e.g., empty prompt) ---
    [[ -z "$TERMSTATS_LAST_CMD" ]] && return

    # echo "DEBUG: precmd is RUNNING, logging async." >&2
    
    # Get end time and status
    _termstats_get_ms
    local end_ms=$REPLY
    local start_ms="${TERMSTATS_CMD_START_MS:-$end_ms}"
    local exit_status=$?
    
    # ASYNC:Run the slow logging in the background
    (_termstats_log_entry_async \
        "$end_ms" \
        "$start_ms" \
        "$TERMSTATS_LAST_CMD" \
        "$exit_status" \
        "$PWD" \
        "$USER" \
        "$TERMSTATS_SESSION_ID" \
        "$TERMSTATS_LOGFILE" \
        & ) 

    export TERMSTATS_LAST_CMD=""
}

autoload -U add-zsh-hook
add-zsh-hook preexec termstats_preexec
add-zsh-hook precmd termstats_precmd
