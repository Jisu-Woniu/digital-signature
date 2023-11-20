node_modules:
	pnpm install

.PHONY: dev
dev: node_modules
	pnpm tauri dev

.PHONY: build
build: node_modules
	pnpm tauri build -b none

.PHONY: clean
clean:
	git clean -dfX

.PHONY: default
default: build
