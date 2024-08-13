#/usr/bin/env bash

# Defining as one large function to hide subfunctions from env
_cxd() {
    # Defines for all functions
    local CXD="${COMP_WORDS[0]}" WORD="${COMP_WORDS[$COMP_CWORD]}" LAST_WORD
    if [ $COMP_CWORD -ne 0 ]; then
        LAST_WORD="${COMP_WORDS[$COMP_CWORD - 1]}"
    fi

    # All operations must have an _op_<OP_CHAR>() function defined
    local OPERATIONS="--add -a --remove -r --list -l --clear"
    local GLOBAL_OPTIONS="--file -f --help -h --version"
    local GLOBAL_SKIPS="--file 1 -f 1 --help 0 -h 0 --version 0"

    # Calls cxd to get a list of valid command names
    _cxd_names() {
        # Doing some unsplitting to replace newlines with spaces
        local names=($($CXD --list --short))
        echo "${names[@]}"
    }

    # Counts number of free args in ${COMP_WORDS} after start_index
    # skip_opt options will have their corresponding skip_num args skipped in the count
    # Usage: _count_free_args start_index [skip_opt skip_num]...
    _cxd_count_free_args() {
        local start_index=$1
        shift;
        local skip_opt=(${@})
        local skip_next=0 free_args=0 first_arg_index index=$((start_index-1))
        for item in "${COMP_WORDS[@]:$start_index}"; do
            index=$((index + 1))
            # Was last a skip opt
            if [ $skip_next -ne 0 ]; then
                skip_next=$((skip_next - 1))
                continue
            elif [[ $item =~ /$-/ ]]; then
                continue
            fi
            # Check if its a skip opt
            local i=0
            for skip in ${skip_opt[@]}; do
                i=$((i + 1))
                if [ "$item" = "$skip" ]; then
                    # Next op is number to skip
                    skip_next=${skip_opt[$i]}
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
    _cxd_op_a() {
        local ADD_OPTIONS="--cwd -c --dir -d --env -e"
        _cxd_count_free_args $((OP_LOC + 1)) --dir 1 -d 1 --env 3 -e 3
        case $FREE_ARGS in
            0|1) 
                if [ "$LAST_WORD" = "--file" ] || [ "$LAST_WORD" = "-f" ]; then
                    COMPREPLY=($(_cxd_compgen -f))
                elif [ "$LAST_WORD" = "--env" ] || [ "$LAST_WORD" = "-e" ]; then
                    COMPREPLY=()
                elif [ "$LAST_WORD" = "--dir" ] || [ "$LAST_WORD" = "-d" ]; then
                    COMPREPLY=($(_cxd_compgen -d))
                else
                    COMPREPLY=($(_cxd_compgen -W "$ADD_OPTIONS $GLOBAL_OPTIONS")) 
                fi
                ;;
            2) COMPREPLY=($(_cxd_compgen -abc)) ;;
            *) _command_offset $((FIRST_ARG_INDEX + 1)) ;;
        esac
    }

    # Remove operation
    _cxd_op_r() {
        local REMOVE_OPTIONS="--id -i"
        if [ "$LAST_WORD" = "--file" ] || [ "$LAST_WORD" = "-f" ]; then
            COMPREPLY=($(_cxd_compgen -f))
        elif [ "$LAST_WORD" = "--id" ] || [ "$LAST_WORD" = "-i" ]; then
            COMPREPLY=()
        else
            COMPREPLY=($(_cxd_compgen -W "$REMOVE_OPTIONS $GLOBAL_OPTIONS $(_cxd_names)"))
        fi
    }

    # List operation
    _cxd_op_l() {
        local LIST_OPTIONS="--short -s"
        if [ "$LAST_WORD" = "--file" ] || [ "$LAST_WORD" = "-f" ]; then
            COMPREPLY=($(_cxd_compgen -f))
        else
            COMPREPLY=($(_cxd_compgen -W "$LIST_OPTIONS $GLOBAL_OPTIONS"))
        fi
    }

    # Clear operation
    _cxd_op_c() {
        local CLEAR_OPTIONS=""
        if [ "$LAST_WORD" = "--file" ] || [ "$LAST_WORD" = "-f" ]; then
            COMPREPLY=($(_cxd_compgen -f))
        else
            COMPREPLY=($(_cxd_compgen -W "$CLEAR_OPTIONS $GLOBAL_OPTIONS"))
        fi
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

    # Are we an operation?
    local i=0
    for item in "${COMP_WORDS[@]}"; do
        for op in ${OPERATIONS[@]}; do
            if [ "$item" = "$op" ]; then
                # Sets OP and OP_LOC if found. OP is the first letter of the operation name
                OP=$(echo $op | sed -r 's/^--?([a-z]{1}).*$/\1/');
                OP_LOC=$i
                break 2
            fi
        done
        i=$(( $i + 1 ))
    done
    if [ -n "$OP" ] && [ "$OP_LOC" != "$COMP_CWORD" ]; then
        _cxd_op_${OP}
    else
        COMPREPLY=($(_cxd_compgen -W "$OPERATIONS $GLOBAL_OPTIONS $(_cxd_names)"))
    fi
}

complete -F _cxd cxd
