-- Add migration script here
create table if not exists companies(
    id integer primary key,
    corporation_type varchar(10),
    listing_type varchar(10),
    linking_type varchar(10),
    setting_day varchar(10),
    edinet_code varchar(10) unique,
    sec_code integer,
    jcn bigint unique,
    capital integer,
    location varchar(100),
    industry_type varchar(20),
    name varchar(100) unique,
    name_eng varchar(100) unique,
    name_kana varchar(100) unique
);