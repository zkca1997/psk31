#!/bin/bash

# update repositories and install important packages
apt-get update -y
apt-get install -y git curl
git config --global user.email "zachary.johnson183@gmail.com"
git config --global user.name "zkca1997"

# configure local wifi server for ssh
apt-get install -y hostapd dnsmasq
systemctl stop hostapd
systemctl stop dnsmasq
echo "interface wlan0
static ip_address=10.10.27.1/24" >> /etc/dhcpcd.conf
mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
echo "interface=wlan0
				dhcp-range=10.10.27.2,10.10.27.20,255.255.255.0,24h" > /etc/dnsmasq.conf
echo "interface=wlan0
hw_mode=g
channel=7
wmm_enabled=0
macaddr_acl=0
auth_algs=1
ignore_broadcast_ssid=0
wpa=2
wpa_key_mgmt=WPA-PSK
wpa_pairwise=TKIP
rsn_pairwise=CCMP
ssid=pinet
wpa_passphrase=fluffycockroach*8" > /etc/hostapd/hostapd.conf
echo 'DAEMON_CONF="/etc/hostapd/hostapd.conf"' >> /etc/default/hostapd

# configure NeoVim
apt-get install -y neovim
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
       https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
mkdir -p $HOME/.config/nvim
echo "call plug#begin('~/.vim/plugged')
Plug 'itchyny/lightline.vim'
Plug 'rust-lang/rust.vim'
call plug#end()

set nocompatible
set ruler
set showmatch
set tabstop
set autoindent
set number" > $HOME/.config/nvim/init.vim

# configure Rust environment
apt-get install -y build-essentials
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# clone git repository into home folder
cd $HOME
git clone https://github.com/zkca1997/psk31.git
