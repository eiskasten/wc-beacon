#!/usr/bin/env sh
# SPDX-License-Identifier: GPL-3.0-only

DEV="$1"

nmcli device set "$DEV" managed no
ip link set "$DEV" down
iw "$DEV" set monitor none
ip link set "$DEV" up