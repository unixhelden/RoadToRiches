# 1. Vorbereitungen
$distDir = "dist"
$distAssetsDir = "$distDir\assets"
$releaseExe = "target\release\roadtoriches.exe"

Write-Host "--- Starte Build-Prozess (Release) ---" -ForegroundColor Cyan

# 2. Release Build ausführen
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Fehler beim Kompilieren!" -ForegroundColor Red
    exit
}

# 3. Dist-Ordner leeren oder erstellen
if (Test-Path $distDir) {
    Remove-Item -Recurse -Force $distDir
}
New-Item -ItemType Directory -Path $distDir
# Wir erstellen den Assets-Ordner im dist-Verzeichnis
New-Item -ItemType Directory -Path $distAssetsDir

# 4. Dateien kopieren
Write-Host "Kopiere Dateien in den dist-Ordner..." -ForegroundColor Yellow

# Die EXE kopieren (kommt direkt in /dist)
if (Test-Path $releaseExe) {
    Copy-Item $releaseExe -Destination $distDir
}
else {
    Write-Host "EXE nicht gefunden in $releaseExe" -ForegroundColor Red
}

# Die Sprachdateien aus dem lokalen /assets/ Ordner nach /dist/assets/ kopieren
if (Test-Path "assets\german.json") { 
    Copy-Item "assets\german.json" -Destination $distAssetsDir 
    Write-Host "  -> assets\german.json erfolgreich kopiert"
}
else {
    Write-Host "FEHLER: assets\german.json nicht im Quellordner gefunden!" -ForegroundColor Red
}

if (Test-Path "assets\english.json") { 
    Copy-Item "assets\english.json" -Destination $distAssetsDir 
    Write-Host "  -> assets\english.json erfolgreich kopiert"
}
else {
    Write-Host "FEHLER: assets\english.json nicht im Quellordner gefunden!" -ForegroundColor Red
}

Write-Host "--- Fertig! ---" -ForegroundColor Green
Write-Host "Checke jetzt den Ordner: $distDir"