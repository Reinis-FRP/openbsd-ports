const vscode = require('vscode');
const fs = require('node:fs');
const path = require('node:path');

const home = process.env.HOME;
const report = (name, value) => fs.writeFileSync(path.join(home, name), value);
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

async function start() {
    const git = (await vscode.extensions.getExtension('vscode.git').activate()).getAPI(1);
    const deadline = Date.now() + 60000;
    while (!git.repositories.length && Date.now() < deadline) {
        await sleep(200);
    }
    if (!git.repositories.length) throw new Error('Git did not open the workspace');
    const repo = git.repositories[0];
    const root = repo.rootUri.fsPath;
    const file = path.join(root, 'sample.txt');
    const ts = await vscode.workspace.openTextDocument(path.join(root, 'symbols.ts'));
    await vscode.window.showTextDocument(ts);
    await vscode.extensions.getExtension('vscode.typescript-language-features').activate();
    const tsDeadline = Date.now() + 45000;
    let definition;
    while (!definition && Date.now() < tsDeadline) {
        const locations = await vscode.commands.executeCommand('vscode.executeDefinitionProvider',
            ts.uri, new vscode.Position(1, 4));
        definition = locations?.find(location => {
            const uri = location.targetUri ?? location.uri;
            const range = location.targetSelectionRange ?? location.range;
            return uri.fsPath === ts.uri.fsPath && range.start.line === 0;
        });
        if (!definition) await sleep(250);
    }
    if (!definition) throw new Error('TypeScript did not resolve sampleValue');
    report('typescript.result', 'symbols.ts line 1');
    const kerberos = require(path.join(vscode.env.appRoot, 'node_modules.asar', 'kerberos'));
    const client = await kerberos.initializeClient('HTTP@localhost');
    if (typeof client.step !== 'function') throw new Error('No GSS client');
    report('kerberos.result', 'initialized');

    const document = await vscode.workspace.openTextDocument(file);
    await vscode.window.showTextDocument(document);
    const check = () => {
        if (document.getText().includes('external-acceptance-token')) {
            report('external.seen', document.getText());
        }
        const changed = repo.state.workingTreeChanges.map(change => path.basename(change.uri.fsPath));
        report('git.state', JSON.stringify(changed));
    };
    vscode.workspace.onDidChangeTextDocument(event => { if (event.document === document) check(); });
    repo.state.onDidChange(check);
    check();
    report('extension.ready', await vscode.commands.executeCommand('openbsd.knownResult'));
}

exports.activate = context => {
    context.subscriptions.push(vscode.commands.registerCommand('openbsd.knownResult',
        () => 'code-oss-extension-result-148'));
    context.subscriptions.push(vscode.commands.registerCommand('openbsd.twoTerminals', async () => {
        for (const name of ['first', 'second']) {
            const terminal = vscode.window.createTerminal({ name, shellPath: '/bin/ksh' });
            terminal.show();
            terminal.sendText("printf '%s\\n' " + name + ' > "$HOME/' + name +
                '.out"; tty >> "$HOME/' + name + '.out"');
        }
    }));
    start().catch(err => report('extension.error', err.stack));
};
