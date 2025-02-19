# Music Downloading Service Backend

This is the backend service for a music downloading application, built using Rust and Actix-web. It provides a RESTful API to manage and serve music files, artist information, and user authentication.

## Features

  * **Serve Music Files:** Hosts music files and album/artist covers for download.
  * **Song Management:**
      * Retrieve a list of all songs.
      * Add new songs to the database, ensuring artist profiles exist.
      * Search songs by title (supports partial matching).
  * **Artist Management:**
      * Add new artist profiles with details like name, cover image, and biography.
      * Search artists by name (supports partial matching).
  * **Album Management:**
      * Search albums by name (supports partial matching).
      * Retrieve all songs belonging to a specific album.
  * **User Authentication:**
      * User login functionality to authenticate users.
      * Retrieve user information by username.

## Technologies Used

  * **Rust:** Programming language for backend development, known for performance and safety.
  * **Actix-web:**  A powerful, pragmatic, and extremely fast web framework for Rust.
  * **SQLite:**  A lightweight, serverless, and self-contained SQL database engine.
  * **sqlx:**  A Rust SQL toolkit that provides compile-time checked queries against your database.
  * **serde:**  A framework for serializing and deserializing Rust data structures efficiently and generically.
  * **actix-files:**  An Actix-web extension for serving static files.

## Prerequisites

  * **Rust Toolchain:** Make sure you have Rust and Cargo installed. You can install them from [https://www.rust-lang.org/tools/install](https://www.google.com/url?sa=E&source=gmail&q=https://www.rust-lang.org/tools/install).
  * **SQLite:** Ensure SQLite is installed on your system.

## Getting Started

### 1\. Clone the repository

```bash
git clone [repository-url]
cd [repository-directory]
```

### 2\. Create and Migrate Database

The application uses an SQLite database file named `test.db`.  You need to create this file in the project root directory.

*(**Note:** This README assumes you have a `test.db` database file ready with the necessary tables: `Songs`, `Artist`, `Album`, and `User`. If you need to create these tables, you can use an SQLite client or add SQL migration scripts to your project.)*

Example table schemas (Illustrative - Adapt to your actual schema):

**Songs Table:**

| Column     | Type    |
|------------|---------|
| id         | INTEGER |
| name       | TEXT    |
| release    | TEXT    |
| album      | TEXT    |
| cover      | TEXT    |
| location   | TEXT    |

**Artist Table:**

| Column     | Type    |
|------------|---------|
| id         | INTEGER |
| name       | TEXT    |
| cover      | TEXT    |
| about      | TEXT    |

**Album Table:**

| Column     | Type    |
|------------|---------|
| id         | INTEGER |
| name       | TEXT    |
| artist     | TEXT    |
| release    | TEXT    |

**User Table:**

| Column     | Type    |
|------------|---------|
| id         | INTEGER |
| username   | TEXT    |
| password   | TEXT    |
| cover      | TEXT    |

### 3\. Run the Application

```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`.

## API Endpoints

Base URL: `http://127.0.0.1:8080`

| Endpoint                     | Method | Description                                     | Request Body (JSON)          | Response Body (JSON)                               |
|------------------------------|--------|-------------------------------------------------|-------------------------------|----------------------------------------------------|
| `/`                           | GET    | Welcome message                                 | None                          | `{"message": "Welcome To Music Downloading Service"}` |
| `/get/file/{filename:.*}`    | GET    | Host and download files from `./assets/` directory | None                          | File download                                        |
| `/get/song`                   | GET    | Get all songs                                   | None                          | `[Song, Song, ...]`                                  |
| `/post/song`                  | POST   | Add a new song                                  | `PostSong`                     | `{"status": "Ok"}` or `{"status": "Not Acceptable"}` or `{"status": "NotFound", "message": "Add artist profile"}` |
| `/search/{title}`             | GET    | Search songs by title                           | None                          | `[Song, Song, ...]` or `{"status": "NotFound"}`    |
| `/post/artist`                 | POST   | Add a new artist                                | `Artist`                       | `{"status": "Ok"}` or `{"status": "Not Acceptable"}` |
| `/search/artist/{title}`       | GET    | Search artists by name                          | None                          | `[Artist, Artist, ...]` or `{"status": "NotFound"}`  |
| `/login`                      | POST   | User login                                      | `User`                         | `User` or `{"status": "NotFound"}`                  |
| `/user/{username}`            | GET    | Get user information by username                | None                          | `User` or `{"status": "BadRequest"}`                |
| `/search/album/{name}`         | GET    | Search albums by name                           | None                          | `[Album, Album, ...]` or `{"status": "Ok", "data": []}` |
| `/get/album/{name}`            | GET    | Get songs from a specific album                  | None                          | `[Song, Song, ...]`                                  |

**Data Structures:**

```json
User
{
    "id": 1,
    "username": "user1",
    "password": "password123",
    "cover": "user1_cover.jpg"
}

Song
{
    "id": 101,
    "name": "Song Title",
    "release": "2025-02-19",
    "album": "Album Name",
    "cover": "song_cover.jpg",
    "location": "songs/song_file.mp3"
}

PostSong (for POST /post/song) - includes artist name for validation
{
    "id": 102,
    "name": "New Song Title",
    "release": "2025-02-20",
    "album": "New Album Name",
    "cover": "new_song_cover.jpg",
    "location": "songs/new_song_file.mp3",
    "artist": "Artist Name"
}

Artist
{
    "id": 201,
    "name": "Artist Name",
    "cover": "artist_cover.jpg",
    "about": "Artist biography..."
}

Album
{
    "id": 301,
    "name": "Album Name",
    "artist": "Artist Name",
    "release": "2025-02-15"
}
```

**Example Requests:**

  * **Get all songs:**
    `GET http://127.0.0.1:8080/get/song`

  * **Search for songs with "love" in the title:**
    `GET http://127.0.0.1:8080/search/love`

  * **Add a new artist (Artist data in JSON body):**
    `POST http://127.0.0.1:8080/post/artist`

  * **Login (User credentials in JSON body):**
    `POST http://127.0.0.1:8080/login`

  * **Get user information for username "user1":**
    `GET http://127.0.0.1:8080/user/user1`

  * **Download a file named "song\_file.mp3" from the assets directory:**
    `GET http://127.0.0.1:8080/get/file/song_file.mp3`

## Assets Directory

The `/get/file/{filename:.*}` endpoint serves files from the `./assets/` directory. You should place your music files, cover images, and other static assets in this directory.

## Error Handling

The API returns standard HTTP status codes to indicate the success or failure of requests. Common error responses include:

  * **404 Not Found:**  Resource not found (e.g., song or artist not found).
  * **406 Not Acceptable:**  Request is not acceptable (e.g., failed to insert data, possibly due to database constraints).
  * **400 Bad Request:**  Invalid request (e.g., user not found).
