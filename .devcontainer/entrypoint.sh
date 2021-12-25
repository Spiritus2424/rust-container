#!/usr/bin/bash

FILE=/etc/udev/rules.d/60-openocd.rules
if [ ! -f "$FILE" ]; then
	mkdir -p /etc/udev/rules.d/
	sudo cp /lib/udev/rules.d/60-openocd.rules /etc/udev/rules.d/
    sudo udevadm control --reload-rules
fi
