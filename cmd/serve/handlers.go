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
	"fmt"
	"mime/multipart"
	"net/http"
	"sort"

	"github.com/deathlabs/finch/models"
	oscalTypes_1_1_3 "github.com/defenseunicorns/go-oscal/src/types/oscal-1-1-3"
	"github.com/google/uuid"
	"github.com/labstack/echo/v5"
	"gopkg.in/yaml.v2"
)

// State is the state of the server. It holds the file path and data of the System Security Plan.
type State struct {
	CurrentStep                int
	SystemSecurityPlanFilePath string
	SystemSecurityPlan         *oscalTypes_1_1_3.SystemSecurityPlan
	Components                 []models.ComponentDefinition
}

// ComponentBlockData is the data structure passed to the component form block template.
type ComponentBlockData struct {
	Index             any
	AvailableControls []string
}

// Step describes one step in the stepper and the template that renders it.
type Step struct {
	Number   int
	Label    string
	Template string
}

// stepper defines the order in which to render each template.
var stepper = []Step{
	{Number: 1, Label: "Load System Security Plan", Template: "step-1.html"},
	{Number: 2, Label: "Create OSCAL Components", Template: "step-2.html"},
	{Number: 3, Label: "Map OSCAL Components to Security Rules", Template: "step-3.html"},
}

// viewData assembles the data every template needs.
func (state *State) viewData() map[string]any {
	return map[string]any{
		"Steps":                      stepper,
		"CurrentStep":                state.CurrentStep,
		"SystemSecurityPlanFilePath": state.SystemSecurityPlanFilePath,
		"SystemSecurityPlan":         state.SystemSecurityPlan,
		"InitialBlock":               ComponentBlockData{Index: 0, AvailableControls: state.getAvailableControlIDs()},
		"Components":                 state.Components,
		"AvailableRulesByTool":       state.getAvailableRulesByTool(),
	}
}

// getIndex is the handler for the "/" endpoint.
func (state *State) getIndex(context *echo.Context) error {
	return context.Render(http.StatusOK, "index.html", state.viewData())
}

// postSystemSecurityPlan is the handler for the "/system-security-plan" endpoint.
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
	state.CurrentStep = 2

	return context.Render(http.StatusOK, "stepper.html", state.viewData())
}

// getAvailableControlIDs returns a list of all control IDs present in the loaded System Security Plan.
func (state *State) getAvailableControlIDs() []string {
	var (
		controlIDs  []string
		requirement oscalTypes_1_1_3.ImplementedRequirement
	)

	if state.SystemSecurityPlan == nil || state.SystemSecurityPlan.ControlImplementation.ImplementedRequirements == nil {
		return controlIDs
	}

	// Collect all control IDs present in the uploaded SSP data
	for _, requirement = range state.SystemSecurityPlan.ControlImplementation.ImplementedRequirements {
		controlIDs = append(controlIDs, requirement.ControlId)
	}

	// Return the list of control IDs to populate the component form checkboxes.
	return controlIDs
}

func (state *State) getAvailableRulesByTool() map[string][]string {
	return map[string][]string{
		"ZAP": {
			"10021",
			"10062",
			"40012",
			"40003",
			"90019",
			"90022",
		},
		"Semgrep": {
			"python.django.security.audit.xss.filter-with-is-safe.filter-with-is-safe",
			"python.django.security.audit.xss.formathtml-fstring-parameter.formathtml-fstring-parameter",
			"python.django.security.injection.raw-html-format.raw-html-format",
			"python.django.security.injection.raw-html-template.raw-html-template",
			"python.django.security.injection.raw-html-template-parameter.raw-html-template-parameter",
			"python.django.security.injection.raw-html-template-variable.raw-html-template-variable",
			"python.django.security.injection.raw-html-variable.raw-html-variable",
			"python.django.security.injection.template-parameter.template-parameter",
			"python.django.security.injection.template-variable.template-variable",
		},
	}
}

// getComponentBlock is the handler for the "/components/new-block" endpoint.
func (state *State) getComponentBlock(context *echo.Context) error {
	var (
		data  map[string]any
		index string
	)

	// Parse a query parameter to know which block index we are creating.
	index = context.QueryParam("index")
	if index == "" {
		index = "0"
	}

	// Assemble the data for the template, including the list of available control IDs to populate the checkboxes.
	data = map[string]any{
		"Index":             index,
		"AvailableControls": state.getAvailableControlIDs(),
	}

	// Returns just the individual block markup.
	return context.Render(http.StatusOK, "oscal-component-form.html", data)
}

