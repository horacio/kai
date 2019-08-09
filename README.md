# uttu (`)

cli pomodoro client

## Features 

| Feature   | description                       | Command                    |
|-----------|-----------------------------------|----------------------------|
| timer     | countdown timer                   | `uttu start 25`             |
| sessions  | named collection of pomodoros     | `uttu todo "Fix login bug"` |
| analytics | track your work habbits           | `uttu log`                  |
| Todo      | to decide on the day’s activities | `uttu todo today`           |
| controls  | interactive manager and commands  | ``                         |

### Usage

if you are familiar with the pomodoro
technique for time management then uttu will fit like a 
glove. but uttu is very simple even worms use uttu (`) so
it should not be difficult for a human to use uttu.

uttu uses git as the database this means we allot of
the benefits of git.

In the world of uttu only 3 entities exist:

- **todo**
- **time**
- **log**

#### todo
Creating todo lists:

create a new todo list and save it with 
todays date:

```bash
uttu todo today 
```

create a new todo list and save it with
a custom date:

```bash
uttu todo "Front-end stuff"
```

#### time
