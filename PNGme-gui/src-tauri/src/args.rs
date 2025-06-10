use pngme::{
    args::DecodeArgs, args::EncodeArgs, args::PrintArgs, args::RemoveArgs, decode_with_args,
    encode_with_args, print_with_args, remove_with_args, Command,
};

#[tauri::command]
pub fn decode_gui(path: String, chunk_type: String) -> String {
    let runner = Command::new();
}
