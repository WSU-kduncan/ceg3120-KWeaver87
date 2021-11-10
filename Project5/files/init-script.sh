#!/bin/bash -xe
# Upgrade installed packages first
apt-get update &&
apt-get upgrade -y &&

# Install desired packages
apt-get install -y git fish doc-base curl ripgrep fd-find fail2ban w3m &&

# Append friendly names to hosts
echo " \
10.0.0.8 proj5-proxy \
10.0.0.11 httpd-1 \
10.0.0.12 httpd-2" >> /etc/hosts &&

# Set up Starship prompt on fish shell
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- -y &&
echo "starship init fish | source" >> /etc/fish/config.fish &&

# Make fish default shell
sed -i 's#/bin/bash#/usr/bin/fish#g' /etc/passwd &&
