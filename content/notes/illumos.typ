#metadata((
  title: "illumos",
  desc: "The powerful Unix operating system",
  homepage: false,
)) <page>

#import "../_shared/template.typ": post, link-new-tab 
#show: post

#link-new-tab("illumos", "https://illumos.org/") is a Unix OS "core"
 - Like the Linux kernel, it can't be used directly -- distros must be made around it
 - Provides much more out-of-the-box than Linux (distros do less)

It derives from Unix:
 - AT&T System V Unix → Solaris → OpenSolaris → illumos


= Key Features
+ *ZFS*: File-system + volume management
  - CoW, integrity, snapshots, pooling, RAID (built-in)
+ *DTrace*:
  - (TODO link to note page)
  - Interpretted in the Kernel
  - Gives ability to "hook" in anywhere to modify functionality, monitor, trace, profile, etc.
  - Uses language inspired by C and `awk`
    - Comparitively, eBPF's bytecode is more flexible (allows DTrace-like languages and any language that compiles to LLVM)
+ *Zones*:
  - Zones still use the same kernel, but provide sandboxing
  - Much lighter than full VMs -- again, it's using the same kernel and there's no interpreter (runs directly on hardware)
  - In many cases, can remove the need for virtualization (IE you'd run many Docker containers on a server for sandboxing, while zones can provide this directly)

= Zones
The GZ (global zone) is where you work (when you boot in).
It is the host system/real root. It can see and
control other zones and where you adminster zones from.

== Brands
+ Spare (traditional):
   - share `/usr`, `/lib`, `/sbin` from GZ (RO loopback mounts)
   - have own `/var`, `/etc`, `/opt`
   - lightweight, fast to create
   - upgrades happen at global zone level
+ Whole root:
   - complete separate copy of filesystem
   - can have diff pkg versions than GZ
+ ipkg zones:
   - use IPS (image packaging system)
   - can have indepenedent package state
   - more flexible than spare

== States
+ *configured*: zone defn exists, not on disk
+ *incomplete*: installing..
+ *installed*: zone fs setup, not booted
+ *ready*: booted, but zone processes can't run yet
+ *running*

== Management

- `zoneadm`:
   - `list`
   - `install`: configured → installed
   - `uninstall`: installed → configured
   - `boot`: installed → running
   - `shutdown` | `halt`: running → installed
   - `ready`: installed → ready (prime it)
- `zonecfg`: define zone props, resources, networking
- `zlogin`: think ssh into zone


= Blizzard Cheat-sheet
== View Zones
```bash
sudo zoneadm list -cv 
```

== Login to Zone
```bash
sudo zlogin NAME
```

== Making Zone

*Configure*:
```bash
sudo zonecfg -z NAME
# then in the tool
create
set zonepath=/zones/NAME
set brand=ipkg
set autoboot=true
add net
set physical=NAME0  # no dashes
end
set ip-type=exclusive
verify
commit
exit
```

*Create VNIC*:
```bash
sudo dladm create-vnic -l i40e0 NAME0
# verify
dladm show-vnic
```

*Install & Boot Zone*:
```bash
sudo zoneadm -z NAME install
sudo zoneadm -z NAME boot
# verify
sudo zoneadm list -cv 
```

*Login & Setup*:
```bash
sudo zlogin -C NAME
# once logged in
ipadm create-addr -T dhcp NAME0/v4
ipadm show-addr
hostname flake1-NAME
echo "flake1-NAME" > /etc/hostname
dladm show-vnic NAME0
echo "nameserver 192.168.5.1" > /etc/resolv.conf
vim /etc/nsswitch.conf # change `hosts: files` -> `hosts: files dns`
~~. # logout
sudo zoneadm -z NAME reboot
```

== Deleting Zone
```bash
sudo zoneadm -z NAME halt
sudo zoneadm -z NAME uninstall
sudo zonecfg -z NAME delete
sudo dladm delete-vnic NAME0
```

