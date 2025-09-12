use std::fs;
use std::fs::File;
// use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn copy_permission(source: &str, target: &str) -> io::Result<()> {
	let source_metadata = fs::metadata(source)?;
	let source_permissions = source_metadata.permissions();
	let source_modified = source_metadata.modified()?;

	let f = File::open(target)?;
	f.set_permissions(source_permissions)?;
	f.set_modified(source_modified)?;
	// cloud not set atime and ctime in std

	Ok(())
}

fn copy(source: &str, target: &str) -> io::Result<()> {
	let sf = File::open(source)?;
	let tf = File::create(target)?;
	/* OpenOptions version
	let mut tf = OpenOptions::new()
		.write(true)
		.create(true)
		.append(false)
		.open(target)?;
	*/
	let mut reader = BufReader::new(sf);
	let mut writer = BufWriter::new(tf);

	let mut buf = [0u8; 8 * 1024];
	loop {
		let n = reader.read(&mut buf)?;
		if n == 0 {
			break;
		}
		writer.write_all(&buf[..n])?;
	}
	
	writer.flush()?;

	Ok(())
}

pub fn copy_file(source: &str, target: &str, permission: bool) -> io::Result<()> {
	// let result = fs::copy(source, target)?;
	copy(source, target)?;
	if permission {
		copy_permission(source, target)?;
	}

	Ok(())
}

pub fn copy_recursive(source: &str, target: &str, permission: bool) -> io::Result<()> {
	if Path::new(source).is_dir() {
        fs::create_dir_all(target)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let new_target = format!("{}/{}", target, entry.file_name().to_string_lossy());

            if file_type.is_dir() {
                copy_recursive(&entry.path().to_string_lossy(), &new_target, permission)?;
            } else {
                copy_file(&entry.path().to_string_lossy(), &new_target, permission)?;
            }
        }
    } else {
        copy_file(source, target, permission)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
	use super::*;
	use std::fs;

	const TEST_DIR: &str = "testdata/tmp_dir";
	struct TestContext;

	impl TestContext {
		fn new() -> Self {
			fs::create_dir_all(TEST_DIR).unwrap();
			TestContext
		}
	}

	impl Drop for TestContext {
		fn drop(&mut self) {
			fs::remove_dir_all(TEST_DIR).unwrap();
		}
	}

	fn touch_file(file: &str) -> io::Result<()> {
		let _ = File::create(file)?;
		Ok(())
	}
	#[test]
	fn test_copy_file() {
		let _tc = TestContext::new(); // create dir and drop(delete dir)
		let source = format!("{TEST_DIR}/a");
		let target = format!("{TEST_DIR}/b");

		let _ = touch_file(&source);

		let _ = copy(&source, &target);

		let path = Path::new(&target);
		assert!(path.exists());
	}
}
