<h1 align="center">Tally Lines</h1>

### Overview

This is a blazing fast ⚡, command line tool that calculates the number of lines of code in a specific directory

I know there is the option to view the number of lines of code in most IDEs, but is usually limited to just one file, if you're curious about the number of lines you've coded for any of your projects, this tool you is for you!

This is one of my first Rust projects, and I thought a CLI tool would be great to start with!

### Installation

#### Using Cargo (Recommended)

```bash
cargo install lines-tally
```

#### From source

```bash
git clone https://github.com/27Saumya/lines-tally.git
cd lines-tally
cargo install --path .
```

### Usage

```bash
lines-tally [flags] <directory>
```

#### Flags

`--noGit` - Exclude the files in your `.gitignore` if present or the `.git` directory.

`--noTarget` - Exclude the files in your `target` directory of Rust.