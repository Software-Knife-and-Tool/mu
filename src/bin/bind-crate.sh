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
    echo "single crate argument" >&2
    exit 1
fi
arg_index="$((${OPTIND} - 1))"

crate_name="${args[${arg_index}]}"
crate_bind_dir=${crate_name}/mu-bindgen
crate_lib_src=${crate_name}/src/lib.rs

clone_out=`cargo clone ${crate_name} 2>&1`
crate_about=${clone_out//.*Downloading/}

mkdir ${crate_bind_dir}

crate_bind_sym_path=${crate_bind_dir}/lib.sym
crate_bind_src_path=${crate_bind_dir}/lib.rs
    
if ${verbose}; then
    echo ";;; bind-crate:   " ${crate_name}
    echo ";;;   namespace:  " ${crate_name}
    echo ";;;   info:       " ${crate_about}
    echo ";;;   lib.rs:     " ${crate_lib_src}
    echo ";;;   symbols:    " ${crate_bind_sym_path}
    echo ";;;   bindings:   " ${crate_bind_src_path}
fi

mu-bindgen --namespace ${crate_name} --symbols ${crate_bind_sym_path} --bindings ${crate_bind_src_path} --verbose ${crate_lib_src}
