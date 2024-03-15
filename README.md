This project is my learning diary mainly about docker and kubernetes.
During 2024-spring I had a uni course about cloud infrastructure, where
the teacher was axiomatically against teaching even a single docker command.
Therefore I thought to document this semesters studying, progression and what else.

# Project structure
During the semester, we used docker-compose and kubernetes,
but we didn't have a central project, everybody just came up with something.
My idea was simulating a library, with a Postgresql Database, a REST Api written in Rust and <to-be-announced>.

### Folders
Originally each directory would entail a service with its own Dockerfile , with the exeption of `scripts`
which entails a number of small commands that helped me set up some crucial things about the database,
and since it was my first database from scratch, I decided to save them here.
