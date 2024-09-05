//go:build mage
// +build mage

package main

import (
	"bytes"
	"fmt"

	"github.com/princjef/mageutil/bintool"
	"github.com/princjef/mageutil/shellcmd"
)

var (
	golines = bintool.Must(bintool.NewGo(
		"github.com/segmentio/golines",
		"v0.12.2",
	))
	linter = bintool.Must(bintool.New(
		"golangci-lint{{.BinExt}}",
		"1.57.2",
		"https://github.com/golangci/golangci-lint/releases/download/v{{.Version}}/golangci-lint-{{.Version}}-{{.GOOS}}-{{.GOARCH}}{{.ArchiveExt}}",
	))
)

// Format formats the code.
func Format() error {
	if err := golines.Ensure(); err != nil {
		return err
	}

	return golines.Command(`-m 80 --no-reformat-tags -w .`).Run()
}

// Lint lints the code.
func Lint() error {
	if err := linter.Ensure(); err != nil {
		return err
	}

	return linter.Command(`run`).Run()
}

// Test runs the unit tests.
func Test() error {
	return shellcmd.Command(`go test`).Run()
}

// TestClean runs the unit tests with no test cache.
func TestClean() error {
	return shellcmd.RunAll(`go clean -testcache`, `go test`)
}

// CI runs format, lint, and test.
func CI() error {
	if err := Format(); err != nil {
		return err
	}

	if err := Lint(); err != nil {
		return err
	}

	if err := Test(); err != nil {
		return err
	}

	return nil
}

// CIVerify runs CI and verifies no thrashing occurred.
func CIVerify() error {
	// Run CI.
	if err := CI(); err != nil {
		return err
	}

	// Check git status for any modified files.
	modified, err := shellcmd.Command(`git ls-files -mz`).Output()
	if err != nil {
		return err
	}
	if len(modified) > 0 {
		files := bytes.Split(modified, []byte{0})
		return fmt.Errorf(
			`found modified files - %s`,
			bytes.Join(files[:len(files)-1], []byte(", ")),
		)
	}

	return nil
}
