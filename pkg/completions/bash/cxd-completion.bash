#/usr/bin/env bash

# All operations must have an _op_<OP_CHAR>() function defined
OPERATIONS="--add -a --remove -r --list -l --clear"
GLOBAL_OPTIONS="--file -f --help -h --version"

# Looks through the current line for the first operation (if any)
# Sets OP and OP_LOC if found. OP is the first letter of the operation name
_cxd_operation() {
    OP=""
    OP_LOC=""
    local i=0
    for item in "${COMP_WORDS[@]}"; do
        for op in ${OPERATIONS[@]}; do
            if [ "$item" = "$op" ]; then
                OP=$(echo $op | sed -r 's/^--?([a-z]{1}).*$/\1/');
                OP_LOC=$i
                return
            fi
        done
        i=$(( $i + 1 ))
    done
}

# Counts number of free args in ${COMP_WORDS} after INDEX
# SKIP_OPT options will have their corresponding args skipped in the count
# Usage: _count_free_args INDEX [SKIP_OPT]...
_cxd_count_free_args() {
    local INDEX=$1
    shift
    local SKIP_OPT=${@}
    local skip_next="" free_args=0 first_arg_index index=$((INDEX-1))
    for item in "${COMP_WORDS[@]:$INDEX}"; do
        index=$((index + 1))
        # Was last a skip opt
        if [ -n "$skip_next" ]; then
            skip_next=""
            continue
        fi
        # Check if its a skip opt
        for skip in "${SKIP_OPT[@]}"; do
            if [ "$item" = "$skip" ]; then
                skip_next=1
                continue 2
            fi
        done
        if [ $free_args -eq 0 ]; then
            first_arg_index=$index
        fi
        free_args=$((free_args + 1))
    done
    FREE_ARGS=$free_args
    FIRST_ARG_INDEX=$first_arg_index
}

# Add operation
ADD_OPTIONS="--cwd -c --dir -d --env -e"
_cxd_op_a() {
    if [ "$LAST_WORD" = "--env" ] || [ "$LAST_WORD" = "-e" ]; then
        COMPREPLY=()
    elif [ "$LAST_WORD" = "--dir" ] || [ "$LAST_WORD" = "-d" ]; then
        COMPREPLY=($(_cxd_compgen -d))
    else
        _cxd_count_free_args $((OP_LOC + 1)) --env -e --dir -d
        case $FREE_ARGS in
            0) COMPREPLY=($(_cxd_compgen -W "$ADD_OPTIONS $GLOBAL_OPTIONS")) ;;
            1) COMPREPLY=($(_cxd_compgen -W "$ADD_OPTIONS $GLOBAL_OPTIONS")) ;;
            2) COMPREPLY=($(_cxd_compgen -abc)) ;;
            *) _command_offset $((FIRST_ARG_INDEX + 1)) ;;
        esac
    fi
}

# Remove operation
REMOVE_OPTIONS="--id -i"
_cxd_op_r() {
    if [ "$LAST_WORD" = "--id" ] || [ "$LAST_WORD" = "-i" ]; then
        COMPREPLY=()
    else
        COMPREPLY=($(_cxd_compgen -W "$REMOVE_OPTIONS $GLOBAL_OPTIONS"))
    fi
}

# List operation
LIST_OPTIONS="--short -s"
_cxd_op_l() {
    COMPREPLY=($(_cxd_compgen -W "LIST_OPTIONS $GLOBAL_OPTIONS"))
}

# Clear operation
CLEAR_OPTIONS=""
_cxd_op_c() {
    COMPREPLY=($(_cxd_compgen -W "$CLEAR_OPTIONS $GLOBAL_OPTIONS"))
}

# Internal invocation of compgen due to how it parses trailing '--'
# WORD - Current (maybe partial) word
_cxd_compgen() {
    if [ -n "$WORD" ]; then
        compgen "${@}" -- "$WORD"
    else 
        compgen "${@}"
    fi
}

_cxd() {
    # Defines for all functions
    WORD="${COMP_WORDS[$COMP_CWORD]}"
    if [ $COMP_CWORD -ne 0 ]; then
        LAST_WORD="${COMP_WORDS[$COMP_CWORD - 1]}"
    fi

    # Are we an operation?
    _cxd_operation
    if [ -n "$OP" ] && [ "$OP_LOC" != "$COMP_CWORD" ]; then
        _cxd_op_${OP}
    else
        COMPREPLY=($(_cxd_compgen -W "$OPERATIONS $GLOBAL_OPTIONS"))
    fi
}

complete -F _cxd cxd
