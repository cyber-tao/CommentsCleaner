# Code Comment Cleaning Tool (Comments Cleaner)

A powerful command-line tool for automatically removing comments from code in multiple programming languages.

## Features

- 🚀 **Multi-language support**: C/C++, Java, JavaScript, TypeScript, Python, HTML, CSS
- 🔍 **Auto-detection**: Automatically recognizes programming language based on file extension
- 📁 **Batch processing**: Supports single file and recursive directory processing
- 💾 **Flexible output**: Supports in-place modification, specified output path, automatic backup
- 🎯 **Smart parsing**: Correctly handles comment symbols in strings, won't mistakenly delete
- ✨ **Whitespace optimization**: Automatically merges excessive consecutive empty lines to keep code clean
- 🎨 **Friendly interface**: Colored output with detailed processing information

## Supported Programming Languages

| Language   | File Extensions                                 | Comment Types                           |
| ---------- | ----------------------------------------------- | --------------------------------------- |
| C          | `.c`, `.h`                                  | `//` and `/* */`                    |
| C++        | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx` | `//` and `/* */`                    |
| Java       | `.java`                                       | `//` and `/* */`                    |
| JavaScript | `.js`, `.jsx`                               | `//` and `/* */`                    |
| TypeScript | `.ts`, `.tsx`                               | `//` and `/* */`                    |
| Python     | `.py`, `.pyw`                               | `#` and `"""..."""` / `'''...'''` |
| HTML       | `.html`, `.htm`                             | `<!-- -->`                            |
| CSS        | `.css`                                        | `/* */`                               |

## Installation

### Download pre-built binaries

You can download the latest pre-built executable from the [Releases](https://github.com/cyber-tao/comments-cleaner/releases) page.

1. Go to the [Releases page](https://github.com/cyber-tao/comments-cleaner/releases)
2. Download the appropriate binary for your operating system
3. Extract the executable and add it to your system PATH (optional)

### Build from source

```bash
git clone https://github.com/cyber-tao/comments-cleaner.git
cd comments-cleaner
cargo build --release
```

The compiled executable will be located at `target/release/cclean.exe` (Windows) or `target/release/cclean` (Linux/macOS)

### Install to system

```bash
cargo install --path .
```

## Usage

### Basic usage

```bash
cclean <file or directory path> [options]
```

### Common examples

#### 1. Process single file (generate new file)

```bash
cclean src/main.cpp
```

Output: `src/main_cleaned.cpp`

#### 2. Modify file in-place

```bash
cclean src/main.cpp -i
```

#### 3. Specify output file

```bash
cclean src/main.cpp -o output/main.cpp
```

#### 4. Create backup and modify in-place

```bash
cclean src/main.cpp -i -b
```

Output: Modify original file, create `src/main.cpp.bak`

#### 5. Recursive directory processing

```bash
cclean src/ -r -i
```

#### 6. Process directory and output to specified directory

```bash
cclean src/ -r -o cleaned/
```

#### 7. Process only specific extension files

```bash
cclean src/ -r -e "cpp,h,hpp"
```

#### 8. Manually specify programming language

```bash
cclean script.txt -l cpp -o script_cleaned.txt
```

#### 9. Dry run (no actual modification)

```bash
cclean src/ -r --dry-run -v
```

#### 10. Verbose mode

```bash
cclean src/ -r -i -v
```

## Command-line Options

| Option                 | Short  | Description                                          |
| ---------------------- | ------ | ---------------------------------------------------- |
| `--output <PATH>`    | `-o` | Specify output file or directory path                |
| `--recursive`        | `-r` | Recursively process all files in directory           |
| `--in-place`         | `-i` | Modify original file directly                        |
| `--backup`           | `-b` | Create backup file (.bak)                            |
| `--lang <LANGUAGE>`  | `-l` | Manually specify programming language                |
| `--verbose`          | `-v` | Show detailed processing information                 |
| `--dry-run`          |        | Dry run, do not actually modify files                |
| `--extensions <EXT>` | `-e` | Specify file extensions to process (comma-separated) |
| `--help`             | `-h` | Show help information                                |
| `--version`          | `-V` | Show version information                             |

## Supported Language Identifiers

When using `-l` or `--lang` option, you can use the following language identifiers:

- `c`: C language
- `cpp` or `c++`: C++
- `java`: Java
- `js` or `javascript`: JavaScript
- `ts` or `typescript`: TypeScript
- `python` or `py`: Python
- `html`: HTML
- `css`: CSS

## Important Notes

1. **String safety**: The tool can correctly identify comment symbols in strings and won't mistakenly delete them
2. **Backup recommendation**: When processing important files, it's recommended to use the `-b` option to create backups
3. **Testing recommendation**: For first-time use, it's recommended to run with `--dry-run` first
4. **Encoding support**: Currently supports UTF-8 encoded files

## Example Scenarios

### Scenario 1: Clean comments for entire project

```bash
cclean ./project -r -o ./project_cleaned -v
```

### Scenario 2: Clean code before release

```bash
cclean ./src -r -i -b -v
```

This will:

- Recursively process src directory
- Modify all files in-place
- Create .bak backup for each file
- Show detailed processing information

### Scenario 3: Clean only JavaScript files

```bash
cclean ./src -r -e "js,jsx" -o ./dist
```

## Technical Details

### Comment Processing Rules

- **C/C++/Java/JS/TS**: Remove `//` single-line comments and `/* */` multi-line comments
- **Python**: Remove `#` comments and triple-quoted strings `"""..."""` / `'''...'''` (including docstrings, replaced with empty string `""`)
- **HTML**: Remove `<!-- -->` comments, also process comments in embedded `<script>` and `<style>` tags
- **CSS**: Remove `/* */` comments

### Whitespace Handling

After removing comments, the tool will automatically:

- Merge multiple consecutive empty lines into a single empty line
- Keep code structure clear
- Reduce file size

### Smart Parsing

The tool uses state machine parsing to correctly handle:

- Comment symbols in strings (won't be deleted)
- Escape characters
- Multi-line strings
- Nested structures

### Important Notes

**Python triple-quoted string handling**:

- All triple-quoted strings (`"""..."""` and `'''...'''`) content will be cleared
- Replaced with empty string `""` to maintain syntax correctness
- Including docstrings
- If code depends on `__doc__` attribute or uses triple quotes to store data, use with caution
- For detailed information, see: `PYTHON_CLEANUP_DEMO.md`

## Development

### Run tests

```bash
cargo test
```

### Development mode run

```bash
cargo run -- <arguments>
```

Example:

```bash
cargo run -- test.cpp -v
```

## License

MIT License

## Contributing

Issues and Pull Requests are welcome!

## Changelog

### v0.1.0

- Initial version
- Support for C/C++, Java, JavaScript, TypeScript, Python, HTML, CSS
- Basic command-line arguments
- Single file and directory processing
- Backup and in-place modification features
