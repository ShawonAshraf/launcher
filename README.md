# launcher

> A barebones application launcher for Windows, built with Tauri + React.


## Should you use it?
Probably no. I happen to have a lot of legacy applications that I need to run on Windows, and I wanted a simple way to launch them. 
It's not pretty, but it works for my use-case.

## Build Instructions

```bash
npm install

# for dev
npm run tauri dev

# build
npm run tauri build
```

The built executable will be in the `src-tauri/target/release` directory.