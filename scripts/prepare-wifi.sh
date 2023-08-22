#!/usr/bin/env sh
# SPDX-License-Identifier: GPL-3.0-only

DEV="$1"

echo "Unmanage $DEV by NetworkManager"
nmcli device set "$DEV" managed no
echo "Take device down"
ip link set "$DEV" down
echo "Put device into monitor mode"
iw "$DEV" set monitor none
echo "Put device up again"
ip link set "$DEV" up
echo "Set to channel 7"
iw dev "$DEV" set channel 7