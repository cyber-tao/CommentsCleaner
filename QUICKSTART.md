# Quick Start Guide

## 1. Build the project

```bash
cd comments-cleaner
cargo build --release
```

## 2. Test the tool

### Test single file

```bash
cargo run -- examples/test.cpp -v
```

This will create `examples/test_cleaned.cpp`

### View help

```bash
cargo run -- --help
```

### Test all example files

```bash
cargo run -- examples/test.cpp -o examples/test_cleaned.cpp -v
cargo run -- examples/test.py -o examples/test_cleaned.py -v
cargo run -- examples/test.js -o examples/test_cleaned.js -v
cargo run -- examples/test.html -o examples/test_cleaned.html -v
```

### Recursively process entire examples directory

```bash
cargo run -- examples/ -r -o examples_cleaned/ -v
```

### Dry run (no actual file modification)

```bash
cargo run -- examples/ -r --dry-run -v
```

## 3. Usage Scenarios

### Scenario 1: Clean single file, create new file

```bash
cclean main.cpp
```

### Scenario 2: Modify original file directly (use with caution)

```bash
cclean main.cpp -i
```

### Scenario 3: Modify original file and create backup

```bash
cclean main.cpp -i -b
```

### Scenario 4: Batch process project files

```bash
cclean ./src -r -o ./src_cleaned -v
```

### Scenario 5: Process only specific file types

```bash
cclean ./project -r -e "cpp,h" -i -b
```

## 4. Verify Results

After processing, compare original and cleaned files to confirm:
- All comments have been removed
- Code logic is intact
- Comment symbols in strings were not deleted
- Files can compile/run normally

## 5. Common Questions

### Q: How to test the tool safely?
A: Use the `--dry-run` option for simulation

### Q: How to avoid accidentally deleting important files?
A: Always use the `-b` option to create backups, or output to new files first

### Q: Which files will the tool process?
A: By default, it automatically recognizes based on file extensions, you can also manually specify language with `-l`

### Q: How to handle files without extensions?
A: Use the `-l` parameter to manually specify language, e.g.: `cclean myfile -l cpp`

## 6. Advanced Usage

### Create alias (Linux/macOS)

Add to `~/.bashrc` or `~/.zshrc`:

```bash
alias cclean='/path/to/comments-cleaner/target/release/cclean'
```

### Create alias (Windows PowerShell)

Add to PowerShell configuration file:

```powershell
function cclean { & "C:\path\to\comments-cleaner\target\release\cclean.exe" $args }
```

### Batch script example

```bash
#!/bin/bash

SRC_DIR="./src"
OUTPUT_DIR="./dist"

cclean "$SRC_DIR" -r -o "$OUTPUT_DIR" -v
```

## 7. Next Steps

- Read the complete [README.md](README.md)
- View more examples
- Customize the tool as needed
