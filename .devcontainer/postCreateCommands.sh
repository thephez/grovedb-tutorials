#/!bin/sh

# Command to run during postCreateCommand
# Done in script because when done via object the commands are run in parallel
#  (which isn't always desirable)
# https://containers.dev/implementors/spec/#parallel-exec
# https://containers.dev/implementors/json_reference/#formatting-string-vs-array-properties

# Needed to build
sudo apt update
sudo apt install -y libclang-dev
