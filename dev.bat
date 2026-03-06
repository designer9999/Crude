@echo off
title Croc Transfer [DEV]
cd /d "%~dp0"
set "ROOT=%~dp0"
set "PY=%LOCALAPPDATA%\Programs\Python\Python313\python.exe"

:: ── NPM dependencies ──
if not exist node_modules (
    echo [1/2] Installing npm packages...
    call npm install
) else (
    echo [1/2] npm packages OK
)

:: ── Start Vite dev server ──
echo [2/2] Starting servers...
start "Croc Transfer - Vite" cmd /c "cd /d %ROOT% && npx vite --host"

timeout /t 3 /nobreak >nul

:: ── Launch pywebview (blocks until window closes) ──
echo.
echo  Croc Transfer DEV is running. Close the window to stop.
echo.
if exist "%PY%" (
    "%PY%" main.py --dev
) else (
    python main.py --dev
)

:: ── Cleanup: kill Vite ──
taskkill /FI "WINDOWTITLE eq Croc Transfer - Vite*" /F >nul 2>&1
for /f "tokens=5" %%p in ('netstat -aon ^| findstr ":5173" ^| findstr "LISTENING"') do (
    taskkill /PID %%p /F >nul 2>&1
)
echo Croc Transfer stopped.
