; NSIS installer for 优道翻译
; Generated manually to support custom install directory on systems without Tauri NSIS toolchain download.

Unicode true
ManifestDPIAware true

!define PRODUCT_NAME "优道翻译"
!define PRODUCT_VERSION "1.0.4"
!define PRODUCT_PUBLISHER "优道"
!define PRODUCT_WEB_SITE ""
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\app.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKCU"
!define PRODUCT_STARTMENU_REGVAL "NSIS:StartMenuDir"

SetCompressor lzma
RequestExecutionLevel user

; MUI 2
!include "MUI2.nsh"

; Variables
Var StartMenuFolder

; Interface settings
!define MUI_ABORTWARNING
!define MUI_ICON "..\icons\icon.ico"
!define MUI_UNICON "..\icons\icon.ico"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_STARTMENU Application $StartMenuFolder
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "..\..\dist-installer\YouDaoTranslate_${PRODUCT_VERSION}_setup.exe"
InstallDir "$LOCALAPPDATA\Programs\YouDaoTranslate"
InstallDirRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

; Check WebView2 and install if missing (for Windows 10 compatibility)
Section "-WebView2"
  DetailPrint "正在检查 WebView2 运行时..."
  Push $0
  ReadRegStr $0 HKLM "SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" "pv"
  StrCmp $0 "" install_webview2 done_webview2
  install_webview2:
    DetailPrint "未检测到 WebView2，正在静默安装..."
    SetDetailsPrint listonly
    !define WEBVIEW2_URL "https://go.microsoft.com/fwlink/p/?LinkId=2124703"
    NSISdl::download /TIMEOUT=30000 "${WEBVIEW2_URL}" "$TEMP\MicrosoftEdgeWebview2Setup.exe"
    Pop $0
    StrCmp $0 "success" download_ok download_failed
    download_ok:
      ExecWait '"$TEMP\MicrosoftEdgeWebview2Setup.exe" /silent /install' $0
      Delete "$TEMP\MicrosoftEdgeWebview2Setup.exe"
      DetailPrint "WebView2 安装完成"
      Goto done_webview2
    download_failed:
      DetailPrint "WebView2 下载失败，请手动安装 https://developer.microsoft.com/microsoft-edge/webview2/"
  done_webview2:
    SetDetailsPrint both
    Pop $0
SectionEnd

Section "MainProgram" SEC01
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
  File "..\target\release\app.exe"

  ; Dictionary database
  SetOutPath "$INSTDIR\models\dict"
  File "..\..\models\dict\ecdict.db"
  SetOutPath "$INSTDIR"

  ; Create start menu shortcuts
  !insertmacro MUI_STARTMENU_WRITE_BEGIN Application
  CreateDirectory "$SMPROGRAMS\$StartMenuFolder"
  CreateShortcut "$SMPROGRAMS\$StartMenuFolder\优道翻译.lnk" "$INSTDIR\app.exe"
  CreateShortcut "$SMPROGRAMS\$StartMenuFolder\卸载 优道翻译.lnk" "$INSTDIR\uninst.exe"
  !insertmacro MUI_STARTMENU_WRITE_END

  ; Create desktop shortcut
  CreateShortcut "$DESKTOP\优道翻译.lnk" "$INSTDIR\app.exe"
SectionEnd

Section -Post
  WriteUninstaller "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\app.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\app.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "StartMenuFolder" "$StartMenuFolder"
SectionEnd

Section Uninstall
  ReadRegStr $StartMenuFolder ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "StartMenuFolder"

  Delete "$INSTDIR\uninst.exe"
  Delete "$INSTDIR\app.exe"
  Delete "$INSTDIR\models\dict\ecdict.db"
  RMDir "$INSTDIR\models\dict"
  RMDir "$INSTDIR\models"

  Delete "$SMPROGRAMS\$StartMenuFolder\优道翻译.lnk"
  Delete "$SMPROGRAMS\$StartMenuFolder\卸载 优道翻译.lnk"
  RMDir "$SMPROGRAMS\$StartMenuFolder"

  Delete "$DESKTOP\优道翻译.lnk"

  RMDir "$INSTDIR"

  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_DIR_REGKEY}"
  SetAutoClose true
SectionEnd
