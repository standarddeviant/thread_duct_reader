# `thread_duct_reader`

This is a small example I created when wrapping an `egui` program around
a command-line process. When reading from `stdout` of the child process,
the read operation can block. If we use `std::thread` and the right I/O,
we can avoid blocking the main thread where the GUI is running.
