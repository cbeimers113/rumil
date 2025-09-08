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

REM --- Create build directory ---
if not exist build (
    mkdir build
)

cd build

REM --- Run CMake ---
cmake ..
cmake --build .

cd ..
endlocal
