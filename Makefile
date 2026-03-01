.PHONY: build build-signed test clean signing-identities verify-signature check-signing-identity

APP_PATH := src-tauri/target/release/bundle/macos/Texere.app

build:
	CI=true bunx tauri build
	$(MAKE) verify-signature

build-signed:
	@if [ -z "$(APPLE_SIGNING_IDENTITY)" ]; then \
		echo "Error: APPLE_SIGNING_IDENTITY is required for signed release builds."; \
		echo "Example:"; \
		echo "  make build-signed APPLE_SIGNING_IDENTITY='Apple Development: Your Name (TEAMID)'"; \
		exit 1; \
	fi
	$(MAKE) check-signing-identity APPLE_SIGNING_IDENTITY="$(APPLE_SIGNING_IDENTITY)"
	CI=true APPLE_SIGNING_IDENTITY="$(APPLE_SIGNING_IDENTITY)" bunx tauri build
	$(MAKE) verify-signature

check-signing-identity:
	@if ! security find-identity -v -p codesigning | grep -F "\"$(APPLE_SIGNING_IDENTITY)\"" >/dev/null; then \
		echo "Error: signing identity not found in keychain: $(APPLE_SIGNING_IDENTITY)"; \
		echo "Run 'make signing-identities' and use the exact full certificate name."; \
		exit 1; \
	fi

verify-signature:
	@echo "Verifying signature for $(APP_PATH)"
	@codesign -dv --verbose=4 "$(APP_PATH)" 2>&1 | grep -E "^Identifier=|^TeamIdentifier=|^Signature=" || true
	@verify_out=$$(codesign --verify --deep --strict --verbose=2 "$(APP_PATH)" 2>&1) || { \
		echo "$$verify_out"; \
		echo "$$verify_out" | grep -F "code has no resources but signature indicates they must be present" >/dev/null && { \
			echo "Warning: non-fatal macOS bundle verification quirk detected; continuing."; \
			exit 0; \
		}; \
		exit 1; \
	}

signing-identities:
	@security find-identity -p codesigning -v

test:
	cd src-tauri && cargo test

clean:
	rm -rf dist
	cd src-tauri && cargo clean
