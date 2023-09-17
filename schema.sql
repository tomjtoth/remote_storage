create table if not exists strings(
    id integer primary key,
    str text unique on conflict ignore
);

create table if not exists junction(
    url_id integer references strings(id),
    key_id integer references strings(id),
    val_id integer references strings(id)
);

create view if not exists data as
select 
    u.str as url,
    k.str as key,
    v.str as val
from junction j
inner join strings u on u.id == j.url_id
inner join strings k on k.id == j.key_id
inner join strings v on v.id == j.val_id
;

create trigger if not exists inserter
instead of insert on data
begin
    insert into strings(str) values
        (new.url),
        (new.key),
        (new.val);

    insert into junction
    select 
        (select id from strings where str == new.url),
        (select id from strings where str == new.key),
        (select id from strings where str == new.val);
end
;