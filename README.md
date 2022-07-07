**Michflix** will be made up of three main components:
- The *movie library* show the available movies to watch, with thumbnails, covers, genres and so on.
- The *downloader* will grant the user the ability to provide a magnet link to a movie, which will be downloaded and decorated with metadata from a popular movie API.
- The *streamer* will generate urls to an open stream of the chosen movie.


### Instructions
To start the services run `docker-compose up` in the main directory.


### Features
- [ ] Support for a streaming protocol
- [ ] Ability to generate links to streams
- [ ] Ability to add magnets to download
- [ ] Movie library manager

- [ ] Login with universal password
- [ ] Multiple qualities of the same movie
- [ ] Internal player


#### Notes
MongoBD offers the possibilty to access the data with a [REST API](https://www.mongodb.com/atlas/app-services/data-api), might be a good idea to initially use this solution to handle the database connection.
