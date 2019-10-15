# sesh

Pomodoro cli/client

## Features 

|       | desc                                                | cmd                         |
|-------|-----------------------------------------------------|-----------------------------|
| clock | pomodoro/countdown timer                            | `sesh clock 25`             |
| todo  | collection of tasks/pomodoros (in todo list format) | `sesh todo "Fix login bug"` |
| log   | track your work habbits                             | `sesh log`                  |

### Usage

If you are familiar with the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique)
technique then `sesh` will fit like a glove, but `sesh` is very simple even if you don't know about the Pomodoro, it is
a simple way to manage and track your time.

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
