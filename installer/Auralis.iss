; Auralis Desktop - Instalador local para Windows x64
; Basado en el ejecutable de Auralis-Portable previamente compilado por el usuario.
; No descarga programas, no inicia sesion y no solicita permisos administrativos.
; No incluye el indicador portable.txt: la instalacion usa el perfil normal del usuario.

#define MyAppName "Auralis"
#define MyAppVersion "3.1.0"
#define MyAppExe "auralis.exe"

[Setup]
AppId={{A69A9FA1-CE6C-4BFB-9B9A-D54F4C63D008}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName=Auralis Desktop 3.0
AppPublisher=Auralis Desktop (derivado de BitChord PC)
DefaultDirName={localappdata}\Programs\Auralis
DefaultGroupName=Auralis
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
MinVersion=10.0
OutputDir=..\Auralis-Instalador
OutputBaseFilename=Auralis-Setup-{#MyAppVersion}
SetupIconFile=Auralis.ico
UninstallDisplayIcon={app}\{#MyAppExe}
LicenseFile=..\LICENSE-GPL-3.0.txt
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "..\Auralis-Portable\auralis.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\Auralis-Portable\WebView2Loader.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "Auralis.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE-GPL-3.0.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\AURALIS.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "CREDITOS.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Auralis"; Filename: "{app}\{#MyAppExe}"; IconFilename: "{app}\Auralis.ico"
Name: "{autoprograms}\Desinstalar Auralis"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Auralis"; Filename: "{app}\{#MyAppExe}"; IconFilename: "{app}\Auralis.ico"; Tasks: desktopicon

[Registry]
; Abre los enlaces auralis:// con el programa local, para usuarios que activen esa funcion.
Root: HKCU; Subkey: "Software\Classes\auralis"; ValueType: string; ValueName: ""; ValueData: "URL:Auralis Protocol"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\auralis"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\auralis\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExe}"",0"
Root: HKCU; Subkey: "Software\Classes\auralis\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExe}"" ""%1"""

[Run]
Filename: "{app}\{#MyAppExe}"; Description: "{cm:LaunchProgram,{#MyAppName}}"; Flags: nowait postinstall skipifsilent

; No se borran datos personales (favoritos, estadisticas y preferencias) al desinstalar.
