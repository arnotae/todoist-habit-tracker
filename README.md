# Todoist Habit Tracker

1. Add a task with `[day 0]` at the end of the content
2. Schedule it `every day`
3. and the magic will occur when you complete it  
Ex: `Enjoy your day [day 0]` -> `Enjoy your day [day 1]` -> `Enjoy your day [day 2]` 

ℹ️ I know my README is not really complete, but I'm not sure anyone will find it in the GitHub blackhole 😄, feel free to write me if you need !

## Todoist side 

1. Create an app https://developer.todoist.com/appconsole.html
2. Add a webhook with `item:completed` and your callback url `domain.com/api/webhook/complete-item`
3. Go through oAuth authentification procedure https://developer.todoist.com/sync/v8/#oauth to add an user to your app (Can be done via Postman)  
4. Get the ApiKey from https://todoist.com/prefs/integrations set `TODOIST_TOKEN` in `.env` 

## Env
Replace `.env.dist` to `.env` and fill it with your value

## Deployment

- Build : `cargo build --release` 
- I use the Api via Nginx (reverse proxy) & systemctl (see help ⬇️)

## Roadmap

- Add a job to reset to 0 when a task was not completed one day (see inspiration) or maybe juste reset to 1 if I check the scheduling date of the task

## Inspiration & help

- Made with Rust ❤️
- this repo was my inspiration, but I didn't want to wait for the worklow every night 😄 ttps://github.com/amitness/habitist  
- Deployement of rust app https://gist.github.com/belst/ff36c5f3883f7bf9b06c379d0a7bed9e

## Licence
This project is licensed under the MIT License
