#/bin/bash
# Intended for initial setup of Ubuntu (maybe Debian?) VMs with passwordless sudo

# Exit on error
set -e

# Don't Repeat Yourself
USER=$(whoami)
USERHOME=/home/$USER
WHICH_ZSH=$(which zsh)

# Update and install apt packages
sudo apt update
sudo apt upgrade -y
sudo apt install build-essential git zsh curl wget aptitude mc fail2ban -y

# Set zsh to default shell
sudo chsh -s $WHICH_ZSH $USER
sudo chsh -s $WHICH_ZSH root
touch ~/.zshrc
zsh

# Install oh-my-zsh and make $ZSH_CUSTOM available
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
source ~/.zshrc

# Download and set desired zsh theme
curl -fsSL http://raw.github.com/zakaziko99/agnosterzak-ohmyzsh-theme/master/agnosterzak.zsh-theme > $ZSH_CUSTOM/themes/agnosterzak.zsh-theme
sed -i 's:="robbyrussell:="agnosterzak:' ~/.zshrc

# Install Nix single-user and update $PATH
curl -L https://nixos.org/nix/install | sh -s -- --no-daemon
source ~/.zshrc

# Install nix packages
nix-env -iA nixpkgs.bat nixpkgs.bottom nixpkgs.bpytop nixpkgs.du-dust nixpkgs.cht-sh nixpkgs.exa nixpkgs.fd nixpkgs.ripgrep nixpkgs.sd nixpkgs.bandwhich

# Alias for l command
echo "
alias l=\"exa -lah\"
" | tee -a ~/.zshrc > /dev/null

# Set up oh-my-zsh and nix binaries for root
sudo cp -r $USERHOME/.oh-my-zsh /root/
sudo cp $USERHOME/.zshrc /root/
sudo sed -i "s:$USERHOME:/root:g" /root/.zshrc
sudo mkdir /root/.nix-profile
sudo ln -s $USERHOME/.nix-profile/bin /root/.nix-profile/bin
echo "
if [ -d \"/root/.nix-profile/bin\" ] ; then
  PATH=\"/root/.nix-profile/bin:\$PATH\"
fi" | sudo tee -a /root/.zshrc > /dev/null

# Reboot machine immediately
sudo shutdown -r now
