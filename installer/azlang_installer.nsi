!define APP_NAME "AzLang"
!define APP_VERSION "0.1.0"
!define APP_EXE "azcli.exe"

OutFile "AzLang_Setup.exe"
InstallDir "$PROGRAMFILES\AzLang"
Name "${APP_NAME} ${APP_VERSION}"

RequestExecutionLevel admin

!include "MUI2.nsh"
!include "WinMessages.nsh"

; --- Default MUI pages (English) ---
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File "${APP_EXE}"

    CreateShortcut "$SMPROGRAMS\AzLang.lnk" "$INSTDIR\${APP_EXE}"

    ; Add to PATH if not exists
    ReadRegStr $0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"

    Push "$INSTDIR"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 +3
        WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" "$0;$INSTDIR"
        SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000

    WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\${APP_EXE}"
    Delete "$INSTDIR\Uninstall.exe"
    Delete "$SMPROGRAMS\AzLang.lnk"
    RMDir "$INSTDIR"
SectionEnd

Function StrStr
  Exch $R1
  Exch
  Exch $R2
  Push $R3
  Push $R4
  Push $R5
  StrLen $R3 $R1
  StrCpy $R4 0
  loop:
    StrCpy $R5 $R2 $R3 $R4
    StrCmp $R5 $R1 done
    StrCmp $R5 "" done
    IntOp $R4 $R4 + 1
    Goto loop
  done:
  StrCpy $R1 $R2 "" $R4
  Pop $R5
  Pop $R4
  Pop $R3
  Pop $R2
  Exch $R1
FunctionEnd
