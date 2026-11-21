package tui

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"
	"unicode/utf8"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/charmbracelet/x/ansi"
	"github.com/deathlabs/finch/models"
	oscal "github.com/defenseunicorns/go-oscal/src/types/oscal-1-1-3"
	"github.com/google/uuid"
	"gopkg.in/yaml.v2"
)

type model struct {
	step              int
	input             string
	path              string
	plan              *oscal.SystemSecurityPlan
	controls          []string
	components        []models.ComponentDefinition
	component         models.ComponentDefinition
	rules             []models.ToolRules
	tool              string
	message           string
	busy              bool
	width             int
	height            int
	zapIndex          int
	controlIndex      int
	controlSelected   map[string]bool
	zapSelected       map[string]bool
	zapManual         bool
	completionMatches []string
	completionIndex   int
	completionRequest int
}
type loaded struct {
	plan *oscal.SystemSecurityPlan
	path string
	err  error
}
type completions struct {
	request int
	matches []string
	err     error
}
type saved struct {
	path string
	err  error
}

func newModel(path string) model { return model{path: path, width: 80} }
func (m model) Init() tea.Cmd {
	if m.path != "" {
		return load(m.path)
	}
	return nil
}
func load(path string) tea.Cmd {
	return func() tea.Msg {
		var (
			data     []byte
			document struct {
				Plan *oscal.SystemSecurityPlan `json:"system-security-plan"`
			}
			err error
		)
		data, err = os.ReadFile(path)
		if err == nil {
			err = json.Unmarshal(data, &document)
		}
		if err == nil && document.Plan == nil {
			err = fmt.Errorf("missing system-security-plan")
		}
		return loaded{plan: document.Plan, path: path, err: err}
	}
}
func save(path string, document models.ExportDocument) tea.Cmd {
	return func() tea.Msg {
		var (
			data []byte
			err  error
		)
		data, err = yaml.Marshal(document)
		if err == nil {
			err = os.WriteFile(path, data, 0600)
		}
		return saved{path: path, err: err}
	}
}

// findCompletions searches from the current directory without blocking the interface.
func findCompletions(root string, prefix string, request int) tea.Cmd {
	return func() tea.Msg {
		var (
			err     error
			matches []string
		)
		err = filepath.WalkDir(root, func(path string, entry fs.DirEntry, walkError error) error {
			var (
				relative      string
				relativeError error
			)
			if walkError != nil {
				return walkError
			}
			if entry.IsDir() && entry.Name() == ".git" {
				return filepath.SkipDir
			}
			if !entry.Type().IsRegular() || !strings.EqualFold(filepath.Ext(path), ".json") {
				return nil
			}
			relative, relativeError = filepath.Rel(root, path)
			if relativeError != nil {
				return relativeError
			}
			if strings.HasPrefix(prefix, "."+string(filepath.Separator)) {
				relative = "." + string(filepath.Separator) + relative
			}
			if strings.HasPrefix(relative, prefix) {
				matches = append(matches, relative)
			}
			return nil
		})
		return completions{request: request, matches: matches, err: err}
	}
}

func (m *model) moveControl(offset int) {
	if len(m.controls) == 0 {
		return
	}
	m.controlIndex = (m.controlIndex + offset + len(m.controls)) % len(m.controls)
}

func (m *model) toggleControl(index int) {
	if m.controlSelected == nil {
		m.controlSelected = make(map[string]bool)
	}
	m.controlIndex = index
	m.controlSelected[m.controls[index]] = !m.controlSelected[m.controls[index]]
}

func (m *model) clickControl(row int) {
	var (
		control string
		index   int
		line    string
		lines   []string
	)
	lines = strings.Split(m.View(), "\n")
	if row < 0 || row >= len(lines) {
		return
	}
	line = ansi.Strip(lines[row])
	for index, control = range m.controls {
		if line == "> [ ] "+control || line == "> [x] "+control || line == "  [ ] "+control || line == "  [x] "+control {
			m.toggleControl(index)
			return
		}
	}
}

