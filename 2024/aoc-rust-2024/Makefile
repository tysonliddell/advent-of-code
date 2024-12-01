VENV_DIR=.pre-commit-venv

# Default target
.PHONY: all
all: pre-commit

.PHONY: venv
venv:
	@if [ ! -d $(VENV_DIR) ]; then \
		python3 -m venv $(VENV_DIR); \
	else \
		echo "Pre-commit virtual environment already set up."; \
	fi

.PHONY: pre-commit
pre-commit: venv
	$(VENV_DIR)/bin/pip install pre-commit
	$(VENV_DIR)/bin/pre-commit install


.PHONY: clean
clean:
	echo "Removing pre-commit hooks"
	$(VENV_DIR)/bin/pre-commit uninstall
	rm -r $(VENV_DIR)
