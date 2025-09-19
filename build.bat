@echo off
setlocal

REM --- Get script directory (project root) ---
set "ROOT_DIR=%~dp0"
set "ROOT_DIR=%ROOT_DIR:~0,-1%"  REM remove trailing backslash

set "CURRENT_DIR=%cd%"

REM --- Check if running from project root ---
if /I not "%CURRENT_DIR%"=="%ROOT_DIR%" (
    echo Please run build script from project root
    exit /b 1
)

REM --- Clean up existing Rust artifacts ---
cd lib
cargo clean
cd ..

REM --- Create fresh build directory ---
if exist build (
    rmdir /s /q build
)
mkdir build

cd build

REM --- Run CMake ---
cmake ..
cmake --build .

cd ..
endlocal
