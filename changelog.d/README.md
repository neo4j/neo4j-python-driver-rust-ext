To create a new entry, you can run:
```bash
towncrier create "<PR number>.<type>"
```
This will create a new file in the `changelog.d/` directory for you to fill in.
If there is no issue or PR number, you can use `+.<type>` instead.
The `<type>` determines how the entry will be grouped in the changelog.
For available types, see the `[[tool.towncrier.type]]` entries in the `pyproject.toml` file.

You can include `<ISSUES_LIST>` in the entry to determine where the list of PR/issue links goes.
Usually, the entries will look like this:

```markdown
Some summary of the change<ISSUES_LIST>.
```

or if there's more to say (note the trailing spaces after the first line):
```markdown
Some summary of the change<ISSUES_LIST>.  
Some more details. Feel free to use markdown features like
* lists
* more lists
  * nested lists
  * [links](https://example.com)
* etc.
```
