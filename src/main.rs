mod mymod;

fn main() {
    assert_eq!(mymod::is_match(&"(xx((hello))xxx)"), true);
    assert_eq!(mymod::is_match(&"(t)est("), false);
    assert_eq!(mymod::is_match(&"(((x))"), false);
}

/*
1. qemu-img create -f raw <vd> <size>
2. -append "root=/dev/sda init=/linuxrc ip=dhcp nokaslr console=ttyS0"
-drive format=raw,file=<vd>
3. INSTALL_MOD_PATH=<vd-mp> make modules_install
4. /etc/init.d/rcS
mount -t proc none /proc
mount -t sysfs none /sys
*/