func (m model) Update(message tea.Msg) (tea.Model, tea.Cmd) {
	var (
		command     tea.Cmd
		key         tea.KeyMsg
		ok          bool
		result      loaded
		savedResult saved
		choices     []string
		index       int
	)
	switch message := message.(type) {
	case tea.MouseMsg:
		if m.step == 5 {
			if message.Button == tea.MouseButtonWheelUp {
				m.moveControl(-1)
			}
			if message.Button == tea.MouseButtonWheelDown {
				m.moveControl(1)
			}
			if message.Button == tea.MouseButtonLeft && message.Action == tea.MouseActionPress {
				m.clickControl(message.Y)
			}
		}
		return m, nil
	case tea.WindowSizeMsg:
		m.width = message.Width
		m.height = message.Height
		return m, nil
	case loaded:
		result = message
		m.busy = false
		if result.err != nil {
			m.message = result.err.Error()
			m.step = 0
			return m, nil
		}
		m.plan = result.plan
		m.path = result.path
		m.controls = nil
		for _, requirement := range m.plan.ControlImplementation.ImplementedRequirements {
			m.controls = append(m.controls, requirement.ControlId)
		}
		sort.Strings(m.controls)
		m.step = 1
		m.input = ""
		m.message = ""
		return m, nil
	case completions:
		if message.request != m.completionRequest || m.step != 0 {
			return m, nil
		}
		if message.err != nil {
			m.message = message.err.Error()
			return m, nil
		}
		m.completionMatches = message.matches
		m.completionIndex = 0
		if len(m.completionMatches) == 0 {
			m.message = "No matching JSON files found."
			return m, nil
		}
		m.input = m.completionMatches[0]
		m.message = ""
		return m, nil
	case saved:
		savedResult = message
		m.busy = false
		if savedResult.err != nil {
			m.message = savedResult.err.Error()
			return m, nil
		}
		m.message = "Saved " + savedResult.path
		m.step = 11
		return m, nil
	}
	key, ok = message.(tea.KeyMsg)
	if !ok {
		return m, nil
	}
	if key.Type == tea.KeyCtrlC {
		return m, tea.Quit
	}
	if m.busy || m.step == 11 {
		return m, nil
	}
	if m.step == 5 {
		switch key.Type {
		case tea.KeyUp:
			m.moveControl(-1)
			return m, nil
		case tea.KeyDown:
			m.moveControl(1)
			return m, nil
		case tea.KeySpace:
			if len(m.controls) > 0 {
				m.toggleControl(m.controlIndex)
			}
			return m, nil
		case tea.KeyEnter, tea.KeyEsc:
		default:
			return m, nil
		}
	}
	if m.step == 7 && m.tool == "ZAP" {
		if key.Type == tea.KeyTab {
			m.zapManual = !m.zapManual
			return m, nil
		}
		if !m.zapManual {
			switch key.Type {
			case tea.KeyUp:
				m.zapIndex = (m.zapIndex + len(zapRules) - 1) % len(zapRules)
				return m, nil
			case tea.KeyDown:
				m.zapIndex = (m.zapIndex + 1) % len(zapRules)
				return m, nil
			case tea.KeySpace:
				if m.zapSelected == nil {
					m.zapSelected = make(map[string]bool)
				}
				m.zapSelected[zapRules[m.zapIndex]] = !m.zapSelected[zapRules[m.zapIndex]]
				return m, nil
			case tea.KeyEnter, tea.KeyEsc:
			default:
				return m, nil
			}
		}
	}
	if m.step == 6 && key.Type == tea.KeyCtrlD {
		m.step = 8
		m.input = ""
		m.message = ""
		return m, nil
	}
	if m.step == 2 || m.step == 6 {
		choices = componentTypes
		if m.step == 6 {
			choices = securityTools
		}
		if key.Type == tea.KeyUp || key.Type == tea.KeyDown {
			for index = 0; index < len(choices); index++ {
				if choices[index] == m.input {
					break
				}
			}
			if index == len(choices) {
				index = 0
			}
			if key.Type == tea.KeyDown {
				index = (index + 1) % len(choices)
			}
			if key.Type == tea.KeyUp {
				index = (index + len(choices) - 1) % len(choices)
			}
			m.input = choices[index]
			return m, nil
		}
		if key.Type != tea.KeyEnter && key.Type != tea.KeyEsc {
			return m, nil
		}
	}
	if key.Type == tea.KeyTab && m.step == 0 {
		if len(m.completionMatches) > 0 {
			m.completionIndex = (m.completionIndex + 1) % len(m.completionMatches)
			m.input = m.completionMatches[m.completionIndex]
			return m, nil
		}
		m.completionRequest++
		return m, findCompletions(".", m.input, m.completionRequest)
	}
	if key.Type == tea.KeyBackspace || key.Type == tea.KeyCtrlH || key.Type == tea.KeyRunes || key.Type == tea.KeySpace || key.Type == tea.KeyEnter {
		m.completionMatches = nil
		m.completionRequest++
	}
	if key.Type == tea.KeyEsc {
		if m.step > 1 && m.step < 9 {
			m.step--
			m.input = m.previous()
			m.message = ""
		}
		return m, nil
	}
	if key.Type == tea.KeyBackspace || key.Type == tea.KeyCtrlH {
		if len(m.input) > 0 {
			_, size := utf8.DecodeLastRuneInString(m.input)
			m.input = m.input[:len(m.input)-size]
		}
		return m, nil
	}
	if key.Type == tea.KeyEnter {
		command = m.submit()
		return m, command
	}
	if key.Type == tea.KeyRunes {
		m.input += string(key.Runes)
	}
	if key.Type == tea.KeySpace {
		m.input += " "
	}
	return m, nil
}
func (m model) previous() string {
	switch m.step {
	case 1:
		return m.component.Title
	case 2:
		return m.component.Type
	case 3:
		return m.component.Description
	case 4:
		return m.component.Purpose
	case 5:
		return strings.Join(m.component.ControlsImplementedByThisComponent, ", ")
	case 6:
		return m.tool
	}
	return ""
}
func split(value string) []string {
	var (
		item   string
		items  []string
		result []string
	)
	items = strings.Split(value, ",")
	for _, item = range items {
		item = strings.TrimSpace(item)
		if item != "" {
			result = append(result, item)
		}
	}
	return result
}
func (m *model) submit() tea.Cmd {
	var (
		control   string
		controls  []string
		document  models.ExportDocument
		found     bool
		input     string
		item      string
		metadata  *models.ExportMetadata
		rule      string
		ruleIDs   []string
		seen      map[string]bool
		character rune
	)
	input = strings.TrimSpace(m.input)
	m.message = ""
	switch m.step {
	case 0:
		if input == "" {
			input = defaultSSPPath
		}
		m.busy = true
		return load(input)
	case 1:
		if input == "" {
			input = defaultComponentTitle
		}
		m.component.Title = input
	case 2:
		input = strings.ToLower(input)
		if input == "" {
			input = "software"
		}
		if input != "software" && input != "service" && input != "hardware" {
			m.message = "Enter software, service, or hardware."
			return nil
		}
		m.component.Type = input
	case 3:
		if input == "" {
			input = defaultComponentDescription
		}
		m.component.Description = input
	case 4:
		if input == "" {
			input = defaultComponentPurpose
		}
		m.component.Purpose = input
		if m.controlSelected == nil {
			m.controlSelected = make(map[string]bool)
			for _, control = range m.component.ControlsImplementedByThisComponent {
				m.controlSelected[control] = true
			}
		}
	case 5:
		controls = split(input)
		for control = range m.controlSelected {
			if m.controlSelected[control] {
				controls = append(controls, control)
			}
		}
		for _, control = range controls {
			found = false
			for _, item = range m.controls {
				if control == item {
					found = true
					break
				}
			}
			if !found {
				m.message = "Control not found in SSP: " + control
				return nil
			}
		}
		sort.Strings(controls)
		m.component.ControlsImplementedByThisComponent = controls
	case 6:
		if input == "" {
			input = securityTools[0]
		}
		m.tool = input
	case 7:
		ruleIDs = split(input)
		if m.tool == "ZAP" {
			seen = make(map[string]bool)
			for _, rule = range zapRules {
				if m.zapSelected[rule] {
					seen[rule] = true
				}
			}
			for _, rule = range ruleIDs {
				for _, character = range rule {
					if character < '0' || character > '9' {
						m.message = "ZAP rule IDs must contain digits only."
						return nil
					}
				}
				seen[rule] = true
			}
			ruleIDs = nil
			for rule = range seen {
				ruleIDs = append(ruleIDs, rule)
			}
			sort.Strings(ruleIDs)
		}
		if len(ruleIDs) == 0 {
			m.message = "Select or enter at least one rule ID."
			return nil
		}
		m.rules = append(m.rules, models.ToolRules{Tool: m.tool, Rules: ruleIDs})
		m.zapSelected = nil
		m.zapIndex = 0
		m.zapManual = false
		m.step = 6
		m.input = ""
		return nil
	case 8:
		if input != "" && strings.ToLower(input) != "y" && strings.ToLower(input) != "n" {
			m.message = "Enter y or n."
			return nil
		}
		m.component.UUID = uuid.NewString()
		m.component.SecurityRulesMitigatedByThisComponent = m.rules
		m.components = append(m.components, m.component)
		m.component = models.ComponentDefinition{}
		m.controlSelected = nil
		m.controlIndex = 0
		m.rules = nil
		m.tool = ""
		if strings.ToLower(input) == "y" {
			m.step = 1
			m.input = ""
			return nil
		}
		m.step = 9
		m.input = "mappings.yaml"
		return nil
	case 9:
		if input == "" {
			m.message = "Enter an output file path."
			return nil
		}
		metadata = &models.ExportMetadata{SystemSecurityPlanUUID: m.plan.UUID, SystemSecurityPlanTitle: m.plan.Metadata.Title, SystemSecurityPlanVersion: m.plan.Metadata.Version, SystemSecurityPlanLastModified: fmt.Sprint(m.plan.Metadata.LastModified), GeneratedAt: time.Now().UTC().Format(time.RFC3339)}
		document = models.ExportDocument{Metadata: metadata, Components: m.components}
		m.busy = true
		return save(input, document)
	}
	m.step++
	m.input = ""
	return nil
}

