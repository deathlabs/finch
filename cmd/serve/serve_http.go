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
	"mime/multipart"
	"net/http"

	"github.com/deathlabs/finch/models"
	oscalTypes_1_1_3 "github.com/defenseunicorns/go-oscal/src/types/oscal-1-1-3"
	"github.com/go-playground/validator/v10"
	"github.com/labstack/echo/v5"
	"github.com/labstack/echo/v5/middleware"
	"github.com/spf13/cobra"
)

//go:embed templates/*
var templateDir embed.FS

//go:embed static/*
var staticDir embed.FS

var port int

var serveHTTPCmd = &cobra.Command{
	Use:   "http",
	Short: "Serve a HTTP-based UI for creating mappings between OSCAL components and security rule IDs",
	RunE:  serveHTTP,
}

func getSSP(cmd *cobra.Command) error {
	var (
		err         error
		sspFilePath string
	)

	sspFilePath, err = cmd.Flags().GetString("ssp")
	if err != nil {
		return err
	}
	fmt.Println(sspFilePath)
	return nil
}

func getPlugins(cmd *cobra.Command) error {
	var (
		err        error
		pluginsDir string
	)

	pluginsDir, err = cmd.Flags().GetString("plugins-dir")
	if err != nil {
		return err
	}
	fmt.Println(pluginsDir)
	return nil
}

func getAddress(cmd *cobra.Command) (string, error) {
	var (
		address string
		err     error
		port    int
	)

	port, err = cmd.Flags().GetInt("port")
	if err != nil {
		return "", err
	}

	address = fmt.Sprintf(":%d", port)
	return address, nil
}

func getHomePage(context *echo.Context) error {
	return context.Render(http.StatusOK, "index.html", nil)
}

func uploadSystemSecurityPlan(context *echo.Context) error {
	var (
		decoder  *json.Decoder
		err      error
		file     multipart.File
		fileData struct {
			SystemSecurityPlan oscalTypes_1_1_3.SystemSecurityPlan `json:"system-security-plan"`
		}
		fileHeader *multipart.FileHeader
	)

	// Get the file.
	fileHeader, err = context.FormFile("System-Security-Plan")
	if err != nil {
		return context.String(http.StatusBadRequest, "could not get the file")
	}

	// Open the file.
	file, err = fileHeader.Open()
	if err != nil {
		return context.String(http.StatusInternalServerError, "could not open the file")
	}

	// Init a JSON decoder to decode the file.
	decoder = json.NewDecoder(file)

	// Decode the file.
	err = decoder.Decode(&fileData)
	if err != nil {
		return context.String(http.StatusBadRequest, "invalid json: "+err.Error())
	}

	// Close the file.
	err = file.Close()
	if err != nil {
		return context.String(http.StatusInternalServerError, "could not close the file")
	}

	return context.Render(http.StatusOK, "system-security-plan.html", fileData.SystemSecurityPlan.Metadata.Title)
}

func getServer() (*echo.Echo, error) {
	var server *echo.Echo

	server = echo.New()

	server.Renderer = &models.CustomTemplateRenderer{
		Templates: template.Must(template.ParseFS(templateDir, "templates/*.html")),
	}

	server.Validator = &models.CustomValidator{
		Validator: validator.New(),
	}

	server.Use(middleware.RequestLogger())

	server.GET("/", getHomePage)
	server.GET("/static/*", echo.WrapHandler(http.FileServer(http.FS(staticDir))))
	server.POST("/system-security-plan", uploadSystemSecurityPlan)

	return server, nil
}

func serveHTTP(cmd *cobra.Command, args []string) error {
	var (
		address string
		err     error
		server  *echo.Echo
	)

	address, err = getAddress(cmd)
	if err != nil {
		return err
	}

	server, err = getServer()
	if err != nil {
		return err
	}

	err = server.Start(address)
	if err != nil {
		return err
	}
	return nil
}

func init() {
	serveHTTPCmd.PersistentFlags().IntVarP(&port, "port", "p", 8000, "Output filename for the component definition template")
}
