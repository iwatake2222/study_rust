```
docker pull rust:1.55
docker run -p 8080:8080 -v /C/iwatake/devel:/devel  -e DISPLAY="192.168.1.2:0" -itd --name="rust_01" rust:1.55

docker start rust_01
docker exec -it rust_01 bash
```

```
root@557554973fa5:/# cat /etc/hosts
# 127.0.0.1       localhost
0.0.0.0 localhost
```

