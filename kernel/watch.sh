
clear; make -s clean all

inotifywait --monitor --quiet --recursive --event close_write ./src |
while read -r filename event; do
    clear; make -s clean all
done
