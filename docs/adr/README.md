# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records (ADRs) for the Everything App project.

## What is an ADR?

An ADR is a document that captures an important architectural decision made along with its context and consequences.

## ADR Process

1. **Create a new ADR** when a significant architectural decision needs to be made
2. **Use the template** (0000-TEMPLATE.md) for consistency
3. **Number sequentially** (0001, 0002, etc.)
4. **Store in this directory** with descriptive names
5. **Update status** as decisions are accepted, rejected, or superseded

## Current ADRs

| Number | Title | Status | Date |
|--------|-------|--------|------|
| [0000](0000-TEMPLATE.md) | Template | Accepted | 2024-07-30 |

*No ADRs have been created yet. Add them as architectural decisions are made.*

## How to Create an ADR

```bash
# Copy the template
cp docs/adr/0000-TEMPLATE.md docs/adr/0001-my-decision.md

# Edit the new ADR file
# - Update the number in filename and header
# - Fill in the context, decision, alternatives, and consequences
# - Set the initial status to "Proposed"

# Create a PR with your ADR
# - Discuss with the team
# - Update based on feedback
# - Change status to "Accepted" when agreed upon
```

## ADR Status Values

- **Proposed**: Decision is being discussed
- **Accepted**: Decision has been agreed upon and is being implemented
- **Rejected**: Decision was considered but not accepted
- **Deprecated**: Decision was accepted but is no longer relevant
- **Superseded**: Decision was replaced by a newer ADR

## Resources

- [ADR GitHub Organization](https://adr.github.io/)
- [ADR Template](https://github.com/adr/adr-tools/blob/master/templates/adr-template.md)
- [ADR Examples](https://github.com/joel-costigliola/adr-examples)

---

*Last Updated: July 2024*