const (
	defaultSSPPath              = "ssp.json"
	defaultComponentTitle       = "WAF"
	defaultComponentPurpose     = "Filter web traffic"
	defaultComponentDescription = "Web App Firewall"
)

var (
	componentTypes = []string{"software", "service", "hardware"}
	securityTools  = []string{"ZAP", "Grype", "Semgrep"}
	zapRules       = []string{"10020", "10021", "10038", "10062", "10202", "40012", "40003", "40018", "90019", "90022"}
	titleStyle     = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.AdaptiveColor{Light: "#5E188F", Dark: "#E0C2F5"})
	promptStyle    = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.AdaptiveColor{Light: "#5E188F", Dark: "#E0C2F5"})
	inputStyle     = lipgloss.NewStyle().Foreground(lipgloss.AdaptiveColor{Light: "#1A1A1A", Dark: "#F2F2F2"})
	detailStyle    = lipgloss.NewStyle().Foreground(lipgloss.AdaptiveColor{Light: "#595959", Dark: "#A6A6A6"})
	errorStyle     = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Color("1"))
	helpStyle      = lipgloss.NewStyle().Foreground(lipgloss.AdaptiveColor{Light: "#595959", Dark: "#A6A6A6"})
	successStyle   = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Color("2"))
)

func renderLine(style lipgloss.Style, value string, width int) string {
	if width > 0 {
		value = ansi.Hardwrap(value, width, true)
	}
	return style.Render(value)
}

