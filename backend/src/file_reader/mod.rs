pub(crate) mod reader;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::LinuxFileReader as SaveFileReader;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub(crate) use windows::WindowsFileReader as SaveFileReader;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
pub(crate) use mac::MacFileReader as SaveFileReader;

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_index_returnsJsonVec0() {}

    #[tokio::test]
    async fn test_empires_returnsListOfEmpireNames() {}
}
