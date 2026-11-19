use {
    crate::{
        ledger::{LEDGER_VID, LedgerWallet, is_valid_ledger},
        locator::Manufacturer,
        remote_wallet::{Device, RemoteWalletError, RemoteWalletInfo, RemoteWalletType},
    },
    std::{
        ffi::{CStr, c_char, c_int},
        fs::{self, File, OpenOptions},
        io::{self, Read, Write},
        os::{fd::AsRawFd, unix::fs::FileTypeExt},
        rc::Rc,
    },
};

unsafe extern "C" {
    fn agave_ledger_wait(fd: c_int, timeout_ms: c_int) -> c_int;
    fn agave_ledger_product(
        fd: c_int,
        vendor: u16,
        model: *mut c_char,
        serial: *mut c_char,
        size: usize,
    ) -> c_int;
}

pub enum LedgerDevice {
    HidApi(hidapi::HidDevice),
    Uhid(File),
}

fn hid_error(err: io::Error) -> RemoteWalletError {
    RemoteWalletError::Hid(err.to_string())
}

impl LedgerDevice {
    pub(crate) fn write(&self, buf: &[u8]) -> Result<usize, RemoteWalletError> {
        match self {
            Self::HidApi(device) => Ok(device.write(buf)?),
            Self::Uhid(file) => (&*file).write(buf).map_err(hid_error),
        }
    }

    pub(crate) fn read(&self, buf: &mut [u8]) -> Result<usize, RemoteWalletError> {
        match self {
            Self::HidApi(device) => Ok(device.read(buf)?),
            Self::Uhid(file) => (&*file).read(buf).map_err(hid_error),
        }
    }
}

const DISCOVERY_WAIT_MS: i32 = 5000;
const CONFIGURATION_WAITS_MS: [i32; 3] = [1000, 1000, DISCOVERY_WAIT_MS];

/// Configuration retries may leave extra replies on the ordered HID input pipe.
/// Consume those replies before accepting the single public-key response.
pub(crate) struct Discovery<'a> {
    file: &'a File,
    duplicates: Vec<Vec<u8>>,
}

impl<'a> Discovery<'a> {
    pub(crate) fn new(file: &'a File) -> Self {
        Self { file, duplicates: Vec::new() }
    }

    pub(crate) fn configuration(
        &mut self,
        mut send: impl FnMut() -> Result<(), RemoteWalletError>,
    ) -> Result<Vec<u8>, RemoteWalletError> {
        for (retries, wait_ms) in CONFIGURATION_WAITS_MS.into_iter().enumerate() {
            send()?;
            if let Some(reply) = self.next_reply(wait_ms)? {
                self.duplicates.extend(std::iter::repeat_n(reply.clone(), retries));
                return Ok(reply);
            }
        }
        Err(RemoteWalletError::Hid(
            "no Ledger configuration reply after three attempts on the same uhid handle".into(),
        ))
    }

    pub(crate) fn pubkey_reply(&mut self) -> Result<Vec<u8>, RemoteWalletError> {
        self.next_reply(DISCOVERY_WAIT_MS)?.ok_or_else(|| RemoteWalletError::Hid(
            "timed out waiting for the Ledger discovery public key".into(),
        ))
    }

    fn next_reply(&mut self, wait_ms: i32) -> Result<Option<Vec<u8>>, RemoteWalletError> {
        loop {
            // Only this reader consumes frames from the exclusively opened uhid file.
            match unsafe { agave_ledger_wait(self.file.as_raw_fd(), wait_ms) } {
                -1 => return Err(hid_error(io::Error::last_os_error())),
                0 => return Ok(None),
                _ => {}
            }
            // Both discovery commands return a single unnumbered 64-byte report.
            let mut frame = [0; 64];
            let n = (&*self.file).read(&mut frame).map_err(hid_error)?;
            let length = u16::from_be_bytes([frame[5], frame[6]]) as usize;
            if n != frame.len() || frame[..5] != [1, 1, 5, 0, 0] || !(2..=57).contains(&length) {
                return Err(RemoteWalletError::Protocol("Invalid Ledger discovery report"));
            }
            let reply = &frame[7..7 + length];
            if let Some(i) = self.duplicates.iter().position(|old| old == reply) {
                self.duplicates.swap_remove(i);
                continue;
            }
            return Ok(Some(reply.to_vec()));
        }
    }
}

fn identify(file: &File, path: String) -> Option<RemoteWalletInfo> {
    let mut model = [0; 128];
    let mut serial = [0; 128];
    // The C helper uses the system USB headers and NUL-terminates both buffers
    // on success. It verifies the descriptor on this file before any writes.
    let product = unsafe {
        agave_ledger_product(
            file.as_raw_fd(),
            LEDGER_VID,
            model.as_mut_ptr(),
            serial.as_mut_ptr(),
            model.len(),
        )
    };
    if !is_valid_ledger(LEDGER_VID, u16::try_from(product).ok()?) {
        return None;
    }
    let model = unsafe { CStr::from_ptr(model.as_ptr()) }.to_string_lossy();
    let serial = unsafe { CStr::from_ptr(serial.as_ptr()) }.to_string_lossy();
    Some(RemoteWalletInfo {
        manufacturer: Manufacturer::Ledger,
        model: model.to_lowercase().replace(' ', "-"),
        serial: serial.into_owned(),
        host_device_path: path,
        ..Default::default()
    })
}

pub(crate) fn scan(
    previous: &[Device],
    detected: &mut Vec<Device>,
    errors: &mut Vec<RemoteWalletError>,
) -> Result<(), RemoteWalletError> {
    for entry in fs::read_dir("/dev").map_err(hid_error)? {
        let entry = entry.map_err(hid_error)?;
        let name = entry.file_name();
        let Some(unit) = name.to_str().and_then(|n| n.strip_prefix("uhid")) else {
            continue;
        };
        if unit.is_empty()
            || !unit.bytes().all(|b| b.is_ascii_digit())
            || !entry.file_type().map_err(hid_error)?.is_char_device()
        {
            continue;
        }
        let path = entry.path();
        let path = path.to_string_lossy().into_owned();
        // uhid opens are exclusive. Reuse our handle when refreshing the list.
        let existing = previous.iter().find_map(|d| match &d.wallet_type {
            RemoteWalletType::Ledger(ledger) if d.path == path => match &ledger.device {
                LedgerDevice::Uhid(file) => Some(file),
                _ => None,
            },
            _ => None,
        });
        let open = || OpenOptions::new().read(true).write(true).open(&path);
        let probe = |file: File| identify(&file, path.clone()).map(|info| (file, info));
        let candidate = existing
            .and_then(|f| f.try_clone().ok())
            .and_then(probe)
            .or_else(|| open().ok().and_then(probe));
        let Some((file, info)) = candidate else {
            continue;
        };
        let mut ledger = LedgerWallet::from_device(LedgerDevice::Uhid(file));
        match ledger.read_wallet_info(info) {
            Ok(info) => {
                ledger.pretty_path = info.get_pretty_path();
                detected.push(Device {
                    path,
                    info,
                    wallet_type: RemoteWalletType::Ledger(Rc::new(ledger)),
                });
            }
            Err(err) => errors.push(err),
        }
    }
    Ok(())
}
