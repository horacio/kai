# kai

Time management system.

## Features 

| Feature    | Description                                                                           | Command                        |
|------------|---------------------------------------------------------------------------------------|--------------------------------|
| `clock`    | countdown timer (defaults to 25 minutes)                                              | `kai clock`             |
| `todo`     | collection of tasks (in todo list format)                                             | `kai todo -i "Fix login bug"` |
| `log`      | Analyse and track your work habbits                                                   | `kai log`                  |
| `json`     | all the data is stored locally in `json` format in the home directory (`~/.pomocli/`) | automatic                      |
| `markdown` | kai uses markdown syntax to log todos, todos can also be created from makrdown files                                 | `kai log today > today.md` |

### Usage

---
#### todo
Create a new todo list using a interactive wizard,
the todo will be saved with todays date as its name:
```bash
$ kai todo
# The todo command defaults to today
# but if you like typing you can use the flag:
$ kai todo -i today
```

Create a new todo list and save it with a custom name:
```bash
$ kai todo -i "Front-end stuff"
```

Create a todo from a makrdown file:
```bash
$ kai todo -i backend_todo.md
---
```

The markdown file should have the following structure:
```text
# Backend
Tasks for the backend

## Tasks:
- [ ] Skate
- [ ] Eat
```

#### clock
Clock **today**'s todo for 25min (default)
```bash
$ kai clock

# The clock command defaults to today
# but if you like typing you can use the flag:
$ kai clock -i today
```

Clock the "Fix frontend" todo 
```bash
$ kai clock -i "Fix frontend"
```
___

#### log
Log **all** Todos
```bash
$ kai log
```

Log **today**'s Todo
```bash
$ kai log -i today
```

Log a todo by **name**
```bash
$ kai log -i "Fix frontend"
```

Export to **markdown**
```bash
$ kai log -i "Fix frontend" > ./front_end_work_log.md
```
