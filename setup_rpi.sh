#!/bin/bash

# update repositories and install important packages
apt-get update -y
apt-get install-y git curl

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

# configure Rust environment
apt-get install -y build-essentials
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# clone git repository into home folder
git 
