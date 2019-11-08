# pomocli

Pomodoro cli/client

`pomocli` is a simple command line tool inspired by the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique).

## Features 

| Feature    | Description                                                                           | Command                        |
|------------|---------------------------------------------------------------------------------------|--------------------------------|
| `clock`    | countdown timer (defaults to 25 minutes)                                              | `pomocli clock 25`             |
| `todo`     | collection of tasks (in todo list format)                                             | `pomocli todo "Fix login bug"` |
| `log`      | Analyse and track your work habbits                                                   | `pomocli log`                  |
| `json`     | all the data is stored locally in `json` format in the home directory (`~/.pomocli/`) | automatic                      |
| `markdown` | pomocli uses markdown syntax to log pomodoro sessions                                 | `pomocli log today > today.md` |

### Usage

---
#### todo

```bash
# create a new todo list and save it with todays date:
pomocli todo today 
```

```bash
# create a new todo list and save it with a custom name:
pomocli todo "Front-end stuff"
```
---
#### clock

```bash
# work on today's session for 25min
pomocli clock today
```

```bash
# work on today's session for 40min
pomocli clock -m 40 today
```

```bash
# work on "Fix frontend" session for 60min
pomocli clock -m 60 "Fix frontend"
```
___

#### log
```bash
# log all (Todos)
pomocli log all

# #2019-11-08
# - Tasks: 3
# - Pomodoros: 0
# - Date Started: 2019-11-08
# - Date Ended: Ongoing
# - Total Time Spend: 0

# #2019-10-25
# - Tasks: 3
# - Pomodoros: 0
# - Date Started: 2019-10-25
# - Date Ended: Ongoing
# - Total Time Spend: 0

```

```bash
# log today's Todo
pomocli log today

# # 2019-10-24

# 1. [ ] Ui
# 2. [X] CSS Bugs

# ------------------------
# Pomodoros: 0
# Date Started: 2019-10-24
# Date Ended: Ongoing
# Total Time Spend: 25min
```

```bash
# log a session by name
pomocli log "Fix frontend"
```

```bash
# export to markdown
pomocli log "Fix frontend" > ./front_end_work_log.md
```
