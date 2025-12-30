# 1. Vorbereitungen
$projectRoot = Get-Item "$PSScriptRoot\.."
Set-Location $projectRoot.FullName

$distDir = "dist"
$zipName = "RoadToRiches_Release.zip"
# Dies ist der Ordnername, den der User nach dem Entpacken sieht
$folderInsideZip = "RoadToRiches"
$tempZipDir = "temp_zip_build"

Write-Host "--- Starte Build-Prozess (Release) mit ZIP-Archivierung ---" -ForegroundColor Cyan

# 2. Release Build ausführen
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Fehler beim Kompilieren!" -ForegroundColor Red
    exit
}

# 3. Alte Verzeichnisse aufräumen
if (Test-Path $distDir) { Remove-Item -Recurse -Force $distDir }
if (Test-Path $tempZipDir) { Remove-Item -Recurse -Force $tempZipDir }
if (Test-Path $zipName) { Remove-Item $zipName }

# 4. Struktur für das ZIP aufbauen
# Wir erstellen: temp_zip_build/RoadToRiches/assets
New-Item -ItemType Directory -Path "$tempZipDir\$folderInsideZip\assets"

# 5. Dateien kopieren
Write-Host "Kopiere Dateien..." -ForegroundColor Yellow

# EXE kopieren
Copy-Item "target\release\roadtoriches.exe" -Destination "$tempZipDir\$folderInsideZip\"

# Assets kopieren
if (Test-Path "assets\german.json") { 
    Copy-Item "assets\german.json" -Destination "$tempZipDir\$folderInsideZip\assets\"
}
if (Test-Path "assets\english.json") { 
    Copy-Item "assets\english.json" -Destination "$tempZipDir\$folderInsideZip\assets\"
}

# 6. ZIP erstellen
Write-Host "Erstelle ZIP-Archiv: $zipName..." -ForegroundColor Green
# Wir zippen den Inhalt des temp_zip_build Ordners
Compress-Archive -Path "$tempZipDir\*" -DestinationPath $zipName

# 7. Aufräumen
# Wir verschieben das fertige ZIP in den dist-Ordner und löschen den Temp-Müll
New-Item -ItemType Directory -Path $distDir
Move-Item $zipName -Destination $distDir
Remove-Item -Recurse -Force $tempZipDir

Write-Host "--- Fertig! ---" -ForegroundColor Green
Write-Host "Dein Paket liegt in: $distDir\$zipName"
Write-Host "Wenn der User es entpackt, entsteht der Ordner '$folderInsideZip'."