pre-commit: format lint test

format:
	uv run ruff format

lint: format
	uv run ruff check --fix
	uv run mypy .

test: lint
	uv run pytest

test-last-failed: lint
	uv run pytest --last-failed

run: pre-commit
	poetry run python __main__.py
