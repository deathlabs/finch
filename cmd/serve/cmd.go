package serve

import (
	"github.com/spf13/cobra"
)

var (
	Cmd = &cobra.Command{
		Use:   "serve",
		Short: "Serve a HTTP-based UI for creating mappings between OSCAL components and security rule IDs",
		RunE:  server,
	}
	port int
)

func init() {
	Cmd.PersistentFlags().IntVarP(&port, "port", "p", 8000, "Port to run the HTTP server on")
}
