Isntructions for getting RustMart to work on windows.
*note steps 1 & 2 are used in both RustMart and Rust-2048 and do not need to be repeated once installed correctly. 

1. Download rustup-init.exe (32 or 64, depending on your OS).
*note you may need to install Visual Studio C++ build tools when prompted to do so.

2. Install and Open Git CMD.

3. Run these commands in Git CMD;
	cargo install wasm-pack
	cargo install cargo-make
	cargo install simple-http-server
*note you may need to manually install wasm-pack from "https://crates.io/crates/wasm-pack".

   When done close this window.

4. Extract rustmart.zip to "C:\Users\(user)\rustmart". 

5. Open 2 new Git CMD windows, in the first one run; 
	cd rustmart
	cargo make build
*note if working correctly you should see green info text.

  And in the second run;
	cd rustmart
	cargo make serve
*note if working correctly you should see blue server info text.

6. Open web browser of your choice (tested in chrome and firefox) and go to "http://localhost:3000/".
7. Try adding things to the cart.

