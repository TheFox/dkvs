#!/usr/bin/env bash

while true; do
    pid=$(ps -ef | grep dkvs_server | grep -v grep | awk '{ print $2 }')
    if [[ ! -z "${pid}" ]] ; then
        echo "pid: ${pid}"
        lsof -Pnp "${pid}"
        echo
    fi
    sleep 1
done
