// Package suppressions turns a Finch mappings document into scanner-native
// suppression files that a CI pipeline can feed to the scanner.
package suppressions

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/deathlabs/finch/models"
)

// File is a generated file, ready to be written to disk.
type File struct {
	Name    string
	Content []byte
}

// Result is what a generator produces.
type Result struct {
	// Files are the generated files.
	Files []File
	// RuleCount is the number of distinct scanner rules that will be suppressed.
	RuleCount int
}

// Options are settings shared by all generators.
type Options struct {
	// FinchVersion is recorded in the suppression report.
	FinchVersion string
}

// Supported lists the tool names Generate accepts.
var Supported = []string{"zap", "grype"}

// Generate produces suppression files for the named tool. It fails closed:
// if anything in the mappings is invalid, it returns an error and no files, so
// a pipeline never runs with a partial or malformed suppression set.
func Generate(tool string, document models.ExportDocument, options Options) (Result, error) {
	switch strings.ToLower(strings.TrimSpace(tool)) {
	case "zap":
		return generateZAP(document, options)
	case "grype":
		return generateGrype(document, options)
	default:
		return Result{}, fmt.Errorf("unsupported tool %q (supported: %s)", tool, strings.Join(Supported, ", "))
	}
}

// WriteFiles writes files into directory, creating it if needed.
func WriteFiles(directory string, files []File) error {
	var (
		err  error
		file File
	)

	// Validate every name before touching the disk.
	for _, file = range files {
		if file.Name == "" || file.Name != filepath.Base(file.Name) {
			return fmt.Errorf("refusing to write %q: file names must not contain path separators", file.Name)
		}
	}

	err = os.MkdirAll(directory, 0o755)
	if err != nil {
		return fmt.Errorf("could not create %q: %w", directory, err)
	}

	for _, file = range files {
		err = os.WriteFile(filepath.Join(directory, file.Name), file.Content, 0o644)
		if err != nil {
			return fmt.Errorf("could not write %q: %w", file.Name, err)
		}
	}

	return nil
}

// sortedNumericKeys returns the keys of set ordered by their numeric values.
func sortedNumericKeys(set map[string]int) []string {
	var keys = make([]string, 0, len(set))
	for key := range set {
		keys = append(keys, key)
	}
	sort.Slice(keys, func(i, j int) bool { return set[keys[i]] < set[keys[j]] })
	return keys
}
