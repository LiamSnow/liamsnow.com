#metadata((
  title: "My self-hosting journey",
  desc: "Docker, Kubernetes, NixOS, Helios Illumos",
  date: "2026-02-01",
  homepage: true
)) <page>

#import "/_shared/template.typ": post, link-new-tab
#show: post

Once upon a time, I decided that I really wanted to start self-hosting
many of services. Specifically, hosting my photos and files.
I did this for a few reasons:
 + I want control of my data
 + I was not happy with existing options (Google Drive, Proton Drive, etc. all have slow syncing and don't have ignore rules)

= A First Version: Monolith

I took a bunch of old PC parts I had lying around and scrapped
together a mid-tower build. I set up Arch Linux on it and ran a few things in Docker containers:
 + #link-new-tab("NextCloud", "https://nextcloud.com/") for files, photos, and CalDAV
 + #link-new-tab("Tailscale Serve", "https://tailscale.com/kb/1312/serve") to expose services under a public URL which requires users to be on my Tailnet
   - creates domains like `<service>.<tailnet name>.ts.net`

#link-new-tab("Tailscale", "https://tailscale.com") made remote management
easy. But I had a problem - my disk is encrypted so what happens if my
server reboots? It'll just be stuck at cryptlvm login.
To fix this I added
#link-new-tab("mkinitcpio-dropbear", "https://github.com/grazzolini/mkinitcpio-dropbear") which adds
#link-new-tab("Dropbear SSH", "https://matt.ucc.asn.au/dropbear/dropbear.html") 
into the `/boot` volume and runs it before the disk is decrypted.

== I Hate NextCloud
NextCloud continuously frustrated me with how
complex, slow, and overall just bad it was.
File syncing was a horrible experience and it wouldn't let me add ignore
rules (ex. for `node_modules`, `target`).
It felt over-engineered for what I was doing.

I tried a few competitors to NextCloud, but none of them really had what
I wanted. So, I decided to scrap the frontend entirely, opting to use
#link-new-tab("Syncthing", "https://syncthing.net/") to host my files.
This was genuinely an amazing experience. Not only did it allow great ignore rules, it
was super fast. Combining it with #link-new-tab("GNU Stow", "https://www.gnu.org/software/stow/manual/stow.html")
enabled me to sync my dotfiles between my laptop and PC.

== More Services
I added:
 + #link-new-tab("Immich", "https://immich.app/") for photos (very much like Google Photos)
 + #link-new-tab("Radicale", "https://radicale.org/v3.html") for CalDAV
 + #link-new-tab("Kanidm", "https://kanidm.com/") OIDC
 + linkding, mealie, searxng, ..

As I started adding services, I needed a way to manage them.
I had a folder in `/home` which contained folders for each
service. Each service had its own `docker-compose.yml`.
I created a variety of bash scripts to update
and manage all of them.

While this worked fine, I wanted something more purpose built
for what I was doing. Furthermore, I just did not
feel good enough about the security of this to expose
services to the internet.

= Moving to Kubernetes: Blizzard
== Building a Rack
I took apart my server PC and sold each component. With this money,
I was able to find some great Facebook Marketplace deals for old
servers.

I made a rack in closet with the following:
 + Ubiquiti Cable Internet (Cable Modem)
 + Ubiquiti UDM Pro
 + Ubiquiti 10G switch (from my Dad)
 + 2x Dell PowerEdge R430
   - dual E5-2640v3
   - 128GB ECC DDR4 2133MHz 
   - 4x 750GB SAS SSDs
   - Intel(R) 10GbE 2P X710 Adapter 
   - PERC H730P Mini (RAID controller I bought separately)
   - Dual 550W power supplies
   - Dual (redundant) SD card module (IDSDM)
 + 1x Skullcandy Intel NUCs (from my Dad)
 + Rack mount UPS

== OS
I installed #link-new-tab("Talos Linux", "https://www.talos.dev/")
on all the 2 Dell servers and NUC. All were configured as both control planes and workers.

Talos Linux is a super cool operating system, meant solely to run Kubernetes (k8s).
It's an immutable OS with a very small attack surface.
You simply just configure each node with a YAML file and boot it.
It doesn't even have SSH!

== Services
I used #link-new-tab("Helm", "https://helm.sh/")
and #link-new-tab("helmfile", "https://github.com/helmfile/helmfile")
to configure my cluster.

I ran many of the same services I had on the old setup, but also:
 - #link-new-tab("metallb", "https://metallb.io/") to expose a virtual IP for the cluster (to port forward to)
 - #link-new-tab("traefik", "https://traefik.io/traefik") reverse proxy
 - #link-new-tab("openebs mayastor", "https://github.com/openebs/mayastor") to replicate storage across all machines for #link-new-tab("high availability (HA)", "https://en.wikipedia.org/wiki/High_availability")
 - Prometheus + Grafana + Ntfy (mobile notifications)
 - #link-new-tab("Firefox syncserver", "https://github.com/mozilla-services/syncserver")

Getting all my old services running here took a *lot* of time.
Very few self-hosters run Kubernetes, so almost none of the services
make it easy to run in Kubernetes.

== Results
Overall I was super happy with this setup.
I was very happy with the security model of this,
it had HA, and I publicly exposed it to the internet
on `<service>.liamsnow.com` domains.

However, it limited what services I could run.
I gave up trying to run Jellyfin (and accompanying services).
It was also quite a pain to manage - because I am just
not a Kubernetes expert. In many ways it taught me
I never wanted to work in infrastructure lol.

== The Blizzard Revolution
I made giant mistakes in this cluster.
Since Talos was immutable, I placed the OS on the dual redundant SD cards
and used the RAID array entirely for mayastor.

While I was studying abroad in Morocco I noticed that randomly all
my services went down. After a long investigation, I think I understand what happened:
+ Flake 1 (dell server) lost IDSDM SD card redundancy because 1 of the SD cards failed. Soon after, it lost the other SD card.
  - While Talos is immutable, it has an ephemeral partition for logs, container data, and etcd data (in this case on the IDSDM) #link-new-tab("source", "https://docs.siderolabs.com/talos/v1.10/configure-your-talos-cluster/storage-and-disk-management/disk-management")
  - SD cards quickly degrade at this high of write frequency
  - This never alerted me because I never finished setting up iDRAC alerts over Ntfy
+ Flake 1 didn't go out cleanly. It was holding on for dear life:
  - Its corrupted SD card caused the WAL to become inconsistent. etcd lost track of what it had actually persisted, so its view of the Raft log diverged from what it had already communicated to Flake 2 & 3
  - Before fully dying, it was sending stale or malformed AppendEntries RPCs to the other nodes, poisoning their view of the log
  - Timing out as leader or failing heartbeats, triggering reelections
  - Once Flake 1 was fully down, Flake 2 & 3 were left disagreeing with each other on the state of the cluster

Some sources:
 - #link-new-tab("raft.github.io", "https://raft.github.io")
 - #link-new-tab("Announcing etcd 3.4", "https://kubernetes.io/blog/2019/08/30/announcing-etcd-3-4/")
 - #link-new-tab("etcd 3.4 FAQ", "https://etcd.io/docs/v3.4/faq/")
 - #link-new-tab("etcd 3.4 Tuning", "https://etcd.io/docs/v3.4/tuning/")
 - #link-new-tab("Key metrics for monitoring etcd", "https://www.datadoghq.com/blog/etcd-key-metrics/")

So, I worked on getting Flake 1 back up and running. I had to:
+ SSH tunnel iDRAC through my PC so I could remotely manage Flake1
+ Split the RAID array into two parts (one for Talos, one for mayastor)
+ Reinstall Talos, by using virtual media in iDRAC
+ Set up the node back into the cluster

But this didn't work. The cluster was broken and Flake 1 couldn't recover the damage it did.

I was frustrated and wanted to give up. But I couldn't go back to
a bunch of Docker containers..

= NixOS
NixOS seemed like everything I wanted.
Everything was defined from config files,
it made adding services easy, and had
proper segmentation of services without needing docker.

It was quite a great experience. I basically got immich running with just:

```nix
services.immich.enable = true;
```

I could secure the machine easily, add anything I wanted with ease,
rollback when things broke, add automatic updating, and more.

But I soon encountered that NixOS is double-edged sword:
 + Services work great when their popular. Their nixpkgs are maintained and constantly upgraded
 + Unpopular services have either no nixpkg, it's out of date, or its #link-new-tab("completely broken", "https://github.com/NixOS/nixpkgs/issues/438433")

I do love self-hosting, but I can't always dedicate that much time to it.
This means that I am just simply not very good at writing Nix derivations
for services I want to run. It just makes the experience
pretty horrible sometimes.

Even with its faults, I still choose to host most of my of services on NixOS.

= Helios
I am planning to apply to #link-new-tab("Oxide", "https://oxide.computer/").
So, I want to familiarize myself with
#link-new-tab("Helios", "https://github.com/oxidecomputer/helios"),
Oxide's OS built on #link-new-tab("illumos", "https://illumos.org/").

Since I am just running NixOS on Flake 2, I have Flake 1 free
to be my experimentation server. I managed to get Helios
running on it with no trouble at all. In fact, I would say it
was easier than Arch linux to setup.

It has only been a great experience learning more about illumos.
I had some trouble getting some things running
(ex. #link-new-tab("my fish shell fork", "https://github.com/LiamSnow/illumos-fish-shell"))
but nothing was too hard.

I have moved my reverse proxy
(#link-new-tab("Caddy", "https://caddyserver.com/docs/quick-starts/reverse-proxy"))
and this website to Helios, with each in their own zone.

= Conclusion
I am super happy I decided to start self-hosting.
It has taught me so many invaluable skills and let
me take back control of my data.

I'm excited for what the future holds for my homelab!

