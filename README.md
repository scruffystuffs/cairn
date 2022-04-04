# Cairn, a simple reminder

Most todo-based tools have too many features, or required data fields, and
all I need is a simple reminder tool to be just a little bit annoying in
just the right way.  So I wrote this.

```shell
$ cairn add 'Foo all of the bars'
$ cairn add --status pending "Don't leave this silly example in the README"
$ cairn summary
ACTIVE Foo all of the bars
ACTIVE Don't leave this silly example in the README
```
 
Then I configure my shell to run `cairn summary` at startup.

## Roadmap

This tool is in alpha at the moment, almost 0 planned features are implemented.

- Read/Write/Delete existing tasks:
  - Changing task status
  - Output task db as JSON
  - Add details to tasks
  - Produce deterministic task IDs
- Improved summary
  - `snooze` command - Hide tasks from summaries for a certain amount of time.
  - Colored summary output
  - Auto-integrate with shells (Automatically install the summary greeting)