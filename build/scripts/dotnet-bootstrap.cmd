:<<"::CMDLITERAL"
@ECHO off

SETLOCAL

SET SCRIPTS=%~dp0
SET BUILD=%SCRIPTS%..
SET ROOT=%BUILD%\..
SET DOTNET_SDK=%ROOT%\.dotnet\dotnet
SET BOOTSTRAP=%BUILD%\DotNetBootstrap
SET PUBLISH=%BOOTSTRAP%\bin\Bootstrap
SET EXECUTABLE=%PUBLISH%\DotNetBootstrap.dll
SET VERSION_SOURCE=%BOOTSTRAP%\version.txt
SET VERSION_DEST=%PUBLISH%\version.txt
SET TARGET_SCRIPT_FILE=%tmp%\pragmastat-dotnet-bootstrap-%RANDOM%.cmd

IF NOT EXIST "%EXECUTABLE%" (
    GOTO bootstrap
) ELSE (
    IF NOT EXIST "%VERSION_DEST%" (
        GOTO bootstrap
    ) ELSE (
        fc "%VERSION_SOURCE%" "%VERSION_DEST%" > nul
        IF errorlevel 1 GOTO bootstrap
    )
)
GOTO run

:bootstrap

ECHO Bootstrapping DotNetBootstrap...

PowerShell -NoProfile -NoLogo -ExecutionPolicy unrestricted -Command "[System.Threading.Thread]::CurrentThread.CurrentCulture = ''; [System.Threading.Thread]::CurrentThread.CurrentUICulture = '';& '%SCRIPTS%\dotnet-install.ps1' %*; exit $LASTEXITCODE"

CALL "%DOTNET_SDK%" publish --configuration Release --output "%PUBLISH%" "%BOOTSTRAP%\DotNetBootstrap.csproj" /p:PublishReadyToRun="true"
IF %ERRORLEVEL% NEQ 0 (Echo [dotnet-bootstrap.cmd] ERROR: Failed to publish DotNetBootstrap 1>&2 &Exit /b 1)

COPY /Y "%VERSION_SOURCE%" "%VERSION_DEST%"
IF %ERRORLEVEL% NEQ 0 (Echo [dotnet-bootstrap.cmd] ERROR: Failed to copy the DotNetBootstrap version marker 1>&2 &Exit /b 1)

:run

CALL "%DOTNET_SDK%" "%EXECUTABLE%" --target-script-file "%TARGET_SCRIPT_FILE%" %*
IF %ERRORLEVEL% NEQ 0 (Echo [dotnet-bootstrap.cmd] ERROR: Failed to execute DotNetBootstrap 1>&2 &Exit /b 1)

CALL "%TARGET_SCRIPT_FILE%"
IF %ERRORLEVEL% NEQ 0 (Echo [dotnet-bootstrap.cmd] ERROR: Failed to execute the bootstrapped application 1>&2 &Exit /b 1)

ENDLOCAL

:end

@GOTO :EOF
::CMDLITERAL

set -u

die () {
    echo
    echo "\033[0;31m[dotnet-bootstrap.cmd] ERROR: $*\033[0m" >&2
    echo
    exit 1
}

die_if_error () {
    if [ $? -ne 0 ]; then
        die "$*"
    fi
}

if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
    die "Please install wget or curl (it's needed to download dotnet-sdk so that we can compile command-line tools)"
fi

SCRIPTS="$(cd "$(dirname "$0")"; pwd)"
die_if_error "Failed to cd to the script directory"
BUILD="$SCRIPTS/.."
ROOT="$BUILD/.."
DOTNET_SDK="$ROOT/.dotnet/dotnet"
BOOTSTRAP="$BUILD/DotNetBootstrap"
PUBLISH="$BOOTSTRAP/bin/Bootstrap-$(uname -m)"
die_if_error "Failed to execute 'uname -m'"
EXECUTABLE="$PUBLISH/DotNetBootstrap.dll"
VERSION_SOURCE="$BOOTSTRAP/version.txt"
VERSION_DEST="$PUBLISH/version.txt"
TARGET_SCRIPT_FILE="$(mktemp).sh"
die_if_error "Failed to create a temporary file for the target script file"

"$SCRIPTS/dotnet-install.sh"

if [ ! -e "$EXECUTABLE" ] || [ ! -e "$VERSION_DEST" ] || ! cmp -s "$VERSION_SOURCE" "$VERSION_DEST"; then
    echo "Bootstrapping DotNetBootsrap..."

    "$DOTNET_SDK" publish --configuration Release --output "$PUBLISH" "$BOOTSTRAP/DotNetBootstrap.csproj" /p:PublishReadyToRun="true"
    die_if_error "Failed to publish DotNetBootsrap"
  
    cp "$VERSION_SOURCE" "$VERSION_DEST"
    die_if_error "Failed to copy '$VERSION_SOURCE' to '$VERSION_DEST'"

    echo "DotNetBootsrap is successfully bootstrapped"
    echo
fi

"$DOTNET_SDK" "$EXECUTABLE" --target-script-file "$TARGET_SCRIPT_FILE" "$@"
die_if_error "Failed to execute DotNetBootsrap; Arguments: '$@'"

if [ -f "$TARGET_SCRIPT_FILE" ]; then
    chmod +x "$TARGET_SCRIPT_FILE"
    die_if_error "Failed to set +x for '$TARGET_SCRIPT_FILE'"
    exec "$TARGET_SCRIPT_FILE"
else
    die "Failed to execute the bootstrapped application"
fi
