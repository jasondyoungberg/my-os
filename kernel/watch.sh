
clear; make -s clean all

inotifywait -qme close_write ./src |
while read -r filename event; do
    clear; make -s clean all
done
