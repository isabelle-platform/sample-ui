#!/bin/bash
# Release generation script
# Usage: ./tools/release.sh
# Script is supposed to be run from main folder, e.g. ./tools/release.sh
output=""

. $(cd "$(dirname "$(which "$0")")"/.. ; pwd -P)/tools/lib/core.sh


while test -n "$1" ; do
    case $1 in
        --out)
            output="$2"
            ;;
    esac
    shift 1
done

function get_hash()
{
    pushd ${TOP_DIR} > /dev/null
    git rev-parse --short HEAD
    popd > /dev/null
}

if [ "$output" == "" ] ; then
    echo "No output set"
    exit 1
fi

build_folder="$(cd ${build_folder} && pwd)"
target_folder="$TOP_DIR/dist"
output="$(lib_core_normalize_filepath ${output})"

cd "${target_folder}"
echo $(get_hash) > hash
tar cJvf out.tar.xz *
cp out.tar.xz "${output}"
