// Switch to "windows" mode to hide the console in release version
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{DirEntry, File};
use std::io::{BufReader, BufRead, Write};
use std::process::Command;
use std::{env, io, fs};

use std::path::{PathBuf, Path};

use path_clean::{PathClean};

use std::fs::OpenOptions;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

use druid::widget::{Button, Flex};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};

fn main() -> Result<(), PlatformError> {

    start_up();

    let main_window = WindowDesc::new(ui_builder()).title("Optogon Starter 2.1").resizable(false).window_size_policy(druid::WindowSizePolicy::Content);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .launch(data)
}

fn start_up() {
    rename_sam2d_folder();
}

fn clean_path(path: String) -> String {
    PathBuf::from(path).clean().to_str().unwrap().to_string()
}

fn rename_sam2d_folder() {
    if Path::new("./sam2d/.hint").exists() {
        let file = OpenOptions::new().read(true).open("./sam2d/.hint").unwrap();
        let mut buf = String::new();
        BufReader::new(file).read_line(&mut buf).unwrap();
        let original_name = buf.trim().to_string();

        let mut perms = fs::metadata("./sam2d").unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions("./sam2d", perms).unwrap();
        fs::rename("./sam2d", &original_name).unwrap();

        fs::remove_file(format!("./{}/.hint", original_name)).unwrap();
    }
}

fn list_dirs(path: &str) -> io::Result<Vec<DirEntry>> {
    let mut entries: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path)? {

        let dir_entry = entry?;
        if dir_entry.metadata().unwrap().is_dir() {
            let name_file_search_path = absolute_path(".").unwrap().join(dir_entry.path()).join("name.txt").to_str().unwrap().replace("\\", "/");
            let name_file_search_path_cleaned = clean_path(name_file_search_path); 
            if Path::new(&name_file_search_path_cleaned).exists() {
                entries.push(dir_entry);
            }                       
        }
    }

    Ok(entries)
}

fn rename_current_used_optic_folder(cuo: &String) -> io::Result<()> {
    fs::rename(format!("{}", cuo),"./sam2d")
}

fn create_original_folder_name_hint(dir_entry: &DirEntry) {
    let dir_name = dir_entry.path().file_name().unwrap().to_str().unwrap().trim().to_string();

    if dir_name != "sam2d" {
        let hint_file_path = absolute_path(".").unwrap().join(dir_entry.path()).join(".hint").to_str().unwrap().replace("\\", "/");
        let hint_file_path_cleaned = clean_path(hint_file_path); 
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(hint_file_path_cleaned).unwrap();
        file.write_all(dir_name.as_bytes()).unwrap();
    }    
}

fn first_line_from_file(path: String) -> String {
    let file = File::open(path).expect("No such file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Could not read line");

    line
}

fn ui_builder() -> impl Widget<u32> {

    let mut layout = Flex::column();
    layout.add_spacer(2.0);
    
    for dir_entry in list_dirs(".").unwrap() {
        let dir_name = dir_entry.path().file_name().unwrap().to_str().unwrap().trim().to_string();

        let target_path = PathBuf::from(absolute_path(".").unwrap().join("./sam2d/samlight/sam_light.exe")).clean();

        let name_file_search_path = absolute_path(".").unwrap().join(dir_entry.path()).join("name.txt").to_str().unwrap().replace("\\", "/");
        let name_file_search_path_cleaned = clean_path(name_file_search_path);

        let button_text = first_line_from_file(name_file_search_path_cleaned);
        let button = Button::new(format!("{}", button_text))
        .on_click(move |_ctx, _data, _env| {
            rename_sam2d_folder();
            create_original_folder_name_hint(&dir_entry);
            let cuo = dir_name.clone();
            if rename_current_used_optic_folder(&cuo).is_ok() {
                Command::new(target_path.as_os_str()).spawn().unwrap();
            };            
        })
        .padding(1.0)
        .expand_width()
        .height(50.0);

        layout.add_child(button);
    }

    layout.fix_size(300.0, 1000.0)
}