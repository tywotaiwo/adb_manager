[Setup]
AppName=ADB Manager
AppVersion=1.0
WizardStyle=modern
DefaultDirName={autopf}\ADB Manager
DefaultGroupName=ADB Manager
OutputBaseFilename=adb_manager_setup
Compression=lzma2
SolidCompression=yes

[Files]
Source: "target\release\adb_manager.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "platform-tools\*"; DestDir: "{app}\platform-tools"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\ADB Manager"; Filename: "{app}\adb_manager.exe"
Name: "{commondesktop}\ADB Manager"; Filename: "{app}\adb_manager.exe"

[Run]
Filename: "{app}\adb_manager.exe"; Description: "Launch ADB Manager"; Flags: nowait postinstall skipifsilent 