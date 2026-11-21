package tui

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/deathlabs/finch/models"
	"gopkg.in/yaml.v2"
)

func TestMappingWorkflow(t *testing.T) {
	var (
		command    tea.Cmd
		document   models.ExportDocument
		message    tea.Msg
		modelValue model
		output     []byte
		path       string
		planPath   string
		err        error
	)
	planPath = filepath.Join(t.TempDir(), "ssp.json")
	err = os.WriteFile(planPath, []byte(`{"system-security-plan":{"uuid":"ssp-id","metadata":{"title":"Example","version":"1"},"control-implementation":{"implemented-requirements":[{"control-id":"ac-1"}]}}}`), 0600)
	if err != nil {
		t.Fatal(err)
	}
	modelValue = newModel(planPath)
	message = modelValue.Init()()
	modelValue, _ = updateModel(modelValue, message)
	if modelValue.step != 1 || len(modelValue.controls) != 1 {
		t.Fatalf("SSP not loaded: %+v", modelValue)
	}
	for _, value := range []string{"Gateway", "service", "Description", "Purpose", "ac-1", "ZAP", "40012"} {
		modelValue.input = value
		command = modelValue.submit()
		if command != nil || modelValue.message != "" {
			t.Fatalf("input %q: %s", value, modelValue.message)
		}
	}
	modelValue, _ = updateModel(modelValue, tea.KeyMsg{Type: tea.KeyCtrlD})
	modelValue.input = "n"
	modelValue.submit()
	if modelValue.step != 9 || len(modelValue.components) != 1 {
		t.Fatalf("unexpected state: %+v", modelValue)
	}
	path = filepath.Join(t.TempDir(), "mappings.yaml")
	modelValue.input = path
	command = modelValue.submit()
	message = command()
	modelValue, _ = updateModel(modelValue, message)
	if modelValue.step != 11 {
		t.Fatalf("save failed: %s", modelValue.message)
	}
	output, err = os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	err = yaml.Unmarshal(output, &document)
	if err != nil {
		t.Fatal(err)
	}
	if document.Metadata.SystemSecurityPlanUUID != "ssp-id" || document.Components[0].SecurityRulesMitigatedByThisComponent[0].Rules[0] != "40012" {
		t.Fatalf("unexpected output: %s", output)
	}
}

func TestInvalidControlAndLoadError(t *testing.T) {
	var (
		current model
		result  tea.Msg
	)
	current = newModel("")
	result = load(filepath.Join(t.TempDir(), "missing"))()
	current, _ = updateModel(current, result)
	if current.step != 0 || current.message == "" {
		t.Fatal("load error was not shown")
	}
	current.step = 5
	current.controls = []string{"ac-1"}
	current.input = "ac-2"
	current.submit()
	if current.step != 5 || !strings.Contains(current.message, "ac-2") {
		t.Fatal("invalid control accepted")
	}
}

func updateModel(current model, message tea.Msg) (model, tea.Cmd) {
	var (
		command tea.Cmd
		updated tea.Model
	)
	updated, command = current.Update(message)
	return updated.(model), command
}

func TestPathCompletion(t *testing.T) {
	var (
		command tea.Cmd
		current model
		err     error
		file    string
		root    string
	)
	root = t.TempDir()
	err = os.Mkdir(filepath.Join(root, "nested"), 0700)
	if err != nil {
		t.Fatal(err)
	}
	for _, file = range []string{"alpha.json", "nested/beta.JSON", "notes.txt"} {
		err = os.WriteFile(filepath.Join(root, file), []byte("{}"), 0600)
		if err != nil {
			t.Fatal(err)
		}
	}
	current = newModel("")
	current.completionRequest = 1
	current, _ = updateModel(current, findCompletions(root, "", 1)())
	if current.input != "alpha.json" {
		t.Fatalf("first completion: %q", current.input)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyTab})
	if current.input != filepath.Join("nested", "beta.JSON") {
		t.Fatalf("second completion: %q", current.input)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyTab})
	if current.input != "alpha.json" {
		t.Fatalf("completion did not wrap: %q", current.input)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune("x")})
	if len(current.completionMatches) != 0 {
		t.Fatal("editing did not reset matches")
	}
	current.completionRequest++
	command = findCompletions(root, "nested/", current.completionRequest)
	current, _ = updateModel(current, command())
	if current.input != filepath.Join("nested", "beta.JSON") {
		t.Fatalf("prefix completion: %q", current.input)
	}
	current, _ = updateModel(current, findCompletions(root, "missing", current.completionRequest-1)())
	if current.message != "" {
		t.Fatal("stale result changed the model")
	}
	if !strings.Contains(current.View(), "System Security Plan File Path") {
		t.Fatal("path prompt missing")
	}
}

