# Build the release version
cargo build --release

# Create dist directory structure
New-Item -ItemType Directory -Force -Path "dist"
New-Item -ItemType Directory -Force -Path "dist\platform-tools"

# Copy the executable
Copy-Item "target\release\adb_manager.exe" -Destination "dist\" -Force

# Copy platform-tools (assuming they're in the project root)
if (Test-Path "platform-tools") {
    Copy-Item "platform-tools\*" -Destination "dist\platform-tools\" -Recurse -Force
}

# Build installer if Inno Setup is installed
if (Get-Command "iscc" -ErrorAction SilentlyContinue) {
    iscc installer.iss
} else {
    Write-Warning "Inno Setup Compiler (iscc) not found in PATH. Skipping installer creation."
} 