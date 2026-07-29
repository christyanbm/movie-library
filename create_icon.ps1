$w = 16
$h = 16
$bpp = 32
$ms = New-Object System.IO.MemoryStream
$bw = New-Object System.IO.BinaryWriter($ms)

# ICO Header
$bw.Write([uint16]0)
$bw.Write([uint16]1)
$bw.Write([uint16]1)

# Pixel data offset = 6 + 16 = 22
$pixelDataSize = $w * $h * 4
$andMaskSize = 32
$bmpHeaderSize = 40
$imageDataSize = $bmpHeaderSize + $pixelDataSize + $andMaskSize
$dirEntryOffset = 22

# Save position, write placeholder dir entry
$dirPos = $ms.Position
$bw.Write([byte[]]::new(16))

# BMP Info Header
$bw.Write([int32]40)
$bw.Write([int32]$w)
$bw.Write([int32]($h * 2))
$bw.Write([int16]1)
$bw.Write([int16]$bpp)
$bw.Write([int32]0)
$bw.Write([int32]0)
$bw.Write([int32]0)
$bw.Write([int32]0)
$bw.Write([int32]0)

# Pixel data: dark red BGRA
for ($i = 0; $i -lt $w * $h; $i++) {
    $bw.Write([byte]50)
    $bw.Write([byte]0)
    $bw.Write([byte]200)
    $bw.Write([byte]255)
}

# AND mask: all zeros
$bw.Write([byte[]]::new($andMaskSize))

# Go back and write directory entry
$ms.Position = $dirPos
$bw.Write([byte]$w)
$bw.Write([byte]$h)
$bw.Write([byte]0)
$bw.Write([byte]0)
$bw.Write([int16]1)
$bw.Write([int16]$bpp)
$bw.Write([int32]$imageDataSize)
$bw.Write([int32]$dirEntryOffset)

$bw.Flush()
$bytes = $ms.ToArray()
$bw.Dispose()
$ms.Dispose()

[System.IO.File]::WriteAllBytes("C:\Users\chris\Downloads\movie-library\src-tauri\icons\icon.ico", $bytes)
Write-Host "icon.ico created: $($bytes.Length) bytes"
