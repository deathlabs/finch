package generate

import (
	"fmt"
	"os"
	"strings"

	"github.com/deathlabs/finch/models"
	"github.com/deathlabs/finch/suppressions"
	"github.com/spf13/cobra"
	"gopkg.in/yaml.v2"
)

var (
	Cmd = &cobra.Command{
		Use:   "generate",
		Short: "Generate scanner suppression files from a mappings file",
		Long: "Generate scanner suppression files from the mappings file exported by \"finch serve\".\n\n" +
			"For ZAP, three files are written: zap-rules.conf (for the -c option of ZAP's scan scripts),\n" +
			"zap-alert-filters.yaml (an Automation Framework alertFilter job), and suppression-report.json\n" +
			"(the platform components and controls that justify each suppression).\n\n" +
			"If the mappings file is invalid, nothing is written.",
		Example: "  finch generate --mappings mappings.yaml --tool zap --output-dir finch-out",
		Args:    cobra.NoArgs,
		RunE:    run,
	}
	mappingsFilePath string
	outputDirectory  string
	tool             string
)

func init() {
	Cmd.Flags().StringVar(&mappingsFilePath, "mappings", "", "File path to the mappings file exported by \"finch serve\"")
	Cmd.Flags().StringVar(&tool, "tool", "zap", "Scanner to generate suppression files for (supported: "+strings.Join(suppressions.Supported, ", ")+")")
	Cmd.Flags().StringVar(&outputDirectory, "output-dir", "finch-out", "Directory to write the generated files to")
	_ = Cmd.MarkFlagRequired("mappings")
}

// run is the handler for the "generate" command.
func run(command *cobra.Command, _ []string) error {
	var (
		data     []byte
		document models.ExportDocument
		err      error
		result   suppressions.Result
	)

	data, err = os.ReadFile(mappingsFilePath)
	if err != nil {
		return fmt.Errorf("could not read the mappings file: %w", err)
	}

	// Strict parsing rejects unknown keys, so a typo cannot silently drop a mapping.
	err = yaml.UnmarshalStrict(data, &document)
	if err != nil {
		return fmt.Errorf("could not parse the mappings file: %w", err)
	}

	result, err = suppressions.Generate(tool, document, suppressions.Options{FinchVersion: command.Root().Version})
	if err != nil {
		return err
	}

	err = suppressions.WriteFiles(outputDirectory, result.Files)
	if err != nil {
		return err
	}

	_, err = fmt.Fprintf(command.OutOrStdout(), "wrote %d files to %s (%d %s rules suppressed)\n",
		len(result.Files), outputDirectory, result.RuleCount, strings.ToUpper(tool))
	return err
}
