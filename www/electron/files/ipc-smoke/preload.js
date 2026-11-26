const { contextBridge, ipcRenderer } = require('electron');

if (!process.sandboxed) throw new Error('Expected a sandboxed renderer');
contextBridge.exposeInMainWorld('electronPort', {
  ping: value => ipcRenderer.invoke('electron-port:ping', value)
});
