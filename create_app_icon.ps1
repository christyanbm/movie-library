# Create a simple 1024x1024 PNG icon (dark red with "M" letter)
# Using a minimal valid PNG with solid color

Add-Type -AssemblyName System.Drawing
$bmp = New-Object System.Drawing.Bitmap(1024, 1024)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

# Background
$bgBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(229, 9, 20))
$g.FillEllipse($bgBrush, 0, 0, 1024, 1024)

# Letter M
$font = New-Object System.Drawing.Font("Arial", 600, [System.Drawing.FontStyle]::Bold)
$textBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$size = $g.MeasureString("M", $font)
$x = (1024 - $size.Width) / 2
$y = (1024 - $size.Height) / 2 - 30
$g.DrawString("M", $font, $textBrush, $x, $y)

$bmp.Save("C:\Users\chris\Downloads\movie-library\app-icon.png", [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose()
$bmp.Dispose()
Write-Host "app-icon.png created (1024x1024)"