func TestComponentTypeSelection(t *testing.T) {
	var (
		current model
		view    string
	)
	current = newModel("")
	current.step = 2
	view = current.View()
	if !strings.Contains(view, "Component Type") || !strings.Contains(view, "> software") {
		t.Fatalf("unexpected view: %s", view)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyDown})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 3 || current.component.Type != "service" {
		t.Fatalf("unexpected selection: %+v", current.component)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEsc})
	if !strings.Contains(current.View(), "> service") {
		t.Fatal("selection was not retained")
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyUp})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyUp})
	if current.input != "hardware" {
		t.Fatalf("selection did not wrap: %q", current.input)
	}
}

func TestSecurityToolSelection(t *testing.T) {
	var (
		current model
		view    string
	)
	current = newModel("")
	current.step = 6
	view = current.View()
	if !strings.Contains(view, "Security Tool") || !strings.Contains(view, "> ZAP") {
		t.Fatalf("unexpected view: %s", view)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 7 || current.tool != "ZAP" {
		t.Fatalf("unexpected tool: %s", current.tool)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEsc})
	if !strings.Contains(current.View(), "> ZAP") {
		t.Fatal("tool selection was not retained")
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyCtrlD})
	if current.step != 8 {
		t.Fatalf("finish rules did not advance: %d", current.step)
	}
}

func TestZAPRuleSelection(t *testing.T) {
	var (
		current model
		view    string
	)
	current = newModel("")
	current.step = 7
	current.tool = "ZAP"
	current, _ = updateModel(current, tea.WindowSizeMsg{Width: 60, Height: 20})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeySpace})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyDown})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeySpace})
	view = current.View()
	if !strings.Contains(view, "[x] 10020") || !strings.Contains(view, "[x] 10021") {
		t.Fatalf("selections missing: %s", view)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyTab})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune("10020, 12345")})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 8 || len(current.rules) != 1 || strings.Join(current.rules[0].Rules, ",") != "10020,10021,12345" {
		t.Fatalf("unexpected rules: %+v", current.rules)
	}
}

func TestZAPRuleManualValidation(t *testing.T) {
	var current model
	current = newModel("")
	current.step = 7
	current.tool = "ZAP"
	current.zapManual = true
	current.input = "invalid"
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 7 || !strings.Contains(current.message, "digits") {
		t.Fatalf("invalid rule accepted: %+v", current)
	}
}

func TestComponentFieldDefaults(t *testing.T) {
	var current model
	current = newModel("")
	current.step = 1
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.component.Title != "WAF" {
		t.Fatalf("default title: %q", current.component.Title)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.component.Description != "Web App Firewall" {
		t.Fatalf("default description: %q", current.component.Description)
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEsc})
	current.input = "Custom description"
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.component.Description != "Custom description" {
		t.Fatalf("custom description: %q", current.component.Description)
	}
}

func TestSSPPathDefault(t *testing.T) {
	var (
		command tea.Cmd
		current model
		result  loaded
	)
	current = newModel("")
	if !strings.Contains(current.View(), "ssp.json") {
		t.Fatal("default path placeholder missing")
	}
	current, command = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if command == nil || !current.busy {
		t.Fatal("default path did not start loading")
	}
	result = command().(loaded)
	if result.path != "ssp.json" {
		t.Fatalf("path = %q", result.path)
	}
}

func TestWorkflowTabs(t *testing.T) {
	var (
		current model
		step    int
		tab     string
		view    string
	)
	for step, tab = range map[int]string{0: "SSP", 1: "Component", 4: "Component", 5: "Controls", 6: "Rules", 8: "Rules", 9: "Export", 11: "Export"} {
		current = newModel("")
		current.step = step
		current.width = 40
		view = current.View()
		if !strings.Contains(view, tab) {
			t.Fatalf("step %d missing tab %s: %s", step, tab, view)
		}
	}
}

