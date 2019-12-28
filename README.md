# kai

Time management system.

## Features 

| Feature    | Description                                                                           | Command                        |
|------------|---------------------------------------------------------------------------------------|--------------------------------|
| `clock`    | countdown timer (defaults to 25 minutes)                                              | `kai clock 25`             |
| `todo`     | collection of tasks (in todo list format)                                             | `kai todo "Fix login bug"` |
| `log`      | Analyse and track your work habbits                                                   | `kai log`                  |
| `json`     | all the data is stored locally in `json` format in the home directory (`~/.pomocli/`) | automatic                      |
| `markdown` | kai uses markdown syntax to log pomodoro sessions                                 | `kai log today > today.md` |

### Usage

---
#### todo
Create a new todo list and save it with todays date:
```bash
$ kai todo today
```

Create a new todo list and save it with a custom name:
```bash
$ kai todo "Front-end stuff"
```
---
#### clock
Work on today's session for 25min (default)
```bash
$ kai clock today
```

Work on today's session for 40min
```bash
$ kai clock -m 40 today
```

Work on "Fix frontend" session for 60min
```bash
$ kai clock -m 60 "Fix frontend"
```
___

#### log
Log all (Todos)
```bash
$ kai log all
```

Log today's Todo
```bash
$ kai log today
```

Log a session by name
```bash
$ kai log "Fix frontend"
```

Export to markdown
```bash
$ kai log "Fix frontend" > ./front_end_work_log.md
```
