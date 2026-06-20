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
	"encoding/json"
	"mime/multipart"
	"net/http"

	oscalTypes_1_1_3 "github.com/defenseunicorns/go-oscal/src/types/oscal-1-1-3"
	"github.com/labstack/echo/v5"
)

// State is the state of the server. It holds the file path and data of the System Security Plan.
type State struct {
	SystemSecurityPlanFilePath string
	SystemSecurityPlan         *oscalTypes_1_1_3.SystemSecurityPlan
}

// getIndex is the handler for the index page. It renders the index.html template and passes the System Security Plan data and file path to the template.
func (state *State) getIndex(context *echo.Context) error {
	return context.Render(http.StatusOK, "index.html", map[string]any{
		"SystemSecurityPlanFilePath": state.SystemSecurityPlanFilePath,
		"SystemSecurityPlan":         state.SystemSecurityPlan,
	})
}

// postSystemSecurityPlan is the handler for the /system-security-plan endpoint. It handles the upload of a System Security Plan file, decodes it, and updates the state with the loaded data.
func (state *State) postSystemSecurityPlan(context *echo.Context) error {
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

	// Update the state with the loaded System Security Plan.
	state.SystemSecurityPlanFilePath = fileHeader.Filename
	state.SystemSecurityPlan = &fileData.SystemSecurityPlan

	return context.Render(http.StatusOK, "show-system-security-plan.html", map[string]any{
		"SystemSecurityPlanFilePath": state.SystemSecurityPlanFilePath,
		"SystemSecurityPlan":         state.SystemSecurityPlan,
	})
}

// postReset clears the server's state and re-renders the main content.
func (state *State) postReset(context *echo.Context) error {
	state.SystemSecurityPlanFilePath = ""
	state.SystemSecurityPlan = nil

	return context.Render(http.StatusOK, "load-system-security-plan.html", map[string]any{
		"SystemSecurityPlanFilePath": state.SystemSecurityPlanFilePath,
		"SystemSecurityPlan":         state.SystemSecurityPlan,
	})
}
