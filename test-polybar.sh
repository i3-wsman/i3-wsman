#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "$(readlink -f ${BASH_SOURCE[0]})" )" &> /dev/null && pwd )
I3WSM_ROOT=$(realpath $SCRIPT_DIR)
cd $I3WSM_ROOT

MACHINE=$MACHINE cargo run polybar "$@" 2>/dev/null
