#!/bin/bash

clear; make -s all tidy

inotifywait --monitor --quiet --recursive --event close_write ./src |
while read -r filename event; do
    clear; make -s all tidy
done
