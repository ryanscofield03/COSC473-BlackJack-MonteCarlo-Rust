# Blackjack Monte Carlo Sim

This is a small Monto Carlo Simulator for Blackjack odds using Rust to run our simulation and displayed on a Vue.js frontend.
The rest of this README shows the project working, and then describes installation and setup.

## Video Demo

<a href="http://www.youtube.com/watch?v=iJiACsxxViM" title="COSC473 Black Jack Monte Carlo Rust Demo">
  <img src="http://img.youtube.com/vi/iJiACsxxViM/0.jpg" alt="Watch the demo here" style="width: 100%;" />
</a>

## How to Run: 

#### Install Rust

This project requires rust to run. Follow https://www.rust-lang.org/tools/install for help installing rust.

#### Clone the project

Open a terminal and enter the following commands

```
git clone https://github.com/ryanscofield03/COSC473-BlackJack-MonteCarlo-Rust.git
```

```
cd COSC473-BlackJack-MonteCarlo-Rust
```

```
npm install
```

#### Build wasm code

```
cd wasm-module
```

```
cargo install wasm-pack
```

```
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web
```

#### Build vue project

```
cd ..
```

```
npm run build
```

#### Run the project locally
```
npm run preview
```
Open http://localhost:4173/ (or the port given in the terminal). This is where the frontend of this app will be running!  
