build: venv
	(cd ./py_typos; $(VENV)/maturin develop)
	(cd ./dr_demo; $(VENV)/maturin develop)

present: venv
	$(VENV)/jupyter notebook

start: build present

include Makefile.venv