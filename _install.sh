
sudo apt update
sudo apt install -y libjavascriptcoregtk-4.0-dev libsoup2.4-dev libwebkit2gtk-4.0-dev     build-essential     curl     wget     file     libssl-dev     libgtk-3-dev     libayatana-appindicator3-dev     librsvg2-dev 
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
source "/workspace/.cargo/env"
cd tauri-next
npm install
npm run tauri dev