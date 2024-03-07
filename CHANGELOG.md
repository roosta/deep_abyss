# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.7.0](https://github.com/roosta/deep_abyss/compare/0.6.0..0.7.0) - 2024-03-07
#### Bug Fixes
- clamp aspect ratio - ([0393b3a](https://github.com/roosta/deep_abyss/commit/0393b3a9bc059271a86204aa2be268d91986e0c4)) - [@roosta](https://github.com/roosta)
- reposition camera - ([7472b8c](https://github.com/roosta/deep_abyss/commit/7472b8c2aabf2b2f6ad8d07f2b0e0e5f6b1992ed)) - [@roosta](https://github.com/roosta)
- apply drag always - ([aa14230](https://github.com/roosta/deep_abyss/commit/aa142300e090180cff7a15711266a6c37eb7a253)) - [@roosta](https://github.com/roosta)
#### Features
- **(lvl)** increase width - ([73bf0bf](https://github.com/roosta/deep_abyss/commit/73bf0bf1f3c4d7d85ad477c521dda6e3ff726d0b)) - [@roosta](https://github.com/roosta)
- **(lvl)** increase height, more tiles - ([db23a60](https://github.com/roosta/deep_abyss/commit/db23a60811a133117a25baef0cdda70c1c47a2e5)) - [@roosta](https://github.com/roosta)
- clamp camera to level edges - ([59b43ef](https://github.com/roosta/deep_abyss/commit/59b43eff9ce1a7bf3ae2ec1663087cff18aed2cb)) - [@roosta](https://github.com/roosta)
- center camera on player - ([0d2d80f](https://github.com/roosta/deep_abyss/commit/0d2d80f583725f589c9ace3d9ccfe784eba420e1)) - [@roosta](https://github.com/roosta)
- enable slowdown and speedup - ([2b62408](https://github.com/roosta/deep_abyss/commit/2b62408c0a3ae075cd0efbe3ae2ad8aa2804fbc5)) - [@roosta](https://github.com/roosta)
#### Miscellaneous Chores
- **(fmt)** format camera.rs - ([35f6a2b](https://github.com/roosta/deep_abyss/commit/35f6a2bef2043fcef59741e4258a702d3e02f58a)) - [@roosta](https://github.com/roosta)
- **(fmt)** format player.rs - ([e2c54c4](https://github.com/roosta/deep_abyss/commit/e2c54c468222d6261d2fe8380aff9ffe2aa3e756)) - [@roosta](https://github.com/roosta)
#### Refactoring
- setup camera bundle - ([dee1179](https://github.com/roosta/deep_abyss/commit/dee1179b72dce8210c8c4a46d500e2acec08c075)) - [@roosta](https://github.com/roosta)
- define dimension consts - ([79138c0](https://github.com/roosta/deep_abyss/commit/79138c047c5d899681ca69437e196d5ccb0dd830)) - [@roosta](https://github.com/roosta)
- move player related camera its own fn - ([0cb68e9](https://github.com/roosta/deep_abyss/commit/0cb68e9bcd67b92557a33a61e7a03a600d3b34c9)) - [@roosta](https://github.com/roosta)
- move camera to plugin - ([202d485](https://github.com/roosta/deep_abyss/commit/202d485bbde28b2d1771895ba9de8bfc90195ee7)) - [@roosta](https://github.com/roosta)

- - -

## [0.6.0](https://github.com/roosta/deep_abyss/compare/0.5.0..0.6.0) - 2024-02-08
#### Bug Fixes
- **(dev)** disable missing value warning - ([9f0c1b1](https://github.com/roosta/deep_abyss/commit/9f0c1b16cb673269da68e45d2f18f36d9730da2b)) - [@roosta](https://github.com/roosta)
- chain player updates - ([a738d16](https://github.com/roosta/deep_abyss/commit/a738d161b81a2cfd82f94b53975cab21993f2342)) - [@roosta](https://github.com/roosta)
- on ground toggle - ([1f0dd48](https://github.com/roosta/deep_abyss/commit/1f0dd4814e41fb68a2fd95c5b0f7d33754ac65c4)) - [@roosta](https://github.com/roosta)
- set velocity.y to 0 when on ground - ([2f28615](https://github.com/roosta/deep_abyss/commit/2f28615a0b89570ce7602e7d14dced200e538125)) - [@roosta](https://github.com/roosta)
#### Documentation
- update debug docstrings - ([23ed1c3](https://github.com/roosta/deep_abyss/commit/23ed1c389bb5bdef9f7709b84e7fc8450bba5500)) - [@roosta](https://github.com/roosta)
#### Features
- **(dev)** add FPS counter - ([8d2bdee](https://github.com/roosta/deep_abyss/commit/8d2bdeecbc66a2bac4594e46754d2f386a465cb7)) - [@roosta](https://github.com/roosta)
- **(dev)** enable toggling collision rects - ([174cf6f](https://github.com/roosta/deep_abyss/commit/174cf6fb5ddecd86af1ef5b088cc511e2090cd31)) - [@roosta](https://github.com/roosta)
- **(lvl)** move player start - ([17a1662](https://github.com/roosta/deep_abyss/commit/17a16628755eb0f99be03b03348e79e44a03371e)) - [@roosta](https://github.com/roosta)
- swap physics values based on ground state - ([8d7b799](https://github.com/roosta/deep_abyss/commit/8d7b799a2d9f3aad537ea1502f5a0ef9e6e8eb9e)) - [@roosta](https://github.com/roosta)
- add player ground check - ([6afcc75](https://github.com/roosta/deep_abyss/commit/6afcc7563df24e13f99f057f17a7054f18d6362b)) - [@roosta](https://github.com/roosta)
- apply drag + adjust movement consts - ([d9c66e4](https://github.com/roosta/deep_abyss/commit/d9c66e44437608c7a1deed7163d399ceb456c6bc)) - [@roosta](https://github.com/roosta)
- add gravity - ([3dbd0ed](https://github.com/roosta/deep_abyss/commit/3dbd0edf6b908e4d2d4852e740199fd2c7e7cf0e)) - [@roosta](https://github.com/roosta)
#### Miscellaneous Chores
- **(fmt)** rust format on src files - ([36b20b4](https://github.com/roosta/deep_abyss/commit/36b20b445b860d6f65f423eca76b544f448f69f7)) - [@roosta](https://github.com/roosta)
#### Refactoring
- move consts to structs - ([58038a2](https://github.com/roosta/deep_abyss/commit/58038a217aeb05757c33e41ef41ccdf28d67661a)) - [@roosta](https://github.com/roosta)

- - -

## [0.5.0](https://github.com/roosta/deep_abyss/compare/0.4.0..0.5.0) - 2024-02-04
#### Documentation
- update spawn_collisions docstring - ([29bd72f](https://github.com/roosta/deep_abyss/commit/29bd72fb6056ceb5a806f3f78fdaf6f9af5f9cd2)) - [@roosta](https://github.com/roosta)
#### Features
- **(dev)** enable asset hot reloading - ([9083c70](https://github.com/roosta/deep_abyss/commit/9083c705c07bea600dee35c25a3be6dc36038608)) - [@roosta](https://github.com/roosta)
- **(dev)** create custom debug UI - ([2349600](https://github.com/roosta/deep_abyss/commit/2349600b1b8f07306db1ba8c5310f2bc50c85e47)) - [@roosta](https://github.com/roosta)
- add ZIndex debug resource - ([1e6452a](https://github.com/roosta/deep_abyss/commit/1e6452af7a52c234ef69ce530d242e15c9f2dc18)) - [@roosta](https://github.com/roosta)
- add player map wall collision handling - ([45b646c](https://github.com/roosta/deep_abyss/commit/45b646c12f1a55e22fa8ebb7ae3afbcbb82f42ef)) - [@roosta](https://github.com/roosta)
- use random color for each collision block - ([511bae7](https://github.com/roosta/deep_abyss/commit/511bae7b6a590907358ca950d8a0345607a83cf1)) - [@roosta](https://github.com/roosta)
- add rand dependency - ([f225b37](https://github.com/roosta/deep_abyss/commit/f225b37118afed0e2e23ca3e7a8bc956cb06d74f)) - [@roosta](https://github.com/roosta)
- render collision grid - ([c2cfae9](https://github.com/roosta/deep_abyss/commit/c2cfae97af00095c9bffd513d32581a489b10a60)) - [@roosta](https://github.com/roosta)
#### Miscellaneous Chores
- upgrade tilemap libs & LDtk project - ([535ce96](https://github.com/roosta/deep_abyss/commit/535ce96f07e90107a057587cc4a780c4f92a6021)) - [@roosta](https://github.com/roosta)
#### Refactoring
- move debug code to new module - ([70630c8](https://github.com/roosta/deep_abyss/commit/70630c880343a1b919046c90ea9302e76425ad21)) - [@roosta](https://github.com/roosta)
- change player & tilemap to plugins - ([562bd5a](https://github.com/roosta/deep_abyss/commit/562bd5a09dae548194b9310e9061a4a4ff442654)) - [@roosta](https://github.com/roosta)

- - -

## [0.4.0](https://github.com/roosta/deep_abyss/compare/0.3.0..0.4.0) - 2024-01-16
#### Bug Fixes
- **(lvl)** remove copypaste tutorial description - ([e78cd01](https://github.com/roosta/deep_abyss/commit/e78cd010a24ff335c049b3cb8e8a1236b3db8c59)) - [@roosta](https://github.com/roosta)
- **(lvl)** downgrade ldtk to 1.4.1 - ([5889aae](https://github.com/roosta/deep_abyss/commit/5889aaedaab5a2ee23626b8259690f03b46b3df8)) - [@roosta](https://github.com/roosta)
- fix camera pos - ([2db2718](https://github.com/roosta/deep_abyss/commit/2db271868d21e2ee3b80e4e7e1a8577a02fe8961)) - [@roosta](https://github.com/roosta)
#### Documentation
- add LDtk + asset note - ([a9c7a82](https://github.com/roosta/deep_abyss/commit/a9c7a82c547ddf16333411022648608e0f914c54)) - [@roosta](https://github.com/roosta)
#### Features
- **(lfs)** add placeholder spritesheet - ([386b707](https://github.com/roosta/deep_abyss/commit/386b707a43190435789faa70e8fa75621cb959a6)) - [@roosta](https://github.com/roosta)
- **(lfs)** add temporary atlas files - ([6d1c6eb](https://github.com/roosta/deep_abyss/commit/6d1c6eb7b4af585a2ec10cdd89e2d5b929690e45)) - [@roosta](https://github.com/roosta)
- **(lfs)** resize player to 16x16 - ([613de89](https://github.com/roosta/deep_abyss/commit/613de894dd9c5e4cc5683039c00bb89ce8049b41)) - [@roosta](https://github.com/roosta)
- **(lvl)** change identifier, world layout - ([27d7048](https://github.com/roosta/deep_abyss/commit/27d7048a5d87c0841bbc2e99474de56effd03e55)) - [@roosta](https://github.com/roosta)
- **(lvl)** add player entity - ([9a4f991](https://github.com/roosta/deep_abyss/commit/9a4f991e45dada35995894b40a76ac5c3ea5fd39)) - [@roosta](https://github.com/roosta)
- **(lvl)** add initial project ldtk file - ([1f73420](https://github.com/roosta/deep_abyss/commit/1f734209e4e13718143756616c43ac040b50f460)) - [@roosta](https://github.com/roosta)
- load player from level data - ([d212f82](https://github.com/roosta/deep_abyss/commit/d212f82919845c4b4fd4a579697b8ffb2e169a2b)) - [@roosta](https://github.com/roosta)
- setup ldtk - ([d0dfce1](https://github.com/roosta/deep_abyss/commit/d0dfce138a3fd76069f7b4ca3f2cffd8727afb10)) - [@roosta](https://github.com/roosta)

- - -

## [0.3.0](https://github.com/roosta/deep_abyss/compare/0.2.2..0.3.0) - 2024-01-14
#### Continuous Integration
- use PAT for checkout - ([860b966](https://github.com/roosta/deep_abyss/commit/860b9668b2963e29d1d495e2cf80a372de71cef8)) - [@roosta](https://github.com/roosta)
- update step names - ([d4f97e1](https://github.com/roosta/deep_abyss/commit/d4f97e116b0c65c79c709d083d3673e933a1575d)) - [@roosta](https://github.com/roosta)
- fix release syntax error - ([c35ca3a](https://github.com/roosta/deep_abyss/commit/c35ca3ad2b79937eb2e5e9197e7726298832dfb5)) - [@roosta](https://github.com/roosta)
- set names for workflows - ([aed5d73](https://github.com/roosta/deep_abyss/commit/aed5d73e70e3e4cd53c74990fd68cdb66566879a)) - [@roosta](https://github.com/roosta)
- separate release/build, add check - ([f43586b](https://github.com/roosta/deep_abyss/commit/f43586b706a5ddc568f249e2de84d3f0af17b3f2)) - [@roosta](https://github.com/roosta)
#### Features
- set filtering to nearest - ([0d653c8](https://github.com/roosta/deep_abyss/commit/0d653c8c35696a831c38df93812d32b70ffa91dc)) - [@roosta](https://github.com/roosta)
- - -

## [0.2.2](https://github.com/roosta/deep_abyss/compare/1a6719c1d75238091ab0c80c58d67d4bcc20e520..0.2.2) - 2024-01-13
#### Bug Fixes
- clamp player position - ([76ba55d](https://github.com/roosta/deep_abyss/commit/76ba55d8c137f2a9cea817720897d10c6838c2b6)) - [@roosta](https://github.com/roosta)
#### Continuous Integration
- remove non-working events - ([015548b](https://github.com/roosta/deep_abyss/commit/015548ba6430fc59f3474cefc16ec0e71ae1cbc0)) - [@roosta](https://github.com/roosta)
- only release from main - ([528bf72](https://github.com/roosta/deep_abyss/commit/528bf72b76416b5f1bb00bfef864ce899860788e)) - [@roosta](https://github.com/roosta)
- enable generating release notes - ([63983eb](https://github.com/roosta/deep_abyss/commit/63983eb8c4b16a48cf46df4dd84814847bd6e3bb)) - [@roosta](https://github.com/roosta)
- rearrange jobs - ([a012f25](https://github.com/roosta/deep_abyss/commit/a012f2582623bbce666bc0700d49739c1f2cfb8d)) - [@roosta](https://github.com/roosta)
- release workflow - ([296fb5b](https://github.com/roosta/deep_abyss/commit/296fb5bbbee7d4ab7e93963b1dbcb24aabc8ec2e)) - [@roosta](https://github.com/roosta)
#### Documentation
- add cog config - ([b729711](https://github.com/roosta/deep_abyss/commit/b72971172ac95c0601f682f3e50fdfe47a860716)) - [@roosta](https://github.com/roosta)
- remove old design sketch - ([8ce31f6](https://github.com/roosta/deep_abyss/commit/8ce31f640c903b8b10c74092eaf8c4eec3f27817)) - [@roosta](https://github.com/roosta)
- add dependency notice + link - ([793dd56](https://github.com/roosta/deep_abyss/commit/793dd5622d7d4e2eda2c925193ffef18dfa12c4c)) - [@roosta](https://github.com/roosta)
- add some initial details - ([df11f0b](https://github.com/roosta/deep_abyss/commit/df11f0b318db9aa3c25a5fe074d738b58d96ba84)) - [@roosta](https://github.com/roosta)
- update license and readme - ([fc56a48](https://github.com/roosta/deep_abyss/commit/fc56a48948bd0aaa3efb8ca1fc8d1705e81970e4)) - [@roosta](https://github.com/roosta)
#### Features
- add inspector - ([f162400](https://github.com/roosta/deep_abyss/commit/f16240016d0acd592e4b235b89f349423da9bd52)) - [@roosta](https://github.com/roosta)
- setup basic movement - ([e697ff9](https://github.com/roosta/deep_abyss/commit/e697ff9c6596c7d8dddd04c3c2877ef5b299d2f8)) - [@roosta](https://github.com/roosta)
- rm tutorial code, load placeholder sprite - ([593aba5](https://github.com/roosta/deep_abyss/commit/593aba59499a57fd85dd687c08cf6e9ce220ea4e)) - [@roosta](https://github.com/roosta)
- add bevy book example code - ([c7d04ef](https://github.com/roosta/deep_abyss/commit/c7d04efc4a6426012f74ad4e595c18f79660e4b2)) - [@roosta](https://github.com/roosta)
- setup bevy - ([f6cae29](https://github.com/roosta/deep_abyss/commit/f6cae293750498bd9bf341dfdc5ff22c8af0d732)) - [@roosta](https://github.com/roosta)
#### Miscellaneous Chores
- bump version to v0.2.2 - ([b271eb3](https://github.com/roosta/deep_abyss/commit/b271eb3fb1bcaf868e6ac991d23d901cb3fab708)) - [@roosta](https://github.com/roosta)
- bump version to v0.2.1 - ([4e6f81b](https://github.com/roosta/deep_abyss/commit/4e6f81b299403373d103bb045c28637240068da2)) - [@roosta](https://github.com/roosta)
- bump version - ([e9d6b24](https://github.com/roosta/deep_abyss/commit/e9d6b24047bfb929238f09f0feddc6f6ebd2a663)) - [@roosta](https://github.com/roosta)
- move player placeholder - ([9a1010a](https://github.com/roosta/deep_abyss/commit/9a1010a1615dd7612d59502e476345f4a123ea16)) - [@roosta](https://github.com/roosta)
- setup LFS - ([4f8a1de](https://github.com/roosta/deep_abyss/commit/4f8a1de387261e9b192d2b9ed2a6a0cb1e7e4a72)) - [@roosta](https://github.com/roosta)
#### Refactoring
- remove unneeded bounds - ([b59db56](https://github.com/roosta/deep_abyss/commit/b59db56d8c10b7b04fa312ec760992dfaa8db809)) - [@roosta](https://github.com/roosta)
- store delta in vars - ([a2640ea](https://github.com/roosta/deep_abyss/commit/a2640ea1316980a8f9c2bc06b6a56f9b51381c07)) - [@roosta](https://github.com/roosta)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).