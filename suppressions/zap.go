package suppressions

import (
	"bytes"
	"encoding/json"
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/deathlabs/finch/models"
)

const (
	// ZAPRulesFileName is the file passed to ZAP's packaged scan scripts using -c.
	ZAPRulesFileName = "zap-rules.conf"
	// ZAPAlertFiltersFileName is an Automation Framework alertFilter job.
	ZAPAlertFiltersFileName = "zap-alert-filters.yaml"
	// ZAPReportFileName records why each rule is suppressed.
	ZAPReportFileName = "suppression-report.json"

	zapToolName = "ZAP"
)

// zapRuleID matches ZAP's numeric rule (plugin) IDs. Rule IDs are written into
// tab-separated and YAML files, so anything but digits is rejected.
var zapRuleID = regexp.MustCompile(`^[0-9]{1,9}$`)

// reportComponent is a platform component cited as the reason for a suppression.
type reportComponent struct {
	Title    string   `json:"title"`
	UUID     string   `json:"uuid"`
	Controls []string `json:"controls"`
}

// reportSuppression is one suppressed rule and the components that mitigate it.
type reportSuppression struct {
	RuleID     string            `json:"rule_id"`
	Components []reportComponent `json:"components"`
}

// report is the audit trail written next to the suppression files.
type report struct {
	Tool         string                 `json:"tool"`
	FinchVersion string                 `json:"finch_version,omitempty"`
	Mappings     *models.ExportMetadata `json:"mappings_metadata,omitempty"`
	Suppressions []reportSuppression    `json:"suppressions"`
}

// generateZAP builds the ZAP suppression files from the components in document.
func generateZAP(document models.ExportDocument, options Options) (Result, error) {
	var (
		alertFilters []byte
		err          error
		rules        []reportSuppression
		rulesConfig  []byte
		reportData   []byte
	)

	rules, err = collectZAPRules(document)
	if err != nil {
		return Result{}, err
	}

	rulesConfig = renderZAPRulesConfig(rules)
	alertFilters = renderZAPAlertFilters(rules)

	reportData, err = json.MarshalIndent(report{
		Tool:         zapToolName,
		FinchVersion: options.FinchVersion,
		Mappings:     document.Metadata,
		Suppressions: rules,
	}, "", "  ")
	if err != nil {
		return Result{}, fmt.Errorf("could not build the suppression report: %w", err)
	}
	reportData = append(reportData, '\n')

	return Result{
		Files: []File{
			{Name: ZAPRulesFileName, Content: rulesConfig},
			{Name: ZAPAlertFiltersFileName, Content: alertFilters},
			{Name: ZAPReportFileName, Content: reportData},
		},
		RuleCount: len(rules),
	}, nil
}

// collectZAPRules gathers every ZAP rule mapped to a component, merging
// components that mitigate the same rule, ordered by numeric rule ID.
func collectZAPRules(document models.ExportDocument) ([]reportSuppression, error) {
	var (
		bySuppression = map[string]*reportSuppression{}
		component     models.ComponentDefinition
		id            string
		numeric       int
		order         = map[string]int{}
		rawRule       string
		suppression   *reportSuppression
		toolRules     models.ToolRules
		err           error
		ok            bool
	)

	for _, component = range document.Components {
		for _, toolRules = range component.SecurityRulesMitigatedByThisComponent {
			if !strings.EqualFold(strings.TrimSpace(toolRules.Tool), zapToolName) {
				continue
			}

			for _, rawRule = range toolRules.Rules {
				id = strings.TrimSpace(rawRule)
				if !zapRuleID.MatchString(id) {
					return nil, fmt.Errorf("component %q: %q is not a valid ZAP rule ID", component.Title, rawRule)
				}

				// A suppression must cite the platform control that backs it.
				if len(component.ControlsImplementedByThisComponent) == 0 {
					return nil, fmt.Errorf("component %q maps ZAP rule %s but implements no controls, so there is nothing to cite", component.Title, id)
				}

				// Normalize (for example, 010038 becomes 10038).
				numeric, err = strconv.Atoi(id)
				if err != nil {
					return nil, fmt.Errorf("component %q: %q is not a valid ZAP rule ID", component.Title, rawRule)
				}
				id = strconv.Itoa(numeric)

				suppression, ok = bySuppression[id]
				if !ok {
					suppression = &reportSuppression{RuleID: id}
					bySuppression[id] = suppression
					order[id] = numeric
				}

				if !hasComponent(suppression.Components, component.UUID, component.Title) {
					suppression.Components = append(suppression.Components, reportComponent{
						Title:    component.Title,
						UUID:     component.UUID,
						Controls: append([]string(nil), component.ControlsImplementedByThisComponent...),
					})
				}
			}
		}
	}

	var rules = make([]reportSuppression, 0, len(bySuppression))
	for _, id = range sortedNumericKeys(order) {
		rules = append(rules, *bySuppression[id])
	}
	return rules, nil
}

// hasComponent reports whether components already holds the given component.
func hasComponent(components []reportComponent, uuid string, title string) bool {
	var component reportComponent
	for _, component = range components {
		if component.UUID == uuid && component.Title == title {
			return true
		}
	}
	return false
}

// renderZAPRulesConfig renders the file ZAP's scan scripts read with -c. Every
// line has three tab-separated columns (ID, action, name), which is the layout
// the scripts write themselves when run with -g. The name is informational.
func renderZAPRulesConfig(rules []reportSuppression) []byte {
	var (
		buffer bytes.Buffer
		rule   reportSuppression
	)

	buffer.WriteString("# Generated by Finch. Do not edit.\n")
	buffer.WriteString("# Rules mitigated by the deployment platform. See suppression-report.json.\n")
	for _, rule = range rules {
		fmt.Fprintf(&buffer, "%s\tIGNORE\t(mitigated by platform)\n", rule.RuleID)
	}
	return buffer.Bytes()
}

// renderZAPAlertFilters renders an Automation Framework alertFilter job. Unlike
// IGNORE, it keeps the alerts in the ZAP report and marks them False Positive.
func renderZAPAlertFilters(rules []reportSuppression) []byte {
	var (
		buffer bytes.Buffer
		rule   reportSuppression
	)

	buffer.WriteString("# Generated by Finch. Do not edit.\n")
	buffer.WriteString("# Add this job to the jobs list of your ZAP Automation Framework plan.\n")
	if len(rules) == 0 {
		buffer.WriteString("[]\n")
		return buffer.Bytes()
	}
	buffer.WriteString("- type: alertFilter\n")
	buffer.WriteString("  parameters:\n")
	buffer.WriteString("    deleteGlobal: false\n")
	buffer.WriteString("  alertFilters:\n")
	for _, rule = range rules {
		fmt.Fprintf(&buffer, "    - ruleId: %s\n", rule.RuleID)
		buffer.WriteString("      newRisk: \"False Positive\"\n")
	}
	return buffer.Bytes()
}
