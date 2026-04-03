; Add Windows Firewall rule after installation so LanDrop can accept
; incoming LAN connections without the user having to manually allow it.
!macro NSIS_HOOK_POSTINSTALL
  ; Add inbound TCP rule for LanDrop
  nsExec::ExecToLog 'netsh advfirewall firewall add rule name="LanDrop" dir=in action=allow program="$INSTDIR\landrop.exe" protocol=TCP profile=private,public enable=yes'
  ; Add inbound UDP rule for mDNS discovery
  nsExec::ExecToLog 'netsh advfirewall firewall add rule name="LanDrop" dir=in action=allow program="$INSTDIR\landrop.exe" protocol=UDP profile=private,public enable=yes'
!macroend

; Remove firewall rules on uninstall
!macro NSIS_HOOK_PREUNINSTALL
  nsExec::ExecToLog 'netsh advfirewall firewall delete rule name="LanDrop" program="$INSTDIR\landrop.exe"'
!macroend
