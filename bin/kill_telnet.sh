#!/usr/bin/env bash

pid=$(ps -ef | grep telnet | grep -v grep | grep -v kill_telnet.sh | awk '{ print $2 }' | tail -1)

if [[ -z "${pid}" ]] ; then
    echo "no pid found"
else
    kill $pid
fi
