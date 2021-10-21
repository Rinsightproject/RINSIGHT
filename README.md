# rust-up-to-you

### Environment
Linux （Centos 7 通过）
> curl https://sh.rustup.rs -sSf | sh
> 
> default...
>
>source $HOME/.cargo/env
>
>rustup install nightly 
>
>rustup default nightly
>
>rustup component add rustc-dev llvm-tools-preview

### 运行
> cargo run ~/tikv

you can get json by run
> cargo run ~/tikv -j

run and analyze

or configure scripts/config

./scripts/shell/download-all.sh

scripts usage: ./scripts/README.md  


## Rust Projects (8 domians 36 projects)
### Database: 4
#### tikv
A distributed KV database in Rust    
https://github.com/tikv/tikv  
#### indradb
Rust based graph database  
https://github.com/indradb/indradb.git   
#### Materialize
Streaming SQL database powered by Timely Dataflow   
https://github.com/MaterializeInc/materialize    
#### Noria
Dynamically changing, partially-stateful data-flow for web application backends.   
https://github.com/mit-pdos/noria  


### Operating System: 3
#### redox-os
Redox is an operating system written in Rust, a language with focus on safety and high performance.    
https://github.com/redox-os/kernel  
#### tock-os
A secure embedded operating system for Cortex-M based microcontrollers.   
https://github.com/tock/tock
#### nebulet
A microkernel that implements a WebAssembly "usermode" that runs in Ring 0.   
https://github.com/nebulet/nebulet  


### Gaming: 4
#### rust-doom 
A renderer for Doom, may progress to being a playable game.   
https://github.com/cristicbz/rust-doom  

#### citybound
Citybound is a city building game with a focus on realism, collaborative planning and simulation of microscopic details.    
https://github.com/citybound/citybound  

#### Veloren
Veloren is a multiplayer voxel RPG written in Rust.   
https://github.com/veloren/veloren  

#### Zemeroth
Zemeroth is a turn-based hexagonal tactical game written in Rust.  
https://github.com/ozkriff/zemeroth   

### Image Processing: 3
#### resvg
An SVG rendering library.  
https://github.com/RazrFalcon/resvg  

#### svgbob
converts ASCII diagrams into SVG graphics  
https://github.com/ivanceras/svgbob  

#### svgcleaner
tidies SVG graphics  
https://github.com/RazrFalcon/svgcleaner  

### Cryptocurrencies: 5
#### Grin
Evolution of the MimbleWimble protocol.   
https://github.com/mimblewimble/grin/  

#### diem
Diem’s mission is to enable a simple global currency and financial infrastructure that empowers billions of people.  
https://github.com/diem/diem  

#### polkadot
Heterogeneous multi‑chain technology with pooled security  
https://github.com/paritytech/polkadot  

#### substrate
Generic modular blockchain template written in Rust  
https://github.com/paritytech/substrate  

#### zcash
Zcash is an implementation of the "Zerocash" protocol  
https://github.com/zcash/zcash  

### Security tools: 4
#### RustScan
Make Nmap faster with this port scanning tool  
https://github.com/RustScan/RustScan  

#### sniffglue
A secure multithreaded packet sniffer  
https://github.com/kpcyrd/sniffglue  

#### feroxbuster
A simple, fast, recursive content discovery tool written in Rust  
https://github.com/epi052/feroxbuster  

#### sn0int
A semi-automatic OSINT framework and package manager  
https://github.com/kpcyrd/sn0int  

### System Tools: 7
#### zoxide
A fast alternative to cd that learns your habits  
https://github.com/ajeetdsouza/zoxide  

#### bandwhich
Terminal bandwidth utilization tool  
https://github.com/imsnif/bandwhich  

#### dust
A more intuitive version of du  
https://github.com/bootandy/dust  

#### fselect
Find files with SQL-like queries  
https://github.com/jhspetersson/fselect  

#### gitui
Blazing fast terminal client for git written in Rust.  
https://github.com/extrawurst/gitui  

#### lsd
An ls with a lot of pretty colors and awesome icons  
https://github.com/Peltoche/lsd  

#### exa
A replacement for 'ls'  
https://github.com/ogham/exa  


### Web: 6
#### zola
A fast static site generator in a single binary with everything built-in.  
https://github.com/getzola/zola  
#### deno
A secure JavaScript/TypeScript runtime built with V8, Rust, and Tokio  
https://github.com/denoland/deno  
#### servo
A prototype web browser engine  
https://github.com/servo/servo  
#### actix-web
Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust  
https://github.com/actix/actix-web  
#### hyper
A fast and correct HTTP implementation for Rust.  
https://github.com/hyperium/hyper  
  
#### Rocket
Rocket is an async web framework for Rust with a focus on usability, security, extensibility, and speed.  
https://github.com/SergioBenitez/Rocket  


