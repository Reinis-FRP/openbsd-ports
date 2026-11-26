const { app, BrowserWindow, ipcMain } = require('electron');
const { randomUUID } = require('node:crypto');
const path = require('node:path');

const timeout = setTimeout(() => {
  console.error('Timed out waiting for renderer IPC');
  app.exit(1);
}, 30000);

app.whenReady().then(async () => {
  const window = new BrowserWindow({
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true
    }
  });
  const nonce = randomUUID();
  ipcMain.handle('electron-port:ping', (event, value) => {
    if (event.sender !== window.webContents || value !== nonce)
      throw new Error('Unexpected IPC sender or value');
    return 'pong/' + value;
  });
  await window.loadURL('data:text/html,<title>OpenBSD Electron IPC</title>');
  const reply = await window.webContents.executeJavaScript(
    'window.electronPort.ping(' + JSON.stringify(nonce) + ')'
  );
  if (reply !== 'pong/' + nonce) throw new Error('Unexpected IPC reply');
  const expected = { electron: '43.8.10', chrome: '150.0.7871.250', node: '24.22.1' };
  for (const [name, version] of Object.entries(expected)) {
    if (process.versions[name] !== version)
      throw new Error(`Unexpected ${name} version ${process.versions[name]}`);
  }
  console.log(JSON.stringify({ electron: process.versions.electron,
    chromium: process.versions.chrome, node: process.versions.node, reply }));
  clearTimeout(timeout);
  window.close();
  app.quit();
}).catch(error => {
  console.error(error);
  app.exit(1);
});