// postBack is the handler for the "/back" endpoint and moves the stepper back one step.
func (state *State) postBack(context *echo.Context) error {
	if state.CurrentStep > 1 {
		state.CurrentStep--
	}
	return context.Render(http.StatusOK, "stepper.html", state.viewData())
}

// postNext is the handler for the "/next" endpoint and moves the stepper forward one step.
func (state *State) postNext(context *echo.Context) error {
	var (
		component        models.ComponentDefinition
		controlKey       string
		controls         []string
		descriptionKey   string
		err              error
		index            int
		parsedComponents []models.ComponentDefinition
		purposeKey       string
		ok               bool
		request          *http.Request
		titleKey         string
		typeKey          string
	)

	// Parse the form data from the request.
	request = context.Request()
	if err = request.ParseForm(); err != nil {
		return context.String(http.StatusBadRequest, "failed to parse form data")
	}

	// If transitioning out of Step 2, parse and commit the components to state.
	if state.CurrentStep == 2 {
		index = 0
		for {
			titleKey = fmt.Sprintf("component_title_%d", index)
			if _, ok = request.Form[titleKey]; !ok {
				break
			}

			component = models.ComponentDefinition{
				UUID:  uuid.NewString(),
				Title: request.Form.Get(titleKey),
			}

			typeKey = fmt.Sprintf("component_type_%d", index)
			component.Type = request.Form.Get(typeKey)

			descriptionKey = fmt.Sprintf("component_description_%d", index)
			component.Description = request.Form.Get(descriptionKey)

			purposeKey = fmt.Sprintf("component_purpose_%d", index)
			component.Purpose = request.Form.Get(purposeKey)

			controlKey = fmt.Sprintf("component_control_%d", index)
			if controls, ok = request.Form[controlKey]; ok {
				sort.Strings(controls)
				component.ControlsImplementedByThisComponent = controls
			}

			parsedComponents = append(parsedComponents, component)
			index++
		}

		state.Components = parsedComponents
	}

	// If transitioning out of Step 3, parse and commit rule mappings to each component.
	if state.CurrentStep == 3 {
		state.applyRuleMappings(request)
	}

	// Move to the next step if not already at the end of the stepper.
	if state.CurrentStep < len(stepper) {
		state.CurrentStep++
	}

	// Render the new step.
	return context.Render(http.StatusOK, "stepper.html", state.viewData())
}

// applyRuleMappings parses submitted rule checkboxes and commits them to each component in state.
func (state *State) applyRuleMappings(request *http.Request) {
	var (
		index     int
		ok        bool
		ruleKey   string
		rules     []string
		toolRules []models.ToolRules
		tool      string
		tools     []string
	)

	tools = make([]string, 0, len(state.getAvailableRulesByTool()))
	for tool = range state.getAvailableRulesByTool() {
		tools = append(tools, tool)
	}
	sort.Strings(tools)

	for index = range state.Components {
		toolRules = nil
		for _, tool = range tools {
			ruleKey = fmt.Sprintf("component_rules_%d_%s", index, tool)
			if rules, ok = request.Form[ruleKey]; ok {
				sort.Strings(rules)
				toolRules = append(toolRules, models.ToolRules{
					Tool:  tool,
					Rules: rules,
				})
			}
		}
		state.Components[index].SecurityRulesMitigatedByThisComponent = toolRules
	}
}

// postExport is the handler for the "/export" endpoint.
func (state *State) postExport(context *echo.Context) error {
	var (
		document models.ExportDocument
		err      error
		output   []byte
		request  *http.Request
	)

	request = context.Request()
	if err = request.ParseForm(); err != nil {
		return context.String(http.StatusBadRequest, "failed to parse form data")
	}

	state.applyRuleMappings(request)

	document = models.ExportDocument{Components: state.Components}

	output, err = yaml.Marshal(document)
	if err != nil {
		return context.String(http.StatusInternalServerError, "failed to generate yaml: "+err.Error())
	}

	context.Response().Header().Set("Content-Disposition", "attachment; filename=mappings.yaml")
	return context.Blob(http.StatusOK, "application/x-yaml", output)
}
