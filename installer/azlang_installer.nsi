!define APP_NAME "AzLang"
!define APP_VERSION "0.1.0"
!define APP_EXE "azlang.exe"

OutFile "AzLang_Setup.exe"
InstallDir "$PROGRAMFILES\AzLang"
Name "${APP_NAME} ${APP_VERSION}"

!include "MUI2.nsh"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "Visual Studio Runtime"
    SetOutPath "$TEMP"
    File "vc_redist.x64.exe"
    DetailPrint "Installing Visual C++ Redistributable..."
    ExecWait '"$TEMP\vc_redist.x64.exe" /quiet /norestart'
    Delete "$TEMP\vc_redist.x64.exe"
SectionEnd

Section "Install"
    SetOutPath "$INSTDIR"
    File "azlang.exe"

    CreateShortcut "$SMPROGRAMS\AzLang.lnk" "$INSTDIR\azlang.exe"

    WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\azlang.exe"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir "$INSTDIR"
SectionEnd
