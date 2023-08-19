#!/usr/bin/env sh

DEV="$1"

nmcli device set "$DEV" managed no
ip link set "$DEV" down
iw "$DEV" set monitor none
ip link set "$DEV" up