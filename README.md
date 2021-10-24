# RINSIGHT

## Introduction
RINSIGHT is a prototype tool to analyzing how Rust’s language features used in real-world Rust projects.
In our expriment, it was used on 36 open source Rust projects, from 8 domains, with a total 5,108,432 lines of Rust source code.

## Dependencies

### Operating system
Linux （Centos 7）

### Rust toolchain installation
```
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup install nightly
$ rustup default nightly
$ rustup component add rustc-dev llvm-tools-preview
```

## Run examples

run the tikv test.
```
$ cargo run ~/tikv
```

run the tikv test and get json.
```
$ cargo run ~/tikv -j
``` 


## Test Rust Projects (8 domians 36 projects)
### Database: 4
1. [tikv](https://github.com/tikv/tikv):
A distributed KV database in Rust  
2. [indradb](https://github.com/indradb/indradb.git):
Rust based graph database  
3. [Materialize](https://github.com/MaterializeInc/materialize):
Streaming SQL database powered by Timely Dataflow   
4. [Noria](https://github.com/mit-pdos/noria):
Dynamically changing, partially-stateful data-flow for web application backends.   


### Operating System: 3
1. [redox-os](https://github.com/redox-os/kernel):
Redox is an operating system written in Rust, a language with focus on safety and high performance.    
2. [tock-os](https://github.com/tock/tock):
A secure embedded operating system for Cortex-M based microcontrollers.   
3. [nebulet](https://github.com/nebulet/nebulet):
A microkernel that implements a WebAssembly "usermode" that runs in Ring 0.   
  


### Gaming: 4
1. [rust-doom](https://github.com/cristicbz/rust-doom): 
A renderer for Doom, may progress to being a playable game.   
2. [citybound](https://github.com/citybound/citybound):
Citybound is a city building game with a focus on realism, collaborative planning and simulation of microscopic details.    
3. [Veloren](https://github.com/veloren/veloren):
Veloren is a multiplayer voxel RPG written in Rust.   
4. [Zemeroth](https://github.com/ozkriff/zemeroth):
Zemeroth is a turn-based hexagonal tactical game written in Rust.  
   

### Image Processing: 3
1. [resvg](https://github.com/RazrFalcon/resvg):
An SVG rendering library.  
2. [svgbob](https://github.com/ivanceras/svgbob):
converts ASCII diagrams into SVG graphics  
3. [svgcleaner](https://github.com/RazrFalcon/svgcleaner):
tidies SVG graphics  
  

### Cryptocurrencies: 5
1. [Grin](https://github.com/mimblewimble/grin/):
Evolution of the MimbleWimble protocol.   
2. [diem](https://github.com/diem/diem):
Diem’s mission is to enable a simple global currency and financial infrastructure that empowers billions of people.  
3. [polkadot](https://github.com/paritytech/polkadot):
Heterogeneous multi‑chain technology with pooled security  
4. [substrate](https://github.com/paritytech/substrate):
Generic modular blockchain template written in Rust  
5. [zcash](https://github.com/zcash/zcash):
Zcash is an implementation of the "Zerocash" protocol  
  
### Security tools: 4
1. [RustScan](https://github.com/RustScan/RustScan):
Make Nmap faster with this port scanning tool  
2. [sniffglue](https://github.com/kpcyrd/sniffglue):
A secure multithreaded packet sniffer  
3. [feroxbuster](https://github.com/epi052/feroxbuster):
A simple, fast, recursive content discovery tool written in Rust  
4. [sn0int](https://github.com/kpcyrd/sn0int):
A semi-automatic OSINT framework and package manager  
  
### System Tools: 7
1. [zoxide](https://github.com/ajeetdsouza/zoxide):
A fast alternative to cd that learns your habits  
2. [bandwhich](https://github.com/imsnif/bandwhich):
Terminal bandwidth utilization tool  
3. [dust](https://github.com/bootandy/dust):
A more intuitive version of du  
4. [fselect](https://github.com/jhspetersson/fselect):
Find files with SQL-like queries  
5. [gitui](https://github.com/extrawurst/gitui):
Blazing fast terminal client for git written in Rust.  
6. [lsd](https://github.com/Peltoche/lsd):
An ls with a lot of pretty colors and awesome icons  
7. [exa](https://github.com/ogham/exa):
A replacement for 'ls'  

### Web: 6
1. [zola](https://github.com/getzola/zola ):
A fast static site generator in a single binary with everything built-in.  
2. [deno](https://github.com/denoland/deno):
A secure JavaScript/TypeScript runtime built with V8, Rust, and Tokio  
3. [servo](https://github.com/servo/servo):
A prototype web browser engine  
4. [actix-web](https://github.com/actix/actix-web):
Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust  
5. [hyper](https://github.com/hyperium/hyper):
A fast and correct HTTP implementation for Rust.  
6. [Rocket](https://github.com/SergioBenitez/Rocket):
Rocket is an async web framework for Rust with a focus on usability, security, extensibility, and speed.  
  


