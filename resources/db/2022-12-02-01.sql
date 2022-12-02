create table version
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    version     varchar(20) NOT NULL UNIQUE
);

insert into `version` (`version`) values ('2022-09-21-00');

create table goods
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    qrcode      varchar(100) NOT NULL UNIQUE,
    name        varchar(100),
    cover       varchar(100),
    price       decimal(8, 2),
    unit        varchar(20)
);

insert into `goods` (`cover`, `create_time`, `id`, `name`, `price`, `qrcode`, `update_time`, `unit`) values ('/store/58f0af678f83a6f14718ef8c0b4a156fc5405ba399cf64f829b781c834ec80a6.jpg', '2022-08-24 17:00:54', 2, '小熊软糖', '1.50', 'T1234567890', '2022-09-14 15:38:26', '包');
insert into `goods` (`cover`, `create_time`, `id`, `name`, `price`, `qrcode`, `update_time`, `unit`) values (NULL, '2022-08-30 21:22:46', 9, '混合坚果A', '198.00', '6971924870274', '2022-08-30 21:22:46', '盒');
insert into `goods` (`cover`, `create_time`, `id`, `name`, `price`, `qrcode`, `update_time`, `unit`) values (NULL, '2022-08-30 21:24:38', 10, '多加衣垃圾袋', '5.20', '6971981175688', '2022-08-30 21:24:38', '5袋');
insert into `goods` (`cover`, `create_time`, `id`, `name`, `price`, `qrcode`, `update_time`, `unit`) values (NULL, '2022-08-30 21:25:14', 11, '云南白药创可贴', '21.60', '81770082194646059012', '2022-08-30 21:25:14', '盒');
insert into `goods` (`cover`, `create_time`, `id`, `name`, `price`, `qrcode`, `update_time`, `unit`) values (NULL, '2022-08-30 21:26:53', 12, '迪迪仕保温壶', '69.00', '6940018681553', '2022-08-30 21:26:53', '个');
