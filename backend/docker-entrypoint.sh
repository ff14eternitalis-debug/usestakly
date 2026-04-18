#!/bin/sh

set -u

echo "[usestakly-backend] booting"
/usr/local/bin/usestakly-backend
code=$?

if [ "$code" -ne 0 ]; then
  echo "[usestakly-backend] exited with code $code; keeping container alive for 300 seconds for debugging"
  sleep 300
fi

exit "$code"
