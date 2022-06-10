-- Add up migration script here
create table if not exists users(
    id serial,
    username varchar(30) not null,
    email varchar(50) unique not null,
    password varchar(100) not null,
    primary key (id)
);

create table if not exists companies(
    corporation_type varchar(10) not null,
    listing_type varchar(10) not null,
    linking_type varchar(10) not null,
    setting_day varchar(10) not null,
    edinet_code varchar(10) unique not null,
    sec_code integer,
    jcn bigint unique not null,
    capital integer not null,
    location varchar(100),
    industry_type varchar(20) not null,
    name varchar(100) unique not null,
    name_eng varchar(100) unique not null,
    name_kana varchar(100) unique not null,
    primary key (sec_code)
);