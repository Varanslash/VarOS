$fileString = Get-Content -Path "C:\Users\$env:USERNAME\Downloads\beep.amp3" -Raw
$notes = $fileString -split "\n"
foreach ($line in $notes) {
    if ($line.StartsWith("#")) {

    }
    else {
        $note = $line -split " "
        $note1 = $note[0] -as [int]
        $note2 = $note[1] -as [int]
        [Console]::Beep($note1, $note2)
    }
}
