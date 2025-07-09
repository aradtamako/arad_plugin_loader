# arad_plugin_loader
- The ARAD plugin loader

### How to use
1. Build x64 DLL yourself
```
build.bat
```

2. Copy DLL to Arad client directory
```
copy version.dll C:\Nexon\ARAD\version.dll
```

3. Copy Original DLL
```
copy C:\Windows\System32\version.dll C:\Nexon\ARAD\version.dll_
```

4. Stat Arad normaly

### Build Requirements
- [Rust](https://www.rust-lang.org/)
