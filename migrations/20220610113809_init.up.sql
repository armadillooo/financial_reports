-- Add up migration script here
create table if not exists users(
    id varchar(50) not null,
    username varchar(50) not null,
    email varchar(50) unique not null,
    primary key (id)
);

create table if not exists companies(
    stock_id varchar(10) not null,
    name varchar(100) unique not null,
    sector varchar(50) not null,
    industry varchar(50) not null,
    primary key (stock_id)
);

create table if not exists stocks(
    stock_id varchar(10) not null,
    date date not null,
    volume integer not null,
    start_price integer not null,
    end_price integer not null,
    high_price integer not null,
    low_price integer not null,
    foreign key (stock_id) references companies(stock_id) on delete cascade,
    primary key (stock_id, date)
);

create table if not exists favorites(
    user_id varchar(50) not null,
    stock_id varchar(10) not null,
    foreign key (user_id) references users(id) on delete cascade,
    foreign key (stock_id) references companies(stock_id) on delete cascade,
    primary key (user_id, stock_id)
);

create table if not exists portfolio(
    user_id varchar(50) not null,
    stock_id varchar(10) not null,
    stock_count integer not null,
    purchase integer not null,
    foreign key (user_id) references users(id) on delete cascade,
    foreign key (stock_id) references companies(stock_id) on delete cascade,
    primary key (user_id, stock_id)
);