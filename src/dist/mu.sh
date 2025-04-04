#!/usr/bin/env bash
BASE=${MU_HOME:=/opt/mu}

usage () {
    echo "usage: $0 [session options] [runtime options] src-file..."    >&2
    echo "[session options]"                                            >&2
    echo "  -h, --help                 print this message and exit."    >&2
    echo "  --config=config-list       runtime configuration."          >&2
    echo "[runtime options]"                                            >&2
    echo "  --eval=form                evaluate form and print result." >&2
    echo "  --quiet-eval=form          evaluate form quietly."          >&2
    echo "  --load=src-file            load src-file in sequence."      >&2
    echo "  --version                  print version and exit."         >&2
    exit 2
}

optspec=":h-:"
while getopts "$optspec" optchar; do
    case "${optchar}" in
        -)
            case "${OPTARG}" in
                eval=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    OPTIONS+=" -e \"${val}\""
                    ;;
                load=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    OPTIONS+=" -l \"${val}\""
                    ;;
                quiet-eval=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    OPTIONS+=" -q \"${val}\""
                    ;;
                config=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    OPTIONS+=" -c \"${val}\""
                    ;;
                help)
                    usage
                    ;;
                version)
                    $BASE/bin/mu-sys -v
                    echo
                    exit 2
                    ;;
                pipe)
                    OPTIONS+=" -p"
                    ;;
                *)
                    if [ "$OPTERR" = 1 ] && [ "${optspec:0:1}" != ":" ]; then
                        echo "Unknown option --${OPTARG}" >&2
                    fi
                    ;;
            esac;;
        h)
            usage
            ;;
        *)
            if [ "$OPTERR" != 1 ] || [ "${optspec:0:1}" = ":" ]; then
                echo "Non-option argument: '-${OPTARG}'" >&2
            fi
            ;;
    esac
done

for (( i=${OPTIND}; i<="${#@}"; i++ )); do SOURCES+=" \"${!i}\"" ; done

eval $BASE/bin/mu-sh -l $BASE/dist/core.fasl $OPTIONS # $BASE/dist/mu.l ${SOURCES[@]}
