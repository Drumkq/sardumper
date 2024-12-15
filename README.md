# ⚠️ About
**Works only on Windows**

This is a simple **dumper** for Super Animal Royale, which is designed to calculate offsets for [**software from unknowncheats.me**](https://www.unknowncheats.me/forum/other-games/465314-super-animal-royale-hack.html).

The way it works is very simple: parsing the PE file, getting the `il2cpp` section, searching for certain bytes by patterns in the section date and finally calculating offsets

# ⁉️ How to use
Download or compile (see `Compilation guide`) the application, run via the command line, **specifying the path** to `GameAssembly.dll` from **Super Animal Royale**

Example:
```bash
# if dumper and assembly placed in the same folder
sardumper.exe GameAssembly.dll
```

The result will be output like this in the console: ``NameOfOffset:offset``

Example:
```json
ReplaceSquareOffset:0
PerfectHop:0
AccelOffset:0
ReplaceZoomOffset:0
PlayerVision:0
BananaStun:0
RecoilPtr:0
ReplaceSpineOffset:0
SpeedOffset:0
ZoomSpeedOffset:0
ItemVision:0
AutoHop:0
```

# 💡 Implemented scans
- ✅ ReplaceSquareOffset
- ✅ PerfectHop
- ✅ AccelOffset
- ✅ ReplaceZoomOffset
- ✅ PlayerVision
- ✅ BananaStun
- ✅ RecoilPtr
- ✅ ReplaceSpineOffset
- ✅ SpeedOffset
- ✅ ZoomSpeedOffset
- ✅ ItemVision
- ❌ AutoHop

# ⚒️ Compilation guide
Before compilation, you need an ready environment to work with **Rust** and **C++ for Windows**

1. Download or clone the repository
2. open a command line in the project directory
3. Type the command below:
```bash
cargo build --release
```

*The compiled file by standard is located in **/target/release***
