# Rust / Python Demo

This repository is a demo repo for building python extensions in rust!

## Prerequisites

### Python 3.5+

If you are working with an older version of Python, you will need to upgrade to run libraries including Rust code. Follow these instructions to upgrade:

- [Windows](https://docs.python-guide.org/starting/install3/win/)
- [Mac](https://docs.python-guide.org/starting/install3/osx/)
- [Linux])(https://docs.python-guide.org/starting/install3/linux/)

### Rust

Rust is required to compile the Rust code. Setup is very easy and instructions can be found [here](https://www.rust-lang.org/tools/install). After installing rustup, you'll also need to switch you rust version to the nightly build. This can be done by running:

```bash
rustup toolchain install nightly
rustup default nightly
```

## Setup

### Creating the Virtualenv

This project uses [Makefile.venv](https://github.com/sio/Makefile.venv) to setup your python environment. Running the following command will build your virtualenv and install needed dependencies

```bash
make venv
```

### Compiling Rust libraries

Now that you have a virtuanenv, activate it using:

```bash
# windows
.venv\Scripts\activate.bat

# linux/mac
.venv/bin/activate
```

Now just run:

```bash
make build
```

This will compile the rust code and place it in the current virtualenv. If you do not have your virtualenv active, it will fail.

## Running the code

### Jupyter

Running

```bash
make present
```

will start a local Jupyter instance using your virtualenv, even if it not activated.

You can also view the raw slides by opening python/Demo.slides.html in a browser. You will not be able to run the code from those slides though.

### Console

If you instead prefer to run the code from a python repl in a terminal, you can run:

```bash
# starts the python interpreter in the current shell
make python

# starts an ipython instance in the current shell
make ipython
```

Like with jupyter option, this will run in your virtualenv even if you have not activated it.