func renderHelp(width int, shortcuts ...[2]string) string {
	var (
		labels   []string
		shortcut [2]string
	)
	for _, shortcut = range shortcuts {
		labels = append(labels, strings.ToLower(shortcut[0])+": "+shortcut[1])
	}
	return renderLine(helpStyle, strings.Join(labels, " | "), width)
}

func (m model) workflowStage() int {
	switch {
	case m.step == 0:
		return 0
	case m.step <= 4:
		return 1
	case m.step == 5:
		return 2
	case m.step <= 8:
		return 3
	default:
		return 4
	}
}

func (m model) renderProgress() string {
	var (
		active int
		index  int
		label  string
		labels []string
		row    string
		stages []string
	)
	stages = []string{"Load SSP", "Create a Component", "Map Controls to Component", "Map Rules to Controls", "Export Mappings"}
	active = m.workflowStage()
	for index, label = range stages {
		if index < active || m.step == 11 {
			labels = append(labels, titleStyle.Render("● "+label))
		} else if index == active {
			labels = append(labels, promptStyle.Render("● "+label))
		} else {
			labels = append(labels, detailStyle.Render("○ "+label))
		}
	}
	row = strings.Join(labels, "  ")
	if m.width > 0 && lipgloss.Width(row) > m.width {
		row = renderLine(promptStyle, fmt.Sprintf("%d/5 %s", active+1, stages[active]), m.width)
	}
	return row
}

