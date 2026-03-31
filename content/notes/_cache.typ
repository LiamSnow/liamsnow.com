#metadata((
  title: "CPU Cache",
  desc: "TODO",
  homepage: false,
)) <page>

#import "../_shared/template.typ": post, link-new-tab 
#show: post

*Locality*:
- RAM + L3 are shared between cores
- L1 + L2 are core-local

*Cache Lines*: Cache operates on cache lines (chunks of RAM, usually 64-byte) not on every possible address in RAM

*Cache Coherence Protocol* negotiates between caches to ensure validity
 - In most cases, its *MESI* (modify, exclusive, shared, invalid) or MESI-like

