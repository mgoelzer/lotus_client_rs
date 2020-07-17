/// Returns the min and max block to scan on the chain. CLI args
/// are used, and, if absent, the defaults. Possible flags:
///     --min=NNN                 # where NNN is a positive number
///     --max=NNN                 # where NNN is a postive number
///     --help, -h                # print usage and exit
/// # Arguments
///
/// * `defaults` - A pair (min_height, max_height) to default to
/// 
/// # Errors
/// Errors are handled by printing an error message + usage and 
/// temrinating the process.
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// let max_chain_height = 5000;
/// let args = process_cli_args((0,max_chain_height));
/// let min_height = args.0;
/// let max_height = args.1;
/// ```
pub fn process_cli_args(defaults: (u64,u64)) -> (u64,u64)
{
    //
    // Command line args:  `--min=N` and `--max=N` define which blocks of the chain to read
    //
    let min_cli_arg = "--min=";
    let max_cli_arg = "--max=";
    let mut min_height : u64 = defaults.0;
    let mut max_height : u64 = defaults.1;

    let mut args : Vec<String> = std::env::args().collect();
    let progname = args.remove(0);
    let usage_and_exit = |ret| { 
        let path = std::path::Path::new(&progname);
        let progname = path.file_name().unwrap().to_string_lossy();
        eprintln!(concat!(
            "\nUSAGE:  {} [-v|vv|vvv|vvvv] [--min=N] [--max=N]\n\N",
            "-v => log level ERROR, -vv => log level WARM, etc. Max is -vvvv (DEBUG)\n"), 
            progname); 
        std::process::exit(ret); 
    };
    let error_usage_exit = |msg,ret| {
        eprintln!("error: {}", msg);
        usage_and_exit(ret);
    };
    for mut argv in args {
        if argv.starts_with(min_cli_arg) {
            argv = argv[min_cli_arg.len()..].to_string();
            if let Ok(n) = argv.parse::<u64>() {
                min_height = n;
            } else {
                error_usage_exit(format!("malformed min: '{}'",argv),1);
            }
        } else if argv.starts_with(max_cli_arg) {
            argv = argv[max_cli_arg.len()..].to_string();
            if let Ok(n) = argv.parse::<u64>() {
                max_height = n;
            } else {
                error_usage_exit(format!("malformed min: '{}'",argv),1);
            }
        } else if argv.starts_with("-h") || argv.starts_with("--h") {
            usage_and_exit(0);
        } else {
            eprintln!("command line argument '{}' is not well formed",&argv);
            usage_and_exit(1);
        }
    }
    (min_height,max_height)
}
