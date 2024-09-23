# uki windows installator

# it's such an ugly language
$folderPath = "C:\Program Files (x86)\smokingplaya\uki"
if (-Not (Test-Path -Path $folderPath)) {
  New-Item -Path $folderPath -ItemType Directory
}

# downloading file
$exeUrl = "https://github.com/smokingplaya/uki/releases/latest/download/uki-windows_x86.exe"
$exePath = Join-Path -Path $folderPath -ChildPath "uki.exe"
Invoke-WebRequest -Uri $exeUrl -OutFile $exePath

# add file in path
$currentPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
if (-Not ($currentPath -like "*$folderPath*")) {
  [System.Environment]::SetEnvironmentVariable("Path", $currentPath + ";$folderPath", [System.EnvironmentVariableTarget]::Machine)
}

Write-Host "Installation completed"