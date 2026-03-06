@echo off
title Croc Transfer
cd /d "%~dp0"
set "PY=%LOCALAPPDATA%\Programs\Python\Python313\python.exe"

if exist "%PY%" (
    "%PY%" main.py
) else (
    python main.py
)

if errorlevel 1 (
    echo.
    echo Something went wrong. Make sure pywebview is installed:
    echo   pip install pywebview
    pause
)
