@echo off
REM Release automation script for CommentsCleaner (Batch version)
REM Simple wrapper for PowerShell script

setlocal enabledelayedexpansion

if "%1"=="" (
    echo Usage: release.bat VERSION [OPTIONS]
    echo.
    echo Examples:
    echo   release.bat 0.1.0
    echo   release.bat 0.1.0 dry-run
    echo   release.bat 0.1.0 build-only
    echo   release.bat 0.1.0 publish-cargo
    echo.
    goto :eof
)

set VERSION=%1
set DRY_RUN=
set BUILD_ONLY=
set PUBLISH_CARGO=

:parse_args
if "%2"=="dry-run" (
    set DRY_RUN=-DryRun
    shift
    goto :parse_args
)
if "%2"=="build-only" (
    set BUILD_ONLY=-BuildOnly
    shift
    goto :parse_args
)
if "%2"=="publish-cargo" (
    set PUBLISH_CARGO=-PublishCargo
    shift
    goto :parse_args
)

echo Running release script for version %VERSION%
powershell -ExecutionPolicy Bypass -File "release.ps1" -Version %VERSION% %DRY_RUN% %BUILD_ONLY% %PUBLISH_CARGO%
