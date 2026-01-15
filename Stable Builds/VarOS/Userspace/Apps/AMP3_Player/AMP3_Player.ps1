$filename = Read-Host -Prompt "What file would you like to play? (Do not include extension.)"

$fileString = Get-Content -Path "C:\Users\$env:USERNAME\Downloads\VarOS\Userspace\Apps\AMP3_Player\Songs\$filename.amp3" -Raw
$notes = $fileString -split "\n"
foreach ($line in $notes) {
    if ($line.StartsWith("#")) {

    }
    elseif ($line.StartsWith("rest")) {
        $rest = $line -split " "
        $restval = $rest[1] -as [int]
        Start-Sleep -Seconds $restval
    }
    elseif ($line.StartsWith("print; ")) {
        $print = $line -split "; "
        $printstr = $print[1]
        Write-Host $printstr
    }
    else {
        $note = $line -split " "
        $note1 = $note[0] -as [int]
        $note2 = $note[1] -as [int]
        [Console]::Beep($note1, $note2)
    }
}