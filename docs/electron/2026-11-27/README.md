# Electron 43.8.10p0 desktop regression run, 2026-11-27

The installed Electron package passed the sandboxed BrowserWindow IPC smoke test on
the OpenBSD desktop. The upstream regression run used Vitest with one Electron
worker, the Rolldown WASI binding, a private Xvfb display and a test-specific
unveil file. Workers could read the spec tree and execute the build-tree binary.
The saved terminal output begins partway through the run. [Original log](electron_test_errors.txt)

`make test` exited with status 1 after 2 h 6 min 36 s. Vitest reported 3,398
passed, 96 failed, 2 expected failures, 2,649 skipped, 2 todo and 2 unhandled
errors. Repeated failure lines include retries. [Final counts](electron_test_errors.txt#L1974)

| Finding | Evidence and interpretation |
| --- | --- |
| Two native crashes to isolate first | The worker running `extensions.spec.ts` exited with `SIGILL`. A child in `api-process.spec.ts` exited with `SIGSEGV` during concurrent environment-variable access. These signals identify the affected cases. Determining the cause requires a native backtrace. [Extension worker](electron_test_errors.txt#L1966) · [Environment test](electron_test_errors.txt#L1469) |
| Crash reporting | 37 of the 96 distinct failing tests are in `api-crash-reporter.spec.ts`. The BSD patch disables Crashpad initialization and returns an empty uploaded-report list. Cases that wait for uploads or minidumps repeatedly time out. [Port patch](../../../www/electron/patches/patch-electron_shell_browser_api_electron_api_crash_reporter_cc) · [Timeout](electron_test_errors.txt#L1062) |
| Fixture writes | The test unveil file grants read access to the spec tree. The protocol suite gets `EACCES` creating `fixtures/video.webm`, then its cleanup gets `ENOENT`. BrowserWindow tests also fail to write preload fixtures. Rerun these with writable disposable fixtures. [Protocol setup and cleanup](electron_test_errors.txt#L173) · [BrowserWindow preload](electron_test_errors.txt#L507) |
| Downloads and tracing | Eight session download tests timed out or reported an interrupted download. Nine content-tracing tests failed assertions. Their causes remain open. [Session download](electron_test_errors.txt#L1513) · [Content tracing](electron_test_errors.txt#L642) |
| Test harness | The certificate-revocation suite could not load `electron_common_testing`. The other unhandled error, alongside the worker's `SIGILL`, came from the Vitest runner reading `beforeAll` from undefined suite hooks. The final Python exception propagates Vitest's exit status. [Missing binding](electron_test_errors.txt#L197) · [Runner hook](electron_test_errors.txt#L1939) |

For Code - OSS, the next validation is the editor's installed desktop smoke test.
For Electron, rerun the two native crashes individually and collect backtraces before
repeating the full suite. [Editor build and smoke test](../../../README.md#editorsvscode)
