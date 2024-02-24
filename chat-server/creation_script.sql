CREATE TABLE public.public_user
(
    id bigint,
    ipaddr character varying(60) NOT NULL,
    name character varying(30) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.public_user
    OWNER to postgres;


CREATE TABLE public.private_user
(
    id bigint,
    name character varying(60) NOT NULL,
    password text NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.private_user
    OWNER to postgres;


CREATE TABLE public.public_room
(
    id bigint,
    name character varying(150) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.public_room
    OWNER to postgres;


CREATE TABLE public.private_room
(
    id bigint,
    name character varying(150) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.private_room
    OWNER to postgres;


CREATE TABLE public.private_room_user_distro
(
    room_id bigint,
    user_id bigint,
    PRIMARY KEY (room_id, user_id),
    CONSTRAINT fk_private_room_user_id FOREIGN KEY (user_id)
        REFERENCES public.private_user (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT fk_private_room_room_id FOREIGN KEY (room_id)
        REFERENCES public.private_room (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

ALTER TABLE IF EXISTS public.private_room_user_distro
    OWNER to postgres;


CREATE TABLE public.public_message
(
    id bigint,
    user_id bigint NOT NULL,
    room_id bigint NOT NULL,
    datetime timestamp without time zone NOT NULL,
    text text NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_public_message_user FOREIGN KEY (user_id)
        REFERENCES public.public_user (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT fk_public_message_room FOREIGN KEY (room_id)
        REFERENCES public.public_room (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

ALTER TABLE IF EXISTS public.public_message
    OWNER to postgres;


CREATE TYPE public.file_type AS ENUM
    ('IMAGE', 'VIDEO', 'AUDIO', 'DOCUMENT', 'MISC');

ALTER TYPE public.file_type
    OWNER TO postgres;


CREATE TABLE public.file
(
    id bigint,
    original_name character varying(255) NOT NULL,
    path character varying(255) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.file
    OWNER to postgres;


CREATE TABLE public.private_message
(
    id bigint,
    user_id bigint NOT NULL,
    room_id bigint NOT NULL,
    datetime timestamp without time zone NOT NULL,
    text text,
    file_id bigint,
    PRIMARY KEY (id),
    CONSTRAINT uq_private_message_file UNIQUE (file_id),
    CONSTRAINT fk_private_message_user FOREIGN KEY (user_id)
        REFERENCES public.private_user (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT fk_private_message_room FOREIGN KEY (room_id)
        REFERENCES public.private_room (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT fk_private_message_file FOREIGN KEY (file_id)
        REFERENCES public.file (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT check_text_or_file CHECK ((text != NULL) AND (file_id != NULL)) NOT VALID
);

ALTER TABLE IF EXISTS public.private_message
    OWNER to postgres;

ALTER TABLE IF EXISTS public.public_user
ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 );

ALTER TABLE IF EXISTS public.public_room
    ADD CONSTRAINT uq_room_name UNIQUE (name);

ALTER TABLE IF EXISTS public.public_user
    ADD COLUMN join_datetime timestamp without time zone NOT NULL;

ALTER TABLE IF EXISTS public.public_user
    ADD COLUMN close_datetime timestamp without time zone;

ALTER TABLE IF EXISTS public.public_message
ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 );

INSERT INTO public.public_room(id, name)
    VALUES (1, 'general');

ALTER TABLE IF EXISTS public.private_user
    ADD CONSTRAINT uq_private_user_name UNIQUE (name);