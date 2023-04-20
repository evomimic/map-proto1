# map-proto1
Initial Proof-of-Concept of MAP Layer 0

For the use cases in scope for the initial slice, see [wiki](https://github.com/evomimic/map-proto1/wiki)

## Tech Stack:
* rust 1.66.0 (determined by Nix)
* nix 2.13.2
* holochain 0.1.x
* hdk 0.1.1
* hdi 0.2.1

## Setup:

If not already done before,  Enable Nix commands and Nix flakes for your use:

 ```bash
 mkdir -p ~/.config/nix
 echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
 ```

# cachix caching
Run cachix to cache OS specific holochain crates on your system (do this when you open a terminal to ensure you have updates)

```bash
nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz -p cachix --run "cachix use holochain-ci"
```

## Install and run tests:

 ```bash
 nix develop
 npm install
 npm test / npm run sweetest
 
 ```
