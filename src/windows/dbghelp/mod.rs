use crate::windows::kernel32::{GetModuleHandle, GetProcAddressInternal};
use crate::windows::ntdll::IMAGE_SECTION_HEADER;

pub type FnImageDirectoryEntryToDataEx = unsafe extern "system" fn(
    Base: usize,
    MappedAsImage: u32,
    DirectoryEntry: u16,
    Size: *mut u32,
    FoundHeader: *mut *mut IMAGE_SECTION_HEADER,
) -> usize;

pub unsafe fn ImageDirectoryEntryToDataEx(
    Base: usize,
    MappedAsImage: u32,
    DirectoryEntry: u16,
    Size: *mut u32,
    FoundHeader: *mut *mut IMAGE_SECTION_HEADER,
) -> usize {
    let imageDirectoryEntryToDataEx: FnImageDirectoryEntryToDataEx =
        std::mem::transmute(GetProcAddressInternal(
            GetModuleHandle("dbghelp.dll".as_bytes().to_vec()),
            "ImageDirectoryEntryToDataEx".as_bytes(),
        ));

    imageDirectoryEntryToDataEx(Base, MappedAsImage, DirectoryEntry, Size, FoundHeader)
}
