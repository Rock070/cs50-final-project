# URL Shortener Service - Shor

Video Demo: [https://youtu.be/itByMdxLJ2Q](https://youtu.be/itByMdxLJ2Q)

## Description

This project aims to simplify URL sharing for users by converting lengthy URLs into short, manageable links. Provides an intuitive platform for both casual users and businesses looking to track link performance.

This is a user-friendly URL shortening service called `shor` that allows users to convert long URLs into concise ones. The system supports basic functionality for visitors while providing additional advanced features for registered users, such as URL click statistics, viewing created short URLs, and deleting created short URLs.

## Main Features

### 1. **URL Shortening Function**

path: `/`

- No login required
- Automatic short URL generation
- Supports any valid URL format

![hash_url.gif](./assets/hash_url.gif)

#### Flowchart

![url-shortener](./assets/hash_url.jpg)

### 2. **URL Redirection Function**

path: `/<short_url>`

- Automatic redirection from short URL to original URL
- Handles invalid URL cases (displays 404 page)
- Supports [308 permanent redirect](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/308)

![redirect_url.gif](./assets/redirect_url.gif)

#### Flowchart

![url-redirect](./assets/redirect_url.jpg)

### 3. **User System**


path: `/login`, `/register`

- User login

![login](./assets/login.png)

- User registration

![register](./assets/register.png)

#### Flowchart

![user-login](./assets/login.jpg)

### 4. **User Dashboard (require login)**

path: `/account`

- View all personally created short URLs
- Manage short URL records
- Track short URL usage

![account_console](./assets/account_console.jpg)

## Technical Architecture

### Frontend

- Tech Stack:
  - [SvelteKit](https://kit.svelte.dev/) as frontend framework
  - [Tailwind CSS](https://tailwindcss.com/) for style management
  - [TypeScript](https://www.typescriptlang.org/)

### Backend

- Tech Stack:
  - [Rust](https://www.rust-lang.org/) programming language
  - [Axum](https://docs.rs/axum/latest/axum/) as Web framework
  - [SeaORM](https://www.sea-ql.org/SeaORM/) as ORM and database migration tool
  - [PostgreSQL](https://www.postgresql.org/) as database
  - [JWT](https://jwt.io/) authentication
  - [Swagger](https://swagger.io/) API documentation

- Database Schema:

### Database Schema

| Table Name       | Description                    |
|-------------------|--------------------------------|
| `users`          | Stores user information.       |
| `urls`           | Stores original and short URLs.|
| `request_records`| Logs URL access details.       |

![database schema](./assets/schema.jpg)

### File Descriptions

```text
frontend/src/
├── lib/                # Shared components and utility functions
│   ├── components/     # Reusable UI components
│   ├── stores/         # Svelte stores for state management
│   └── utils/          # Utility functions
│
├── routes/             # Page routing components
│   ├── +page.svelte    # Homepage component
│   ├── +layout.svelte  # Global layout component
│   ├── account/        # User account dashboard (manage short urls, view click statistics)
│   │   └── +page.svelte
│   ├── login/          # Login related
│   │   └── +page.svelte
│   └── register/       # Registration related
│       └── +page.svelte
│
└── app.html            # Application HTML template
```

```text
backend/
   ├── src/
      ├── application/     # Application settings and state management
      ├── configuration/    # Configuration file handling
      ├── domain/          # Domain models
      ├── entity/          # Database entities
      ├── handlers/        # API handlers
      ├── error/           # Error handling
      ├── http/            # HTTP related definitions
      ├── telemetry/       # Monitoring and logging
      └── utility/         # Utility functions
   ├── configuration/       # Environment configuration files
   ├── migrations/         # Database migration scripts
```

### URL Shortening Algorithm

1. Combine user ID (empty if not logged in) and original URL into a string
2. Apply SHA-256 hash function to the combined string
3. Take the first 8 bytes of the hash result and convert to a 64-bit integer
4. Convert the integer to a short URL string using Base62 encoding

### Technical Details

- Using SHA-256 ensures uniqueness and distribution of output
- Using first 8 bytes allows for 2^64 possible combinations
- Base62 encoding uses character set [0-9a-zA-Z] to generate human-readable short URLs

### Security Considerations

- Including user ID prevents different users from generating identical short URLs
- SHA-256's collision resistance reduces the probability of duplicate short URLs

### Example

```sh
Input: user_id = "123", url = "https://example.com"
Combined: "123https://example.com"
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
First 8 bytes: e3b0c44298fc1c14
Base62 encoding: "7cWZX1Y5"
Final short URL: "https://shor.cc/7cWZX1Y5"
```

## How to Start

Ensure Docker Engine is installed

```bash
docker compose up -d
```

- visit `http://localhost:5173` to use the frontend service
- visit `http://localhost:3000` to use the backend service
- visit `http://localhost:3000/swagger-ui/#` to use the swagger api documentation

## DB Migration

1. Ensure Rust and Cargo are installed, refer to [Rust & Cargo Installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Enter backend directory

```bash
cd backend
```

3. Install sea-orm-cli

```bash
cargo install sea-orm-cli
```

4. Enter `backend/migrations` directory

```bash
cd migrations
```

5. Execute postgres database migration

```bash
sea-orm-cli migrate fresh -u postgres://postgres:postgres@localhost:5432/postgres
```

### Lessons Learned

This project has been quite a journey! I started learning Rust as a complete beginner, spending countless hours with [the Rust Book](https://doc.rust-lang.org/book/) and figuring out how to build APIs with [Axum](https://github.com/tokio-rs/axum). Getting my head around SeaORM for database operations was challenging, but it was so satisfying when things finally clicked!

Adding Swagger API documentation with [OpenAPI](https://www.openapis.org/) specification was surprisingly straightforward and really helped in organizing my API endpoints. Having some prior experience with Docker made setting up PostgreSQL much smoother than I expected.

The frontend side was another exciting challenge - I went through the entire SvelteKit [tutorial](https://svelte.dev/tutorial/svelte/welcome-to-svelte) and loved how intuitive it felt compared to other frameworks I've tried. I even dipped my toes into [Terraform](https://www.terraform.io/) for frontend and backend deployment (though I decided not to make it public due to cloud costs and security concerns).

When I started this CS50 final project, I set some pretty ambitious goals for myself. There were definitely moments of frustration and countless Stack Overflow searches, but I'm really proud of how far I've come.

I couldn't have done any of this without the amazing open-source community and all the resources they've made available. Their work made this project possible, and I'm incredibly grateful for that.

Thank you!
