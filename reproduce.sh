#!/usr/bin/env bash
# in the ThingOS shell:
# cat /https/example.com
# attr_list /https/example.com
echo "ping -c 1 example.com"
sleep 5
echo "cat /https/example.com > /dev/null"
sleep 10
echo "attr_list /https/example.com"
sleep 5
echo "exit"
