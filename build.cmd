:<<"::CMDLITERAL"
@ECHO off

SETLOCAL

SET ROOT=%~dp0
SET SCRIPTS=%ROOT%build\scripts
SET BUILD=%ROOT%build
SET TARGET_SCRIPT_FILE=%TMP%\pragmastat-%RANDOM%.cmd

@CALL "%SCRIPTS%\dotnet-bootstrap.cmd" --sln-dir "%BUILD%\src" "%BUILD%\src\Entry\Entry.csproj" -- %* "--output-script-path=%TARGET_SCRIPT_FILE%"
IF %ERRORLEVEL% NEQ 0 (Echo [build.cmd] ERROR: Failed to run Entry via dotnet-bootstrap 1>&2 &Exit /b 1)

IF EXIST "%TARGET_SCRIPT_FILE%" (
    @CALL "%TARGET_SCRIPT_FILE%"
    IF %ERRORLEVEL% NEQ 0 (Echo [build.cmd] ERROR: Failed to execute the generated Entry script 1>&2 &Exit /b 1)
)

ENDLOCAL

GOTO :EOF
::CMDLITERAL

set -eu

die () {
    echo
    echo "\033[0;31m[build.cmd] ERROR: $*\033[0m" >&2
    echo
    exit 1
}

die_if_error () {
    if [ $? -ne 0 ]; then
        die "$*"
    fi
}

ROOT="$(cd "$(dirname "$0")"; pwd)"
die_if_error "Failed to cd to the script directory"
SCRIPTS="$ROOT/build/scripts"
BUILD="$ROOT/build"
TARGET_SCRIPT_FILE="$(mktemp).sh"
die_if_error "Failed to create a temporary file for the target script file"

"$SCRIPTS/dotnet-bootstrap.cmd" --sln-dir "$BUILD/src" "$BUILD/src/Entry/Entry.csproj" -- "$@" "--output-script-path=$TARGET_SCRIPT_FILE"
die_if_error "Failed to execute the generated Entry script '$TARGET_SCRIPT_FILE'"

if [ -f "$TARGET_SCRIPT_FILE" ]; then
    exec "$TARGET_SCRIPT_FILE"
    die_if_error "Failed to execute the generated Entry script"
fi