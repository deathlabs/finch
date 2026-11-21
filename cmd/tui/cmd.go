package tui

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/spf13/cobra"
)

// Cmd starts the terminal interface for creating mappings.
var Cmd = &cobra.Command{
	Use:   "tui",
	Short: "Create mappings in a terminal interface",
	RunE:  run,
}

func run(command *cobra.Command, args []string) error {
	var (
		err     error
		path    string
		program *tea.Program
	)
	path, err = command.Flags().GetString("ssp")
	if err != nil {
		return err
	}
	program = tea.NewProgram(newModel(path), tea.WithAltScreen(), tea.WithMouseCellMotion())
	_, err = program.Run()
	return err
}
