struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() -> std::io::Result<()> {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed

    // More practical example
    use std::fs::File;
    use std::path::PathBuf;

    struct TempFile {
        file: File,
        path: PathBuf,
    }

    impl TempFile {
        fn new(path: PathBuf) -> std::io::Result<Self> {
            // Note: File::create() will overwrite existing files
            let file = File::create(&path)?;

            Ok(Self { file, path })
        }
    }

    // When TempFile is dropped:
    // 1. First, our drop implementation will remove the file's name from the filesystem.
    // 2. Then, File's drop will close the file, removing its underlying content from the disk.
    impl Drop for TempFile {
        fn drop(&mut self) {
            if let Err(e) = std::fs::remove_file(&self.path) {
                eprintln!("Failed to remove temporary file: {}", e);
            }
            println!("> Dropped temporary file: {:?}", self.path);
            // File's drop is implicitly called here because it is a field of this struct.
        }
    }

    // Create a new scope to demonstrate drop behavior
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("Temporary file created");
        // File will be automatically cleaned up when temp goes out of scope
    }
    println!("End of scope - file should be cleaned up");

    // We can also manually drop if needed
    let temp2 = TempFile::new("another_test.txt".into())?;
    drop(temp2); // Explicitly drop the file
    println!("Manually dropped file");

    Ok(())
}
