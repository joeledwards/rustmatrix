- add configuration option for density (0.1 - 0.9)
- clear the screan after ctrl+c (seems like termion is supposed to handle this, possible we are not awaiting the completion of a thread)
- traces should update at different rates
- stagger updating of characters (reduce "pulsing")
- permit overwrites (multiple traces in the same column)
- add configuration option for overwrite behavior (deletes the trace it catches vs being followed by it)
- handle terminal resizing (see termion's `terminal_size()` function)
  - when window size decreasing, delete out-of-bounds traces
  - when window size increasing, start sliding again / add new traces
- add configuration option for trace "gravity" (tendency to cluster together rather than simple random placement)
- add support for multiple colors at once via new --colors option which accepts a string of letters indicating
  the weight of each color based on frequency, so "rrb" would be twice as many red as blue

