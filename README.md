# pomocli

Pomodoro cli/client

`pomocli` is a simple time management, command line tool. It is based on 
the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique).

## Features 

| Feature    | Description                                               | Command                        |
|------------|-----------------------------------------------------------|--------------------------------|
| `clock`    | pomodoro/countdown timer                                  | `pomocli clock 25`             |
| `todo`     | collection of tasks/pomodoros (in todo list format)       | `pomocli todo "Fix login bug"` |
| `log`      | Analyse and track your work habbits                       | `pomocli log`                  |
| `json`     | all the data is stored locally in `json` format           | automatic                      |
| `markdown` | pomocli can export pomodoro sessions to `markdown` format | `pomocli log today > today.md` |

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
pomocli clock "today"
```

```bash
# work on today's session for 40min
pomocli clock -m 40 "today"
```

```bash
# work on "Fix frontend" session for 60min
pomocli clock -m 60 "Fix frontend"
```
___

#### log

```bash
# log today's session
pomocli log today
```

```bash
# log a session by name
pomocli log "Fix frontend"
```

```bash
# export to markdown
pomocli log "Fix frontend" > ./front_end_work_log.md

# writes the todo to ./front_end_work_log.md

# # 2019-10-24

# 1. [ ] Ui
# 2. [X] CSS Bugs

# ------------------------
# Pomodoros: 0
# Date Started: 2019-10-24
# Date Ended: Ongoing
# Total Time Spend: 0min
```