func TestSSPHeaderBeforeLoading(t *testing.T) {
	var view string
	view = newModel("").View()
	if !strings.Contains(view, "SSP: None") || !strings.Contains(view, "Components Created: 0") {
		t.Fatalf("missing status: %s", view)
	}
}

func TestControlsSelection(t *testing.T) {
	var (
		current model
		line    string
		row     int
		view    []string
	)
	current = newModel("")
	current.step = 5
	current.controls = []string{"ac-1", "ac-2", "ac-3"}
	current, _ = updateModel(current, tea.WindowSizeMsg{Width: 80, Height: 18})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeySpace})
	current, _ = updateModel(current, tea.MouseMsg{Button: tea.MouseButtonWheelDown})
	view = strings.Split(current.View(), "\n")
	for row, line = range view {
		if strings.Contains(line, "[ ] ac-2") {
			break
		}
	}
	if row >= len(view) {
		t.Fatal("second control not visible")
	}
	current, _ = updateModel(current, tea.MouseMsg{Button: tea.MouseButtonLeft, Action: tea.MouseActionPress, Y: row})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 6 || strings.Join(current.component.ControlsImplementedByThisComponent, ",") != "ac-1,ac-2" {
		t.Fatalf("selected controls: %v", current.component.ControlsImplementedByThisComponent)
	}
}

func TestExportConfirmationHeader(t *testing.T) {
	var (
		current model
		view    string
	)
	current = newModel("")
	current.step = 11
	current.message = "Saved mappings.yaml"
	view = current.View()
	if !strings.Contains(view, "SSP: None") || !strings.Contains(view, "Components Created: 0") || !strings.Contains(view, "Saved mappings.yaml") {
		t.Fatalf("missing status or confirmation: %s", view)
	}
}

func TestExportConfirmationIgnoresEnter(t *testing.T) {
	var (
		command tea.Cmd
		current model
	)
	current = newModel("")
	current.step = 11
	current.message = "Saved mappings.yaml"
	current, command = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 11 || current.message != "Saved mappings.yaml" || command != nil {
		t.Fatalf("confirmation changed: %+v", current)
	}
	_, command = updateModel(current, tea.KeyMsg{Type: tea.KeyCtrlC})
	if command == nil {
		t.Fatal("Ctrl+C did not quit")
	}
}

func TestAddAnotherComponentAfterRules(t *testing.T) {
	var current model
	current = newModel("")
	current.step = 7
	current.tool = "ZAP"
	current.zapSelected = map[string]bool{"10020": true}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 8 || !strings.Contains(current.View(), "Add another component?") {
		t.Fatal("component prompt not shown after rules")
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyCtrlT})
	if current.step != 6 || len(current.rules) != 1 {
		t.Fatal("could not add another tool")
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyCtrlD})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune("y")})
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEnter})
	if current.step != 1 || len(current.components) != 1 {
		t.Fatal("could not start another component")
	}
}

func TestStartOverAfterExport(t *testing.T) {
	var current model
	current = newModel("old.json")
	current.step = 11
	current.message = "Saved mappings.yaml"
	current.components = []models.ComponentDefinition{{Title: "Old component"}}
	current.width = 100
	current.height = 30
	if !strings.Contains(current.View(), "ctrl+r: start over") {
		t.Fatal("restart shortcut missing")
	}
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyCtrlR})
	if current.step != 0 || current.path != "" || current.plan != nil || len(current.components) != 0 || current.message != "" || current.width != 100 || current.height != 30 {
		t.Fatalf("workflow not reset: %+v", current)
	}
}

func TestBackFromComponentTitle(t *testing.T) {
	var current model
	current = newModel("")
	current.step = 1
	current.path = "example.json"
	current, _ = updateModel(current, tea.KeyMsg{Type: tea.KeyEsc})
	if current.step != 0 || current.input != "example.json" {
		t.Fatalf("did not return to SSP path: %+v", current)
	}
}
