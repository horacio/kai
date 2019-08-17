# uttu (`)

Pomodoro cli/client

## Features 

|       | desc                                             | cmd                                        |
|-------|--------------------------------------------------|--------------------------------------------|
| clock | pomodoro timer                                   | `uttu 25` `uttu 4`                         |
| sesh  | named collection of pomodoros                    | `uttu sesh "Fix login bug"`                |
| log   | track your work habbits                          | `uttu log`                                 |

### Usage

if you are familiar with the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique)
technique then uttu will fit like a 
glove but uttu is very simple even if you don't know about the Pomodoro Technique and simply
want a simple way to manage and track your time. 

The Pomodoro is the inspiration, but uttu is a very basic timer that tracks work sessions 
it allows you to create your own work system based on your workflow. 

#### sesh
Creating todo lists:

create a new todo list and save it with 
todays date:

```bash
uttu sesh today 
```

create a new todo list and save it with
a custom name:

```bash
uttu sesh "Front-end stuff"
```
