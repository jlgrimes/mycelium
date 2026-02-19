# GTM Plan — Debugging Wedge V1 (First 100 Users)

## Positioning statement

Mycelium helps software engineers escape debugging loops by forcing a useful frame pivot, then mapping that pivot back to concrete code fixes with verification steps.

Short form:

> Stop repeating failed fixes. Pivot frames, map back, verify fast.

## ICP (Ideal Customer Profile)

Primary ICP:

- Solo developers and small product teams (1–15 engineers)
- Building TypeScript/JavaScript-heavy apps and APIs
- Shipping weekly or faster
- Feeling recurring pain from flaky fixes, stale assumptions, and debugging thrash

Secondary ICP:

- Early devtools builders needing a “debug copilot” API
- Team leads who want reproducible debugging playbooks

## JTBD (Jobs To Be Done)

Functional JTBD:

- “When I’m stuck in a bug loop, help me get one credible next move that I can run now.”

Emotional JTBD:

- “Reduce panic and uncertainty when production behavior doesn’t match my model.”

Social JTBD:

- “Help me show my team a clear rationale, validation plan, and fallback path.”

## Wedge offer

For debugging prompts, Mycelium returns:

- `ABSTRACT` problem shape
- `SEARCH` isomorphic analogs
- `MAP` mapping + confidence
- `SYNTHESIZE` fix steps + verification + fallback pivot

Fast mode:

- `/solve/debug/concise` for operator-speed actions

## First 100 users — channel plan

### Channel 1: GitHub-native discovery (30 users)

Tactics:

- Publish benchmark report diffs in repo updates
- Add clear quickstart and debug examples in README
- Share before/after transcripts in issues/discussions

Success signal:

- 30 developers run debug endpoints at least 3 times each

### Channel 2: Dev social + communities (40 users)

Targets:

- Hacker News “Show HN” + follow-up comment thread
- r/programming, r/webdev, r/rust (quality write-ups, not spam)
- X dev circles with concrete debugging clips

Content angle:

- “How we cut debugging loops by forcing frame pivots (with metrics)”

Success signal:

- 40 users from public channels, 20 retained for 7+ days

### Channel 3: Founder-led outreach (30 users)

Targets:

- Indie hackers and startup engineering leads
- Existing network / prior collaborators

Method:

- 15-minute onboarding call
- One real bug transcript run live
- Capture objections + desired integrations

Success signal:

- 30 users complete at least one real debugging session

## Demo script (10 minutes)

1. **Set context (1 min)**
   - “Here’s a real bug loop we got stuck in.”
2. **Run baseline thought path (1 min)**
   - Show repetitive hypothesis pattern.
3. **Run `/solve/debug` (3 min)**
   - Highlight pivot rationale and mapping confidence.
4. **Run verification step (2 min)**
   - Execute suggested test/assert/log check.
5. **Run fallback pivot if needed (1 min)**
   - Show non-repetitive next action.
6. **Recap (2 min)**
   - Time-to-first-new-action, clarity, and next steps.

## Messaging pillars

- **Loop escape, not generic advice**
- **Actionability over eloquence**
- **Verification-first debugging**
- **Confidence + fallback transparency**

## Activation funnel

- Visitor reads wedge promise
- Runs one curl command (`/solve/debug/concise`)
- Sees immediate verification-ready next move
- Returns with a real production/staging bug
- Shares transcript or outcome

## Metrics for first-100 phase

- Activation: `% users who run 2+ debug prompts in first 24h`
- Value: `% prompts with verification_presence=true`
- Behavior: median `time_to_first_new_action` (manual collection initially)
- Retention: `% users active in week 2`
- Outcome: self-reported “loop escaped” rate

## Risks and mitigations

Risk: output sounds smart but is not runnable

- Mitigation: keep concise mode strict and verification-first

Risk: no measurable edge vs baseline

- Mitigation: publish benchmark deltas and tighten eval criteria weekly

Risk: broad ICP dilutes focus

- Mitigation: remain on JS/TS debugging wedge until retention target hit

## Exit criteria for GTM v1

- 100 users run at least one real debugging workflow
- At least 35 weekly active users in week 4
- Positive qualitative signal: “helped me break a loop” from 20+ users
