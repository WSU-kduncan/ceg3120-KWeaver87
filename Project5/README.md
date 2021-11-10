# Project 5

## Objectives

- Modify the CF template to meet updated requirements
- Run a website of choice using nginx or apache2
- Configure the HAProxy load balancer to direct traffic to two backend systems

## SSH

- Public load balancer: `ssh ubuntu@ceg3120.kweave.net`
- Private web server 1: `ssh ubuntu@ceg3120.kweave.net -p 2201`
- Private web server 2: `ssh ubuntu@ceg3120.kweave.net -p 2202`
- Installed nftables to set up portforwarding from proxy to web servers
  - [Configuration file](./files/nftables.conf) (/etc/nftables.conf)
  - Reference used: [RHEL7 Security Guide](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/7/html/security_guide/sec-configuring_port_forwarding_using_nftables)

## HAProxy

- Install: `apt install haproxy`
- What file(s) where modified & their location
- What configuration(s) were set (if any)
- Restart server (after config change): `systemctl restart haproxy.service`
- Resources used (websites)

## Web server 1 & 2

- Install: `apt install nginx`
- What file(s) where modified & their location
- What configuration(s) were set (if any)
- How to restart the service after a configuration change
- Resources used (websites)

## Screenshots

- one screenshot that shows content from "server 1"
- one screenshot that shows content from "server 2"

## Link

http://ceg3120.kweave.net

The haproxy load balancing may be interfered with by fancy Cloudflare DNS services.
