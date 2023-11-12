# cat-tree

## Overview

A Rust application for generating directory trees with their content.

## Installation

```bash
cargo install --path .
```

## Usage

Run the application with the following flags to customize the output:

- `--exclude [FILE/DIRECTORY]`: Exclude files or directories.
- `--max_depth [DEPTH]`: Set the maximum directory traversal depth.
- `--size`: Display file sizes.
- `--all`: Include hidden files/directories.
- `--follow_links`: Follow symbolic links.
- `--flatten`: Flatten the directory structure.
- `--file_display_template [TEMPLATE]`: Customize the file display format.

### Examples

```bash
# Display tree from a specific root
directory-tree-generator --root /path/to/directory

# Exclude certain directories
directory-tree-generator --exclude .git --exclude node_modules

# Set maximum depth and include file sizes
directory-tree-generator --max_depth 3 --size
```

tributions are welcome. Feel free to submit pull requests or open issues.

## License

GNU Affero General Public License. See [LICENSE](LICENSE) for details.
