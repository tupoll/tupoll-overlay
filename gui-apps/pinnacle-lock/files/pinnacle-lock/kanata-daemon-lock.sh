#!/bin/bash

mycmd="/usr/bin/kanata --cfg /etc/kantata/keymaps-lock.kbd"
mypid="$TMP/kanata.pid"
mylog="/dev/null" # change for a log file

case "$1" in
  start)
    if [ -e "$mypid" ]; then
      echo "kanata is already running"
      exit 1
    else
      nohup $mycmd &> "$mylog" &
      echo $! > $mypid
      echo "kanata have started in background"
    fi
    ;;
  stop)
    if [ -e "$mypid" ]; then
      kill -15 $(cat "$mypid")
      rm "$mypid"
      echo "kanata have stopped"
    else
      echo "kanata is not running"
      exit 2
    fi
    ;;
  status)
    if [ -e "$mypid" ]; then
      echo "kanata is running"
    else
      echo "kanata is stopped"
    fi
    ;;
  *)
    echo "$(basename $0) start|stop|status"
    ;;
esac
