BUILD_DIR := $(PWD)/build
TESTS_DIR := $(PWD)/tests
TOOLS_DIR := $(PWD)/tools

COVERAGE_PROFILE_FILE := $(TESTS_DIR)/c.out
COVERAGE_REPORT_FILE := $(TESTS_DIR)/c.html

TEST_FLAGS := -mod=vendor -v -race -coverprofile='$(COVERAGE_PROFILE_FILE)' -covermode=atomic
COVER_FLAGS := -html='$(COVERAGE_PROFILE_FILE)' -o '$(COVERAGE_REPORT_FILE)'
INSTALL_FLAGS := -mod=vendor

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: dep
dep:
	go mod tidy && go mod vendor && go mod verify

.PHONY: gen
gen:
	PATH='$(TOOLS_DIR):$(PATH)' go generate ./...

.PHONY: test
test: mkdirs dep
	go test $(TEST_FLAGS) ./...
	GOFLAGS='-mod=vendor' go tool cover $(COVER_FLAGS)

.PHONY: clean
clean:
	rm '$(COVERAGE_PROFILE_FILE)'
	rm '$(COVERAGE_REPORT_FILE)'

.PHONY: mkdirs
mkdirs:
	mkdir -p '$(TESTS_DIR)'

.PHONY: bootstrap
bootstrap: install_easyjson

.PHONY: install_easyjson
install_easyjson:
	GOBIN='$(TOOLS_DIR)' go install $(INSTALL_FLAGS) $(PWD)/vendor/github.com/mailru/easyjson/easyjson
