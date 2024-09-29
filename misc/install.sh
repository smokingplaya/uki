#!/bin/sh

if ! command -v curl > /dev/null 2>&1; then
  echo "Execution interruption: curl is not installed"
  exit 1
fi

# you can customize installation process
URL="https://github.com/smokingplaya/uki/releases/latest/download/uki-linux_x86"
OUTPUT="uki"

curl -L $URL -o $OUTPUT
chmod +x $OUTPUT
sudo mv $OUTPUT /usr/local/bin
echo "The file was successfully moved to /usr/local/bin"
