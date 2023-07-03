remove-item .\test.csv

start-sleep 1

start-process cargo "run"

start-sleep 1

copy-item .\pre.csv .\test.csv



