Repro of Command regression in Rust 1.20.0.

Bat files allow specifying their working directory via "%~dp0". In Rust 1.20.0 this changed such that the working directly instead of the file that the batch file runs from is used. See the below output for details. Expected output is "C:\dev\command_repro\src" but instead working directory is printed. Note that this only happens on bat files found in PATH. Calling by specific file(Ex. "src/test.bat") works.

```
PS C:\dev\command_repro> rustup default 1.20.0
info: using existing install for '1.20.0-x86_64-pc-windows-msvc'
info: default toolchain set to '1.20.0-x86_64-pc-windows-msvc'

  1.20.0-x86_64-pc-windows-msvc unchanged - rustc 1.20.0 (f3d6973f4 2017-08-27)

PS C:\dev\command_repro> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\command_repro.exe`
"C:\dev\command_repro\"

PS C:\dev\command_repro> rustup default 1.19.0
info: using existing install for '1.19.0-x86_64-pc-windows-msvc'
info: default toolchain set to '1.19.0-x86_64-pc-windows-msvc'

  1.19.0-x86_64-pc-windows-msvc unchanged - rustc 1.19.0 (0ade33941 2017-07-17)

PS C:\dev\command_repro> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\command_repro.exe`
"C:\dev\command_repro\src\"
```
