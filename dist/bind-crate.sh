#!/usr/bin/env bash
BASE=/opt/mu

verbose=false
usage () {
    echo "usage: $0 [options] crate-specifier" >&2
    echo "[options]" >&2
    echo "  -h, --help                 print this message and exit." >&2
    echo "  --config=config-list       configuration." >&2
    echo "  --version                  print version and exit." >&2
    echo "  --verbose                  verbose operation." >&2
    
    exit 2
}

optspec=":h-:"
while getopts "$optspec" optchar; do
    case "${optchar}" in
        -)
            case "${OPTARG}" in
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
                    exit 0
                    ;;
                verbose)
                    verbose=true
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

args=("$@")
nargs="$((${#@}-${OPTIND}))"
if [ "${nargs}" != 0 ]; then
    echo "single one crate argument" >&2
    exit 1
fi

arg_index="$((${OPTIND} - 1))"
crate=${args[${arg_index}]}

cargo clone ${crate}

crate_dir=(${crate//@/ })
crate_srcs=${crate_dir}/src/*
crate_src_files=(${crate_srcs// /})

if ${verbose}; then
    echo "bind-crate: loading" ${crate_dir}
    echo "bind-crate: sources" ${crate_srcs}
    echo "number of sources" ${#crate_src_files}
    for source in ${crate_src_files}; do
        echo "src:" ${source}
    done
fi

for source in ${crate_src_files}; do
    mu-bindgen --symbols ${crate_dir}.symbols.out --output ${crate_dir}.${source}.out --verbose ${source}
done
