@echo off
REM stop.bat — Lanceur pour le script d'arret
powershell -ExecutionPolicy Bypass -File "%~dp0stop.ps1"
pause
