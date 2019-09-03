# iwah (`)

Pomodoro cli/client

## Features 

|       | desc                                             | cmd                                        |
|-------|--------------------------------------------------|--------------------------------------------|
| clock | pomodoro timer                                   | `iwah 25` `iwah 4`                         |
| sesh  | named collection of pomodoros                    | `iwah sesh "Fix login bug"`                |
| log   | track your work habbits                          | `iwah log`                                 |

### Usage

if you are familiar with the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique)
technique then iwah will fit like a 
glove but iwah is very simple even if you don't know about the Pomodoro Technique and simply
want a simple way to manage and track your time. 

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
