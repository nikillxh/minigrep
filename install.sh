#!/bin/sh

echo "Welcome to minigrep installation!
Script must be in installation files directory:"
while true; do
    echo "For Arch Linux          : Press 1
For Executable Binary   : Press 2
For Exiting Installation: Press 3 or Ctrl + C"
    read input
    if (( $input == 1 )); then
        rm -f minigrep-1.0-1-x86_64.pkg.tar.zst
        cargo build --release
        makepkg -si --noconfirm
    elif (( $input == 2 )); then
        echo "Enter binary install path: "
        read directory_path
        if test -d "$directory_path"; then
            echo "Directory '$directory_path' exists.
Beginning installation......"
            cargo build --release
            cp ./target/release/minigrep $directory_path
            [[ $? -eq 0 ]] && echo "Installed successfully!" && exit
            echo "Some error occured"
            exit
        else
            echo "Directory does not exist, creating."
            mkdir -p "$directory_path"
            if (( $? == 0 )); then
                echo "Directory '$directory_path' created successfully."
                cargo build --release
                cp ./target/release/minigrep $directory_path
                [[ $? -eq 0 ]] && echo "Installed successfully!" && exit
                echo "Some error occured"
                break
            else
                echo "Failed to create directory '$directory_path'."
                echo "Try another directory path next time."
            fi
        fi
    elif (($input == 3)); then
        echo "Exiting.........."
        exit
    else
        echo "Enter number from the given options!
Restarting installer.........."
    fi
done

