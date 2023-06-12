# genpassw - Password Generator

`genpassw` is a simple, "secure" password generator written in Rust🦀. It generates a password of specified length, using a mix of uppercase and lowercase letters, numbers, and special characters. It also allows specifying characters to exclude from the generated password.

## Installation

First, clone this repository to your local machine using `git`:

```bash
git clone https://github.com/jmrada14/genpassw.git
```

Navigate to the cloned repository:

```bash
cd genpassw
```

Then, build and install the program using the installation script:

```bash
sh install.sh
```

## Usage

After installing, you can use genpassw from anywhere on your system. By default, it will generate a 16-character password.

Generate a 16-character password:

```bash
genpassw
```

If you want to generate a password of a different length, you can specify the length using the `--len` argument:

For example, to generate a 20-character password:

```bash
genpassw --len=20
```

You can also specify characters to exclude from the generated password using the `--exclude` argument:

For example, to generate a 16-character password excluding the characters `a`, `b`, and `c`:

```bash
genpassw --exclude="abc"
```

Both arguments can be used together:

For example, to generate a 20-character password excluding the characters `a`, `b`, and `c`:

```bash
genpassw --len=20 --exclude="abc"
```

## Remove genpassw from your system:

To uninstall genpassw, navigate to the directory where you cloned the repository and run the uninstall script:

```bash
sh uninstall.sh
```
