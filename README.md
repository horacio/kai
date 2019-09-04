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

create a new todo list and save it with 
todays date:

```bash
iwah sesh today 
```

create a new todo list and save it with
a custom name:

```bash
iwah sesh "Front-end stuff"
```
---
#### clock

work on today's session:

```bash
iwah clock "today" 25
```

work on a custom session:

```bash
iwah clock "Fix frontend" 60
```

___
