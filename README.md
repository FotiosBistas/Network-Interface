# Useful cli-tools to assist in debugging

Ping using ICMP packets. 
```
ping -I Itun 192.168.1.137
```

To send a TCP packet to the interface. 
```
nc 192.168.1.137 443
``` 

Wireshart for cli for logging.

```
tshark -i Itun
```
