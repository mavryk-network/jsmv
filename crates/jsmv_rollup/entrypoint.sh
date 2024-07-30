#!/usr/bin/env bash
set -e

export JSMV_ROLLUP_MAVKIT_CLIENT_DIR="/root/.mavkit-client"
mkdir -p "$JSMV_ROLLUP_MAVKIT_CLIENT_DIR"

export JSMV_ROLLUP_MAVKIT_ROLLUP_NODE_DIR="/root/.mavkit-smart-rollup-node"
mkdir -p "$JSMV_ROLLUP_MAVKIT_ROLLUP_NODE_DIR"

# shellcheck disable=SC2034 
# JSMV_ROLLUP_MAVKIT_NODE_ENDPOINT is used in the jsmv-rollup command
export JSMV_ROLLUP_MAVKIT_NODE_ENDPOINT="https://rpc.$NETWORK.teztnets.com/"

installer_dir="root/installer"

if [ ! -f "$JSMV_ROLLUP_MAVKIT_CLIENT_DIR/secret_keys" ]; then
    echo "Importing operator secret key..."
    if [ -z "$OPERATOR_SK" ]; then
        echo "OPERATOR_SK is not set"
        exit 1
    fi
    jsmv-rollup operator import-keys --secret-key "$OPERATOR_SK"
fi


run() {
    mkdir -p "$LOGS_DIR"
    jsmv-rollup run \
        --preimages "$installer_dir/preimages" \
        --rollup "$JSMV_ROLLUP_ADDRESS" \
        --logs "$LOGS_DIR"
}

deploy() {
    jsmv-rollup deploy-installer \
        --installer "$installer_dir/installer.wasm" \
        --bridge "$JSMV_ROLLUP_BRIDGE_ADDRESS"
}

main() {
    command="$1"
    shift 1

    case $command in
        "run")
            run
            ;;
        "deploy")
            deploy
            ;;
        *)
            cat <<EOF
Usage: $0 <COMMAND>

Commands: 
    run 
    deploy
EOF
            exit 1
            ;;
    esac
}

if [ "$0" == "${BASH_SOURCE[0]}" ]; then
    main "$@"
fi
