# stealth-win-rs
A Windows framework for hiding strings and obfuscating function calls in Rust. Creates and embeds a PE resource with 
XOR'd strings and AES encrypted payloads.  

# Features
### Resource File
Resource file built with the build script every time build is ran. Randomizes the position of strings and payload, as well as
padding between entries. This resource is put into your final EXE and can be indexed using `util::get_resource_bytes` with
a resource_id, positon, and length.  

### Strings
Strings can be added to the binary in xor'd format with their own key, from the `build_config.rs`  
You can use the built in `crypto_util::get_xor_encrypted_bytes` which returns an SVec<u8>, which contains the bytes of your string  
(with no null terminator). You can push a `0` onto this svec, if you need to pass the string to a function that expects  
a null terminated string. There are also `util::compare_xor_str_and_str_bytes` and `util::compare_xor_str_and_w_str_bytes` function 
which are there to aid in comparing the xor'd strings with non xor'd strings. You can get a slice from any pointer to a 
C string and the `util::strlen` function, which you can pass in to compare with these two functions, as well as the key 
to the xor string. This will not decrypt the string, but rather xor encrypt the target string in place, byte by byte, and 
compare to the xor'd string in the embedded PE resource.  

### SVec
A vector type, mainly based on Vec\<T\> code that can be used in an unmapped PE, by utilizing the internal `GetModuleHandle` 
and `GetProcAddress` to call `VirtualAlloc` and `VirtualFree` to manage a buffer of whatever type you give it.  This does not
require that the PE you are running your code from is mapped into memory properly, or not. It just matters that `kernel32` and
`ntdll` are mapped into memory, which they should be.  

### Win API
Windows API calls and structures, and wrappers to get and call these functions through internal `GetModuleHandleX` and 
`GetProcAddresX`, both of which use the internal xor string comparison algorithm to find the function call.  

### Config
Add strings for your own `GetModuleHandleX`and `GetProcAddresX` searches, target file for shellcode or a DLL payload which is
AES encrypted and stored in the embedded PE Resource with the strings. Can change the resource ID, resource file name or the 
range of randomly generated bytes between resource entries. More to come in the config.  

# Goals
* Stealth  
* Learn more about Windows internals  
* Encourage more Rust RE tooling  

# TODO
