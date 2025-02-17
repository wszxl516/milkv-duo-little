#!/bin/sh
#run on host
#sudo iptables -t nat -A POSTROUTING -s 192.168.42.0/24 -o enp13s0 -j MASQUERADE
ip r add default via $1
echo "nameserver 8.8.8.8" > /etc/resolv.conf
service chronyd restart
echo export http_proxy=http://$1:8888; export https_proxy=http://$1:8888