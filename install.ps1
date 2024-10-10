cargo build --release

if(-not (Test-Path $HOME\.rcp)) {
	mkdir $HOME\.rcp
}

$config = @{
	port=3000
	out_path="$HOME\Downloads"
} | ConvertTo-Json

echo $config > $HOME\.rcp\config.json
Copy-Item .\target\release\rcpd.exe $HOME\.rcp\rcpd.exe
