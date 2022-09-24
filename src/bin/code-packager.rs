use bpaf::Bpaf;
use code_packager::Tags;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version, generate(parse_args))]
struct ProgramArgs {
    /// Include code within the range of these tags.
    #[bpaf(short, long)]
    include: Vec<String>,
    /// Exclude code within the range of these tags, even if they are surrounded by a tag to be
    /// included.
    #[bpaf(short, long)]
    exclude: Vec<String>,
    /// The 'end' tag to mark the end of an in-/excluded section. (Default = @end)
    #[bpaf(short('s'), long, fallback("@end".to_string()))]
    end_tag: String,
    /// The prefix for each tag. (Default = //)
    #[bpaf(short('p'), long, fallback("//".to_string()))]
    tag_prefix: String,
    /// The input file to package.
    #[bpaf(positional("INPUT"))]
    input: String,
    /// The output file for the packaged file.
    #[bpaf(positional("OUTPUT"))]
    output: String,
}

impl<'a> From<&'a ProgramArgs> for Tags<'a> {
    fn from(args: &'a ProgramArgs) -> Self {
        Self::new()
            .end_tag(&args.end_tag)
            .tag_prefix(&args.tag_prefix)
            .include_all(args.include.iter().map(String::as_str))
            .exclude_all(args.exclude.iter().map(String::as_str))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: ProgramArgs = parse_args().run();
    let tags = Tags::from(&args);

    let content = fs::read_to_string(&args.input)?;
    let packaged = tags.package(&content)?;
    fs::write(args.output, packaged)?;

    Ok(())
}

#[test]
fn check_args() {
    parse_args().check_invariants(true)
}
