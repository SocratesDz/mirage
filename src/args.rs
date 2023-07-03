use clap::Parser;

#[derive(Args)]
struct EncodeArgs {
    file_path: String,
    chunk_type: String,
    message: String,
    output: Option<String>,
}

#[derive(Args)]
struct DecodeArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args)]
struct RemoveArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args)]
struct PrintArgs {
    file_path: String,
}
