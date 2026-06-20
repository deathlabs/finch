/*
Copyright © 2026 Victor Fernandez III <@cyberphor>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/
package cmd

import (
	"fmt"
	"os"

	"github.com/deathlabs/finch/cmd/serve"
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
		Version: fmt.Sprintf("%s", finchVersion),
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
	rootCmd.AddCommand(serve.Cmd)
}
