# iwah

Pomodoro cli/client

## Features 

|       | desc                     | cmd                         |
|-------|--------------------------|-----------------------------|
| clock | pomodoro/countdown timer | `iwah clock 25`             |
| sesh  | collection of pomodoros  | `iwah sesh "Fix login bug"` |
| log   | track your work habbits  | `iwah log`                  |

### Usage

If you are familiar with the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique)
technique then `iwah` will fit like a glove, but `iwah` is very simple even if you don't know about the Pomodoro, it is
a simple way to manage and track your time.

---
#### sesh
Creating todo lists:

```bash
# create a new todo list and save it with todays date:
iwah sesh today 
```

```bash
# create a new todo list and save it with a custom name:
iwah sesh "Front-end stuff"
```
---
#### clock

work on today's session:

```bash
# work on today's session for 25min
iwah clock "today"
```

```bash
# work on today's session for 40min
iwah clock -m 40 "today"
```

work on a custom session:

```bash
# work on "Fix frontend" session for 60min
iwah clock -m 60 "Fix frontend"
```
___
