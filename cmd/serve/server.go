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
package serve

import (
	"embed"
	"encoding/json"
	"fmt"
	"html/template"
	"net/http"
	"os"

	"github.com/deathlabs/finch/models"
	oscalTypes_1_1_3 "github.com/defenseunicorns/go-oscal/src/types/oscal-1-1-3"
	"github.com/go-playground/validator/v10"
	"github.com/labstack/echo/v5"
	"github.com/labstack/echo/v5/middleware"
	"github.com/spf13/cobra"
)

//go:embed images/*
var imagesDir embed.FS

//go:embed scripts/*
var scriptsDir embed.FS

//go:embed styles/*
var stylesDir embed.FS

//go:embed templates/*
var templateDir embed.FS

// getSystemSecurityPlanFilePath gets the file path of the System Security Plan from the command line flags.
func getSystemSecurityPlanFilePath(cmd *cobra.Command) (string, error) {
	var (
		err         error
		sspFilePath string
	)

	// Get the file path of the System Security Plan.
	sspFilePath, err = cmd.Flags().GetString("ssp")
	if err != nil {
		return "", err
	}

	// Return the file path of the System Security Plan.
	return sspFilePath, nil
}

// loadSystemSecurityPlan loads the System Security Plan from the file path provided in the command line flags.
func loadSystemSecurityPlan(path string) (*oscalTypes_1_1_3.SystemSecurityPlan, error) {
	var (
		data     []byte
		err      error
		fileData struct {
			SystemSecurityPlan oscalTypes_1_1_3.SystemSecurityPlan `json:"system-security-plan"`
		}
	)

	// Read the file data.
	data, err = os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	// Unmarshal the file data into the fileData struct.
	err = json.Unmarshal(data, &fileData)
	if err != nil {
		return nil, err
	}

	// Return the System Security Plan data.
	return &fileData.SystemSecurityPlan, nil
}

// getPlugins gets the plugins directory from the command line flags.
func getPlugins(cmd *cobra.Command) (string, error) {
	var (
		err        error
		pluginsDir string
	)

	pluginsDir, err = cmd.Flags().GetString("plugins-dir")
	if err != nil {
		return "", err
	}

	return pluginsDir, nil
}

// getAddress gets the address to listen on from the command line flags.
func getAddress(cmd *cobra.Command) (string, error) {
	var (
		address string
		err     error
		port    int
	)

	// Get the port to listen on.
	port, err = cmd.Flags().GetInt("port")
	if err != nil {
		return "", err
	}

	// Format the address to listen on.
	address = fmt.Sprintf(":%d", port)

	// Return the address to listen on.
	return address, nil
}

// getServer initializes an HTTP server with the appropriate routes and handlers.
func getServer(cmd *cobra.Command) (*echo.Echo, error) {
	var (
		err                        error
		server                     *echo.Echo
		state                      *State
		systemSecurityPlan         *oscalTypes_1_1_3.SystemSecurityPlan
		systemSecurityPlanFilePath string
	)

	// Get the file path of the System Security Plan.
	systemSecurityPlanFilePath, err = getSystemSecurityPlanFilePath(cmd)
	if err != nil {
		return nil, err
	}

	if systemSecurityPlanFilePath != "" {
		// Load the System Security Plan specified.
		systemSecurityPlan, err = loadSystemSecurityPlan(systemSecurityPlanFilePath)
		if err != nil {
			return nil, err
		}

		// Init the state, starting at the review step since the SSP is already loaded.
		state = &State{
			SystemSecurityPlan:         systemSecurityPlan,
			SystemSecurityPlanFilePath: systemSecurityPlanFilePath,
			CurrentStep:                2,
		}
	} else {
		// Init the state without a System Security Plan.
		state = &State{CurrentStep: 1}
	}

	// Init an HTTP server.
	server = echo.New()

	// Set the HTML template renderer.
	server.Renderer = &models.CustomTemplateRenderer{
		Templates: template.Must(template.ParseFS(templateDir, "templates/*.html")),
	}

	// Set the HTTP request validator.
	server.Validator = &models.CustomValidator{
		Validator: validator.New(),
	}

	// Set up middleware.
	server.Use(middleware.RequestLogger())

	// Set up routes.
	server.GET("/", state.getIndex)
	server.GET("/images/*", echo.WrapHandler(http.FileServer(http.FS(imagesDir))))
	server.GET("/scripts/*", echo.WrapHandler(http.FileServer(http.FS(scriptsDir))))
	server.GET("/styles/*", echo.WrapHandler(http.FileServer(http.FS(stylesDir))))
	server.POST("/system-security-plan", state.postSystemSecurityPlan)
	server.GET("/components/new-block", state.getComponentBlock)
	server.POST("/back", state.postBack)
	server.POST("/next", state.postNext)
	server.POST("/export", state.postExport)

	// Return the initialized HTTP server.
	return server, nil
}

// server starts the HTTP server that will serve the UI for creating mappings between OSCAL components and security rule IDs.
func server(cmd *cobra.Command, args []string) error {
	var (
		address string
		err     error
		server  *echo.Echo
	)

	// Get the address to listen on.
	address, err = getAddress(cmd)
	if err != nil {
		return err
	}

	// Init an HTTP server.
	server, err = getServer(cmd)
	if err != nil {
		return err
	}

	// Start the HTTP server.
	err = server.Start(address)
	if err != nil {
		return err
	}
	return nil
}
