# rtree
这是一个使用rust编写的，用来对ps输出进行格式化的工具，主要是将ps的输出转换成进程树结构。

由于很多定制系统中的ps，并不支持直接输出进程树信息，而只能单纯的输出
```
ps -ef
```
本工具就是将```ps -ef```的输出转换成进程树信息。

# build
```
$ cd rtree
$ cargo build
```

# Usage
1. 从文件中获取进程信息并进行格式化
```
$ ps -ef &> ps-ef.log
$ rtree --file ps-ef.log 
===================================================
init second_stage [1]
   ├─ylog [3878]
   │  └─sh /system/bin/ylogdebug.sh [4068]
   │      └─sleep 60s [14066]
   └─zygote64 [3895]
       ├─system_server [4906]
       └─com.android.phone [6939]
```

2. 或者直接接受标准输入的管道
```
$ ps -ef | rtree
===================================================
/sbin/init splash [1]
   ├─/lib/systemd/systemd-journald [382]
   ├─/lib/systemd/systemd-udevd [394]
   ├─/usr/sbin/ModemManager --filter-policy=strict [1097]
   ├─/usr/lib/accountsservice/accounts-daemon [1098]
   ├─avahi-daemon: running [username.local] [1100]
   │  └─avahi-daemon: chroot helper [1104]
   ├─/usr/sbin/cron -f [1102]
   ├─/usr/bin/dbus-daemon --system --address=systemd: --nofork --nopidfile --systemd-activation --syslog-only [1103]
   ├─/usr/sbin/NetworkManager --no-daemon [1105]
   ├─/usr/sbin/rsyslogd -n -iNONE [1110]
   ├─/usr/sbin/smartd -n [1112]
   ├─/lib/systemd/systemd-logind [1113]
   ├─/usr/lib/udisks2/udisksd [1115]
   ├─/sbin/wpa_supplicant -u -s -O /run/wpa_supplicant [1117]
   ├─/usr/lib/policykit-1/polkitd --no-debug [1172]
   ├─/usr/bin/deepin-devicemanager-server [1236]
   ├─/usr/lib/deepin-daemon/dde-system-daemon [1564]
   ├─/usr/sbin/cupsd -l [1645]
   ├─/usr/lib/upower/upowerd [1649]
   ├─/usr/bin/dde-dconfig-daemon [1748]
   ├─/usr/lib/deepin-authenticate/deepin-authentication [2025]
   ├─/lib/systemd/systemd-timesyncd [2673]
   ├─/bin/sh /usr/lib/bluetooth/bluetoothd.sh [5344]
   │  └─/usr/lib/bluetooth/bluetoothd [5346]
   ├─/usr/bin/deepin-home-appstore-daemon [18671]
   │  ├─[apt-get] <defunct> [23790]
   │  └─[apt-get] <defunct> [24287]
   ├─/usr/sbin/lightdm [19554]
   │  ├─/usr/lib/xorg/Xorg -background none :0 -seat seat0 -auth /var/run/lightdm/root/:0 -nolisten tcp vt1 -novtswitch [19559]
   │  └─lightdm --session-child 12 23 24 [19658]
   │      └─/usr/bin/startdde [20188]
   │          ├─/bin/sh /usr/bin/kwin_no_scale [20344]
   │          │  └─kwin_x11 -platform dde-kwin-xcb:appFilePath=/usr/bin/kwin_no_scale [20376]
   │          ├─/usr/lib/deepin-daemon/dde-session-daemon [20345]
   │          ├─/usr/bin/dde-desktop [20347]
   │          ├─/usr/bin/dde-dock -r [20388]
   │          ├─/usr/lib/polkit-1-dde/dde-polkit-agent [20779]
   │          ├─/usr/lib/deepin-deepinid-daemon/deepin-deepinid-daemon [20910]
   │          ├─/usr/bin/dde-clipboard [20911]
   │          ├─/usr/bin/dde-lock [20912]
   │          ├─dde-printer-helper [20913]
   │          ├─/usr/bin/deepin-system-monitor-daemon [20915]
   │          ├─/usr/bin/deepin-home-daemon --autostart [20916]
   │          ├─/usr/bin/dde-launcher [22228]
   │          ├─deepin-home-appstore-client [23038]
   │          ├─/bin/sh -c /usr/lib/deepin-daemon/default-terminal [25371]
   │          │  └─/usr/lib/deepin-daemon/default-terminal [25372]
   │          │      └─/usr/bin/deepin-terminal [25382]
   │          │          ├─/bin/bash [25389]
   │          │          │  └─./trojan -c config.json [26209]
   │          │          ├─/bin/bash [28740]
   │          │          └─/bin/bash [32882]
   │          ├─/usr/share/code/code --unity-launch --enable-crashpad [28308]
   │          │  └─/usr/share/code/code --type=zygote --no-zygote-sandbox --enable-crashpad --enable-crashpad [28311]
   │          │     └─/usr/share/code/code --type=gpu-process --enable-crashpad --crashpad-handler-pid=28328 
   │          ├─dde-file-manager -n [71111]
   │          └─/usr/lib/deepin-daemon/dde-osd [88683]
   ├─/lib/systemd/systemd --user [20170]
   │  ├─(sd-pam) [20171]
   │  ├─/usr/bin/pulseaudio --daemonize=no --log-target=journal [20182]
   │  ├─/usr/bin/dbus-daemon --session --address=systemd: --nofork --nopidfile --systemd-activation --syslog-only [20198]
   │  ├─/usr/lib/gvfs/gvfsd [20285]
   │  │  ├─/usr/lib/gvfs/gvfsd-trash --spawner :1.7 /org/gtk/gvfs/exec_spaw/0 [28965]
   │  │  ├─/usr/lib/gvfs/gvfsd-network --spawner :1.7 /org/gtk/gvfs/exec_spaw/1 [29010]
   │  │  └─/usr/lib/gvfs/gvfsd-dnssd --spawner :1.7 /org/gtk/gvfs/exec_spaw/5 [29044]
   │  ├─/usr/lib/gvfs/gvfsd-fuse /run/user/1000/gvfs -f -o big_writes [20295]
   │  ├─/usr/lib/x86_64-linux-gnu/bamf/bamfdaemon [20365]
   │  ├─/usr/lib/dconf/dconf-service [20367]
   │  ├─/usr/bin/deepin-wm-dbus [20383]
   │  ├─/usr/bin/kglobalaccel5 [20417]
   │  ├─/usr/lib/gvfs/gvfs-udisks2-volume-monitor [20831]
   │  ├─/usr/lib/bluetooth/obexd [20832]
   │  ├─/usr/lib/gvfs/gvfs-afc-volume-monitor [20845]
   │  ├─/usr/lib/gvfs/gvfs-gphoto2-volume-monitor [20850]
   │  ├─/usr/lib/gvfs/gvfs-mtp-volume-monitor [20854]
   │  ├─/usr/lib/gvfs/gvfs-goa-volume-monitor [20860]
   │  ├─/usr/bin/dde-clipboard-daemon [20999]
   │  ├─/usr/bin/deepin-appstore-session-daemon [21118]
   │  ├─/usr/bin/deepin-deepinid-client [21144]
   │  │  └─/usr/lib/x86_64-linux-gnu/qt5/libexec/QtWebEngineProcess --type=zygote --no-zygote-sandbox 
   │  │      └─/usr/lib/x86_64-linux-gnu/qt5/libexec/QtWebEngineProcess --type=zygote 
   │  ├─/usr/libexec/xdg-desktop-portal [22322]
   │  ├─/usr/libexec/xdg-document-portal [22333]
   │  ├─/usr/libexec/xdg-permission-store [22340]
   │  ├─/usr/libexec/xdg-desktop-portal-gtk [22353]
   │  ├─/usr/bin/dde-select-dialog-x11 [28978]
   │  └─/usr/lib/gvfs/gvfsd-metadata [29062]
   ├─/usr/bin/gnome-keyring-daemon --daemonize --login [20185]
   │  └─/usr/bin/ssh-agent -D -a /run/user/1000/keyring/.ssh [59957]
   ├─/usr/bin/fcitx-implugin-service [20278]
   ├─/usr/bin/fcitx -d 2 [20280]
   └─/usr/bin/fcitx-gsettingtool [20282]
```