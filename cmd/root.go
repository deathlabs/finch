package cmd

import (
	"os"

	"github.com/deathlabs/finch/cmd/generate"
	"github.com/deathlabs/finch/cmd/serve"
	"github.com/deathlabs/finch/cmd/tui"
	"github.com/spf13/cobra"
)

const (
	finchVersion = "v0.1.0"
)

var (
	pluginsDirectory string
	rootCmd          = &cobra.Command{
		Use:     "finch",
		Short:   "Finch is a tool for mapping OSCAL components to security rules and generating security suppression files.",
		Version: finchVersion,
	}
	sspFilePath string
)

func Execute() {
	var err = rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.PersistentFlags().StringVarP(&sspFilePath, "ssp", "", "", "File path to System Security Plan (in OSCAL format)")
	rootCmd.PersistentFlags().StringVarP(&pluginsDirectory, "plugins-dir", "", "plugins", "Plugins directory")
	rootCmd.AddCommand(generate.Cmd)
	rootCmd.AddCommand(serve.Cmd)
	rootCmd.AddCommand(tui.Cmd)
}
