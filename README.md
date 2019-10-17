# sesh

Pomodoro cli/client

`sesh` is a simple time management, command line tool. It is based on 
the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique).

## Features 

| Feature    | Description                                           | Command                     |
|------------|-------------------------------------------------------|-----------------------------|
| `clock`    | pomodoro/countdown timer                              | `sesh clock 25`             |
| `todo`     | collection of tasks/pomodoros (in todo list format)   | `sesh todo "Fix login bug"` |
| `log`      | Analyse and track your work habbits                   | `sesh log`                  |
| `json`     | all the data is stored locally in `json` format       | automatic                   |
| `markdown` | sesh can export pomodoro session to `markdown` format | `sesh export today`         |

### Usage

---
#### todo

```bash
# create a new todo list and save it with todays date:
sesh todo today 
```

```bash
# create a new todo list and save it with a custom name:
sesh todo "Front-end stuff"
```
---
#### clock

```bash
# work on today's session for 25min
sesh clock "today"
```

```bash
# work on today's session for 40min
sesh clock -m 40 "today"
```

```bash
# work on "Fix frontend" session for 60min
sesh clock -m 60 "Fix frontend"
```
___
