import fs from 'node:fs/promises';
import path from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';

const [profile, view, ...words] = process.argv.slice(2);
const selector = view === 'search' ? '.search-view .monaco-list-rows' : '.scm-view .monaco-list-rows';
const deadline = Date.now() + 45000;
let last = '';
while (Date.now() < deadline) {
    const [port] = (await fs.readFile(path.join(profile, 'DevToolsActivePort'), 'utf8')).split('\n');
    const targets = await fetch('http://127.0.0.1:' + port + '/json/list').then(r => r.json());
    const target = targets.find(t => t.type === 'page' && t.url.includes('workbench.html'));
    if (target) {
        const socket = new WebSocket(target.webSocketDebuggerUrl);
        try {
            await new Promise((resolve, reject) => {
                socket.onopen = resolve;
                socket.onerror = reject;
            });
            const reply = new Promise((resolve, reject) => {
                socket.onmessage = event => {
                    const message = JSON.parse(event.data);
                    if (message.id === 1) resolve(message.result.result.value);
                };
                socket.onerror = reject;
            });
            const expression = `Array.from(document.querySelectorAll(${JSON.stringify(selector)}))
                .filter(node => node.getBoundingClientRect().height > 0)
                .map(node => node.innerText).join('\\n')`;
            socket.send(JSON.stringify({id: 1, method: 'Runtime.evaluate',
                params: {expression, returnByValue: true}}));
            last = await reply;
            if (words.every(word => last.includes(word))) {
                console.log(last);
                process.exitCode = 0;
                break;
            }
        } finally {
            socket.close();
        }
    }
    await delay(250);
}
if (!words.every(word => last.includes(word))) {
    throw new Error('Missing ' + view + ' results: ' + JSON.stringify(last));
}
