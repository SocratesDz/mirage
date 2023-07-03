use clap::Args;

#[derive(Args, Debug)]
pub struct EncodeArgs {
    file_path: String,
    chunk_type: String,
    message: String,
    output: Option<String>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    file_path: String,
}
