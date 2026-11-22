const assert = require("node:assert/strict");
const anchor = require("@anchor-lang/core");

async function testCounter() {
    assert.equal(process.env.ANCHOR_PROVIDER_URL, "http://127.0.0.1:8899");
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.counter;
    const [counter] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("counter")], program.programId
    );
    const connection = provider.connection;
    const deployment = await connection.getAccountInfoAndContext(program.programId, "confirmed");
    assert.ok(deployment.value?.executable, "deployed program must be executable");
    // Agave makes an upgradeable program callable in the slot after deployment.
    const deadline = Date.now() + 60_000;
    while (await connection.getSlot("confirmed") <= deployment.context.slot) {
        assert.ok(Date.now() < deadline, "confirmed slot did not advance after deployment");
        await new Promise(resolve => setTimeout(resolve, 250));
    }
    assert.equal(await connection.getAccountInfo(counter, "confirmed"), null);
    const options = { commitment: "confirmed", preflightCommitment: "confirmed" };
    await program.methods.initialize().accountsPartial({ counter }).rpc(options);
    const before = await program.account.counter.fetch(counter, "confirmed");
    assert.equal(before.count.toString(), "0");
    await program.methods.increment().accountsPartial({ counter }).rpc(options);
    const after = await program.account.counter.fetch(counter, "confirmed");
    assert.equal(after.count.toString(), "1");
    assert.ok(after.authority.equals(provider.wallet.publicKey));
    console.log("Local counter advanced from 0 to 1");
}

testCounter().catch(error => {
    console.error(error);
    process.exitCode = 1;
});
