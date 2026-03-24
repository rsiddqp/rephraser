use std::error::Error;

const SERVICE_NAME: &str = "com.rephraser.desktop";

#[cfg(target_os = "macos")]
pub fn set(account: &str, secret: &str) -> Result<(), Box<dyn Error>> {
    use security_framework::passwords::set_generic_password;
    set_generic_password(SERVICE_NAME, account, secret.as_bytes())?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn get(account: &str) -> Result<Option<String>, Box<dyn Error>> {
    use security_framework::passwords::get_generic_password;
    match get_generic_password(SERVICE_NAME, account) {
        Ok(bytes) => {
            let value = String::from_utf8(bytes.to_vec())?;
            if value.is_empty() {
                Ok(None)
            } else {
                Ok(Some(value))
            }
        }
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            // Handle all variants of "not found" errors from Security framework
            if msg.contains("-25300")
                || msg.contains("not found")
                || msg.contains("could not be found")
                || msg.contains("specified item")
                || msg.contains("errSecItemNotFound")
            {
                Ok(None)
            } else {
                Err(e.into())
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn delete(account: &str) -> Result<(), Box<dyn Error>> {
    use security_framework::passwords::delete_generic_password;
    match delete_generic_password(SERVICE_NAME, account) {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            if msg.contains("-25300")
                || msg.contains("not found")
                || msg.contains("could not be found")
                || msg.contains("specified item")
            {
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn set(account: &str, secret: &str) -> Result<(), Box<dyn Error>> {
    use std::ptr;
    use windows::Win32::Security::Credentials::*;
    use windows::core::PWSTR;

    let target: Vec<u16> = format!("{}/{}", SERVICE_NAME, account)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let blob = secret.as_bytes();

    let cred = CREDENTIALW {
        Type: CRED_TYPE_GENERIC,
        TargetName: PWSTR(target.as_ptr() as *mut _),
        CredentialBlobSize: blob.len() as u32,
        CredentialBlob: blob.as_ptr() as *mut _,
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        ..Default::default()
    };

    unsafe {
        CredWriteW(&cred, 0)?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn get(account: &str) -> Result<Option<String>, Box<dyn Error>> {
    use windows::Win32::Security::Credentials::*;
    use windows::core::PCWSTR;

    let target: Vec<u16> = format!("{}/{}", SERVICE_NAME, account)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut pcred: *mut CREDENTIALW = std::ptr::null_mut();
        match CredReadW(PCWSTR(target.as_ptr()), CRED_TYPE_GENERIC, 0, &mut pcred) {
            Ok(()) => {
                let cred = &*pcred;
                let bytes = std::slice::from_raw_parts(
                    cred.CredentialBlob,
                    cred.CredentialBlobSize as usize,
                );
                let value = String::from_utf8_lossy(bytes).to_string();
                CredFree(pcred as *mut _);
                if value.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(value))
                }
            }
            Err(e) => {
                if e.to_string().contains("1168") {
                    Ok(None) // ERROR_NOT_FOUND
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn delete(account: &str) -> Result<(), Box<dyn Error>> {
    use windows::Win32::Security::Credentials::*;
    use windows::core::PCWSTR;

    let target: Vec<u16> = format!("{}/{}", SERVICE_NAME, account)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        match CredDeleteW(PCWSTR(target.as_ptr()), CRED_TYPE_GENERIC, 0) {
            Ok(()) => Ok(()),
            Err(e) => {
                if e.to_string().contains("1168") {
                    Ok(())
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn set(_account: &str, _secret: &str) -> Result<(), Box<dyn Error>> {
    Err("Secure credential storage not available on this platform".into())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn get(_account: &str) -> Result<Option<String>, Box<dyn Error>> {
    Ok(None)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn delete(_account: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
