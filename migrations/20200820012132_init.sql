-- Invite Codes
create table invite_codes (
    id varchar(10) primary key
);

insert into invite_codes (id) values ('aho');

-- Users
create table users (
    user_id serial primary key,
    avatar text,
    email text not null unique,
    password text not null,
    invite_code varchar not null unique references invite_codes(id),
    username varchar(40) not null unique,
    taunt text,
    verified boolean default false,
    created_at timestamp not null default current_timestamp
);

-- Group
create table groups (
    group_id serial primary key,
    name varchar(40) not null,
    description text,
    avatar text,
    created_at timestamp not null default current_timestamp
);

-- Group Members
create table groups_members (
    group_id integer not null references groups(group_id) on delete cascade,
    user_id integer not null references users(user_id),
    role varchar(40),
    created_at timestamp not null default current_timestamp,
    primary key (group_id, user_id)
);


-- Friends
create table friends (
    user_id integer not null references users(user_id),
    friend_id integer not null references users(user_id),
    friend_status text not null, -- friend, requests, blocked
    created_at timestamp not null default current_timestamp,
    primary key (user_id, friend_id)
);

-- Tags
create table tags (
    tag_id serial primary key,
    group_id integer not null references groups(group_id),
    name text not null,
    created_at timestamp not null default current_timestamp
);

-- Events
create table events (
    event_id serial primary key,
    group_id integer not null references groups(group_id),
    name text not null,
    description text,
    event_type text not null, -- individual or team
    creator_user_id integer not null references users(user_id),
    created_at timestamp not null default current_timestamp
);

-- Events Tags
create table events_tags (
    event_tag_id serial primary key,
    event_id integer not null references events(event_id),
    tag_id integer not null references tags(tag_id)
);

-- Events Teams
create table events_teams (
    event_team_id serial primary key,
    event_id integer not null references events(event_id),
    name text not null,
    score integer not null default 0,
    winner boolean not null default false,
    created_at timestamp not null default current_timestamp
);

-- Event Team Members
create table events_teams_members (
    event_team_member_id serial primary key,
    event_team_id integer not null references events_teams(event_team_id),
    user_id integer not null references users(user_id),
    created_at timestamp not null default current_timestamp
);



