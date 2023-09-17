-- Add migration script here
-- Add up migration script here
create extension if not exists "uuid-ossp";

create extension if not exists "postgis";

-- User
create type role as enum ('user', 'superuser', 'admin', 'moderator');

create table "user" (
    user_id uuid primary key not null default (uuid_generate_v4()),
    username varchar(100) not null unique,
    email text not null,
    password_hash text not null,
    role role default 'user',
    updated_at timestamp with time zone default now()
);

create table if not exists session (
    session_token BYTEA PRIMARY KEY,
    user_id uuid references "user"(user_id) on delete cascade
);


-- Bike
create table "bike" (
    bike_id uuid primary key not null default (uuid_generate_v4()),
    status varchar(50) not null default 'available', -- Possible statuses: available, rented, maintenance, etc.
    -- location geometry(Point, 4326), -- Storing the current location of the bike using PostGIS extension for spatial data
    last_maintenance_date date,
    updated_at timestamp with time zone default now()
);

-- Rental Session
create table "rental_session" (
    rental_session_id uuid primary key not null default (uuid_generate_v4()),
    user_id uuid references "user"(user_id) on delete set null,
    bike_id uuid references "bike"(bike_id) on delete set null,
    start_time timestamp with time zone not null default now(),
    end_time timestamp with time zone,
    start_location geometry(Point, 4326),
    end_location geometry(Point, 4326),
    updated_at timestamp with time zone default now()
);

-- Payment
create table "payment" (
    payment_id uuid primary key not null default (uuid_generate_v4()),
    rental_session_id uuid references "rental_session"(rental_session_id) on delete cascade,
    amount decimal(10,2) not null,
    payment_method varchar(50) not null, -- Possible methods: credit_card, paypal, etc.
    payment_status varchar(50) not null default 'pending', -- Possible statuses: pending, completed, failed, etc.
    payment_date timestamp with time zone not null default now(),
    updated_at timestamp with time zone default now()
);

-- Maintenance
create table "maintenance" (
    maintenance_id uuid primary key not null default (uuid_generate_v4()),
    bike_id uuid references "bike"(bike_id) on delete set null,
    maintenance_type varchar(100), -- Possible types: repair, inspection, cleaning, etc.
    maintenance_status varchar(50) not null default 'pending', -- Possible statuses: pending, completed, in-progress, etc.
    maintenance_date date,
    notes text,
    updated_at timestamp with time zone default now()
);

-- Add indexes for commonly queried fields to improve query performance
create index idx_bike_status on "bike"(status);
create index idx_rental_session_start_time on "rental_session"(start_time);
create index idx_payment_status on "payment"(payment_status);
create index idx_maintenance_status on "maintenance"(maintenance_status);
