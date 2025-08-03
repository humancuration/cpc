@echo off

REM Build the WebAssembly package
echo Building WebAssembly package...
wasm-pack build --target web --out-dir static/pkg

REM Serve the static files
echo Serving static files...
echo Open http://localhost:8000/static/index.html in your browser
python -m http.server 8000