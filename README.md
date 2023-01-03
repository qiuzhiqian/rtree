# rtree
这是一个使用rust编写的，用来对ps输出进行格式化的工具，主要是将ps的输出转换成进程树结构。

由于很多定制系统中的ps，并不支持直接输出进程树信息，而只能单纯的输出
```
ps -ef
```
本工具就是将```ps -ef```的输出转换成进程树信息。

# dump
```
root             1     0 0 19:10 ?        00:00:24 init second_stage
├─system        3878     1 1 19:23 ?        00:00:30 ylog
│  └─system        4068  3878 0 19:24 ?        00:00:07 sh /system/bin/ylogdebug.sh
│    └─system       14066  4068 4 42:57 ?        00:00:00 sleep 60s
└─root          3895     1 0 19:23 ?        00:00:02 zygote64
   ├─system        4906  3895 1 20:08 ?        00:00:59 system_server
   └─radio         6939  3895 0 21:40 ?        00:00:02 com.android.phone
```