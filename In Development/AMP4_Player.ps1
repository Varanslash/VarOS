$frames = "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "\o\", "|o|", "|o|", "|o|", "|o|", "|o|", "/o/", "/o/", "/o/", "/o/", "/o/"
while ($true) {
    foreach ($frame in $frames) {
        Write-Host "$frame"
        Start-Sleep -Milliseconds 16.7
        Clear-Host
    }
}