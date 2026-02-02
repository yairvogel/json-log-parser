#!/usr/bin/env bash

set -x

if ! command -v cargo >/dev/null 2>&1; then
    >&2 echo "cargo is required for installation"
    >&2 echo "install using brew install cargo"
    exit 1
fi


git clone --depth 1 https://github.com/yairvogel/json-log-parser

pushd json-log-parser

cargo b --release

EXEC_DIR="~/.local/bin"
mkdir -p $EXEC_DIR

cp target/release/json-log-parser $EXEC_DIR
cp kubectl-parse $EXEC_DIR

if command -v k9s >/dev/null 2>&1; then
    read _ K9S_PLUGIN_PATH < <(k9s info | grep Plugins)
    if [ ! -f $K9S_PLUGIN_PATH ]; then
        echo 'plugins:' > "$K9S_PLUGIN_PATH"
    fi
    cat k9s_plugin_def.yaml >> "$K9S_PLUGIN_PATH"
else
    echo "could not find k9s installation. skipping plugin installation"
fi

echo "using $EXEC_DIR as executables directory. make sure it is in your PATH"

popd

rm -rf json-log-parser