func (m model) View() string {
	var (
		prompt      string
		lines       []string
		inputLine   string
		placeholder string
		planTitle   string
		status      string
	)
	if m.step == 0 && m.path != "" && m.plan == nil && m.message == "" {
		return renderLine(titleStyle, "Finch", m.width) + "\n" + m.renderProgress() + "\n\n" + renderLine(detailStyle, "Loading SSP...", m.width) + "\n"
	}
	switch m.step {
	case 0:
		prompt = "System Security Plan File Path"
	case 1:
		prompt = "Component Title"
	case 2:
		prompt = "Component Type"
	case 3:
		prompt = "Component Description"
	case 4:
		prompt = "Component Purpose"
	case 5:
		prompt = "Controls Implemented"
	case 6:
		prompt = "Security Tool"
	case 7:
		prompt = "Rules for " + m.tool + " (comma separated)"
		if m.tool == "ZAP" {
			prompt = "ZAP Rules"
		}
	case 8:
		prompt = "Add Another Component? (y/N)"
	case 9:
		prompt = "Output File Path"
	}
	lines = append(lines, renderLine(titleStyle, "Finch", m.width), "")
	planTitle = "None"
	if m.plan != nil {
		planTitle = m.plan.Metadata.Title
	}
	status = ansi.Hardwrap(detailStyle.Render("SSP: ")+inputStyle.Render(planTitle)+detailStyle.Render(" | Components Created: ")+inputStyle.Render(fmt.Sprint(len(m.components))), m.width, true)
	lines = append(lines, m.renderProgress(), "")
	if m.step == 11 {
		lines = append(lines, renderLine(successStyle, m.message, m.width), "", status, "", renderHelp(m.width, [2]string{"Ctrl+C", "quit"}))
		return strings.Join(lines, "\n") + "\n"
	}
	inputLine = renderLine(inputStyle, "> "+m.input+"_", m.width)
	if m.step == 2 || m.step == 6 {
		var (
			choice   string
			choices  []string
			options  []string
			selected string
		)
		choices = componentTypes
		if m.step == 6 {
			choices = securityTools
		}
		selected = m.input
		if selected == "" {
			selected = choices[0]
		}
		for _, choice = range choices {
			if choice == selected {
				options = append(options, renderLine(inputStyle, "> "+choice, m.width))
				continue
			}
			options = append(options, renderLine(detailStyle, "  "+choice, m.width))
		}
		inputLine = strings.Join(options, "\n")
	}
	if m.step == 5 {
		var (
			end     int
			index   int
			marker  string
			options []string
			prefix  string
			start   int
			visible int
		)
		visible = len(m.controls)
		if m.height > 0 && m.height-15 < visible {
			visible = m.height - 15
		}
		if visible < 1 {
			visible = 1
		}
		if m.controlIndex >= visible {
			start = m.controlIndex - visible + 1
		}
		end = start + visible
		if end > len(m.controls) {
			end = len(m.controls)
		}
		for index = start; index < end; index++ {
			marker = "[ ]"
			if m.controlSelected[m.controls[index]] {
				marker = "[x]"
			}
			prefix = "  "
			if index == m.controlIndex {
				prefix = "> "
			}
			options = append(options, renderLine(inputStyle, prefix+marker+" "+m.controls[index], m.width))
		}
		if len(m.controls) == 0 {
			options = append(options, renderLine(detailStyle, "No controls found in the SSP.", m.width))
		}
		if start > 0 || end < len(m.controls) {
			options = append(options, renderLine(detailStyle, fmt.Sprintf("Controls %d-%d of %d", start+1, end, len(m.controls)), m.width))
		}
		inputLine = strings.Join(options, "\n")
	}
	if m.step == 7 && m.tool == "ZAP" {
		var (
			end     int
			index   int
			lines   []string
			marker  string
			prefix  string
			start   int
			visible int
		)
		visible = len(zapRules)
		if m.height > 0 && m.height-13 < visible {
			visible = m.height - 13
		}
		if visible < 1 {
			visible = 1
		}
		if m.zapIndex >= visible {
			start = m.zapIndex - visible + 1
		}
		end = start + visible
		if end > len(zapRules) {
			end = len(zapRules)
		}
		for index = start; index < end; index++ {
			marker = "[ ]"
			if m.zapSelected[zapRules[index]] {
				marker = "[x]"
			}
			prefix = "  "
			if index == m.zapIndex && !m.zapManual {
				prefix = "> "
			}
			lines = append(lines, renderLine(inputStyle, prefix+marker+" "+zapRules[index], m.width))
		}
		if start > 0 || end < len(zapRules) {
			lines = append(lines, renderLine(detailStyle, fmt.Sprintf("Rules %d-%d of %d", start+1, end, len(zapRules)), m.width))
		}
		prefix = "  "
		if m.zapManual {
			prefix = "> "
		}
		lines = append(lines, renderLine(detailStyle, prefix+"Additional IDs (comma separated): "+m.input, m.width))
		inputLine = strings.Join(lines, "\n")
	}
	if m.input == "" {
		switch m.step {
		case 0:
			placeholder = defaultSSPPath
		case 1:
			placeholder = defaultComponentTitle
		case 3:
			placeholder = defaultComponentDescription
		case 4:
			placeholder = defaultComponentPurpose
		}
		if placeholder != "" {
			inputLine = renderLine(inputStyle, "> ", m.width) + renderLine(detailStyle.Faint(true), placeholder, m.width-2)
		}
	}
	lines = append(lines, renderLine(promptStyle, prompt, m.width), inputLine, "")
	if m.message != "" {
		lines = append(lines, renderLine(errorStyle, m.message, m.width), "")
	}
	if m.busy {
		lines = append(lines, renderLine(detailStyle, "Working...", m.width))
	}
	lines = append(lines, "", status, "")
	if !m.busy {
		if m.step == 0 {
			lines = append(lines, renderHelp(m.width, [2]string{"Tab", "cycle JSON files"}, [2]string{"Enter", "continue"}, [2]string{"Ctrl+C", "quit"}))
		} else if m.step == 5 {
			lines = append(lines, renderHelp(m.width, [2]string{"↑/↓", "move"}, [2]string{"Space / click", "select"}, [2]string{"Enter", "continue"}, [2]string{"Esc", "back"}, [2]string{"Ctrl+C", "quit"}))
		} else if m.step == 7 && m.tool == "ZAP" {
			lines = append(lines, renderHelp(m.width, [2]string{"↑/↓", "move"}, [2]string{"Space", "select"}, [2]string{"Tab", "additional IDs"}, [2]string{"Enter", "continue"}, [2]string{"Esc", "back"}, [2]string{"Ctrl+C", "quit"}))
		} else if m.step == 6 {
			lines = append(lines, renderHelp(m.width, [2]string{"↑/↓", "select"}, [2]string{"Enter", "continue"}, [2]string{"Ctrl+D", "finish rules"}, [2]string{"Esc", "back"}, [2]string{"Ctrl+C", "quit"}))
		} else if m.step == 2 {
			lines = append(lines, renderHelp(m.width, [2]string{"↑/↓", "select"}, [2]string{"Enter", "continue"}, [2]string{"Esc", "back"}, [2]string{"Ctrl+C", "quit"}))
		} else {
			lines = append(lines, renderHelp(m.width, [2]string{"Enter", "continue"}, [2]string{"Esc", "back"}, [2]string{"Ctrl+C", "quit"}))
		}
	}
	return strings.Join(lines, "\n") + "\n"
}
