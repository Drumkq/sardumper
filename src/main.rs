mod memory;
mod patterns;

use std::{
    fs::{self},
    path::Path,
};

use anyhow::{bail, Ok, Result};
use memory::*;
use object::read::coff::*;
use object::read::pe::*;
use object::{pe::*, LittleEndian};

fn dump(path: &Path) -> Result<()> {
    let data = fs::read(path)?;
    let data = data.as_slice();

    let dos_header = ImageDosHeader::parse(data)?;
    let mut offset = dos_header.nt_headers_offset().into();
    let (nt64_headers, _) = ImageNtHeaders64::parse(data, &mut offset)?;
    let header = nt64_headers.file_header();
    let il2cpp_section = match header
        .sections(data, offset)?
        .section_by_name(header.symbols(data)?.strings(), "il2cpp".as_bytes())
    {
        Some(v) => v.1,
        None => bail!("Il2cpp section not found"),
    };

    let va = il2cpp_section.virtual_address.get(LittleEndian) as usize;
    let code = il2cpp_section.pe_data(data)?;

    let scanner = memory::Scanner::new(code.as_ptr().cast(), code.len());

    unsafe {
        println!(
            "ReplaceSquare:{}",
            va + scanner.find(patterns::REPLACE_SQUARE)?
        );
        println!(
            "PerfectHop:{}",
            va + scanner.find(patterns::PERFECT_HOP)?
        );
        println!("AccelOffset:{}", {
            get_virtual_absolute_address(
                va as _,
                code,
                scanner.find(patterns::ACCEL_OFFSET)? as _,
                4,
            )
        });
        println!(
            "ReplaceZoomOffset:{}",
            va + scanner.find(patterns::REPLACE_ZOOM_OFFSET)?
        );
        println!(
            "PlayerVision:{}",
            va + scanner.find(patterns::PLAYER_VISION)?
        );
        println!("BananaStun:{}", {
            let idx = scanner.find(patterns::BANANA_STUN)?;

            get_virtual_absolute_address(va as isize, code, idx as isize, 1)
        });
        println!(
            "RecoilPtr:{}",
            va + scanner.find(patterns::RECOIL_PTR)?
        );
        println!(
            "ReplaceSpineOffset:{}",
            va + scanner.find(patterns::REPLACE_SPINE_OFFSET)?
        );
        println!("SpeedOffset:{}", {
            get_virtual_absolute_address(
                va as _,
                code,
                scanner.find(patterns::SPEED_OFFSET)? as _,
                4,
            )
        });
        println!("ZoomSpeedOffset:{}", {
            get_virtual_absolute_address(
                va as _,
                code,
                scanner.find(patterns::ZOOM_SPEED_OFFSET)? as _,
                4,
            )
        });
        println!(
            "ItemVision:{}",
            va + scanner.find(patterns::ITEM_VISION)?
        );
        // println!("AutoHop:{}", va + scanner.find(patterns::REPLACE_SPINE_OFFSET)?);
    }

    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!(
            "Wrong args passed\nExample usage:\n\tsar-dumper.exe \"<path to Super Animal Royale GameAssembly.dll>\""
        );
        return;
    }

    let game_assembly_path = args[1].clone();
    if !game_assembly_path.ends_with("GameAssembly.dll") {
        eprintln!("Invalid path\nExample usage:\n\tsar-dumper.exe \"<path to Super Animal Royale GameAssembly.dll>\"");
        return;
    }

    match dump(Path::new(&game_assembly_path)) {
        Err(e) => eprintln!("Dump error:\n{}", e),
        _ => (),
    };
}
