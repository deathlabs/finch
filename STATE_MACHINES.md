# State Machines
 
This file describes how things in Finch change over time. [GLOSSARY.md](docs/src/content/GLOSSARY.md) defines what things are; this file defines the stages they pass through, who moves them between stages, and why.
 
A lifecycle is the set of business states an entity moves through. Each lifecycle has one owner (the entity that moves), a fixed set of states, and named transitions. Transitions are written as past-tense events, the way a domain expert would report them.
 
Status of this draft: states, guards, and rules below are inferred from the [glossary](docs/src/content/GLOSSARY.md). Items marked "Ask" are questions for the domain experts and should be resolved before this file is treated as authoritative.
 
Contents:
 
- [Mapping](#mapping)
- [Mappings file version](#mappings-file-version)
- [Finding](#finding)
- [Rules that span lifecycles](#rules-that-span-lifecycles)
---
 
## Mapping

Finch does not yet model Proposed, Accepted, or Rejected. Code derives Expired from the [expiry](docs/src/content/GLOSSARY.md#expiry) and the supplied date. The remaining section describes the target design.
 
Owner: [mapping](docs/src/content/GLOSSARY.md#mapping)
 
Context: reviewing whether an implemented [control](docs/src/content/GLOSSARY.md#control) justifies ignoring a [scanner](docs/src/content/GLOSSARY.md#scanner) rule for a [component](docs/src/content/GLOSSARY.md#component).
 
Also known as: (Ask: do experts say "status", "approval", or "exception" for this?)
 
### States
 
| State | What is true in this state |
| --- | --- |
| Proposed | A [mapping](docs/src/content/GLOSSARY.md#mapping) has been written with a [justification](docs/src/content/GLOSSARY.md#justification) and an [expiry](docs/src/content/GLOSSARY.md#expiry). The [security team](docs/src/content/GLOSSARY.md#security-team) has not decided yet. |
| Accepted | The [security team](docs/src/content/GLOSSARY.md#security-team) agrees the [control](docs/src/content/GLOSSARY.md#control) mitigates the [risk](docs/src/content/GLOSSARY.md#risk). Finch may generate a [suppression](docs/src/content/GLOSSARY.md#suppression) from it. |
| Rejected | The [security team](docs/src/content/GLOSSARY.md#security-team) declined it. No [suppression](docs/src/content/GLOSSARY.md#suppression) is generated. Terminal. |
| Expired | The [expiry](docs/src/content/GLOSSARY.md#expiry) date has passed. Finch no longer generates a [suppression](docs/src/content/GLOSSARY.md#suppression) from it. Terminal. |
 
Initial state: Proposed
 
Terminal states: Rejected, Expired
 
### Transitions
 
| Event | From | To | Triggered by | Guard | Why it matters |
| --- | --- | --- | --- | --- | --- |
| Mapping proposed | (none) | Proposed | [Tenant](docs/src/content/GLOSSARY.md#tenant) or [platform team](docs/src/content/GLOSSARY.md#platform-team) | Has a [justification](docs/src/content/GLOSSARY.md#justification) and an [expiry](docs/src/content/GLOSSARY.md#expiry) in the future | A [mapping](docs/src/content/GLOSSARY.md#mapping) without a reason or an end date cannot be reviewed or audited. |
| Mapping accepted | Proposed | Accepted | [Security team](docs/src/content/GLOSSARY.md#security-team) | Reviewer is not the proposer (Ask) | The [security team](docs/src/content/GLOSSARY.md#security-team) decides whether an app may be deployed, so only it can approve. |
| Mapping rejected | Proposed | Rejected | [Security team](docs/src/content/GLOSSARY.md#security-team) | A reason is recorded (Ask) | A recorded reason lets the proposer fix and resubmit. |
| Mapping expired | Accepted | Expired | Time | [Expiry](docs/src/content/GLOSSARY.md#expiry) date has passed | Acceptance is temporary. The [risk](docs/src/content/GLOSSARY.md#risk) must be re-reviewed periodically. |
 
### Forbidden moves
 
- Proposed to Expired. A [mapping](docs/src/content/GLOSSARY.md#mapping) that was never accepted never became valid, so it has nothing to expire. Ask: or does an unreviewed proposal also lapse?
- Rejected to Accepted. A rejection is final. Resubmission is a new [mapping](docs/src/content/GLOSSARY.md#mapping), so the history of the decision is preserved.
- Expired to Accepted. Renewal is a new [mapping](docs/src/content/GLOSSARY.md#mapping) with a fresh [justification](docs/src/content/GLOSSARY.md#justification) and [expiry](docs/src/content/GLOSSARY.md#expiry). Ask: can a renewal reuse the old [justification](docs/src/content/GLOSSARY.md#justification)?
### Time-based rules
 
- An accepted [mapping](docs/src/content/GLOSSARY.md#mapping) expires at the end of its [expiry](docs/src/content/GLOSSARY.md#expiry) date. Ask: is the date inclusive, and in which time zone?
### Open questions
 
- Can the [security team](docs/src/content/GLOSSARY.md#security-team) revoke an accepted [mapping](docs/src/content/GLOSSARY.md#mapping) before its [expiry](docs/src/content/GLOSSARY.md#expiry)? If so, add a Revoked state.
- Is there a maximum allowed [expiry](docs/src/content/GLOSSARY.md#expiry) interval?
- If a [component](docs/src/content/GLOSSARY.md#component) is removed from an [environment](docs/src/content/GLOSSARY.md#environment), what happens to its accepted [mappings](docs/src/content/GLOSSARY.md#mapping)?
### Diagram
 
```mermaid
stateDiagram-v2
  [*] --> Proposed: Mapping proposed
  Proposed --> Accepted: Mapping accepted
  Proposed --> Rejected: Mapping rejected
  Accepted --> Expired: Mapping expired
  Rejected --> [*]
  Expired --> [*]
```
 
---
 
## Mappings file version
 
Owner: [mappings file version](docs/src/content/GLOSSARY.md#mappings-file-version)
 
Context: releasing a set of [mappings](docs/src/content/GLOSSARY.md#mapping) for one [environment](docs/src/content/GLOSSARY.md#environment) of one [hosting platform](docs/src/content/GLOSSARY.md#hosting-platform).
 
### States
 
| State | What is true in this state |
| --- | --- |
| Draft | The [mappings file](docs/src/content/GLOSSARY.md#mappings-file) is being edited. It may contain [mappings](docs/src/content/GLOSSARY.md#mapping) that are still Proposed. |
| Tagged | The version is fixed. Finch can use it to generate [suppressions](docs/src/content/GLOSSARY.md#suppression). It does not change. |
| Superseded | A newer tagged version exists for the same [environment](docs/src/content/GLOSSARY.md#environment). Kept for history. Terminal. |
 
Initial state: Draft
 
Terminal states: Superseded
 
### Transitions
 
| Event | From | To | Triggered by | Guard | Why it matters |
| --- | --- | --- | --- | --- | --- |
| Version tagged | Draft | Tagged | [Security team](docs/src/content/GLOSSARY.md#security-team) (Ask) | Contains only Accepted [mappings](docs/src/content/GLOSSARY.md#mapping) (Ask) | A tag is a promise that the contents were reviewed. |
| Version superseded | Tagged | Superseded | A newer version being tagged | Same [environment](docs/src/content/GLOSSARY.md#environment) and [hosting platform](docs/src/content/GLOSSARY.md#hosting-platform) | [Components](docs/src/content/GLOSSARY.md#component) and [controls](docs/src/content/GLOSSARY.md#control) differ between [environments](docs/src/content/GLOSSARY.md#environment), so versions are never compared across them. |
 
### Forbidden moves
 
- Tagged to Draft. A tagged version is immutable. Changes go in a new version.
### Open questions
 
- Does Finch ever use more than one tagged version at once, for example during a rollout?
- Are [mappings](docs/src/content/GLOSSARY.md#mapping) that expire while a version is Tagged left in place, or does the version stay valid and Finch skip them?
### Diagram
 
```mermaid
stateDiagram-v2
  [*] --> Draft
  Draft --> Tagged: Version tagged
  Tagged --> Superseded: Version superseded
  Superseded --> [*]
```
 
---
 
## Finding
 
Owner: [finding](docs/src/content/GLOSSARY.md#finding)
 
Context: what happens to a result reported by a [scanner](docs/src/content/GLOSSARY.md#scanner) once [mappings](docs/src/content/GLOSSARY.md#mapping) are applied.
 
### States
 
| State | What is true in this state |
| --- | --- |
| Reported | A [scanner](docs/src/content/GLOSSARY.md#scanner) has reported the [finding](docs/src/content/GLOSSARY.md#finding). No Accepted [mapping](docs/src/content/GLOSSARY.md#mapping) covers it. |
| Suppressed | An Accepted [mapping](docs/src/content/GLOSSARY.md#mapping) covers it, so a [suppression](docs/src/content/GLOSSARY.md#suppression) tells the [scanner](docs/src/content/GLOSSARY.md#scanner) to ignore it. |
| Resurfaced | It was Suppressed, but the [mapping](docs/src/content/GLOSSARY.md#mapping) expired, so the [scanner](docs/src/content/GLOSSARY.md#scanner) reports it again. |
| Resolved | The underlying issue is gone, for example the [tenant](docs/src/content/GLOSSARY.md#tenant) fixed the app or removed the feature ([risk avoidance](docs/src/content/GLOSSARY.md#risk-avoidance)). Terminal. |
 
Initial state: Reported
 
Terminal states: Resolved
 
### Transitions
 
| Event | From | To | Triggered by | Guard | Why it matters |
| --- | --- | --- | --- | --- | --- |
| Finding suppressed | Reported | Suppressed | Finch | A matching [mapping](docs/src/content/GLOSSARY.md#mapping) is Accepted and not Expired | A [suppression](docs/src/content/GLOSSARY.md#suppression) is only valid while the reasoning behind it is. |
| Suppression lapsed | Suppressed | Resurfaced | Time | The covering [mapping](docs/src/content/GLOSSARY.md#mapping) expired | Expired reasoning must not hide [risk](docs/src/content/GLOSSARY.md#risk) forever. |
| Finding resolved | Reported, Suppressed, Resurfaced | Resolved | [Tenant](docs/src/content/GLOSSARY.md#tenant) | The [scanner](docs/src/content/GLOSSARY.md#scanner) no longer reports it | The real fix beats a [suppression](docs/src/content/GLOSSARY.md#suppression). |
| Finding suppressed again | Resurfaced | Suppressed | Finch | A new Accepted [mapping](docs/src/content/GLOSSARY.md#mapping) covers it | Renewal creates a new [mapping](docs/src/content/GLOSSARY.md#mapping) rather than reviving the old one. |
 
### Forbidden moves
 
- Reported to Suppressed without an Accepted [mapping](docs/src/content/GLOSSARY.md#mapping). Nothing may be ignored on a Proposed [mapping](docs/src/content/GLOSSARY.md#mapping).
- Resolved to any other state. Ask: if the same rule fires later, is that a new [finding](docs/src/content/GLOSSARY.md#finding)?
### Diagram
 
```mermaid
stateDiagram-v2
  [*] --> Reported
  Reported --> Suppressed: Finding suppressed
  Suppressed --> Resurfaced: Suppression lapsed
  Resurfaced --> Suppressed: Finding suppressed again
  Reported --> Resolved: Finding resolved
  Suppressed --> Resolved: Finding resolved
  Resurfaced --> Resolved: Finding resolved
  Resolved --> [*]
```
 
---
 
## Rules that span lifecycles
 
These are the dependencies between machines, and the most likely place for hidden expert knowledge.
 
1. A [finding](docs/src/content/GLOSSARY.md#finding) may be Suppressed only while a matching [mapping](docs/src/content/GLOSSARY.md#mapping) is Accepted.
2. When a [mapping](docs/src/content/GLOSSARY.md#mapping) moves to Expired, every [finding](docs/src/content/GLOSSARY.md#finding) it suppressed moves to Resurfaced.
3. A [mappings file version](docs/src/content/GLOSSARY.md#mappings-file-version) may be Tagged only when its [mappings](docs/src/content/GLOSSARY.md#mapping) have been reviewed by the [security team](docs/src/content/GLOSSARY.md#security-team).
4. [Suppressions](docs/src/content/GLOSSARY.md#suppression) are always generated from a Tagged [mappings file version](docs/src/content/GLOSSARY.md#mappings-file-version) for the matching [environment](docs/src/content/GLOSSARY.md#environment).
## How to add a lifecycle
 
1. Name the owner, and make sure it is defined in [GLOSSARY.md](docs/src/content/GLOSSARY.md).
2. List states as conditions the expert would say out loud. Record synonyms they use.
3. Name transitions as past-tense events. For each, record who triggers it, the guard, and the expert's reason.
4. Write the forbidden moves. Experts often reveal their deepest rules by saying "that can never happen".
5. Record open questions instead of guessing.
6. Add a Mermaid diagram generated from the same states and events, so the picture and the tables cannot drift.
7. Link every glossary term, per [AGENTS.md](AGENTS.md).
 