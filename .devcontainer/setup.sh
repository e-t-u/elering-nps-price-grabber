## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  fish \
  gnupg2 \
  jq \
  sudo \
  build-essential \
  openssl

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup component add rustfmt
rustup component add clippy

#cargo install cargo-expand
#cargo install cargo-edit

## setup and install oh-my-zsh
#sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
