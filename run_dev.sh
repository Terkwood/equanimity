#!/bin/bash
MYIP=$(ip addr|grep 192|awk '{ print $2 }'|sed 's;\/[0-9]*;;')
sh build.sh --dev && miniserve --port 9999 -i $MYIP ./dist --index index.html
