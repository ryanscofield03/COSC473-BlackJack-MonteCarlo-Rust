# Blackjack Monte Carlo Sim

## How to Run: 

#### Install Rust

This project requires rust to run. Follow https://www.rust-lang.org/tools/install for help installing rust.

#### Clone the project

Open a terminal and enter the following commands

```
git clone https://eng-git.canterbury.ac.nz/rsc104/COSC473-Assignment1.git
```

```
cd COSC473-Assignment1
```

```
npm install
```

#### Build wasm code

```
cd wasm-module
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
